# HORIZON — The Edge Field Guide

Created: 2026-08-05
Stage: reviewed

This is the sole design authority for the scoped surface. `NORTH_STAR.md`
contributes product semantics—users, jobs, promises, capabilities, domain truth,
and safety/trust constraints—but contributes no visual or interaction design
direction. Existing UI is functional evidence, not a style source, unless this
file explicitly carries something forward.

## 1. Frame

- Surface and routes: `/`, `/start`, `/architecture`, `/patterns`, `/lab`, `/lab/:id`, `/contact`, `/about`, and the not-found state.
- Primary user: a Rust or full-stack engineer evaluating a reproducible Leptos-on-Cloudflare foundation.
- Primary job and success state: understand the request and ownership boundaries quickly enough to decide whether to adopt the starter, then reach a truthful start path without mistaking a demo for production proof.
- Design ambition: make infrastructure legible and memorable without pretending the site has live telemetry or endorsements.
- Scope: public marketing, educational, starter, and bounded demonstration surfaces in this repository.
- Non-goals: changing the Worker/SSR contract, inventing product usage claims, turning the task-board or Contact labs into a production SaaS, or running `scripts/init.sh`.

## 2. Semantic Product Contract

| Product meaning | Citation | Capability or behavior that must remain true | Design freedom |
|---|---|---|---|
| The repository is a public production-minded starter. | `NORTH_STAR.md:3` | The site identifies the artifact as a starter and exposes an adoption path. | Any clear presentation that does not overstate deployed proof. |
| Humans and coding agents should see the correct path. | `NORTH_STAR.md:5` | Architecture, start, and release guidance remain findable and sequenced. | Navigation, hierarchy, route names, and visual language. |
| Runtime responsibilities are explicit. | `NORTH_STAR.md:7` | SSR, hydration, server functions, assets, D1, and WebSockets are described with distinct owners. | Diagrams, labels, annotations, and responsive composition. |
| Security fails closed. | `NORTH_STAR.md:9` | No secret-handling claim weakens the checked-in controls or token doctrine. | Security content hierarchy and status vocabulary. |
| Release claims require matching local proof. | `NORTH_STAR.md:10` | The site distinguishes source-derived, locally verified, provider-observed, and unproven statements. | Evidence labels, ledgers, and explanatory copy. |

## 3. Excluded North Star Design Directives

| Citation | Excluded design instruction | Why it is design, not product meaning | HORIZON replacement |
|---|---|---|---|
| `NORTH_STAR.md:5` | “Make the correct path obvious” does not prescribe a dashboard, card grid, or persistent rail. | Those are presentation choices, not capability. | An editorial field-guide layout with a first-screen start action and annotated request map. |
| `NORTH_STAR.md:7` | The capability list does not prescribe iconography or a linear diagram. | Ownership truth survives different compositions. | Numbered plates and source-bound evidence labels. |
| `NORTH_STAR.md:10` | “Local proof” does not authorize green badges for checks not run. | Status treatment can create false factual claims. | Every status visibly names its proof plane and defaults to unproven until evidence exists. |

## 4. Ground Truth and Functional Inventory

### Observed

- `src/app.rs:49-74` puts `AppLayout` inside `Router`; this placement is a runtime invariant.
- `src/app.rs:55-71` currently owns the home, About, Contact, task-detail, and final wildcard routes.
- `src/components/todo_page.rs` demonstrates D1-backed server functions with loading, empty, error, create, toggle, and delete states.
- `src/components/contact_page.rs` writes a bounded contact record through a server action; it does not prove email delivery or operator response.
- `scripts/verify.sh` is the complete local release-readiness command. Remote CI is intentionally absent.

### Inferred

- Engineers evaluating this starter need architecture comprehension before feature spectacle.
- The existing task-board flow is more credible as a lab specimen than as the homepage identity.
- A warm field-manual visual language better supports inspection and annotation than generic application chrome.

### Proposed

