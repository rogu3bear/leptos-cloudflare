use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, SsrMode, StaticSegment, WildcardSegment,
};

use crate::components::about_page::AboutPage;
use crate::components::app_layout::AppLayout;
use crate::components::architecture_page::ArchitecturePage;
use crate::components::contact_page::ContactPage;
use crate::components::home_page::HomePage;
use crate::components::patterns_page::PatternsPage;
use crate::components::start_page::StartPage;
use crate::components::todo_detail_page::TodoDetailPage;
use crate::components::todo_page::TodoPage;

#[allow(dead_code)]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.svg" type="image/svg+xml"/>
                <link rel="apple-touch-icon" href="/apple-touch-icon.png"/>
                <link rel="manifest" href="/site.webmanifest"/>
                <meta name="theme-color" content="#f3ead7"/>
                <AutoReload options=options.clone()/>
                <HashedStylesheet options=options.clone()/>
                <EdgeHydrationScripts options=options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Leptos CF"/>
        <Meta
            name="description"
            content="A source-derived field guide and full-stack Leptos starter for Cloudflare Workers."
        />

        <Router>
            // AppLayout provides a persistent header + navigation across all pages.
            // Keeping it inside Router gives the shared nav links their routing context.
            <AppLayout>
                <Routes fallback=|| view! { <NotFoundPage/> }.into_view()>
                    // Public field-guide routes.
                    <Route path=StaticSegment("") view=HomePage ssr=SsrMode::OutOfOrder/>
                    <Route path=StaticSegment("start") view=StartPage ssr=SsrMode::OutOfOrder/>
                    <Route path=StaticSegment("architecture") view=ArchitecturePage ssr=SsrMode::OutOfOrder/>
                    <Route path=StaticSegment("patterns") view=PatternsPage ssr=SsrMode::OutOfOrder/>
                    <Route path=StaticSegment("lab") view=TodoPage ssr=SsrMode::OutOfOrder/>
                    <Route path=StaticSegment("about") view=AboutPage ssr=SsrMode::OutOfOrder/>
                    <Route path=StaticSegment("contact") view=ContactPage ssr=SsrMode::OutOfOrder/>

                    // Dynamic route using ParamSegment — demonstrates clean entity
                    // detail pages powered by dedicated server functions + reactivity.
                    <Route
                        path=(StaticSegment("lab"), ParamSegment("id"))
                        view=TodoDetailPage
                        ssr=SsrMode::OutOfOrder
                    />
                    // Preserve old deep links while consumers move to `/lab/:id`.
                    <Route
                        path=(StaticSegment("todo"), ParamSegment("id"))
                        view=TodoDetailPage
                        ssr=SsrMode::OutOfOrder
                    />

                    // Critical for Cloudflare + Leptos on the edge:
                    // This must be last. It ensures deep links and pre-hydration
                    // requests get a full SSR HTML shell (in cooperation with
                    // the generated `build/_worker.js`).
                    <Route path=WildcardSegment("any") view=NotFoundPage ssr=SsrMode::OutOfOrder/>
                </Routes>
            </AppLayout>
        </Router>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Page not found — Leptos CF"/>
        <div class="page-shell page-shell--compact">
            <section class="route-miss section-stack section-stack--related">
                <p class="eyebrow">"Unknown folio"</p>
                <h1>"This route is outside the field guide."</h1>
                <p>"Return to the request map or open the checked-in start path."</p>
                <div class="action-cluster">
                    <a class="control-frame control-frame--standard control-frame--primary" href="/">"Field guide"</a>
                    <a class="control-frame control-frame--standard control-frame--secondary" href="/start">"Use the starter"</a>
                </div>
            </section>
        </div>
    }
}

#[component]
fn HashedStylesheet(options: LeptosOptions) -> impl IntoView {
    let href = asset_href(&options, "css", crate::asset_hashes::CSS_HASH);

    view! {
        <link id="leptos" rel="stylesheet" href=href/>
    }
}

#[component]
fn EdgeHydrationScripts(options: LeptosOptions) -> impl IntoView {
    let js_href = asset_href(&options, "js", crate::asset_hashes::JS_HASH);
    let wasm_href = asset_href(&options, "wasm", crate::asset_hashes::WASM_HASH);
    #[cfg(feature = "ssr")]
    let nonce = leptos::nonce::use_nonce();
    #[cfg(not(feature = "ssr"))]
    let nonce = None::<String>;
    let hydration_script = format!(
        "import({js_href:?}).then(mod => {{ mod.default({{ module_or_path: {wasm_href:?} }}).then(() => {{ mod.hydrate(); }}); }});"
    );

    view! {
        <link rel="modulepreload" href=js_href.clone()/>
        <link rel="preload" href=wasm_href.clone() r#as="fetch" r#type="application/wasm"/>
        <script type="module" nonce=nonce>{hydration_script}</script>
    }
}

fn asset_href(options: &LeptosOptions, extension: &str, hash: &str) -> String {
    let output_name = options.output_name.as_ref();
    let output_name = if output_name.is_empty() {
        env!("CARGO_PKG_NAME")
    } else {
        output_name
    };
    let pkg_dir = options.site_pkg_dir.as_ref();

    if hash.is_empty() {
        format!("/{pkg_dir}/{output_name}.{extension}")
    } else {
        format!("/{pkg_dir}/{output_name}.{hash}.{extension}")
    }
}
