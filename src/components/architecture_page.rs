use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use super::ui::{ActionLink, ControlTone, EvidenceKind, EvidenceTag};

#[component]
pub fn ArchitecturePage() -> impl IntoView {
    view! {
        <Title text="Architecture — Leptos CF"/>
        <Meta name="description" content="The rendering, request-routing, and Cloudflare platform decisions behind leptos-cf."/>

        <div class="page-shell section-stack section-stack--section">
            <header class="page-intro">
                <div class="page-intro__copy section-stack section-stack--related">
                    <p class="eyebrow">"Field note 003 · ownership"</p>
                    <h1>"A boundary is useful when its owner is named."</h1>
                    <p>"leptos-cf keeps browser intent, platform asset routing, Worker execution, server rendering, durable bindings, streamed HTML, and hydration visible as separate responsibilities."</p>
                </div>
                <div class="page-intro__aside"><EvidenceTag kind=EvidenceKind::Source/><p>"This map describes checked-in architecture, not runtime telemetry."</p></div>
            </header>

            <section class="plate" aria-labelledby="rendering-decision-title">
                <header class="plate__header">
                    <p class="plate__label">"Rendering decision"</p>
                    <div>
                        <h2 id="rendering-decision-title">"SSR and CSR are modes, not rival products."</h2>
                        <p>"Choose where the first useful UI is rendered, then decide whether browser WASM should resume it. This template ships the balanced default: SSR + hydration."</p>
                    </div>
                </header>
                <div class="decision-table-wrap">
                    <table class="decision-table">
                        <thead>
                            <tr>
                                <th scope="col">"Mode"</th>
                                <th scope="col">"First document"</th>
                                <th scope="col">"After load"</th>
                                <th scope="col">"Choose it when"</th>
                                <th scope="col">"Template status"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="decision-table__current">
                                <th scope="row">"SSR + hydration"</th>
                                <td>"The Worker streams useful route HTML."</td>
                                <td>"Browser WASM hydrates that DOM. Router navigation avoids a new document request; route data may still call server functions."</td>
                                <td>"You need fast first paint, indexable content, and app-like interaction."</td>
                                <td><strong>"Current template"</strong></td>
                            </tr>
                            <tr>
                                <th scope="row">"SSR only"</th>
                                <td>"The Worker returns useful HTML for every page request."</td>
                                <td>"Links and forms rely on document requests unless small client enhancements are added."</td>
                                <td>"The product is content-led and does not need a persistent reactive client."</td>
                                <td>"Supported direction; not a packaged lane"</td>
                            </tr>
                            <tr>
                                <th scope="row">"Pure CSR"</th>
                                <td>"The platform serves an HTML shell; browser WASM creates the useful UI."</td>
                                <td>"The browser owns rendering and calls APIs for server data."</td>
                                <td>"A named, isolated surface values static hosting or offline behavior more than first-response HTML."</td>
                                <td>"Deferred until a consumer needs it"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <p class="decision-note"><strong>"What this means now:"</strong> "keep SSR + hydration as the core. If pure CSR earns a real use case, add it as an isolated Workers Static Assets pattern with its own entrypoint and proof—do not turn the whole template into an SPA."</p>
            </section>

            <section class="plate" aria-labelledby="platform-decision-title">
                <header class="plate__header">
                    <p class="plate__label">"Platform decision"</p>
                    <div>
                        <h2 id="platform-decision-title">"Workers owns the full-stack lane."</h2>
                        <p>"Workers and Pages can both reach the Workers runtime, but they create different project, routing, deployment, and proof boundaries."</p>
                    </div>
                </header>
                <div class="decision-table-wrap">
                    <table class="decision-table decision-table--platform">
                        <thead>
                            <tr>
                                <th scope="col">"Question"</th>
                                <th scope="col">"Workers Static Assets"</th>
                                <th scope="col">"Cloudflare Pages"</th>
                                <th scope="col">"Decision"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th scope="row">"What deploys?"</th>
                                <td>"The Worker and `target/site` assets deploy as one unit."</td>
                                <td>"A Pages project deploys static output; Pages Functions add server execution."</td>
                                <td><strong>"Workers"</strong></td>
                            </tr>
                            <tr>
                                <th scope="row">"Who owns routing?"</th>
                                <td>"The platform asset router serves exact files; non-assets reach the generated Worker entrypoint."</td>
                                <td>"Pages routing and any Functions adapter become another maintained contract."</td>
                                <td>"Keep one routing owner"</td>
                            </tr>
                            <tr>
                                <th scope="row">"When is it justified?"</th>
                                <td>"The current SSR, server-function, D1, Assets, and realtime boundaries need one runtime."</td>
                                <td>"A named consumer already requires a separate Pages project or Pages-native workflow."</td>
                                <td>"No Pages lane yet"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <p class="decision-note"><strong>"Checked in now:"</strong> "the Workers lane has executable rendering/network boundaries, a current compatibility date, and sampled production observability. Next, an authorized deployment must earn provider readback and live evidence. Add Pages only with a named consumer and an independent maintenance and release-proof owner."</p>
            </section>

            <section class="plate" aria-labelledby="asset-router-title">
                <header class="plate__header">
                    <p class="plate__label">"Platform asset router"</p>
                    <div>
                        <h2 id="asset-router-title">"The first branch happens before Worker code."</h2>
                        <p>"The `[assets]` binding gives Cloudflare the static directory and keeps non-asset requests on the SSR path."</p>
                    </div>
                </header>
                <ol class="request-flow" aria-label="Cloudflare request routing flow">
                    <li><span>"01"</span><div><strong>"Browser request"</strong><p>"A document, identity file, hashed `/pkg/` artifact, API call, or realtime upgrade reaches Cloudflare."</p></div></li>
                    <li><span>"02"</span><div><strong>"Exact asset match?"</strong><p>"Yes: Workers Static Assets responds without invoking user Worker code. No: the request continues to the Worker entrypoint."</p></div></li>
                    <li><span>"03"</span><div><strong>"Worker entrypoint"</strong><p>"The generated shim owns `/realtime/socket`; its explicit asset delegation is a safe fallback if it receives an asset path."</p></div></li>
                    <li><span>"04"</span><div><strong>"Leptos SSR"</strong><p>"Every remaining route reaches Axum + Leptos for pages, typed server functions, status handling, and security headers."</p></div></li>
                    <li><span>"05"</span><div><strong>"Hydration"</strong><p>"The returned HTML loads hashed client assets, then browser WASM resumes the server-rendered DOM."</p></div></li>
                </ol>
            </section>

            <section class="ownership-ledger" aria-label="Request ownership ledger">
                <article><span>"01 · Browser"</span><h2>"Initiates"</h2><p>"Navigates, submits intent, receives HTML, then runs the hydration bundle."</p><strong>"Owner: user agent"</strong></article>
                <article><span>"02 · Asset router"</span><h2>"Branches"</h2><p>"Serves exact static matches before user code and sends non-assets to the Worker."</p><strong>"Owner: Workers Static Assets"</strong></article>
                <article><span>"03 · Worker shim"</span><h2>"Dispatches"</h2><p>"Handles the WebSocket lane, preserves explicit asset delegation, then falls through to SSR."</p><strong>"Owner: generated Worker entrypoint"</strong></article>
                <article><span>"04 · Axum + Leptos"</span><h2>"Renders"</h2><p>"Matches routes, supplies app context, runs server functions, and streams useful HTML."</p><strong>"Owner: Rust SSR bundle"</strong></article>
                <article><span>"05 · D1"</span><h2>"Persists"</h2><p>"D1 owns session-scoped records and is available only through the Worker binding."</p><strong>"Owner: Cloudflare binding"</strong></article>
                <article><span>"06 · Hydration"</span><h2>"Resumes"</h2><p>"The client WASM binds events to server-rendered markup without replacing the application model."</p><strong>"Owner: browser bundle"</strong></article>
                <article><span>"07 · Realtime lane"</span><h2>"Upgrades"</h2><p>"Only `/realtime/socket` upgrades in the template. Shared rooms and presence require Durable Objects."</p><strong>"Owner: Worker, then adopted pattern"</strong></article>
            </section>

            <section class="plate" aria-labelledby="delivery-contract-title">
                <header class="plate__header"><p class="plate__label">"Delivery contract"</p><div><h2 id="delivery-contract-title">"Declaration is not delivery proof."</h2><p>"A route closes only when source, server mount, navigation, host fallback, assets, hydration, and rendered behavior agree."</p></div></header>
                <div class="proof-grid proof-grid--three">
                    <article><h3>"Route tree"</h3><p>"`AppLayout` stays inside `Router`; the wildcard remains last."</p></article>
                    <article><h3>"Static assets"</h3><p>"Workers Assets serves named identity files and immutable `/pkg/` artifacts."</p></article>
                    <article><h3>"Dynamic work"</h3><p>"Non-asset requests fall through to Leptos SSR and typed server functions."</p></article>
                </div>
            </section>

            <section class="launch-strip">
                <div><p class="plate__label">"Next plate"</p><h2>"Choose a pattern only after its owner is clear."</h2></div>
                <div class="action-cluster"><ActionLink href="/patterns">"Explore patterns"</ActionLink><ActionLink href="/start" tone=ControlTone::Secondary>"Use the starter"</ActionLink></div>
            </section>
        </div>
    }
}