- Reframe the site as “The Edge Field Guide” with “See every boundary before you ship.”
- Make “Use the starter” the primary conversion and “Trace the request” the educational secondary action.
- Use evidence labels: source-derived, browser-observed, locally verified, provider-observed, and unproven.

### Content and capability inventory

| Item | User value | Source | Required states | Keep, change, or remove |
|---|---|---|---|---|
| Request path | Explains where work happens. | `NORTH_STAR.md:7` and Worker/build docs | Static SSR content; narrow readable sequence. | Keep capability, replace presentation. |
| Start path | Converts evaluation into action. | `README.md`, `docs/agent-playbook.md` | Commands copyable without JavaScript; prerequisites and caveats visible. | Add dedicated route. |
| Patterns | Shows optional extension points. | `docs/building-features.md`, `docs/realtime.md` | Available/optional labels; no false live status. | Add dedicated route. |
| Task-board lab | Proves full-stack behavior locally. | `src/components/todo_page.rs` | Loading, empty, error, pending, success, and detail states. | Keep behavior, move to `/lab`. |
| Contact lab | Demonstrates bounded D1 intake. | `src/components/contact_page.rs` | Validation, disabled, pending, error, success. | Keep but relabel truthfully. |
| About and trust | Explains principles and proof planes. | Doctrine and release docs | Static SSR content. | Rewrite. |

### Current-design diagnosis

The current task-board-first homepage obscures the starter’s actual value, duplicate `<main>` landmarks weaken document structure, visual hierarchy is generic, nav styling is incomplete, inline spacing forks exist, and Contact looks more operational than its D1-only behavior warrants. Four stat cards are placed in a three-column grid and the narrow composer rule targets grid columns on a flex container.

## 5. Preserve and Replace Boundary

- Preserve product semantics: public starter, explicit runtime ownership, reproducible build inputs, fail-closed security, and proof-plane discipline.
- Preserve behavior: Router-wrapped layout, SSR/hydration split, server functions, D1 session scoping, wildcard-last routing, static asset serving, and the task-board/Contact mutations.
- Free to replace: all visual hierarchy, layout, navigation presentation, component shape, spacing, color, type, imagery, motion, and responsive form unless narrowed here.
- Existing visual elements intentionally retained: the semantic “Leptos CF” name only; existing styling is not authoritative.
- Explicit semantic deltas requiring owner approval: none. New copy must not imply a deployed service, customer adoption, or a working support channel.

## 6. Full Design Coverage

| Route, screen, or region | Primary task | Required content | Key interactions | Critical states | Wide/narrow coverage |
|---|---|---|---|---|---|
| `/` | Decide whether the starter fits. | Promise, request map, two-WASM model, evidence legend, next steps. | Navigate to start, architecture, lab. | Static SSR; no-JS links. | Both. |
| `/start` | Begin safely. | Prerequisites, clone/build/run/verify sequence, deployment caveat. | Copy/read commands and follow docs. | Prerequisite gaps and unconfigured Cloudflare bindings. | Both. |
| `/architecture` | Understand ownership. | Browser, Worker, Axum/Leptos SSR, D1/assets, hydration, realtime boundaries. | Jump among plates and docs. | Static source-derived content. | Both. |
| `/patterns` | Choose extensions. | Core versus optional patterns, adoption questions, source links. | Navigate to lab and documentation. | Available, optional, and out-of-scope labels. | Both. |
| `/lab` | Exercise a real mutation flow. | Existing task board plus truth label. | Create, toggle, inspect, delete. | Loading, empty, error, pending, success. | Both. |
| `/lab/:id` | Inspect one record. | Existing detail and controls. | Toggle, delete, return. | Loading, not found, error, pending, deleted. | Both. |
| `/contact` | Inspect bounded intake. | Scope warning and form. | Validate and submit to D1. | Invalid, disabled, pending, success, error. | Both. |
| `/about` | Evaluate trust model. | Principles, security posture, proof planes, license/source. | Navigate to start and repository docs. | Static SSR. | Both. |
| not found | Recover. | Clear miss and route choices. | Return home or start. | Unknown route. | Both. |

