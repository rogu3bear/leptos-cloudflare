# Agent Playbook: leptos-cf

Instruction set for AI coding agents working on this Leptos + Cloudflare Workers + D1 starter. Follows the repository conventions established in the codebase. Read this before making any changes.

---

## 1. Prerequisites Check

Run the dependency checker first:

```bash
./scripts/check-deps.sh
```

Expected output: every line starts with `[ok]`. Any `[missing]` line is a hard blocker.

If it fails, run the bootstrap script:

```bash
./scripts/bootstrap.sh
```

Bootstrap installs: stable Rust toolchain, `wasm32-unknown-unknown` target, `cargo-leptos`, `worker-build`, and the repo-local `wasm-bindgen-cli` resolved from `Cargo.lock`. It requires `rustup`, `cargo`, and `bun` to already be present. If those are missing, the script exits with an error message that names the missing tool.

After bootstrap, re-run `check-deps.sh` and confirm all checks pass.

Local development needs no provider credential. For this operator workspace's production lane only, verify the Cloudflare account identity and the dedicated short-lived profile:

```bash
test -n "${CLOUDFLARE_ACCOUNT_ID:-}"
cfctl auth status <short-lived-profile> --json
```

Both commands must exit 0. Install the child token through `cfctl auth import-api-token --account <account-id> --value-in <mode-0600-file>` or the repo's gitignored `.env`; never hardcode or print it. The account token-minter credential does not enter this repository or deployment profile.

---

## 2. Governed Production Bootstrap

For credential-free local startup, use README Quick Start. For independent
standalone production operation, use `docs/credentials.md` and the portable
Wrangler lane. The sequence below is for this operator workspace only.
After `scripts/init.sh`, substitute the adopted names from Cargo/Wrangler and
the operation ID in `.cfctl/operations/d1-migrations.toml`.

### 2.1 Read or create the D1 database through cfctl

First bind and audit the checkout/account, then query by the exact database name:

```bash
cfctl workspace add "$PWD" --account "$CLOUDFLARE_ACCOUNT_ID" --json
cfctl workspace audit --json
cfctl call d1-list-databases \
  --selector "account_id=$CLOUDFLARE_ACCOUNT_ID" \
  --query "name=leptos-cf-db" \
  --profile <short-lived-profile> \
  --json
```

If no exact-name database exists, prepare a create plan:

```bash
printf '%s' '{"name":"leptos-cf-db","read_replication":{"mode":"disabled"}}' | \
  cfctl call d1-create-database \
    --selector "account_id=$CLOUDFLARE_ACCOUNT_ID" \
    --profile <short-lived-profile> \
    --body-stdin \
    --json
```

This returns a preview operation. Inspect its selectors, body, cost, rollback, workspace impact, and content hash; obtain explicit approval; then run `cfctl plans approve`, `cfctl plans run`, and `cfctl plans status` for that exact operation. Extract the UUID only from successful provider verification:

```
[[d1_databases]]
binding = "DB"
database_name = "leptos-cf-db"
database_id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
```

Do not create a duplicate after a name lookup, and do not install an ID from plan intent alone.

### 2.2 Derive the ignored production config

Keep tracked `wrangler.toml` provider-neutral. Derive `wrangler.production.toml` only after Worker/D1 names and the D1 UUID have been read back from the provider:

```bash
bun ./scripts/write-production-config.mjs \
  --worker <verified-worker-name> \
  --database <verified-d1-name> \
  --database-id <verified-d1-uuid>
```

The generator rejects partial or invalid identity, an already-bound source template, Pages/`env.production` drift, and missing runtime invariants. It changes only the Worker name, D1 name, and both D1 UUID fields. The output is gitignored and must never become a second tracked source of truth.

### 2.3 Apply migrations

Apply to the local SQLite replica first:

```bash
CI=1 bunx wrangler@4.120.1 d1 migrations apply leptos-cf-db --local
```

Remote schema application is a production data write. Use `leptos-cf.d1-migrations-apply`, the repository-owned operation that binds the clean canonical root and HEAD, every ordered migration blob, operation-pack blob, production-config hash and database identity, a fresh pre-change recovery bookmark, the Wrangler migration ledger, and closed schema readback. Generic D1 import/query paths are not substitutes. If the operation fails closed, stop and resolve its contract; do not bypass it with direct Wrangler or raw SQL.

The migrations create the `todos` and `contact_submissions` tables plus their indexes. Verify with:

```bash
bunx wrangler@4.120.1 d1 execute leptos-cf-db --local --command "SELECT name FROM sqlite_master WHERE type='table'"
```

Expected: `todos` and `contact_submissions` appear in the result.

### 2.4 Build

```bash
bash ./scripts/build-edge.sh
```

This runs the full edge build pipeline: `cargo-leptos` compiles the hydration bundle, `scripts/hash-assets.mjs` fingerprints the client JS/CSS/WASM and updates the SSR asset constants, `worker-build` compiles the Worker bundle, and `scripts/write-worker-shim.mjs` generates the configured Workers entrypoint. Output lands in `target/site/` and `build/`. Expect several minutes on first run due to dependency compilation.

