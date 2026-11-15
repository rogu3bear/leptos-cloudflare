# Building Features

For a newly named application, follow [Adopting the starter](adopting.md).
Provider credentials and the governed/standalone distinction are in
[Credential profiles](credentials.md). Runtime invariants remain required
after replacing field-guide pages or labels.

A practical guide to extending this starter. The codebase is a single Rust crate that compiles to two targets: a Cloudflare Worker (SSR, `ssr` feature) and a WASM bundle hydrated in the browser (`hydrate` feature). Everything here builds on patterns already established in the todo domain — read `src/components/todo_page.rs` and `src/api.rs` before adding anything new.

---

## 1. Adding a new page/route

**Three steps:** create the component, export it from `mod.rs`, register it in `app.rs`.

### Create the component

```rust
// src/components/about_page.rs
use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="page-shell">
            <section class="panel">
                <h1>"About"</h1>
                <p class="hero-lede">"Built with Leptos and Cloudflare Workers."</p>
            </section>
        </div>
    }
}
```

### Export from mod.rs

```rust
// src/components/mod.rs
pub mod about_page;
pub mod todo_page;
```

### Add the route in app.rs

```rust
// src/app.rs
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, SsrMode, StaticSegment, WildcardSegment,
};

use crate::components::about_page::AboutPage;
use crate::components::contact_page::ContactPage;
use crate::components::todo_detail_page::TodoDetailPage;
use crate::components::todo_page::TodoPage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Leptos CF Starter"/>

        <Router>
            <AppLayout>
              <Routes fallback=|| view! { <NotFoundPage/> }.into_view()>
                <Route path=StaticSegment("") view=HomePage ssr=SsrMode::OutOfOrder/>
                <Route path=StaticSegment("lab") view=TodoPage ssr=SsrMode::OutOfOrder/>
                <Route path=StaticSegment("about") view=AboutPage ssr=SsrMode::OutOfOrder/>
                <Route path=StaticSegment("contact") view=ContactPage ssr=SsrMode::OutOfOrder/>
                <Route
                    path=(StaticSegment("lab"), ParamSegment("id"))
                    view=TodoDetailPage
                    ssr=SsrMode::OutOfOrder
                />
                <Route path=WildcardSegment("any") view=NotFoundPage ssr=SsrMode::OutOfOrder/>
              </Routes>
            </AppLayout>
        </Router>
    }
}
```

For dynamic segments (e.g. `/lab/:id`), use `leptos_router::ParamSegment("id")` (or a tuple path like `(StaticSegment("lab"), ParamSegment("id"))`) and read the value with `use_params_map()` from `leptos_router::hooks`.

For a complete collection-to-edit flow, start with [Resource Explorer](../patterns/resource-explorer/README.md) and its compiled components under `src/components/resource_explorer/`. Its routes use `SsrMode::Async` so bounded record data is present in HTML without JavaScript.

The template demonstrates this live at `/lab/:id` (see `TodoDetailPage` + the `GetTodo` server function). The old `/todo/:id` path remains compatibility-only.

**Critical for Cloudflare Workers + Leptos edge:**

Keep the `WildcardSegment("any")` catch-all **last**. Combined with the generated `build/_worker.js` shim, this guarantees that known deep links receive their full server-rendered document and unknown document routes receive the useful server-rendered recovery page with an HTTP `404` status instead of a bare platform response.

Use `<A href=...>` (from `leptos_router::components`) for internal links. After hydration it enables fast client-side navigation; before hydration or on hard refresh it falls back to normal document requests (which the catch-all + SSR path handles correctly).

Add only the new route to the route owner in `src/app.rs`, before the existing wildcard:

```rust
<Route path=StaticSegment("my-feature") view=MyFeaturePage ssr=SsrMode::OutOfOrder/>
```

`SsrMode::OutOfOrder` is the recommended default for most interactive pages on this stack (best perceived performance on the edge).

`AppLayout` owns the document's only `<main id="content">`; page components should return sections or divs, not nested main landmarks. Async `Resource` output must sit inside a `<Suspense>` boundary so streamed SSR markup and hydration agree.

---

## 2. Adding a new server function

Server functions are the only communication channel between the WASM client and the Worker. The pattern has four parts: a shared type in `api.rs`, the `#[server]` fn with a `SendWrapper`, a DB query in `src/server/`, and wiring into a component.