## 7. Hierarchy Contract

| Rank | Element or user question | Target region | Task/DOM order | Visual weight | Wide behavior | Narrow/state behavior | Rationale |
|---|---|---|---|---|---|---|---|
| 1 | What is this and why use it? | Hero main column | 1 | Large editorial headline and short promise | Copy and folio metadata share a split composition. | Metadata follows CTA; no visual reordering. | Five-second comprehension gates every later choice. |
| 2 | How does a request move? | First annotated plate | 2 | Full-width diagram with numbered boundary stops | Horizontal route map. | Vertical ordered steps with the same DOM order. | Architecture is the product’s differentiator. |
| 3 | How do I start? | Hero and launch strip | 3 | Coral primary action | Persistent first-screen action and repeated closing action. | Full-width touch target. | Adoption is the primary site job. |
| 4 | What is proven? | Evidence legend and plate notes | 4 | Compact monospaced labels | Inline beside claims. | Labels precede long explanations. | Prevents design from laundering uncertainty. |
| 5 | Can I inspect it? | Lab invitation | 5 | Secondary cyan link/action | Side-by-side with pattern and verification sections. | Stacked after architecture. | Demonstration follows comprehension. |

## 8. Creative Direction Contract

- Target perception and emotional register: an exacting field manual—calm, tactile, intelligent, and inspectable rather than futuristic or promotional.
- Product-native nouns and verbs: boundary, request, route, render, hydrate, bind, verify, start, inspect, ship.
- Desired contrast, rhythm, material, imagery, and motion: warm paper against charcoal ink; coral routes, cyan browser edges, amber Worker edges, green data; large editorial serif moments balanced by compact mono annotations; hairline rules and numbered plates; motion limited to subtle route tracing and state feedback.
- Range Creative Production may explore: topographic contours, folio metadata, marginal tabs, request arrows, deployment runways, and proof ledgers.
- Explicit avoid list: dark cyber dashboards, AI-purple gradients, fabricated monitoring, testimonial/logo walls, giant decorative code blocks, glassmorphism, and animation that delays content.
- Accessibility constraints that shape the art direction: AA text contrast, semantic DOM order, visible focus, 44px touch targets, no color-only status, reduced-motion support, 200% zoom without horizontal page overflow, and readable diagram alternatives.

## 9. Creative Production Territory

Status: selected from three generated territories under delegated visual discretion.

- Explore path and focused skill: image generation explored editorial field manual, flight recorder, and edge atlas territories.
- Intake context and acknowledged unknowns: no customer logos, traffic evidence, or live deployment proof exists; generated text is concept-only and not product copy authority.
- Durable mood-board or artifact path: `docs/design/edge-field-guide-concept.png`.
- Selected territory and exact item/asset references: the editorial field-manual territory shown in `docs/design/edge-field-guide-concept.png`.
- Palette, motif, material, imagery, composition, and audience cues to preserve: warm paper, charcoal typography, coral request route, cyan browser, amber edge, green state, annotated plates, marginal folio tabs, and an engineer-facing reference tone.
- Rejected territories and why: the dark edge atlas is cinematic but overweights spectacle; the flight recorder is evidence-rich but visually resembles live operations telemetry.
- Claims, logos, generated text, or product-fidelity caveats: all rendered generated text, commands, dates, version labels, repository URLs, check marks, and status values are illustrative and must be replaced by source-derived copy.

## 10. Product Design Options

### Direction A — Edge Field Guide