Decision point: if the build fails with a `wasm-bindgen` version mismatch error, see section 7 (Troubleshooting).

### 2.5 Verify the build artifact

```bash
bunx wrangler@4.120.1 deploy --dry-run
```

This validates the `wrangler.toml` config and the build output without uploading anything. Expected output ends with `Total Upload:` and does not contain `error`.

### 2.6 Deploy through a reviewed plan

```bash
cfctl call wrangler.deploy --query config=wrangler.production.toml --json
```

That command prepares a hash-bound deployment plan. After reviewing and explicitly approving the exact operation, use `cfctl plans run` and `cfctl plans status`. Then independently read back the Worker version, compatibility date, bindings, assets, routes, and observability settings and test the field-guide routes plus the `/lab` D1 write path. Plan completion is not live behavior proof.

---

## 3. Adding a Feature

Use this checklist. Work through it top to bottom; each item that applies requires changes before the next item.

**Do you need a new database table?**
- Create `migrations/NNNN_<name>.sql` (increment N from the last migration file).
- Apply local: `bunx wrangler d1 migrations apply leptos-cf-db --local`
- Apply remote after the feature is complete and tested locally.

**Do you need a new server function?**
- Declare the types in `src/api.rs`: request struct (if needed) and response type. Both must derive `Serialize`, `Deserialize`, `Clone`.
- Add the `#[server(...)]` function in `src/api.rs`. The body must be wrapped in `SendWrapper::new(async move { ... }).await` — see existing server functions for the exact pattern.
- Add the database implementation function in `src/server/todos.rs` (or a new file under `src/server/`). Register any new module in `src/server/mod.rs`.
- Server functions call `use_context::<AppState>()` via the `database()` helper in `todos.rs`. Do not use global state.

**Do you need a new page or route?**
- Create the component file in `src/components/<name>.rs`.
- Export it from `src/components/mod.rs`.
- Add a `<Route path=... view=YourComponent/>` in `src/app.rs` inside the existing `<Routes>` block.

**Do you need new styles?**
- Add CSS to `style/main.css`. There is no Tailwind; write plain CSS. Match the existing naming conventions (BEM-adjacent, lowercase kebab).

**After every change:**
- Run the verification protocol in section 6 before considering the feature done.

---

## 4. Common Operations Reference

| Operation | Command |
|-----------|---------|
| Check all deps | `./scripts/check-deps.sh` |
| Full bootstrap | `./scripts/bootstrap.sh` |
| Read D1 database inventory | `cfctl call d1-list-databases ... --json` |
| Prepare D1 creation plan | `cfctl call d1-create-database ... --body-stdin --json` |
| Apply migrations (local) | `bunx wrangler d1 migrations apply leptos-cf-db --local` |
| Derive production config | `bun scripts/write-production-config.mjs --worker ... --database ... --database-id ...` |
| Apply reviewed migrations (remote) | `cfctl call leptos-cf.d1-migrations-apply --selector account_id=... --selector database_id=... --query config=wrangler.production.toml --json` |
| Execute SQL (local) | `bunx wrangler d1 execute leptos-cf-db --local --command "..."` |
| Verify remote schema | Governed bounded schema-introspection/read capability |
| Build (release) | `bash ./scripts/build-edge.sh` |
| Type-check SSR only | `cargo check --features ssr` |
| Local dev server | `bunx wrangler dev --local --ip 127.0.0.1 --port 57581` |
| Validate before deploy | `bunx wrangler deploy --dry-run` |
| Prepare production deploy | `cfctl call wrangler.deploy --query config=wrangler.production.toml --json` |
| Production secrets | Governed secret capability with secret input/output sinks |

---

## 5. File Ownership Map

Before editing any file, confirm it matches the kind of change you are making.

| What you are changing | File(s) to edit |
|-----------------------|-----------------|
| Shared types, server function signatures | `src/api.rs` |
| D1 query logic | `src/server/todos.rs`, `src/server/contact.rs` (or new file in `src/server/`) |
| New server submodule | `src/server/mod.rs` — add `pub mod <name>` |
| UI components | `src/components/<name>.rs` |
| New component exports | `src/components/mod.rs` |
| Route definitions | `src/app.rs` |
| CSS styles | `style/main.css` |
| Database schema | `migrations/NNNN_<name>.sql` (new file, never edit applied migrations) |
| Portable Cloudflare bindings (D1, KV, R2, etc.) | tracked `wrangler.toml` |
| Provider identity | derived, gitignored `wrangler.production.toml`; never hand-edit tracked identity |
| Governed remote migration contract | `.cfctl/operations/d1-migrations.toml` plus append-only `migrations/*.sql` |
| WebSocket ingress contract | `scripts/write-worker-shim.mjs`, `docs/realtime.md` |
| Shared realtime state | `patterns/realtime-durable-object/` first, then `wrangler.toml` and `scripts/write-worker-shim.mjs` when adopting |
| Local dev secrets | `.dev.vars` (create if absent; never commit this file) |
| Production secrets | `bunx wrangler secret put ...` (stored in CF, not in files) |
| Worker entry point + app state wiring | `src/server/state.rs`, `src/lib.rs` |
| Rust dependencies | `Cargo.toml` |

