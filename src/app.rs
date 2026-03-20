use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
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
                <HydrationScripts options/>
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
        <Stylesheet id="leptos" href="/pkg/leptos-cf.css"/>
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