- Exact generated-image reference: `docs/design/edge-field-guide-concept.png`.
- Hierarchy thesis: adoption promise first, annotated request path second, labs and verification after comprehension.
- Layout grammar: editorial folio, asymmetric hero, full-width plates, three-column evidence blocks, marginal route tabs on wide screens.
- Interaction model: conventional links, calm section anchors, bounded lab controls, and optional route-trace motion.
- Full-design coverage: home, architecture, patterns, lab invitation, verification, quick start, footer; remaining routes inherit the plate grammar.
- Wide-to-narrow transformation: tabs become a compact nav; horizontal paths become numbered vertical sequences without DOM reordering.
- Creative Production territory use: direct use of the selected warm paper/manual vocabulary.
- Tradeoff: distinctive and explanatory, but requires disciplined restraint to avoid faux-vintage decoration.

### Direction B — Flight Recorder

- Exact generated-image reference: `/Users/star/.codex/generated_images/019fd39c-6a5d-7f51-9d14-b05a4e24904d/exec-5f33aca7-c7c7-4b8f-81cc-76477d5286b6.png`.
- Hierarchy thesis: request chain of custody and evidence provenance dominate.
- Layout grammar: dense dark ledger rows, instrument panels, and status columns.
- Interaction model: expandable trace rows and evidence filters.
- Full-design coverage: architecture, quick start, ownership lanes, lab trace, security, release proof, and patterns.
- Wide-to-narrow transformation: tables become ordered disclosure panels.
- Creative Production territory use: cyan/amber/green evidence semantics on near-black.
- Tradeoff: excellent provenance vocabulary, but it can falsely imply live telemetry and is denser than the primary evaluator journey needs.

### Direction C — Edge Atlas

- Exact generated-image reference: `/Users/star/.codex/generated_images/019fd39c-6a5d-7f51-9d14-b05a4e24904d/exec-b1ae129f-c211-4e59-936c-4a0e9babb0c1.png`.
- Hierarchy thesis: a cinematic request line connects every act from entry to launch.
- Layout grammar: dark topographic canvas, vertical acts, floating technical modules, and a launch runway.
- Interaction model: scroll-linked route tracing and staged reveals.
- Full-design coverage: hero through architecture, lab, patterns, verification, and deploy threshold.
- Wide-to-narrow transformation: route line remains central while modules stack.
- Creative Production territory use: high-contrast orange path over contour mapping.
- Tradeoff: memorable but motion-heavy, less document-like, and more expensive to make truthful and accessible.

## 11. Selected Direction

Status: Direction A selected under the user’s explicit discretion to create images and spearhead the website.

- Exact selected Product Design result: `docs/design/edge-field-guide-concept.png`.
- Exact selected coverage-expansion targets by Full Design Coverage row: implement all rows in Section 6, including loading, empty, error, disabled, not-found, and no-JS-relevant states.
- Exact selected Creative Production territory/assets: the warm editorial field guide and the persisted concept asset above.
- Why it wins for the primary job: it makes architecture teachable, differentiates the starter without fabricated product proof, and keeps the adoption path prominent.
- Why alternatives lose: Direction B implies operations telemetry; Direction C makes spectacle and motion compete with source comprehension.
- User feedback incorporated: create a beautiful new multi-page website, keep Leptos Router/cargo-leptos, create images at discretion, and pursue production readiness.
- Known risks: paper texture may hurt contrast, marginal tabs may crowd narrow screens, generated concept check marks are not evidence, and editorial type can become decorative if overused.

## 12. Visualize Full-Design Review

Status: reviewed in the full-design model.

- Review form: interactive inspectable preview.
- Visualization path/directive: `/Users/star/.codex/visualizations/2026/08/05/019fd39c-6a5d-7f51-9d14-b05a4e24904d/edge-field-guide-review.html`.
- Routes, regions, and states covered: home hierarchy, start, architecture, patterns, lab states, contact caveat, about/proof, and not found.
- Interactions represented: route selection, viewport toggle, and state selection.
- Wide-to-narrow behavior represented: side-by-side comparison of structural changes without CSS order drift.
- Decisions or defects revealed: the warm field-manual system survives narrow reflow when the request path becomes a vertical list; the header must wrap rather than hide links; folio facts belong after the hero actions in DOM order; evidence language must remain textual; and the route model should reuse one plate grammar rather than page-local card grids.
- Limits: review instrument only; not Leptos source or runtime proof.

