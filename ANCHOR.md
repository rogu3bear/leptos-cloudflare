# Anchor

This repo is a single-crate Leptos 0.8 starter for Cloudflare Workers.

Stable anchors:

- `wrangler.toml` is the Cloudflare deployment contract.
- `scripts/build-edge.sh` is the production build entrypoint.
- `build/_worker.js` is generated and must stay the Wrangler `main` entrypoint.
- `target/site` is the Workers Assets directory.
- `src/lib.rs` owns the Rust Worker/SSR handler.
- `src/app.rs` owns the Leptos route tree.
- `docs/realtime.md` owns the WebSocket/Durable Object decision contract.
- `docs/agent-playbook.md` owns the agent bootstrap and verification protocol.

Generated and local-only surfaces:

- `build/`, `target/`, `var/`, `.wrangler/`, `.env`, and `.dev.vars*` must not be committed.
- `.env.example` must contain placeholders only.

Release readiness means the commands in `RELEASE.md` pass from a fresh checkout with placeholder Cloudflare IDs, and any expected placeholder warnings are documented.
