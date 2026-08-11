use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use super::ui::{ActionLink, ControlTone, EvidenceKind, EvidenceTag};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Leptos CF — See every boundary before you ship"/>
        <Meta
            name="description"
            content="A source-derived field guide and production-minded starter for full-stack Leptos on Cloudflare Workers."
        />

        <div class="page-shell page-shell--home">
            <section class="field-hero" aria-labelledby="home-title">
                <div class="field-hero__copy section-stack section-stack--related">
                    <p class="eyebrow">"Edge field guide · Leptos 0.8"</p>
                    <h1 id="home-title">"See every boundary before you ship."</h1>
                    <p class="field-hero__lede">
                        "A production-minded reference for building full-stack Leptos applications on Cloudflare Workers. Trace the path, understand the owners, then start from checked-in proof."
                    </p>
                    <div class="action-cluster">
                        <ActionLink href="/start">"Use the starter" <span aria-hidden="true">"→"</span></ActionLink>
                        <ActionLink href="/architecture" tone=ControlTone::Secondary>
                            "Trace the request"
                        </ActionLink>
                    </div>
                </div>

                <dl class="folio" aria-label="Field guide facts">
                    <div><dt>"Folio"</dt><dd>"001"</dd></div>
                    <div><dt>"Subject"</dt><dd>"Request path"</dd></div>
                    <div><dt>"Status"</dt><dd><EvidenceTag kind=EvidenceKind::Source/></dd></div>
                    <div><dt>"Scale"</dt><dd>"Conceptual"</dd></div>
                </dl>
            </section>

            <section class="plate" aria-labelledby="request-path-title">
                <header class="plate__header">
                    <p class="plate__label">"Plate 1 · request path"</p>
                    <div>
                        <h2 id="request-path-title">"One request. Explicit owners."</h2>
                        <p>"The path below is source-derived, not a live trace."</p>
                    </div>
                </header>

                <ol class="request-path">
                    <li class="boundary-step boundary-step--browser">
                        <span class="boundary-step__number">"01"</span>
                        <div><h3>"Browser"</h3><p>"Navigation, intent, then hydration."</p></div>
                    </li>
                    <li class="boundary-step boundary-step--edge">
                        <span class="boundary-step__number">"02"</span>
                        <div><h3>"Asset router"</h3><p>"Serve exact files before user code."</p></div>
                    </li>
                    <li class="boundary-step boundary-step--edge">
                        <span class="boundary-step__number">"03"</span>
                        <div><h3>"Worker"</h3><p>"Dispatch realtime or fall through to SSR."</p></div>
                    </li>
                    <li class="boundary-step boundary-step--data">
                        <span class="boundary-step__number">"04"</span>
                        <div><h3>"Leptos + D1"</h3><p>"Load scoped data and render useful HTML."</p></div>
                    </li>
                    <li class="boundary-step boundary-step--edge">
                        <span class="boundary-step__number">"05"</span>
                        <div><h3>"HTML"</h3><p>"Stream the response to the browser."</p></div>
                    </li>
                    <li class="boundary-step boundary-step--browser">
                        <span class="boundary-step__number">"06"</span>
                        <div><h3>"Hydration"</h3><p>"Resume the same app and interact."</p></div>
                    </li>
                </ol>
                <p class="plate__note">"An exact asset exits at step 02. A document continues through SSR; later server-function calls and data loads cross the boundary again."</p>
            </section>

            <section class="plate split-plate" aria-labelledby="two-wasm-title">
                <div class="split-plate__intro section-stack section-stack--related">
                    <p class="plate__label">"Plate 2 · two-WASM model"</p>
                    <h2 id="two-wasm-title">"Two runtimes, one mental model."</h2>
                    <p>"One compilation target renders at the edge. Another hydrates the returned HTML in the browser. Source ownership stays explicit across both."</p>
                </div>
                <div class="runtime-lanes">
                    <article class="runtime-lane runtime-lane--edge">
                        <span class="runtime-lane__where">"At the edge"</span>
                        <h3>"leptos-ssr (WASM)"</h3>
                        <ul><li>"Match routes"</li><li>"Load scoped data"</li><li>"Render and stream HTML"</li></ul>
                    </article>
                    <article class="runtime-lane runtime-lane--browser">
                        <span class="runtime-lane__where">"In the browser"</span>
                        <h3>"leptos (WASM)"</h3>
                        <ul><li>"Hydrate existing HTML"</li><li>"Handle events"</li><li>"Resume server actions"</li></ul>
                    </article>
                </div>
            </section>

            <section class="plate" aria-labelledby="proof-title">
                <header class="plate__header">
                    <p class="plate__label">"Plate 3 · proof planes"</p>
                    <div><h2 id="proof-title">"A green check must say what it proves."</h2></div>
                </header>
                <div class="proof-grid">
                    <article><EvidenceTag kind=EvidenceKind::Source/><h3>"Repository truth"</h3><p>"The route, binding, or control exists in checked-in source."</p></article>
                    <article><EvidenceTag kind=EvidenceKind::Local/><h3>"Exact-tree proof"</h3><p>"The local release gate passed against one identifiable tree."</p></article>
                    <article><EvidenceTag kind=EvidenceKind::Provider/><h3>"Cloudflare state"</h3><p>"Only a post-apply provider readback can establish it."</p></article>
                    <article><EvidenceTag kind=EvidenceKind::Unproven/><h3>"No visual laundering"</h3><p>"Unknown deployment or usage claims stay visibly unknown."</p></article>
                </div>
            </section>

            <section class="launch-strip" aria-labelledby="launch-strip-title">
                <div>
                    <p class="plate__label">"Start with the source"</p>
                    <h2 id="launch-strip-title">"Map the path. Make it yours. Verify before launch."</h2>
                </div>
                <ActionLink href="/start">"Open the start path" <span aria-hidden="true">"→"</span></ActionLink>
            </section>
        </div>
    }
}