## 13. Shared Design System

| Need | Existing implementation inventory | Keep semantic/behavioral core | Replace visual layer | New shared primitive/token | Migration reason |
|---|---|---|---|---|---|
| Site shell | `src/components/app_layout.rs` | Router-owned persistent navigation. | Replace generic header and nested main. | `SiteHeader`, `SiteFooter`, `.site-main`, `.page-shell`. | One landmark owner and consistent orientation. |
| Actions | Buttons and `<A>` links across page components. | Native link/button semantics and actions. | Replace page-local radii, heights, and tones. | `ActionLink`, `ControlSize`, `ControlTone`, `.control`. | Consolidate geometry without hiding events. |
| Evidence | Ad hoc feedback/status text. | Truthful status and errors. | Introduce proof-plane labels. | `EvidenceTag` and evidence color tokens. | Prevent visual equivalence between source and live proof. |
| Section rhythm | Page-local grids and inline styles. | Content order. | Parent-owned gaps on constrained scales. | `.section-stack`, `.cluster`, `.split`, `.plate-grid`. | Stop spacing drift and child margins. |
| Architecture path | Existing docs SVGs. | Request stages and ownership. | Create accessible HTML/CSS plate with text alternative. | `RequestPath`, `.boundary-step`. | Responsive, source-linked, and readable without image dependence. |

## 14. Leptos Delivery Map

| Surface/region | SSR content | Island interaction | Shared component/token | Data source | Loading/empty/error/disabled states |
|---|---|---|---|---|---|
| Shell/navigation | Brand, routes, footer proof note. | None; whole-app hydration remains canonical. | `SiteHeader`, `SiteFooter`, nav tokens. | Route constants/source. | Active link and narrow menu remain readable without custom JavaScript. |
| Home plates | Headline, request path, model, proof legend. | None. | `RequestPath`, `EvidenceTag`, plate tokens. | Doctrine and docs. | Static SSR content. |
| Start | Commands and prerequisites. | None. | `CommandStep`, action link tokens. | Checked-in scripts/docs. | Unconfigured Cloudflare bindings shown as caveat, not error. |
| Architecture/patterns | Boundary and pattern references. | None. | Shared plate/card primitives. | Repository docs/source. | Optional/out-of-scope labels. |
| Lab and detail | Useful SSR shell and data response. | Existing hydrated server actions. | Existing data components plus shared controls. | D1 server functions. | Loading, empty, error, disabled, pending, success, not found. |
| Contact lab | Scope copy and form. | Existing hydrated server action. | Shared form and evidence tokens. | D1 server function. | Invalid, disabled, pending, error, success. |

- Canonical route-tree change: preserve `AppLayout` inside `Router`; add public routes, move the task board to `/lab`, expose details at `/lab/:id`, and keep wildcard last.
- SSR route generation/server mount: unchanged cargo-leptos/Worker SSR path.
- Feature/bundle boundary: keep the existing whole-app hydration split; no islands are introduced in this cut.
- Navigation/access/capability inventory: all public routes are link-reachable; task-board and Contact mutations keep their current server-action authorization/session behavior.
- No-JS behavior: marketing, architecture, patterns, start, about, navigation links, labels, and lab explanation remain useful; mutations require hydration and say so.

## 15. Idea Server

- Dev-only route: `/__ideas/the-edge-field-guide`.
- Feature/config gate: not implemented; direct implementation is authorized because the selected design covers the full route set and no parallel preview tree will survive promotion.
- Full Design Coverage rows implemented: none in a separate Idea Server.
- HORIZON rationale shown beside or linked from preview: the Visualize review and this file carry rationale.
- Fixture source and mutation policy: no new fixtures; existing local D1 lab data only.
- Run command: `cargo leptos watch` for the real app after implementation.
- Readback URL: the local Wrangler URL used by the release proof.
- Promotion/removal plan: no `/__ideas` route is added; this isolation path is intentionally collapsed into the selected production route tree to avoid duplicate consumers.

