use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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

        <Router>
            <Routes fallback=|| view! { <p class="route-miss">"Page not found."</p> }.into_view()>
                <Route path=StaticSegment("") view=TodoPage/>
            </Routes>
        </Router>
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
    let pkg_dir = options.site_pkg_dir.as_ref();

    if hash.is_empty() {
        format!("/{pkg_dir}/{output_name}.{extension}")
    } else {
        format!("/{pkg_dir}/{output_name}.{hash}.{extension}")
    }
}