The Worker rejects cross-origin `/api/*` POSTs before Leptos dispatch. If you add browser-submitted server functions, keep them same-origin and use normal Leptos client calls or same-origin forms.

Realtime features are not Leptos server functions. WebSocket upgrades reach the generated Worker shim before the Rust router, so shared realtime state should start from `docs/realtime.md` and `patterns/realtime-durable-object/`, not from a Leptos component or `src/server/` module.

### 2a. Keep the shared contract in its domain

Shared response types and validation belong to a domain owner such as `src/todos.rs`. The `#[server]` macro generates the request type from the function arguments; `src/api.rs` remains the stable transport declaration. Shared types crossing the wire derive `Serialize`, `Deserialize` and `Clone`.

### 2b. Reuse the server bridge

The SSR branch delegates through `crate::server::run(...)`, as the current declarations in `src/api.rs` do. That helper owns `SendWrapper` and safe conversion from `AppError` to `ServerFnError`. Keep database work in its `src/server/` domain module.

### 2c. Preserve the session and reviewed-write boundary

Use `request_database()` from `src/server/d1.rs` to obtain the database and session together. Bind the session in every query that reads or mutates a session-owned record. Reuse the current title validation and numeric-conversion helpers, and return `AppResult<T>`; do not expose D1 error strings to the browser.

For a todo rename, reuse the existing `update_todo` command in `src/server/todos.rs`. It validates the title, compares the reviewed original values atomically, and returns Saved, Conflict or Unavailable without disclosing another session's record. A new unscoped `UPDATE todos ... WHERE id = ?` would lose those guarantees.

### 2d. Wire the existing command into a component

```rust
let update_action = ServerAction::<UpdateTodo>::new();
update_action.dispatch(UpdateTodo {
    id: saved.id,
    title: draft_title,
    completed: draft_completed,
    expected_title: saved.title,
    expected_completed: saved.completed,
});
```

Handle all outcomes and preserve the draft on an unconfirmed save. The runnable owner is `src/components/resource_explorer/editor.rs`; it updates the visible saved record only after acknowledgement and supports an explicit review after a conflict. Invalidate or replace affected resources after successful commands, as the existing components demonstrate.

---

## 3. Adding a new D1 table

### 3a. Create and apply the migration

```bash
# Create the file (name it in sequence)
cat > migrations/0004_categories.sql << 'EOF'
CREATE TABLE IF NOT EXISTS categories (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  session_id TEXT NOT NULL,
  name TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(session_id, name)
);
EOF

# Apply locally (preview database)
./scripts/wrangler.sh d1 migrations apply leptos-cf-db --local

# Production is a separate governed operation. Append the migration, update the
# repository operation's closed schema assertions, and prepare:
cfctl call leptos-cf.d1-migrations-apply \
  --selector account_id=<verified-account-id> \
  --selector database_id=<verified-d1-uuid> \
  --query config=wrangler.production.toml \
  --json
```

### 3b. Add a query module

For this session-scoped example, define the shared `Category` DTO in a domain module, then add `src/server/categories.rs` with an `AppResult` interface. A read follows the same boundary as the existing todo module:

```rust
use super::{d1::request_database, AppError, AppResult};
use crate::categories::Category;
use worker::D1Type;

pub async fn list_categories() -> AppResult<Vec<Category>> {
    let (db, session_id) = request_database()?;
    let session = D1Type::Text(&session_id);
    db.prepare("SELECT id, name, created_at FROM categories WHERE session_id = ?1 ORDER BY name ASC")
        .bind_refs(&session)
        .map_err(|_| AppError::Internal)?
        .all().await
        .map_err(|_| AppError::Internal)?
        .results::<Category>()
        .map_err(|_| AppError::Internal)
}
```

This is a feature recipe, not an installed category feature. Its `Category` type must derive the serialization/deserialization traits, and its migration must be appended to the actual current migration sequence. Define write validation, caps, authorization and conflict behavior before exposing mutations; retain the session predicate in writes and follow-up reads too.

### 3c. Register only the added domain

Add `mod categories;` alongside the existing shared domain modules in `src/lib.rs`, and `pub mod categories;` alongside the existing server modules in `src/server/mod.rs`. Keep the existing registrations. Declare the typed transport in `src/api.rs` and delegate the SSR branch through `crate::server::run(crate::server::categories::list_categories()).await`.

