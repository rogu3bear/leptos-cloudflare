use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, SsrMode, StaticSegment, WildcardSegment,
};

use crate::components::about_page::AboutPage;
use crate::components::app_layout::AppLayout;
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
                <meta name="theme-color" content="#171412"/>
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
        <Title text="Leptos CF Starter"/>
        <Meta
            name="description"
            content="A full-stack Leptos starter for Cloudflare Workers with D1-backed todos."
        />

        // AppLayout provides a persistent header + navigation across all pages.
        // This is a practical, production-common way to achieve shared layout UI
        // that works cleanly with Leptos SSR + hydration on the edge.
        //
        // For more advanced router-driven layouts using `<Outlet/>` with deeply
        // nested routes, see the comments in AppLayout and the Leptos router docs.
        // The exact declarative nesting syntax can be sensitive to leptos_router version.
        <AppLayout>
            <Router>
                <Routes fallback=|| view! { <NotFoundPage/> }.into_view()>
                    // Main content routes
                    <Route path=StaticSegment("") view=TodoPage ssr=SsrMode::OutOfOrder/>
                    <Route path=StaticSegment("about") view=AboutPage ssr=SsrMode::OutOfOrder/>

                    // Dynamic route using ParamSegment — demonstrates clean entity
                    // detail pages powered by dedicated server functions + reactivity.
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
            </Router>
        </AppLayout>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <main class="page-shell">
            <section class="feedback feedback--error">
                <p>"Page not found."</p>
            </section>
        </main>
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
    let hydration_script = format!(
        "import({js_href:?}).then(mod => {{ mod.default({{ module_or_path: {wasm_href:?} }}).then(() => {{ mod.hydrate(); }}); }});"
    );

    view! {
        <link rel="modulepreload" href=js_href.clone()/>
        <link rel="preload" href=wasm_href.clone() r#as="fetch" r#type="application/wasm"/>
        <script type="module">{hydration_script}</script>
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
