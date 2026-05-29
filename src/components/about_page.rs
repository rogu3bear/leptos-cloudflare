use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <main class="page-shell">
            <section class="hero">
                <div class="brand-lockup">
                    <img class="brand-mark" src="/app-icon.svg" alt="Leptos CF Starter logo"/>
                    <span>"Leptos CF Starter"</span>
                </div>
                <p class="eyebrow">"About this template"</p>

                <div class="hero-copy">
                    <h1>"A production-minded starting point."</h1>
                    <p class="hero-lede">
                        "This starter demonstrates the complete Leptos 0.8 + Cloudflare Workers + D1 model:
                        SSR on the edge, hydration in the browser, typed server functions, hashed assets,
                        and safe session-scoped demo data."
                    </p>
                    <p class="hero-lede" style="margin-top: 1rem;">
                        "The router, build pipeline, and security headers are set up so you can focus on your domain
                        instead of the edge plumbing."
                    </p>
                </div>
            </section>

            <section class="panel">
                <h2>"Key patterns this template shows"</h2>
                <ul class="todo-list" style="margin-top: 1rem;">
                    <li class="todo-row">
                        <div class="todo-copy">
                            <h3>"Nested & dynamic routes"</h3>
                            <p>"See /todo/:id for ParamSegment + use_params_map() + server functions per route."</p>
                        </div>
                    </li>
                    <li class="todo-row">
                        <div class="todo-copy">
                            <h3>"Progressive navigation with &lt;A&gt;"</h3>
                            <p>"Client-side navigation after hydration while still supporting full SSR deep links and refreshes."</p>
                        </div>
                    </li>
                    <li class="todo-row">
                        <div class="todo-copy">
                            <h3>"Catch-all for edge SSR"</h3>
                            <p>"The WildcardSegment ensures pre-hydration clicks and hard refreshes always get the full HTML shell."</p>
                        </div>
                    </li>
                </ul>
            </section>

            <div style="margin-top: 1.5rem;">
                <A href="/" attr:class="ghost-button">"← Back to the demo"</A>
            </div>
        </main>
    }
}