Then run local migrations, compile both SSR and hydration, and exercise the new commands in two browser sessions before claiming the feature works. Remote migration and deployment retain the separate authority described above.

---

## 4. Working with Leptos components

`TodoRow` in `src/components/todo_page.rs` is the most complete example in this codebase. Reference it when building new components.

### Component basics

```rust
#[component]
pub fn MyWidget(
    label: String,           // owned, passed by value
    count: i32,              // Copy types are fine
    on_click: impl Fn() + 'static,  // callbacks need 'static
) -> impl IntoView {
    view! {
        <button on:click=move |_| on_click()>
            {label} " (" {count} ")"
        </button>
    }
}
```

Props are ordinary function arguments — no separate `Props` struct. `impl Fn() + 'static` is the idiomatic type for event callbacks.

### Reactive state

```rust
// Local signal — lives in this component
let count = RwSignal::new(0i32);

// Read in the view
{move || count.get()}

// Write on event
on:click=move |_| count.update(|n| *n += 1)
```

Signals are `Copy` — close over them freely in closures. Always use `move ||` in the view.

### Loading async data (Resource)

```rust
let action_version = some_action.version();

let data = Resource::new(
    move || action_version.get(),   // re-runs when this changes
    |_| async move { fetch_data().await },
);

// In the view:
{move || match data.get() {
    None => view! { <LoadingState/> }.into_any(),
    Some(Err(e)) => view! { <p>{e.to_string()}</p> }.into_any(),
    Some(Ok(value)) => view! { <DataView data=value/> }.into_any(),
}}
```

`data.get()` returns `None` while the future is in flight, `Some(result)` once it settles. Call `.into_any()` on branches that return different concrete view types.

### Mutations (ServerAction)

```rust
let create = ServerAction::<CreateTodo>::new();

// Dispatch — this is the only way to call a server function from the client
create.dispatch(CreateTodo { title: "Ship it".into() });

// Pending state
create.pending().get()  // bool

// Last result (Option<Result<...>>)
create.value().get()

// What was dispatched — useful for optimistic UI
create.input().get()   // Option<CreateTodo>
```

### Optimistic UI

`TodoRow` shows the pattern: inspect `toggle_action.input()` to find whether the pending dispatch targets _this_ row, then flip `completed` locally before the server responds.

```rust
let is_mine = move || {
    toggle_action.pending().get()
        && toggle_action
            .input()
            .get()
            .map(|input| input.id == id)
            .unwrap_or(false)
};

let optimistic_done = move || if is_mine() { !completed } else { completed };
```

### Show and For

```rust
// Conditional rendering
<Show when=move || some_signal.get()>
    <p>"Only rendered when true."</p>
</Show>

// List rendering — key must be stable and unique
<For
    each=move || items.clone().into_iter()
    key=|item| item.id
    children=move |item| view! { <ItemRow item=item/> }
/>
```

---

## 5. Styling

All styles live in `style/main.css`. There is no Tailwind; don't add it.

### CSS custom properties

The token values and existing surface styles are owned by [style/main.css](../style/main.css). Reuse those semantic variables and the existing control/surface classes; changing a palette should not require updating copied examples here.

```css
.my-card {
  border: 1px solid var(--rule);
  background: var(--paper-soft);
  color: var(--ink);
}
```

### BEM-like naming

Use the block–modifier pattern already in the codebase:

```css
/* Block */
.category-row { ... }

/* State modifier */
.category-row--selected { ... }

/* Element inside block */
.category-row__label { ... }
```

Apply modifiers reactively in Leptos:

```rust
<li
    class="category-row"
    class:category-row--selected=move || is_selected.get()
>
```

### Responsive breakpoints

Two breakpoints are defined:

- `860px` — collapses multi-column grids to single column
- `640px` — reduces padding and stacks inline flex layouts

Add new responsive rules inside the existing `@media` blocks at the bottom of `main.css`.

### Skeleton loading

The `LoadingState` component shows the pattern: add `.skeleton` to get the shimmer animation, then add a sizing modifier:

```css
/* In main.css */
.skeleton--card {
  height: 96px;
}
```

```rust
// In Rust
<div class="skeleton skeleton--card"></div>
```

---

## 6. Error handling

### In server query functions (src/server/)

Errors are `String` — conversion helpers handle the rest:

