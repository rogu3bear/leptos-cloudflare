# Shared Layout Composition

**Problem**: You need consistent UI (header, navigation, footer, etc.) across multiple pages while preserving excellent SSR, hydration, and client navigation behavior.

## Recommended Approach in This Stack

Use a layout component that accepts `children: Children` and renders the persistent UI around whatever the router provides.

```rust
#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <header>...</header>
        <main>
            {children()}
        </main>
    }
}
```

Then wrap your router:

```rust
<AppLayout>
    <Router>
        <Routes>
            ...
        </Routes>
    </Router>
</AppLayout>
```

## Why This Works Well Here

- The layout participates in the initial SSR render on the Worker.
- It hydrates cleanly on the client.
- It is simple and reliable (avoids some current leptos_router 0.8 declarative nesting quirks with `<Outlet/>` inside `view!`).
- Navigation between pages under the layout remains fast client-side after hydration.

## Integration with Other Patterns

- Combine with Dynamic Entity Detail: the layout provides the nav while the detail page handles its own loading states.
- The layout is a great place for global elements (theme switcher, user menu, etc.) that should not re-mount on route changes.

See `src/components/app_layout.rs` and its usage in `src/app.rs` for the current example in this template.
