//! Custom trace span around the application boundary.
//!
//! workers-rs has no tracing API, so this calls `ctx.tracing.enterSpan` on the
//! execution context directly. A runtime without it runs the work unchanged.

use std::{cell::RefCell, future::Future, rc::Rc};
use worker::{
    js_sys::{Function, Promise, Reflect},
    wasm_bindgen::{closure::Closure, JsCast, JsValue},
    wasm_bindgen_futures::{future_to_promise, JsFuture},
    web_sys,
};

const SPAN_SCHEMA_VERSION: u8 = 1;

/// Run `work` inside a `leptos.<boundary>` span. `boundary` is one of the
/// closed telemetry labels; nothing request-specific is attached.
pub(super) async fn in_span<F>(
    ctx: &worker::Context,
    boundary: &'static str,
    work: F,
) -> worker::Result<web_sys::Response>
where
    F: Future<Output = worker::Result<web_sys::Response>> + 'static,
{
    let Some((tracing, enter_span)) = enter_span_function(ctx) else {
        return work.await;
    };

    // The span only observes settlement. The result travels beside it so an
    // application error keeps its Rust type and the runtime's error handling.
    let outcome = Rc::new(RefCell::new(None));
    let callback = Closure::once_into_js({
        let outcome = Rc::clone(&outcome);
        move |span: JsValue| -> Promise {
            set_attribute(
                &span,
                "app.telemetry.schema_version",
                &JsValue::from(SPAN_SCHEMA_VERSION),
            );
            set_attribute(&span, "app.boundary", &JsValue::from_str(boundary));
            future_to_promise(async move {
                let result = work.await;
                *outcome.borrow_mut() = Some(result);
                Ok(JsValue::UNDEFINED)
            })
        }
    });

    let name = JsValue::from_str(&format!("leptos.{boundary}"));
    let settled = enter_span.call2(&tracing, &name, &callback)?;
    JsFuture::from(Promise::resolve(&settled)).await?;
    let result = outcome.borrow_mut().take();
    result.unwrap_or_else(|| {
        Err(worker::Error::RustError(
            "the tracing span settled without running the request".to_owned(),
        ))
    })
}

fn enter_span_function(ctx: &worker::Context) -> Option<(JsValue, Function)> {
    let context: &worker::worker_sys::Context = ctx.as_ref();
    let tracing = Reflect::get(context, &JsValue::from_str("tracing")).ok()?;
    if tracing.is_undefined() || tracing.is_null() {
        return None;
    }
    let enter_span = Reflect::get(&tracing, &JsValue::from_str("enterSpan"))
        .ok()?
        .dyn_into::<Function>()
        .ok()?;
    Some((tracing, enter_span))
}

fn set_attribute(span: &JsValue, key: &str, value: &JsValue) {
    let Some(set) = Reflect::get(span, &JsValue::from_str("setAttribute"))
        .ok()
        .and_then(|set| set.dyn_into::<Function>().ok())
    else {
        return;
    };
    // Tracing is advisory: a span that rejects an attribute must not fail the request.
    let _ = set.call2(span, &JsValue::from_str(key), value);
}
