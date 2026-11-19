# Realtime and WebSockets

For a newly named application, follow [Adopting the starter](adopting.md).
Provider credentials and the governed/standalone distinction are in
[Credential profiles](credentials.md). Runtime invariants remain required
after replacing field-guide pages or labels.

This template includes one explicit WebSocket lane so production agents do not have to infer how realtime traffic should enter the app.

## Routing Contract

The generated `build/_worker.js` is the Cloudflare entrypoint. It handles no request: it re-exports the workers-rs module, whose Rust `fetch` handler (`src/lib.rs`, `src/edge/entry.rs`) is the first code to run. The handler routes requests in this order (`src/edge/route.rs`):

1. WebSocket upgrades for `/realtime/socket` go to the template WebSocket capability endpoint.
2. WebSocket upgrades for any other path return `404`.
3. Plain HTTP requests to `/realtime/socket` return `426 Upgrade Required`.
4. Static assets, including app icons and the web manifest, go to `env.ASSETS.fetch(request)`.
5. Everything else goes to the Leptos handler for SSR, deep routes, and server functions.

On Pages, requests for the private server modules (`/_server`, `/functions`) return `404` before any of these steps.

Only step 5 enters the application telemetry contract. The entry creates a custom span with the closed boundary `ssr` or `server_function`; the Rust handler emits one closed, versioned completion event. Asset names and WebSocket route material therefore do not leak into that contract, and no raw URL, query, identifier, header, cookie, body, IP/user-agent value, D1 identity, or internal error text is logged.

Do not put WebSocket handling behind Leptos client navigation. A browser may do a full document request before hydration, and Cloudflare decides whether the Worker sees the request before any Rust or Leptos router code runs.

## Template Endpoint

`/realtime/socket` is a capability check, not an application feature. It accepts the upgrade, sends a JSON `ready` message, and closes cleanly.

Use it to verify that:

- Cloudflare routes WebSocket upgrades to the Worker.
- The Worker entry keeps WebSocket traffic out of static assets.
- CSP allows `ws:` and `wss:` through `connect-src`.

## Production Rule

Use the template endpoint only for simple request-scoped upgrades and local capability proof.

Use Durable Objects for:

- rooms
- chat
- presence
- collaboration
- fanout
- reconnect state
- long-lived coordination
- any shared state across clients

The durable pattern is one Durable Object per room, document, tenant, or other coordination key. The Worker should authenticate the request, derive the object ID, forward the WebSocket upgrade to that object, and let the object own connection state.

## Shared-State Pattern

Use [Realtime Durable Object](../patterns/realtime-durable-object/) when `/realtime/socket` needs shared state. `upgrade` in `src/edge/realtime.rs` is the one function the entry calls for an upgrade on that route; the example replaces its body and keeps the core route contract, changing the implementation from "accept and close" to:

1. Worker validates the WebSocket upgrade and session.
2. Worker derives a stable room/document key.
3. Worker calls `env.durable_object("REALTIME_ROOM")?.get_by_name(&key)?.fetch_with_request(req)`.
4. Durable Object accepts the server socket with `state.accept_websocket_with_tags(&server, &[&room])`.
5. Durable Object persists important events before broadcasting to `state.get_websockets_with_tag(&room)`.

The required Wrangler binding and migration are documented in `patterns/realtime-durable-object/wrangler.durable-object.example.toml`. Do not add that binding to the core template unless the application adopts the pattern.

## Agent Checklist

Before adding realtime behavior, answer these in the PR or implementation notes:

- What is the WebSocket route?
- Is this request-scoped or shared state?
- If shared state exists, which Durable Object owns it?
- What authenticates the upgrade?
- What closes idle or unauthorized sockets?
- What happens on reconnect?
- Which command proves the route still builds and the WebSocket lane still answers?

Minimum local proof:

```bash
cargo test --features ssr
bash ./scripts/build-edge.sh
bun ./scripts/test-worker-boundaries.mjs
./scripts/wrangler.sh deploy --dry-run
```


Pages advanced mode packages the same Worker module behind its own generated `_worker.js` and sends every request to it. Its local preview must independently prove the upgrade and HTTP boundaries; shared-state guidance above applies to both targets. See [Pages setup](edge-deployment.md#pages-option).
