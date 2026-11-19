# Runtime ownership decisions

| Responsibility | Owner | Preserved contract |
|---|---|---|
| Shared request/asset routing | `src/edge/entry.rs`, `src/edge/route.rs` | Workers and Pages run one Rust routing implementation as the first request handler; deployment differences are data in the generated entry |
| Tool versions and local CLI | `scripts/toolchain.sh`, `scripts/wrangler.sh`, lockfiles | Pinned Wrangler, lockfile-matched wasm-bindgen and genuine inherited resource descriptors |
| Request security and SSR | `src/edge/` | Body/origin guards, CSP, session scope and SSR behavior |
| D1 access | `src/server/d1.rs` and domain modules | Database and session obtained together; prepared statements remain domain-owned |
| Worker-to-Leptos Send bridge | `src/server/mod.rs` | Server declarations share one bridge for non-Send Worker futures |
| Resource collection/detail UI | Existing domain components and hooks | Canonical records, conflict detection and retry remain explicit |

Test behavior at these boundaries, not the spelling of generated source.
Keep Workers entrypoint, Pages private-module separation, API clone isolation,
telemetry privacy and the explicit WebSocket lane when changing the implementation.
See [AGENTS.md](../AGENTS.md), [realtime](realtime.md) and [RELEASE.md](../RELEASE.md)
for the active operating and verification requirements.
