use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use super::ui::{ActionLink, ControlSize, ControlTone, EvidenceKind, EvidenceTag};

#[component]
pub fn StartPage() -> impl IntoView {
    view! {
        <Title text="Start — Leptos CF"/>
        <Meta name="description" content="The checked-in local start and verification path for leptos-cf."/>

        <div class="page-shell section-stack section-stack--section">
            <header class="page-intro">
                <div class="page-intro__copy section-stack section-stack--related">
                    <p class="eyebrow">"Field note 002 · adoption"</p>
                    <h1>"From source to a verified local edge."</h1>
                    <p>"The safe path is intentionally local-first. Configure real provider state only after the source, bindings, and release-shaped build make sense."</p>
                </div>
                <div class="page-intro__aside">
                    <EvidenceTag kind=EvidenceKind::Source/>
                    <p>"Commands are bound to checked-in scripts and the pinned Wrangler version."</p>
                </div>
            </header>

            <section class="step-ledger" aria-label="Local start sequence">
                <article class="command-step">
                    <span class="command-step__number">"01"</span>
                    <div><h2>"Clone the reference"</h2><p>"Keep the original source visible until your application cutover is proven."</p><code>"git clone https://github.com/rogu3bear/leptos-cloudflare.git my-app"</code></div>
                </article>
                <article class="command-step">
                    <span class="command-step__number">"02"</span>
                    <div><h2>"Check the toolchain"</h2><p>"Resolve the repository’s actual cargo-leptos, Worker, and wasm-bindgen requirements."</p><code>"./scripts/check-deps.sh"</code></div>
                </article>
                <article class="command-step">
                    <span class="command-step__number">"03"</span>
                    <div><h2>"Prepare local D1"</h2><p>"Apply checked-in migrations to the local replica. This does not mutate the remote database."</p><code>"CI=1 bunx wrangler@4.120.1 d1 migrations apply leptos-cf-db --local"</code></div>
                </article>
                <article class="command-step">
                    <span class="command-step__number">"04"</span>
                    <div><h2>"Build the deployment shape"</h2><p>"Compile hydration, SSR, hashed assets, and the generated Worker shim together."</p><code>"bash ./scripts/build-edge.sh"</code></div>
                </article>
                <article class="command-step">
                    <span class="command-step__number">"05"</span>
                    <div><h2>"Run locally"</h2><p>"Serve the Worker, Assets binding, SSR fallback, and local D1 at one origin."</p><code>"CI=1 bunx wrangler@4.120.1 dev --local --ip 127.0.0.1 --port 57581"</code></div>
                </article>
                <article class="command-step command-step--gate">
                    <span class="command-step__number">"06"</span>
                    <div><h2>"Earn the local release claim"</h2><p>"Before review or release, run the repository’s complete local source of truth."</p><code>"./scripts/verify.sh"</code></div>
                </article>
            </section>

            <section class="caveat-plate" aria-labelledby="start-caveat-title">
                <EvidenceTag kind=EvidenceKind::Unproven/>
                <div>
                    <h2 id="start-caveat-title">"Production is not initialized yet."</h2>
                    <p>"The checked-in `wrangler.toml` deliberately contains placeholder D1 identifiers. Creating provider resources, applying remote migrations, and deploying are separate governed changes. The current `scripts/init.sh` path is not part of this verified start sequence; prefer an explicit, reviewed application cutover."</p>
                </div>
            </section>

            <div class="action-cluster">
                <ActionLink href="/architecture">"Understand the architecture"</ActionLink>
                <ActionLink href="/lab" size=ControlSize::Compact tone=ControlTone::Quiet>"Open the local lab"</ActionLink>
            </div>
        </div>
    }
}
