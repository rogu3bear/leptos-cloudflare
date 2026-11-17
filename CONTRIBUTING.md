# Contributing

Thanks for helping improve `leptos-cf`.

## Development Setup

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
./scripts/bootstrap.sh
```

## Expected Checks

Follow [Change and verify](README.md#change-and-verify) before opening a pull
request. It covers the required release baseline, full local gate and focused
checks.

`wrangler.toml` contains placeholder D1 IDs for local use. Project naming preserves them. Use the [Workers guide](docs/edge-deployment.md)
or the [Pages guide](docs/pages-production.md) to prepare production configuration.

## Change Boundaries

- Keep generated output out of git.
- Keep docs and verification scripts aligned with runtime behavior.
- Prefer small, explicit examples over hidden magic.
- Do not add real service IDs, tokens, or personal environment values to examples.
- If you add a new Cloudflare binding, document the binding, local setup, deploy proof, and failure mode.

## Realtime Features

Read `docs/realtime.md` before adding WebSocket behavior. Request-scoped capability checks may stay in the Worker. Shared state, rooms, collaboration, presence, fanout, and reconnect state should use Durable Objects.
