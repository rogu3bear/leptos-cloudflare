//! Example only. This is `src/edge/realtime.rs` after adopting shared state:
//! the Worker forwards each `/realtime/socket` upgrade to one Durable Object
//! per room, and the object owns the sockets and the message history.
//!
//! `upgrade` keeps the signature the Worker entry already calls, so nothing
//! else in `src/edge/` changes. `RealtimeRoom` reaches Wrangler through the
//! generated entry's `export *`; no build script changes either.

use serde::{Deserialize, Serialize};
use worker::{
    durable_object, js_sys, DurableObject, Env, Request, Response, ResponseBuilder, Result, State,
    WebSocket, WebSocketIncomingMessage, WebSocketPair,
};

const ROOM_BINDING: &str = "REALTIME_ROOM";
const DEFAULT_ROOM: &str = "lobby";
const MAX_ROOM_NAME_LENGTH: usize = 64;
const MAX_MESSAGE_LENGTH: usize = 1_000;

/// The Worker side. Routing has already established that this is a WebSocket
/// upgrade for `/realtime/socket`; authorize it, pick the room, hand it over.
pub(super) async fn upgrade(req: Request, env: &Env) -> Result<Response> {
    if !authorize(&req) {
        return Response::error("Unauthorized realtime connection.", 401);
    }
    let room = room_name(&req)?;
    env.durable_object(ROOM_BINDING)?
        .get_by_name(&room)?
        .fetch_with_request(req)
        .await
}

fn authorize(_req: &Request) -> bool {
    // Replace this placeholder with session/auth checks before production use.
    // Treat the requested room as an authorization input, not only a route.
    true
}

fn room_name(req: &Request) -> Result<String> {
    let url = req.url()?;
    let requested = url
        .query_pairs()
        .find_map(|(key, value)| (key == "room").then(|| value.into_owned()));
    Ok(format!("room:{}", normalize_room_name(requested.as_deref())))
}

fn normalize_room_name(value: Option<&str>) -> String {
    let mut name = String::new();
    for character in value.unwrap_or_default().trim().chars() {
        let character = character.to_ascii_lowercase();
        let allowed = character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, ':' | '_' | '-');
        let character = if allowed { character } else { '-' };
        if character == '-' && name.ends_with('-') {
            continue;
        }
        if name.len() == MAX_ROOM_NAME_LENGTH {
            break;
        }
        name.push(character);
    }
    if name.is_empty() {
        DEFAULT_ROOM.to_owned()
    } else {
        name
    }
}

fn now() -> String {
    String::from(js_sys::Date::new_0().to_iso_string())
}

#[derive(Serialize, Deserialize)]
struct Attachment {
    room: String,
    connected_at: String,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Event<'a> {
    Ready {
        room: &'a str,
        connected_at: &'a str,
        connections: usize,
    },
    Message {
        id: &'a str,
        room: &'a str,
        text: &'a str,
        sent_at: &'a str,
        connections: usize,
    },
    Error {
        error: String,
    },
}

#[derive(Deserialize)]
struct StoredMessage {
    id: String,
}

/// One object per room. Bind it as `REALTIME_ROOM` with a SQLite migration;
/// see `wrangler.durable-object.example.toml`.
#[durable_object]
pub struct RealtimeRoom {
    state: State,
}

impl DurableObject for RealtimeRoom {
    fn new(state: State, _env: Env) -> Self {
        // SQLite storage is synchronous, so the table exists before the first
        // event is delivered to this object.
        if let Err(error) = state.storage().sql().exec(
            "CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, text TEXT NOT NULL, sent_at TEXT NOT NULL);",
            None,
        ) {
            worker::console_error!("realtime room storage is unavailable: {error}");
        }
        Self { state }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        let upgrade = req.headers().get("Upgrade")?;
        if !upgrade.is_some_and(|value| value.eq_ignore_ascii_case("websocket")) {
            return ResponseBuilder::new()
                .with_status(426)
                .with_header("Upgrade", "websocket")?
                .ok("WebSocket upgrade required.");
        }

        let room = room_name(&req)?;
        let connected_at = now();
        let WebSocketPair { client, server } = WebSocketPair::new()?;
        // Hibernation API: the runtime keeps the socket while this object sleeps.
        self.state.accept_websocket_with_tags(&server, &[&room]);
        server.serialize_attachment(Attachment {
            room: room.clone(),
            connected_at: connected_at.clone(),
        })?;
        server.send(&Event::Ready {
            room: &room,
            connected_at: &connected_at,
            connections: self.state.get_websockets_with_tag(&room).len(),
        })?;
        Response::from_websocket(client)
    }

    async fn websocket_message(
        &self,
        ws: WebSocket,
        message: WebSocketIncomingMessage,
    ) -> Result<()> {
        let WebSocketIncomingMessage::String(message) = message else {
            return ws.send(&Event::Error {
                error: "Only text messages are supported.".to_owned(),
            });
        };
        let text = message.trim();
        if text.is_empty() || text.chars().count() > MAX_MESSAGE_LENGTH {
            return ws.send(&Event::Error {
                error: format!("Message must be 1-{MAX_MESSAGE_LENGTH} characters."),
            });
        }

        let room = ws
            .deserialize_attachment::<Attachment>()?
            .map_or_else(|| format!("room:{DEFAULT_ROOM}"), |attachment| attachment.room);
        let sent_at = now();
        // Persist before fanout: sockets are rebuilt after hibernation, history is not.
        let stored: StoredMessage = self
            .state
            .storage()
            .sql()
            .exec(
                "INSERT INTO messages (id, text, sent_at) VALUES (lower(hex(randomblob(16))), ?, ?) RETURNING id",
                vec![text.into(), sent_at.as_str().into()],
            )?
            .one()?;

        let sockets = self.state.get_websockets_with_tag(&room);
        let event = Event::Message {
            id: &stored.id,
            room: &room,
            text,
            sent_at: &sent_at,
            connections: sockets.len(),
        };
        for socket in sockets {
            // A socket that closed since the last event must not stop the fanout.
            let _ = socket.send(&event);
        }
        Ok(())
    }

    async fn websocket_close(
        &self,
        _ws: WebSocket,
        _code: usize,
        _reason: String,
        _was_clean: bool,
    ) -> Result<()> {
        // With this repository's compatibility date, Cloudflare replies to close frames itself.
        Ok(())
    }

    async fn websocket_error(&self, _ws: WebSocket, _error: worker::Error) -> Result<()> {
        // The attribute always exports this handler and the trait default panics,
        // so a socket error must be answered explicitly. The socket is already gone.
        Ok(())
    }
}
