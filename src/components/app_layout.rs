use leptos::prelude::*;
use leptos_router::components::A;

/// Persistent public shell. This must remain inside `Router` so every `A`
/// receives routing context during SSR and hydration.
#[component]
pub fn AppLayout(children: Children) -> impl IntoView {
    view! {
        <a class="skip-link" href="#content">"Skip to content"</a>
        <header class="site-header">
            <div class="site-header__inner">
                <A href="/" attr:class="site-brand" attr:aria-label="Leptos CF home">
                    <span class="site-brand__name">"Leptos CF"</span>
                    <span class="site-brand__star" aria-hidden="true">"✦"</span>
                    <span class="site-brand__note">"Leptos 0.8 · Cloudflare Workers"</span>
                </A>

                <nav class="site-nav" aria-label="Primary navigation">
                    <A href="/" attr:class="site-nav__link">"Field guide"</A>
                    <A href="/architecture" attr:class="site-nav__link">"Architecture"</A>
                    <A href="/patterns" attr:class="site-nav__link">"Patterns"</A>
                    <A href="/lab" attr:class="site-nav__link">"Lab"</A>
                    <A href="/start" attr:class="site-nav__link site-nav__link--start">"Use the starter"</A>
                </nav>
            </div>
        </header>

        <main id="content" class="site-main">
            {children()}
        </main>

        <footer class="site-footer">
            <div class="site-footer__inner">
                <div class="site-footer__mark" aria-hidden="true">"✦"</div>
                <p>
                    <strong>"Leptos CF"</strong>
                    " is a source-derived field guide and public starter. Local proof is not deployment proof."
                </p>
                <nav class="site-footer__nav" aria-label="Footer navigation">
                    <A href="/about">"About & trust"</A>
                    <A href="/contact">"Intake lab"</A>
                    <a href="https://github.com/rogu3bear/leptos-cloudflare">"Source ↗"</a>
                </nav>
            </div>
        </footer>
    }
}
