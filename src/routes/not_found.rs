use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="container not-found">
            <p class="eyebrow">"404"</p>
            <h1 class="display small">"That page didn't land."</h1>
            <p class="lede">
                "The address you followed isn't in the atlas. Try the explorer — everything is reachable from there."
            </p>
            <A href="/explore" attr:class="btn primary">"Go to explorer"</A>
        </section>
    }
}
