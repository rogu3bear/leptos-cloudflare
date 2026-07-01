# Contributing

Thanks for helping improve `leptos-cf`.

## Development Setup

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
./scripts/bootstrap.sh
```

## Expected Checks

Run the full release readiness verification before opening a pull request:

```bash
./scripts/verify.sh
```

This is the authoritative local release gate. For quick iteration on small changes you can use the lighter protocol described in `docs/agent-playbook.md`.

`wrangler.toml` contains placeholder D1 IDs until a new project initializes the template. The dependency check warning for those placeholders is expected in the template repo.

## Change Boundaries

- Keep generated output out of git.
- Keep docs and verification scripts aligned with runtime behavior.
- Prefer small, explicit examples over hidden magic.
- Do not add real service IDs, tokens, or personal environment values to examples.
- If you add a new Cloudflare binding, document the binding, local setup, deploy proof, and failure mode.

## Realtime Features

Read `docs/realtime.md` before adding WebSocket behavior. Request-scoped capability checks may stay in the Worker. Shared state, rooms, collaboration, presence, fanout, and reconnect state should use Durable Objects.
