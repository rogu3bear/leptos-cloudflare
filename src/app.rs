use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};

use crate::routes::{About, Compare, Explorer, Home, LanguageDetail, NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Langual — vocal capability divergence across human languages"/>
        <Meta
            name="description"
            content="A grounded-truth explorer of phonological divergence across the world's languages, sourced from PHOIBLE, Maddieson, and Ladefoged & Maddieson."
        />

        <Router>
            <Shell/>
        </Router>
    }
}

#[component]
fn Shell() -> impl IntoView {
    view! {
        <a class="skip-link" href="#main">"Skip to content"</a>
        <header class="site-header">
            <div class="container header-row">
                <A href="/" attr:class="brand" attr:aria-label="Langual — home">
                    <span class="brand-glyph" aria-hidden="true">"ʟ"</span>
                    <span class="brand-word">"Langual"</span>
                </A>
                <nav class="primary-nav" aria-label="Primary">
                    <A href="/explore">"Explore"</A>
                    <A href="/compare">"Compare"</A>
                    <A href="/about">"About"</A>
                </nav>
            </div>
        </header>

        <main id="main" role="main">
            <Routes fallback=NotFound>
                <Route path=path!("/") view=Home/>
                <Route path=path!("/explore") view=Explorer/>
                <Route path=path!("/language/:id") view=LanguageDetail/>
                <Route path=path!("/compare") view=Compare/>
                <Route path=path!("/about") view=About/>
            </Routes>
        </main>

        <footer class="site-footer">
            <div class="container footer-row">
                <p class="footer-mark">
                    <span class="mono">"langual"</span>
                    " — human voice as typology."
                </p>
                <p class="footer-sources">
                    "Data: "
                    <a href="https://phoible.org" rel="noopener">"PHOIBLE 2.0"</a>
                    ", "
                    <a href="https://wals.info" rel="noopener">"WALS Online"</a>
                    ", Maddieson (1984), Ladefoged & Maddieson (1996)."
                </p>
            </div>
        </footer>
    }
}
