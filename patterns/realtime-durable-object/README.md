# Realtime Durable Object

**Problem**: You need `/realtime/socket` to coordinate shared realtime state across clients without putting room state in the Leptos component tree, request-local Worker memory, or the core starter.

## Why This Matters on Cloudflare + Leptos

- The Rust Worker entry (`src/edge/entry.rs`) sees WebSocket upgrades before Leptos routing or hydration.
- Workers are request-scoped; shared connection state needs a coordination owner.
- Durable Objects give one single-threaded coordination point per room, document, tenant, or other stable key.
- Durable Object WebSocket hibernation keeps sockets connected without pinning the object in memory during idle periods.

The core template stays minimal by keeping `/realtime/socket` as a capability endpoint. Adopt this pattern when the route needs rooms, presence, collaboration, fanout, reconnect state, or any state shared across clients.

## Example Files

- `realtime.example.rs` is `src/edge/realtime.rs` after adoption. Its `upgrade` function authorizes the request, derives a room key, and forwards the original upgrade request to `env.durable_object("REALTIME_ROOM")?.get_by_name(&room)?.fetch_with_request(req)`. The same file defines the `RealtimeRoom` Durable Object with `#[durable_object]` from the `worker` crate.
- `wrangler.durable-object.example.toml` shows the Durable Object binding and SQLite-backed migration block to add to `wrangler.toml`.

`upgrade` in `src/edge/realtime.rs` is the one extension point: the Worker entry calls it for every WebSocket upgrade on `/realtime/socket` and for nothing else. To adopt the pattern, replace the contents of that file with the example and keep the function's signature. Routing, asset, API-body and telemetry handling in `src/edge/` stay as they are.

Do not edit `build/_worker.js`. It is generated, handles no request, and re-exports everything the workers-rs module exports, so the `RealtimeRoom` class reaches Wrangler without a build-script change. The example is not compiled by the template's gates; compile it in the adopting application with `cargo check --features ssr`.

## Core Pattern

### 1. Worker owns the route decision

Keep `/realtime/socket` as the only public realtime route. The Worker should do cheap request checks before handoff:

- verify `Upgrade: websocket`
- authenticate or authorize the browser session
- derive a stable room/document/tenant key
- call the Durable Object stub for that key

```rust
let room = room_name(&req)?;
env.durable_object("REALTIME_ROOM")?
    .get_by_name(&room)?
    .fetch_with_request(req)
    .await
```

### 2. Durable Object owns shared state

The object accepts the server side of the `WebSocketPair` with `state.accept_websocket_with_tags(&server, &[&room])`, then handles messages in `websocket_message`. Persist critical message or presence events before broadcasting.

```rust
let stored: StoredMessage = self.state.storage().sql().exec(
    "INSERT INTO messages (id, text, sent_at) VALUES (lower(hex(randomblob(16))), ?, ?) RETURNING id",
    vec![text.into(), sent_at.as_str().into()],
)?.one()?;

for socket in self.state.get_websockets_with_tag(&room) {
    let _ = socket.send(&event);
}
```

### 3. Shard by coordination atom

Use one object per logical room or document:

```rust
env.durable_object("REALTIME_ROOM")?.get_by_name(&format!("room:{room_name}"))?
```

Do not send all realtime traffic to one global object. That creates an unnecessary bottleneck and makes authorization boundaries unclear.

## Wrangler Binding

Add the binding only when the application adopts the pattern:

```toml
[[durable_objects.bindings]]
name = "REALTIME_ROOM"
class_name = "RealtimeRoom"

[[migrations]]
tag = "v1_realtime_room"
new_sqlite_classes = ["RealtimeRoom"]
```

The core template intentionally does not include this binding.

## Security and Production Notes

- Replace the example `authorize` placeholder before production use.
- Treat the room name as an authorization input, not just a routing input.
- Keep message size limits small and explicit.
- Persist important events before fanout; in-memory WebSocket sets can be rebuilt after hibernation, but in-memory business state is not durable.
- Close unauthorized sockets before handoff instead of letting the Durable Object infer identity later.
- Prefer Durable Object SQLite storage for per-room history or presence snapshots. Use D1 only when data must be queried across rooms.

## Local Proof

After adopting the pattern into the core runtime, run:

```bash
cargo test --features ssr
bash ./scripts/build-edge.sh
bun ./scripts/test-worker-boundaries.mjs
./scripts/wrangler.sh deploy --dry-run
```

The boundary test opens `/realtime/socket` against a local runtime. Its template assertions (a `ready` message followed by a normal close) describe the capability endpoint, so update them to the adopted room behavior in the same change. The test writes its own Wrangler configuration in `scripts/local-preview.mjs` with only `ASSETS` and D1; add the `REALTIME_ROOM` binding and migration there too, or the adopted `upgrade` fails on both the Workers and Pages runs.

Before claiming the repository is release-ready, run the full gate:

```bash
./scripts/verify.sh
```

This example follows Cloudflare's Durable Object WebSocket hibernation API: the Worker routes the upgrade, and the Durable Object accepts the server socket through its `State` and handles messages with `websocket_message`.
