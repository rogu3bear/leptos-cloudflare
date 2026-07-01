use leptos::prelude::*;
use leptos_router::components::A;

/// Shared layout component that renders a persistent header + navigation
/// around whatever content is passed as children.
///
/// This is a practical, production-common pattern for shared UI (layout)
/// that works reliably with Leptos SSR + hydration on the edge.
/// The layout participates in the initial server render and subsequent hydration.
#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <header class="app-header">
            <div class="header-inner">
                <div class="brand-lockup">
                    <img class="brand-mark" src="/app-icon.svg" alt="Leptos CF Starter logo"/>
                    <span>"Leptos CF"</span>
                </div>

                <nav class="main-nav">
                    <A href="/" attr:class="nav-link">"Todos"</A>
                    <A href="/about" attr:class="nav-link">"About"</A>
                    <A href="/contact" attr:class="nav-link">"Contact"</A>
                </nav>
            </div>
        </header>

        <main>
            {children()}
        </main>
    }
}
