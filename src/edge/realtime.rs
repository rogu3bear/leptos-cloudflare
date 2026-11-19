//! The template WebSocket lane.
//!
//! `upgrade` answers every WebSocket upgrade for `/realtime/socket`, and it is
//! the only function that does. The template body is a capability check. To
//! adopt shared state, replace the body with a forward to a Durable Object;
//! see `patterns/realtime-durable-object`.

use super::route::REALTIME_SOCKET_PATH;
use serde::Serialize;

const NORMAL_CLOSURE: u16 = 1000;

#[derive(Serialize)]
struct Ready {
    #[serde(rename = "type")]
    kind: &'static str,
    transport: &'static str,
    route: &'static str,
    note: &'static str,
}

// `async` and the unused arguments are the extension contract: an adopted
// handler authenticates `req` and awaits a Durable Object from `env`.
pub(super) async fn upgrade(
    _req: worker::Request,
    _env: &worker::Env,
) -> worker::Result<worker::Response> {
    let worker::WebSocketPair { client, server } = worker::WebSocketPair::new()?;
    server.accept()?;
    server.send(&Ready {
        kind: "ready",
        transport: "websocket",
        route: REALTIME_SOCKET_PATH,
        note: "Template endpoint only. Use Durable Objects for rooms, presence, collaboration, fanout, or long-lived state.",
    })?;
    server.close(
        Some(NORMAL_CLOSURE),
        Some("template capability check complete"),
    )?;
    worker::Response::from_websocket(client)
}
