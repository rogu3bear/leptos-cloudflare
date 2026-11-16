# Agent playbook

Start with [AGENTS.md](../AGENTS.md) and the [README](../README.md). Inspect Git
status and preserve existing work. Workers is the default unless the owner chose
Pages. Local work needs no Cloudflare credentials, cfctl or private instructions.

## Set up and run

```bash
./scripts/check-deps.sh
# If build tools are missing, after installing Rust/rustup, Bun and Node.js 22+:
cargo install cargo-audit --locked
./scripts/bootstrap.sh
```

A zero exit passes. `[warn]` lines about placeholder IDs are expected;
`[missing]` or a nonzero exit means a prerequisite is unresolved.
Use the README's local D1/build/preview commands. After adopting a name, derive
the database name from config rather than keeping a template constant:

```bash
LEPTOS_D1_NAME=$(bun -e 'console.log(Bun.TOML.parse(await Bun.file("wrangler.toml").text()).d1_databases[0].database_name)')
CI=1 ./scripts/wrangler.sh d1 migrations apply "$LEPTOS_D1_NAME" --local
bash ./scripts/build-edge.sh
./scripts/wrangler.sh dev --local --ip 127.0.0.1 --port 57581
```

For Pages, use the separate README example from `pages/`. Keep `pages/package.json`:
it prevents discovery of the parent Workers project. Package into a fresh output
directory and keep server modules private.

## Make a change at its owner

| Change | Source |
|---|---|
| Page or route | `src/components/`, exports in `src/components/mod.rs`, route in `src/app.rs` |
| Domain types and validation | `src/todos.rs`, `src/contact_fields.rs` |
| Typed server function | `src/api.rs`, delegating through `crate::server::run(...)` |
| D1 operation | `src/server/todos.rs`, `src/server/contact.rs`; register new modules in `src/server/mod.rs` |
| Request-scoped database and session | `src/server/state.rs`, `src/server/d1.rs` |
| Worker/SSR security boundary | `src/lib.rs`, `src/edge/`, `runtime/dispatch.mjs` |
| Styles | `style/main.css` |
| Schema | A new append-only `migrations/NNNN_name.sql` |
| Workers / Pages bindings | `wrangler.toml` / `pages/wrangler.toml`; keep tracked identities placeholders |
| Generated runtime and assets | Build/package scripts; do not edit generated `build/` or `target/` |

Keep one Router-owned layout and one main landmark. Adding a route does not
require a second Worker allowlist; preserve the SSR not-found response.
Server operations use `request_database()` for database and session together.
Reuse the shared Send bridge rather than wrapping each caller independently.
See [building features](building-features.md) for worked examples and
[Resource Explorer](../patterns/resource-explorer/README.md) for reviewed edits,
conflict handling, retry and persisted readback.

`/contact` stores validated D1 intake; it sends no email or webhook.
`/realtime/socket` is a request-scoped capability route above Leptos. Rooms,
presence, collaboration and other shared state require Durable Objects; follow
[realtime](realtime.md). Do not remove sample consumers or schema until the
application cutover replaces them. Follow [adoption](adopting.md).

## Verify at the required depth

```bash
cargo check --features ssr       # focused server iteration
cargo fmt --check
./scripts/verify.sh              # required before release-readiness claims
```

The full local gate is authoritative; hosted CI is intentionally absent.
Use `bash scripts/build-edge.sh` for both WASM builds and asset/runtime verifiers,
then `./scripts/wrangler.sh deploy --dry-run` for the configured build and deployment
shape. A dry run uploads nothing and proves no live behavior.
Run `bun scripts/check-file-limits.mjs` for the separate source-size inventory;
its exception rules are in [repository cleanup](repository-cleanup.md).

## Diagnose without bypassing

| Failure | Next step |
|---|---|
| Missing tool or dependency audit | Read the named prerequisite, install through its supported owner, then rerun; do not skip checks |
| wasm-bindgen mismatch | Check `./scripts/with-wasm-bindgen-cli.sh --version` against Cargo.lock; rebuild through the wrapper, not an ambient global binary |
| Non-Send Worker future | Delegate through `crate::server::run(...)` in `src/server/mod.rs` |
| Missing D1 table locally | Apply migrations with the configured database name and `--local` |
| Production placeholder ID | Derive ignored production configuration from verified identity; never patch tracked IDs |
| New module not found | Register its `pub mod` in the corresponding module owner |
| Missing app state | Inspect the context wiring in `src/edge/mod.rs`; don't call server implementations outside the server-function context |
| Mutation failed or conflicted | Preserve the record, show the error and allow retry; inspect session ownership and canonical data |
| Host resource admission hold | Follow the host's admission explanation; preserve inherited descriptors and reservations, never delete caches or bypass the hold |

## Provider operations are separate

Select [credential profiles](credentials.md) explicitly. Independent accounts use
the standalone target-specific procedure. An owner-selected cfctl account uses
[installed capability discovery](cfctl.md), account/profile/target binding, exact
plan approval, execution and independent readback. Installation alone grants no
access or authority. A blocked governed capability does not select another lane.

Workers production uses ignored `wrangler.production.toml`. Pages has distinct
production/preview identities and the [production preparation/readback contract](pages-production.md).
Keep migration hashes, assertions, recovery and deployed schema proof intact.
Follow [RELEASE.md](../RELEASE.md) for the full release procedure. Source, local
runtime, publication, provider deployment and live acceptance remain distinct.