Do not edit `src/lib.rs` for feature work. It contains only the Worker `fetch` entry point and the WASM `hydrate` export. Change it only if the routing or app state wiring needs to change.

The public `/contact` route is intentionally local to the template: it validates and persists submissions to D1, applies session-scoped submission caps, and relies on the Worker `/api/*` origin guard before Leptos server functions dispatch. It does not send email, webhooks, or third-party API calls without an explicit integration.

The public `/realtime/socket` route is intentionally a Worker-level capability lane. Keep request-scoped demos in the generated shim; move rooms, presence, collaboration, fanout, reconnect state, or other shared state to `patterns/realtime-durable-object/` before adding Durable Object bindings.

---

## 6. Verification Protocol

For day-to-day iteration, use this lighter sequence:

```bash
# Step 1: Fast type-check for SSR
cargo check --features ssr

# Step 2: Full edge build + verifiers
bash ./scripts/build-edge.sh

# Step 3: Deployment structure validation
bunx wrangler@4.120.1 deploy --dry-run
```

Before pushing or declaring a change complete, run the full release readiness verification instead:

```bash
./scripts/verify.sh
```

This is the authoritative local sequence documented in `RELEASE.md`.

---

## 7. Troubleshooting

## Realtime/WebSocket Rule

WebSocket traffic enters above Leptos. Do not add realtime features by relying on client-side navigation, browser history behavior, or a Leptos component mounting at the right time.

Use the explicit `/realtime/socket` lane from the generated `_worker.js` for request-scoped capability checks. If the feature has rooms, presence, collaboration, fanout, reconnect state, or any shared state across clients, introduce a Durable Object and document the object key.

Required proof after touching realtime routing:

```bash
bash ./scripts/build-edge.sh
bun ./scripts/verify-worker-runtime.mjs
bunx wrangler@4.120.1 deploy --dry-run
```

**`error: wasm-bindgen version mismatch`**

The `wasm-bindgen-cli` version must match what `Cargo.lock` resolves. The production build wraps both `cargo leptos` and `worker-build` with `scripts/with-wasm-bindgen-cli.sh`, which installs the lockfile-matched CLI under `var/cargo-tools/` and puts it first on `PATH` for every WASM-producing phase.

Check the repo-local version:
```bash
./scripts/with-wasm-bindgen-cli.sh --version
```

Fix — refresh the repo-local tool and rerun the build:
```bash
rm -rf var/cargo-tools/wasm-bindgen-*
bash ./scripts/build-edge.sh
```

---

**`the trait Send is not implemented for ...` / `not Send` errors in server functions**

Cloudflare Workers use a single-threaded runtime. The `worker` crate types (`D1Database`, `Env`, etc.) are not `Send`. Leptos server functions require `Send` futures by default.

The fix is already demonstrated in `src/api.rs`: wrap the async block with `SendWrapper::new(async move { ... }).await`. Every server function body that touches `AppState` or D1 must use this wrapper.

```rust
#[server(MyFn)]
pub async fn my_fn() -> Result<MyType, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        send_wrapper::SendWrapper::new(async move {
            crate::server::my_module::my_impl()
                .await
                .map_err(crate::server::server_error)
        })
        .await
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("server functions only execute on the server")
    }
}
```

---

**`no such table: todos` at runtime**

For local development, apply the local migration:

```bash
bunx wrangler d1 migrations apply leptos-cf-db --local
```

For production, inspect the provider schema and prepare the repository-owned `leptos-cf.d1-migrations-apply` operation. If it is blocked, stop and resolve that contract rather than running a direct remote command.

---

**`wrangler.toml contains placeholder D1 IDs`**

This is the required tracked-template state, not a warning. If a production command fails because it used the template, you skipped step 2.2 or omitted `--config wrangler.production.toml`. Re-derive the ignored file only from a live `d1-list-databases` receipt or verified `d1-create-database` readback.

---

**`module not found` after adding a new source file**

Rust requires explicit module registration. If you created `src/server/my_module.rs`, add `pub mod my_module;` to `src/server/mod.rs`. If you created `src/components/my_page.rs`, add `pub mod my_page;` to `src/components/mod.rs`. The compiler error will name the missing declaration.

---

**`Missing app state in Leptos server function context`**

`AppState` is injected into the Leptos context in `src/lib.rs` via `.leptos_routes_with_context`. If you see this error, the server function ran outside the context scope — this should not happen unless you modified `src/lib.rs`. Do not call server implementation functions from outside a server function.

---

**`D1 reported no rows changed during toggle/delete`**

The requested todo ID does not exist. This is a logic error in the caller, not an infrastructure issue. Verify the ID being passed to `toggle_todo` or `delete_todo` exists in the database.

## Application adoption

Use `docs/adopting.md` for the supported name and page cutover. The initializer
preserves application code and schema. `Cargo.toml` selects reference-site
checks explicitly; shared runtime/security checks remain required for every
adopter. The complete gate now requires cargo-audit; absence fails before build.