```rust
// d1_error converts any Display into String
let result = db.prepare("...").all().await.map_err(d1_error)?;

// ensure_row_changed guards against silent no-ops
ensure_row_changed(result, "update")?;

// normalize_title validates and trims text input
let title = normalize_title(title)?;
```

For new query modules, copy these private helpers from `todos.rs` or create analogous ones in your module.

### In server functions (api.rs)

`server_error` converts `String` errors to `ServerFnError`:

```rust
crate::server::my_module::do_thing()
    .await
    .map_err(crate::server::server_error)
```

### In components

The `server_error` derived signal in `TodoPage` is the pattern:

```rust
let server_error = move || {
    create_action
        .value()
        .get()
        .and_then(|r| r.err().map(|e| e.to_string()))
        .or_else(|| {
            delete_action
                .value()
                .get()
                .and_then(|r| r.err().map(|e| e.to_string()))
        })
};
```

Chain `.or_else` for each action you want surfaced. Render it with `Show`:

```rust
<Show when=move || server_error().is_some()>
    <div class="feedback feedback--error" role="status">
        {move || server_error().unwrap_or_default()}
    </div>
</Show>
```

For local validation errors (before the action dispatches), use a separate `RwSignal<Option<String>>` — see `local_error` in `TodoPage`.

### Resource errors

```rust
{move || match data.get() {
    None => { /* loading */ }
    Some(Err(error)) => view! {
        <section class="panel error-panel">
            <h2>"Failed to load"</h2>
            <p>{error.to_string()}</p>
            <button
                class="ghost-button"
                type="button"
                on:click=move |_| refresh_nonce.update(|n| *n += 1)
            >
                "Try again"
            </button>
        </section>
    }.into_any(),
    Some(Ok(data)) => { /* render */ }
}}
```

The `refresh_nonce` pattern — an `RwSignal<usize>` that increments on retry — forces the Resource to re-fetch without a full page reload.

---

## 7. Adding secrets

### Local development

Create `.dev.vars` in the project root (it is gitignored):

```ini
# .dev.vars
STRIPE_SECRET_KEY=<local-stripe-secret>
SENDGRID_API_KEY=<local-sendgrid-secret>
```

Wrangler injects these as environment bindings when running `wrangler dev`.

### Production

```bash
./scripts/wrangler.sh secret put STRIPE_SECRET_KEY
# Prompts for the value — not echoed to the terminal
```

### Accessing secrets in server functions

`AppState` holds the Worker `Env`. Call `.secret()` inside the `ssr` block of a server function:

```rust
// In a server function or server-side query fn:
let state = use_context::<AppState>()
    .ok_or_else(|| "Missing app state.".to_string())?;

let key = state
    .env
    .secret("STRIPE_SECRET_KEY")
    .map_err(|e| e.to_string())?
    .to_string();
```

The `secret()` call fails if the binding is not defined — validate during startup or fail loudly at call time.

---

## 8. Testing and verification

### Type-check the SSR target

```bash
cargo check --features ssr
```

Run this first. It catches the majority of errors — missing imports, wrong types in server functions, broken `use_context` calls. The `hydrate` target is checked implicitly by `cargo leptos build`.

### Full build

```bash
bash ./scripts/build-edge.sh
```

This compiles both the WASM client bundle and the Worker binary, fingerprints the browser assets, and verifies the cache headers/manifest output. Required before deploying or running integration tests. Slow; use `cargo check --features ssr` during development.

### Dry-run deploy

```bash
./scripts/wrangler.sh deploy --dry-run
```

Validates the wrangler config, asset pipeline, and Worker bundle without touching production. Run this before every real deploy to catch configuration drift.

The command above proves the portable template. After deriving the ignored production config from verified provider identity, also run `./scripts/wrangler.sh deploy --dry-run --config wrangler.production.toml`. Neither dry run is provider proof.

### Apply migrations locally before testing

```bash
./scripts/wrangler.sh d1 migrations apply leptos-cf-db --local
```

New tables must exist locally before `wrangler dev` can exercise any queries against them.

### Local dev server

```bash
./scripts/wrangler.sh dev
```

Hot-reloads on Worker changes. Does not hot-reload WASM — rebuild with `cargo leptos build` if you change component logic.


Both deployment targets share the application and D1 code. For Pages, package the completed edge build and use the separate Pages config; see [the Pages option](edge-deployment.md#pages-option). Test affected behavior on both targets before claiming release readiness.
