# Realtime Durable Object

**Problem**: You need `/realtime/socket` to coordinate shared realtime state across clients without putting room state in the Leptos component tree, request-local Worker memory, or the core starter.

## Why This Matters on Cloudflare + Leptos

- The generated `build/_worker.js` sees WebSocket upgrades before Leptos routing or hydration.
- Workers are request-scoped; shared connection state needs a coordination owner.
- Durable Objects give one single-threaded coordination point per room, document, tenant, or other stable key.
- Durable Object WebSocket hibernation keeps sockets connected without pinning the object in memory during idle periods.

The core template stays minimal by keeping `/realtime/socket` as a capability endpoint. Adopt this pattern when the route needs rooms, presence, collaboration, fanout, reconnect state, or any state shared across clients.

## Example Files

- `worker-shim-with-durable-object.example.mjs` shows the generated Worker shim shape after adoption: `/realtime/socket` validates the upgrade, derives a room key, and forwards the original upgrade request to `env.REALTIME_ROOM.getByName(room).fetch(request)`.
- `wrangler.durable-object.example.toml` shows the Durable Object binding and SQLite-backed migration block to add to `wrangler.toml`.

Do not edit `build/_worker.js` directly. It is generated. Copy the handoff and `RealtimeRoom` export into `scripts/write-worker-shim.mjs` so the build keeps regenerating the same runtime contract.

## Core Pattern

### 1. Worker owns the route decision

Keep `/realtime/socket` as the only public realtime route. The Worker should do cheap request checks before handoff:

- verify `Upgrade: websocket`
- authenticate or authorize the browser session
- derive a stable room/document/tenant key
- call the Durable Object stub for that key

```js
const room = env.REALTIME_ROOM.getByName(realtimeRoomName(request));
return room.fetch(request);
```

### 2. Durable Object owns shared state

The object accepts the server side of the `WebSocketPair` with `this.ctx.acceptWebSocket(server)`, then handles messages in `webSocketMessage`. Persist critical message or presence events before broadcasting.

```js
this.ctx.storage.sql.exec(
  "INSERT INTO messages (id, text, sent_at) VALUES (?, ?, ?)",
  payload.id,
  payload.text,
  payload.sentAt,
);

this.broadcast(payload);
```

### 3. Shard by coordination atom

Use one object per logical room or document:

```js
env.REALTIME_ROOM.getByName(`room:${roomName}`)
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

- Replace the example `authorizeRealtimeRequest` placeholder before production use.
- Treat the room name as an authorization input, not just a routing input.
- Keep message size limits small and explicit.
- Persist important events before fanout; in-memory WebSocket sets can be rebuilt after hibernation, but in-memory business state is not durable.
- Close unauthorized sockets before handoff instead of letting the Durable Object infer identity later.
- Prefer Durable Object SQLite storage for per-room history or presence snapshots. Use D1 only when data must be queried across rooms.

## Local Proof

After adopting the pattern into the core runtime, run:

```bash
bash ./scripts/build-edge.sh
bun ./scripts/verify-worker-runtime.mjs
bunx wrangler@4.120.1 deploy --dry-run
```

Before claiming the repository is release-ready, run the full gate:

```bash
./scripts/verify.sh
```

This example follows Cloudflare's Durable Object WebSocket hibernation API: the Worker routes the upgrade, and the Durable Object calls `ctx.acceptWebSocket(server)` and handles messages with `webSocketMessage`.
