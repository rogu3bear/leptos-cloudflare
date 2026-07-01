# Dynamic Entity Detail

**Problem**: You need to display, mutate, and navigate to a single entity (e.g. a todo, post, user record) loaded by an ID from the URL, with excellent loading states, optimistic UI, and proper behavior on the edge.

## Why This Matters on Cloudflare + Leptos

- Workers are stateless per request. You cannot rely on in-memory caches between requests.
- Server functions run in the same isolate as SSR — this is a huge advantage (direct D1 access, no extra network hop).
- Deep links and pre-hydration clicks must receive full SSR shells (hence the critical `WildcardSegment` catch-all + generated `_worker.js` behavior).
- Fine-grained reactivity shines here: you can update only the affected parts of the UI without re-rendering large components.

## Core Pattern

### 1. Routing
Use a dedicated dynamic route with `ParamSegment`:

```rust
<Route
    path=(StaticSegment("todo"), ParamSegment("id"))
    view=TodoDetailPage
    ssr=SsrMode::OutOfOrder
/>
```

Read the param with `use_params_map()`.

### 2. Data Loading
Dedicated server function per entity:

```rust
#[server(GetTodo)]
pub async fn get_todo(id: i64) -> Result<TodoItem, ServerFnError>
```

Backed by a thin implementation in `src/server/todos.rs` that reuses query helpers.

### 3. Reactivity & Actions
- `Resource` for loading (keyed on param + refresh signal).
- `ServerAction` for mutations (`ToggleTodo`, `DeleteTodo`).
- `Memo` for derived UI state (e.g. optimistic status labels).
- Optimistic updates using `action.pending()` + local derivation.

### 4. Post-Mutation Behavior
After a successful destructive action (delete), use `use_navigate()` to redirect. This is a clean integration between server function results and client routing.

### 5. Progressive / Streaming Loading
Use multiple independent `<Suspense>` boundaries. The main entity can load while secondary sections (metadata, activity, related items) stream in separately. This is a natural fit for edge SSR.

## Key Files (in this template)

- `src/app.rs` — Route definition + comments on edge requirements.
- `src/api.rs` — `GetTodo` server function declaration.
- `src/server/todos.rs` — `get_todo` implementation.
- `src/components/todo_detail_page.rs` — Full example: param reading, Resource, multiple Suspense, Memo, optimistic updates, navigation on success.

## Cloudflare-Specific Notes

- Always keep a top-level `WildcardSegment("any")` catch-all outside any layout. This works with the generated `build/_worker.js` to protect deep links.
- Server functions that touch D1 must wrap bodies in `SendWrapper` (Workers are single-threaded).
- Body size limits and security headers (CSP, anti-framing) are applied at the Worker level in `src/lib.rs`.

## Variations & Hardening

- Add row-level authorization inside the server function using the session from `AppState`.
- For very large detail views, consider splitting into multiple server functions + parallel Suspense boundaries.
- Combine with the Shared Layout pattern for consistent navigation.

## Anti-Patterns to Avoid

- Fetching the entity inside the component with `create_resource` without a proper key that includes the route param.
- Relying on client-only state for critical data (breaks deep links and SEO).
- Putting the catch-all route inside a layout (it must be a top-level sibling).

This pattern scales to most "show + edit single record" surfaces in Leptos on Workers.
