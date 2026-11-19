//! The Worker's first request handler. Every request Cloudflare delivers
//! reaches `dispatch` before any other application code, Rust or JavaScript.

use super::{
    realtime,
    route::{self, EdgeManifest, Route},
    span,
};
use std::cell::OnceCell;
use worker::{
    js_sys::{self, Reflect},
    wasm_bindgen::{JsCast, JsValue},
    wasm_bindgen_futures::JsFuture,
    web_sys,
};

/// Set by the generated `_worker.js` before the first request.
const MANIFEST_GLOBAL: &str = "__LEPTOS_EDGE__";
const ASSETS_BINDING: &str = "ASSETS";

thread_local! {
    // One parse per WebAssembly instance. A re-instantiated module reads the
    // global again; the global itself outlives the instance.
    static MANIFEST: OnceCell<EdgeManifest> = const { OnceCell::new() };
}

pub(crate) async fn dispatch(
    req: worker::Request,
    env: worker::Env,
    ctx: worker::Context,
) -> worker::Result<web_sys::Response> {
    let url = req.url()?;
    let upgrade = req.headers().get("Upgrade")?;
    let route = with_manifest(|manifest| {
        route::decide(
            url.path(),
            route::is_websocket_upgrade(upgrade.as_deref()),
            req.inner().body().is_some(),
            manifest,
        )
    })?;

    match route {
        Route::PrivateModule => text(404, "Not found."),
        Route::UnknownSocket => text(404, "Unknown WebSocket route."),
        Route::UpgradeRequired => Ok(worker::ResponseBuilder::new()
            .with_status(426)
            .with_header("Upgrade", "websocket")?
            .ok("WebSocket upgrade required.")?
            .into()),
        Route::RealtimeSocket => Ok(realtime::upgrade(req, &env).await?.into()),
        Route::Asset => asset(&req, &env).await,
        Route::Application {
            alias,
            isolate_body,
        } => application(req, env, &ctx, alias, isolate_body).await,
    }
}

fn text(status: u16, body: &str) -> worker::Result<web_sys::Response> {
    Ok(worker::ResponseBuilder::new()
        .with_status(status)
        .ok(body)?
        .into())
}

/// Hand the request to the `ASSETS` binding and return its response object
/// unchanged, so validators, ranges and asset headers are the platform's own.
async fn asset(req: &worker::Request, env: &worker::Env) -> worker::Result<web_sys::Response> {
    let binding = Reflect::get(env, &JsValue::from_str(ASSETS_BINDING))?;
    if binding.is_undefined() || binding.is_null() {
        return Err(worker::Error::RustError(format!(
            "the {ASSETS_BINDING} binding is not configured"
        )));
    }
    let assets: worker::worker_sys::Fetcher = binding.unchecked_into();
    let response = JsFuture::from(assets.fetch(req.inner())?).await?;
    response.dyn_into().map_err(|_| {
        worker::Error::RustError(format!("{ASSETS_BINDING} did not return a Response"))
    })
}

async fn application(
    req: worker::Request,
    env: worker::Env,
    ctx: &worker::Context,
    alias: Option<String>,
    isolate_body: bool,
) -> worker::Result<web_sys::Response> {
    // Rust rejects declared over-limit bodies early and caps streamed bytes
    // before parsing; dropping a body cancels its stream. Reading through a
    // clone keeps that cancellation off the native request, which stays alive
    // until the response exists.
    let (forwarded, _native) = if isolate_body {
        (req.clone()?, Some(req))
    } else {
        (req, None)
    };
    let mut request = worker::HttpRequest::try_from(forwarded)?;
    if let Some(alias) = alias {
        *request.uri_mut() = route::with_path(request.uri(), &alias)?;
    }

    let observation = crate::telemetry::RequestObservation::start(
        request.uri().path(),
        request.method().as_str(),
    );
    span::in_span(ctx, observation.boundary_label(), async move {
        let result = super::handle(request, env).await;
        observation.finish(
            result
                .as_ref()
                .ok()
                .map(|response| response.status().as_u16()),
            result.is_err(),
        );
        worker::response_to_wasm(result?)
    })
    .await
}

fn with_manifest<T>(decide: impl FnOnce(&EdgeManifest) -> T) -> worker::Result<T> {
    MANIFEST.with(|cell| {
        if let Some(manifest) = cell.get() {
            return Ok(decide(manifest));
        }
        let manifest = read_manifest()?;
        let decision = decide(&manifest);
        // Only this thread-local writes the cell, so it is still empty.
        let _ = cell.set(manifest);
        Ok(decision)
    })
}

/// A missing or malformed manifest is an error for every request: the Pages
/// private-module denial must never be skipped because data failed to load.
fn read_manifest() -> worker::Result<EdgeManifest> {
    let value = Reflect::get(&js_sys::global(), &JsValue::from_str(MANIFEST_GLOBAL))?;
    let json = js_sys::JSON::stringify(&value)?
        .as_string()
        .ok_or_else(|| worker::Error::RustError(format!("{MANIFEST_GLOBAL} is not set")))?;
    EdgeManifest::from_json(&json)
        .map_err(|error| worker::Error::RustError(format!("{MANIFEST_GLOBAL} is invalid: {error}")))
}
