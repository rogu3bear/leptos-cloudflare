use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use super::ui::{ActionLink, ControlTone, EvidenceKind, EvidenceTag};

#[component]
pub fn PatternsPage() -> impl IntoView {
    view! {
        <Title text="Patterns — Leptos CF"/>
        <Meta name="description" content="Core and optional full-stack Leptos patterns in the leptos-cf starter."/>

        <div class="page-shell section-stack section-stack--section">
            <header class="page-intro">
                <div class="page-intro__copy section-stack section-stack--related"><p class="eyebrow">"Field note 004 · capability"</p><h1>"A small core. Explicit extensions."</h1><p>"The starter keeps the default runtime narrow. Adopt a pattern when its state, security, and delivery owner are known—not because the framework can express it."</p></div>
                <div class="page-intro__aside"><EvidenceTag kind=EvidenceKind::Source/><p>"Availability means a checked-in implementation or documented adoption lane, not a hosted feature."</p></div>
            </header>

            <section class="pattern-index" aria-label="Pattern index">
                <article class="pattern-card pattern-card--core"><span>"Core · included"</span><h2>"SSR + hydration"</h2><p>"Useful HTML at the edge, then browser interaction from the same Leptos application."</p><code>"src/app.rs · src/lib.rs"</code></article>
                <article class="pattern-card pattern-card--core"><span>"Core · included"</span><h2>"Typed server functions"</h2><p>"A public client/server boundary with server-side validation and explicit error mapping."</p><code>"src/api.rs"</code></article>
                <article class="pattern-card pattern-card--core"><span>"Core · included"</span><h2>"Session-scoped D1"</h2><p>"A bounded mutation lab that keeps one browser session’s records separate."</p><code>"src/server/todos.rs"</code></article>
                <article class="pattern-card pattern-card--core"><span>"Core · included"</span><h2>"Hashed assets"</h2><p>"Compile-time asset hashes, immutable bundles, and a generated Worker fallthrough contract."</p><code>"scripts/hash-assets.mjs"</code></article>
                <article class="pattern-card pattern-card--optional"><span>"Pattern · optional"</span><h2>"Durable realtime"</h2><p>"Move rooms, presence, fanout, and reconnect state into a Durable Object before adding bindings."</p><code>"patterns/realtime-durable-object/"</code></article>
                <article class="pattern-card pattern-card--bounded"><span>"Capability · bounded"</span><h2>"Contact intake"</h2><p>"Same-origin validation and D1 persistence without email, webhook, Queue, or support-delivery claims."</p><code>"src/server/contact.rs"</code></article>
            </section>

            <section class="decision-questions" aria-labelledby="adoption-questions-title">
                <p class="plate__label">"Before adopting"</p><h2 id="adoption-questions-title">"Ask the boundary questions first."</h2>
                <ol><li>"Who owns the data after this request ends?"</li><li>"What must work before hydration?"</li><li>"Which endpoint becomes public?"</li><li>"What loading, empty, denied, error, and recovery states exist?"</li><li>"Which exact build and live readbacks prove delivery?"</li></ol>
            </section>

            <div class="action-cluster"><ActionLink href="/lab">"Inspect the lab"</ActionLink><ActionLink href="/start" tone=ControlTone::Secondary>"Open the start path"</ActionLink></div>
        </div>
    }
}
