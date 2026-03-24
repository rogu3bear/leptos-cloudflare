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

Bootstrap installs: stable Rust toolchain, `wasm32-unknown-unknown` target, `cargo-leptos`, `wasm-bindgen-cli` (pinned to `0.2.114`). It requires `rustup`, `cargo`, and `bun` to already be present. If those are missing, the script exits with an error message that names the missing tool.

After bootstrap, re-run `check-deps.sh` and confirm all checks pass.

Verify Cloudflare credentials are set in the environment:

```bash
echo $CLOUDFLARE_API_TOKEN
echo $CLOUDFLARE_ACCOUNT_ID
```

Both must be non-empty. If either is missing, set them before proceeding. Wrangler reads these automatically; do not hardcode them.

---

## 2. Bootstrap Sequence

Perform these steps in order. Do not skip or reorder.

### 2.1 Create the D1 database

```bash
bunx wrangler d1 create leptos-cf-db
```

The command prints output that includes a TOML block. Extract the `database_id` value:

```
[[d1_databases]]
binding = "DB"
database_name = "leptos-cf-db"
database_id = "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
```

The UUID in `database_id` is what you need. If the database already exists, Wrangler will error. In that case, run `bunx wrangler d1 list` to retrieve the existing ID.

### 2.2 Update wrangler.toml

Open `wrangler.toml`. Replace both occurrences of `00000000-0000-0000-0000-000000000000` with the real database UUID:

```toml
[[d1_databases]]
binding = "DB"
database_name = "leptos-cf-db"
database_id = "<real-uuid>"
preview_database_id = "<real-uuid>"
migrations_dir = "migrations"
```

Both `database_id` and `preview_database_id` must be updated. The `check-deps.sh` script warns if the placeholder is still present.

### 2.3 Apply migrations

Apply to the local SQLite replica first:

```bash
bunx wrangler d1 migrations apply leptos-cf-db --local
```

Then apply to the remote D1 database:

```bash
bunx wrangler d1 migrations apply leptos-cf-db --remote
```

Expected output for each: `Applying migration 0001_init.sql... done`.

The migration creates the `todos` table and the `(completed, id DESC)` index. Verify with:

```bash
bunx wrangler d1 execute leptos-cf-db --local --command "SELECT name FROM sqlite_master WHERE type='table'"
```

Expected: `todos` appears in the result.

### 2.4 Build

```bash
bash ./scripts/build-edge.sh
```

This runs the full edge build pipeline: `cargo-leptos` compiles the hydration bundle, `scripts/hash-assets.mjs` fingerprints the client JS/CSS/WASM and updates the SSR asset constants, and `worker-build` compiles the Worker bundle. Output lands in `target/site/` and `build/`. Expect several minutes on first run due to dependency compilation.

Decision point: if the build fails with a `wasm-bindgen` version mismatch error, see section 7 (Troubleshooting).

### 2.5 Verify the build artifact

```bash
bunx wrangler deploy --dry-run
```

This validates the `wrangler.toml` config and the build output without uploading anything. Expected output ends with `Total Upload:` and does not contain `error`.

### 2.6 Deploy

```bash
bunx wrangler deploy
```

Wrangler uploads the Worker script and static assets to Cloudflare. The deployed URL is printed at the end. Test it by loading the URL in a browser or with `curl` — the todo page must render and the D1 connection must succeed (create a todo to verify the write path).

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
| Create D1 database | `bunx wrangler d1 create leptos-cf-db` |
| List D1 databases | `bunx wrangler d1 list` |
| Apply migrations (local) | `bunx wrangler d1 migrations apply leptos-cf-db --local` |
| Apply migrations (remote) | `bunx wrangler d1 migrations apply leptos-cf-db --remote` |
| Execute SQL (local) | `bunx wrangler d1 execute leptos-cf-db --local --command "..."` |
| Execute SQL (remote) | `bunx wrangler d1 execute leptos-cf-db --remote --command "..."` |
| Build (release) | `bash ./scripts/build-edge.sh` |
| Type-check SSR only | `cargo check --features ssr` |
| Local dev server | `bunx wrangler dev --local --ip 127.0.0.1 --port 57581` |
| Validate before deploy | `bunx wrangler deploy --dry-run` |
| Deploy to production | `bunx wrangler deploy` |
| Set a secret | `bunx wrangler secret put SECRET_NAME` |
| List secrets | `bunx wrangler secret list` |

