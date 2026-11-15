mod api;
mod app;
#[cfg(feature = "ssr")]
mod asset_hashes;
mod components;
#[cfg(any(feature = "ssr", test))]
mod contact;
mod contact_fields;
#[cfg(any(feature = "ssr", test))]
mod edge;
#[cfg(feature = "ssr")]
mod server;
#[cfg(any(feature = "ssr", test))]
mod telemetry;
mod todos;

#[cfg(feature = "ssr")]
#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    let observation = telemetry::RequestObservation::start(req.uri().path(), req.method().as_str());
    let result = edge::handle(req, env).await;
    observation.finish(
        result
            .as_ref()
            .ok()
            .map(|response| response.status().as_u16()),
        result.is_err(),
    );
    result
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
