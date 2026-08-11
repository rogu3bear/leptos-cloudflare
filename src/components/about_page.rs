use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use super::ui::{ActionLink, ControlTone, EvidenceKind, EvidenceTag};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About & trust — Leptos CF"/>
        <Meta name="description" content="The product, security, and proof-plane principles behind the leptos-cf starter."/>

        <div class="page-shell section-stack section-stack--section">
            <header class="page-intro">
                <div class="page-intro__copy section-stack section-stack--related">
                    <p class="eyebrow">"Field note 007 · trust"</p>
                    <h1>"A starter should reveal the critical path."</h1>
                    <p>"leptos-cf is a public reference implementation for Leptos 0.8 on Cloudflare Workers. Its value is not hidden magic; it is explicit ownership, fail-closed defaults, and a local release gate that matches the deployment shape."</p>
                </div>
                <div class="page-intro__aside"><EvidenceTag kind=EvidenceKind::Source/><p>{format!("Repository package version {}.", env!("CARGO_PKG_VERSION"))}</p></div>
            </header>

            <section class="principle-grid" aria-label="Product principles">
                <article><span>"01"</span><h2>"Boundaries before features"</h2><p>"SSR, hydration, server functions, assets, D1, and WebSockets name their owner."</p></article>
                <article><span>"02"</span><h2>"Checked-in before ambient"</h2><p>"Build inputs resolve from source and lockfiles rather than whatever the machine happens to provide."</p></article>
                <article><span>"03"</span><h2>"Fail closed by default"</h2><p>"Secrets stay out of source; headers, body limits, scoped sessions, and credential boundaries remain intact."</p></article>
                <article><span>"04"</span><h2>"Proof names its plane"</h2><p>"Source, local verification, review, provider state, and live behavior are never collapsed into one green badge."</p></article>
            </section>

            <section class="plate" aria-labelledby="evidence-legend-title">
                <header class="plate__header"><p class="plate__label">"Evidence legend"</p><div><h2 id="evidence-legend-title">"Know what each claim can carry."</h2></div></header>
                <div class="evidence-ledger">
                    <div><EvidenceTag kind=EvidenceKind::Source/><p>"Supported by checked-in source or doctrine."</p></div>
                    <div><EvidenceTag kind=EvidenceKind::Browser/><p>"Seen in a named local browser state and viewport."</p></div>
                    <div><EvidenceTag kind=EvidenceKind::Local/><p>"Verified on one exact local tree."</p></div>
                    <div><EvidenceTag kind=EvidenceKind::Provider/><p>"Read back from Cloudflare after an approved mutation."</p></div>
                    <div><EvidenceTag kind=EvidenceKind::Unproven/><p>"Not established yet, or outside the current proof plane."</p></div>
                </div>
            </section>

            <section class="caveat-plate"><EvidenceTag kind=EvidenceKind::Unproven/><div><h2>"This public template is not a service promise."</h2><p>"The task board and intake form are bounded implementation labs. The repository does not claim customers, uptime, telemetry, support delivery, or a production deployment until those states are separately proven."</p></div></section>

            <div class="action-cluster"><ActionLink href="/start">"Use the starter"</ActionLink><ActionLink href="/architecture" tone=ControlTone::Secondary>"Trace the architecture"</ActionLink></div>
        </div>
    }
}