## 16. Responsive and Inclusive Behavior

- Wide hierarchy: split hero, horizontal request plate, multi-column supporting plates, visible primary navigation.
- Narrow hierarchy: headline, promise, actions, folio facts, then vertical request steps; supporting plates stack in semantic order.
- DOM/reading/focus order: identical task order at every width; no CSS `order` or grid-area reassignment may move meaning ahead of its DOM position.
- Keyboard and visible focus: every control has a high-contrast `:focus-visible` outline and logical tab order; no hover-only information.
- Reflow, overflow, long content, and zoom: command blocks wrap or scroll locally, long labels break safely, and the page has no horizontal overflow at 320 CSS pixels or 200% zoom.
- Reduced motion and contrast: `prefers-reduced-motion` removes nonessential transitions; evidence labels use text and shape, not color alone.

## 17. Comparison and Proof Plan

- Focused source/architecture ratchets: route assertions, main-landmark count, wildcard-last invariant, and forbidden inline-style scans.
- SSR HTML/content assertion: fetch every route from local Wrangler and assert unique titles, expected copy, one main landmark, and security headers.
- SSR/hydrate/render feature builds: run repository fast checks during iteration and `./scripts/verify.sh` before release claims.
- Release-shaped asset check: prove generated Worker shim serves every hashed bundle and any added static asset path.
- Browser route, hydration, and console/page-error check: exercise all routes plus task-board/Contact interactions with local D1 data.
- Matching selected-source, Visualize, and Leptos captures: compare the selected Product Design concept and Visualize model to wide/narrow browser screenshots from the Leptos app.
- Wide/narrow screenshots and comparison: 1440px and 390px at minimum, plus overflow check at 320px.
- Keyboard/accessibility check: keyboard-only navigation, landmarks/headings, labels, focus visibility, reduced motion, and contrast review.
- Accepted deviations and reasons: no marginal vertical tabs on narrow screens; no generated check marks, live telemetry, or fake deployment status.
- Known evidence limits: local browser proof is not merge, deployment, provider state, or authenticated live readback.

## 18. Non-goals and Reversibility

- Non-goals: a CMS, analytics platform, authentication product, support inbox, live tracing system, new database schema, or deployment outside the governed cfctl path.
- Feature flag or isolation boundary: public route/component changes are source-local and can be reverted without data migration; no new feature flag is needed.
- Files/components to remove if rejected: new marketing page components, design tokens, and this HORIZON-owned concept asset.
- Old shared primitives to migrate or retain temporarily: retain task-board/Contact server actions and data types; migrate their shell, spacing, and controls.
- Data or migration impact: none planned.

## 19. Decision Log

| Date | Decision | Evidence/artifact | Consequence |
|---|---|---|---|
| 2026-08-05 | Treat leptos-cf itself as the product. | `NORTH_STAR.md:3-10`, repository routes and docs. | The task-board app becomes a lab rather than homepage identity. |
| 2026-08-05 | Select Direction A, The Edge Field Guide. | `docs/design/edge-field-guide-concept.png`; user delegated visual discretion. | Warm editorial plates become the sole visual authority. |
| 2026-08-05 | Borrow evidence labels from Direction B and a restrained launch line from Direction C. | Generated candidate comparison. | Provenance stays explicit without adopting a fake dashboard or cinematic scroll dependency. |
| 2026-08-05 | Preserve whole-app hydration and Router-owned layout. | `src/app.rs:49-74`; current cargo-leptos architecture. | The redesign changes delivery surfaces, not runtime fundamentals. |
| 2026-08-05 | Accept the Visualize full-design review with responsive corrections. | `/Users/star/.codex/visualizations/2026/08/05/019fd39c-6a5d-7f51-9d14-b05a4e24904d/edge-field-guide-review.html`. | Implement a wrapping nav, vertical narrow request path, folio-after-actions DOM order, and one shared plate grammar. |