---

## 5. File Ownership Map

Before editing any file, confirm it matches the kind of change you are making.

| What you are changing | File(s) to edit |
|-----------------------|-----------------|
| Shared types, server function signatures | `src/api.rs` |
| D1 query logic | `src/server/todos.rs` (or new file in `src/server/`) |
| New server submodule | `src/server/mod.rs` — add `pub mod <name>` |
| UI components | `src/components/<name>.rs` |
| New component exports | `src/components/mod.rs` |
| Route definitions | `src/app.rs` |
| CSS styles | `style/main.css` |
| Database schema | `migrations/NNNN_<name>.sql` (new file, never edit applied migrations) |
| Cloudflare bindings (D1, KV, R2, etc.) | `wrangler.toml` |
| Local dev secrets | `.dev.vars` (create if absent; never commit this file) |
| Production secrets | `bunx wrangler secret put ...` (stored in CF, not in files) |
| Worker entry point + app state wiring | `src/server/state.rs`, `src/lib.rs` |
| Rust dependencies | `Cargo.toml` |

Do not edit `src/lib.rs` for feature work. It contains only the Worker `fetch` entry point and the WASM `hydrate` export. Change it only if the routing or app state wiring needs to change.

---

## 6. Verification Protocol

Run these three commands in order after any code change. Do not skip steps.

```bash
# Step 1: Fast type-check for SSR (catches server-side errors quickly)
cargo check --features ssr

# Step 2: Full build (catches WASM compilation and linking errors)
bash ./scripts/build-edge.sh

# Step 3: Validate the deployment artifact
bunx wrangler deploy --dry-run
```

A change is only verified when all three commands exit 0 without errors. Warnings are acceptable; errors are not.

For changes that only touch `src/components/` or `style/main.css`, step 1 is sufficient for a fast iteration loop. Run steps 2 and 3 before declaring the change done.

---

## 7. Troubleshooting

**`error: wasm-bindgen version mismatch`**

The installed `wasm-bindgen-cli` version must match what the build resolves. The `Cargo.toml` specifies `wasm-bindgen = "0.2.105"` as a minimum version — Cargo may resolve a higher version. The `bootstrap.sh` script pins CLI version `0.2.114` to match the verified build.

Check the installed version:
```bash
wasm-bindgen --version
```

Fix — install the version that `bootstrap.sh` expects:
```bash
cargo install -f wasm-bindgen-cli --version 0.2.114
```

If the error message names a specific version, install that exact version instead.

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

The migration was not applied. Run:

```bash
bunx wrangler d1 migrations apply leptos-cf-db --local   # for local dev
bunx wrangler d1 migrations apply leptos-cf-db --remote  # for production
```

---

**`wrangler.toml still contains placeholder D1 IDs`**

You skipped step 2.2. `check-deps.sh` prints a `[warn]` for this. The Worker will fail to bind to D1 at runtime. Replace both placeholder UUIDs in `wrangler.toml` with the real database ID.

---

**`module not found` after adding a new source file**

Rust requires explicit module registration. If you created `src/server/my_module.rs`, add `pub mod my_module;` to `src/server/mod.rs`. If you created `src/components/my_page.rs`, add `pub mod my_page;` to `src/components/mod.rs`. The compiler error will name the missing declaration.

---

**`Missing app state in Leptos server function context`**

`AppState` is injected into the Leptos context in `src/lib.rs` via `.leptos_routes_with_context`. If you see this error, the server function ran outside the context scope — this should not happen unless you modified `src/lib.rs`. Do not call server implementation functions from outside a server function.

---

**`D1 reported no rows changed during toggle/delete`**

The requested todo ID does not exist. This is a logic error in the caller, not an infrastructure issue. Verify the ID being passed to `toggle_todo` or `delete_todo` exists in the database.
