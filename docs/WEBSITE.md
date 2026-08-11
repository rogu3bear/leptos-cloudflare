# Website

## Sitemap

- `/` — decision and orientation
- `/start` — adoption sequence
- `/architecture` — request and ownership boundaries
- `/patterns` — core and optional capabilities
- `/lab` — D1-backed task-board demonstration
- `/lab/:id` — task record detail
- `/contact` — bounded D1 intake demonstration
- `/about` — product principles, security, and proof planes

## Page Briefs

### / (home)

- Purpose & primary conversion action: establish fit; “Use the starter.”
- Message (from `POSITIONING.md`): “See every boundary before you ship.”
- CTA (direct + transitional): Use the starter; Trace the request.
- Copy blocks: field-guide folio, request path, two-WASM model, evidence legend, lab/start invitations.

### /start

- Purpose & primary conversion action: move from evaluation to a safe local run.
- Message: the shortest path is checked-in and verifiable.
- CTA: Follow the local start sequence.
- Copy blocks: prerequisites, clone/install/configure/run/verify, placeholder-binding caveat.

### /architecture

- Purpose & primary conversion action: understand ownership; continue to patterns or start.
- Message: one request, explicit boundaries.
- CTA: Inspect the starter.
- Copy blocks: browser, Worker, Axum/Leptos SSR, D1/assets, streaming HTML, hydration, realtime.

### /patterns

- Purpose & primary conversion action: choose only the capability needed next.
- Message: core stays small; optional patterns name their owner.
- CTA: Open the lab or start path.
- Copy blocks: core, D1, forms/actions, security, realtime, verification.

### /lab and /contact

- Purpose & primary conversion action: inspect bounded real behavior.
- Message: local interactive proof, not a hosted-service promise.
- CTA: perform the demonstrated action.
- Copy blocks: scope/evidence label, form/board, feedback, source-bound explanation.

## Conversion Elements

| Objection (Big 5) | Counter | Placement | Status |
|---|---|---|---|
| Trust | Proof-plane labels and exact verification command. | Home, About, Start | Planned |
| Fit | Explicit stack, runtime, and non-goals. | Home, Architecture | Planned |
| Effort | Ordered start path with prerequisites. | Home, Start | Planned |
| Timing | No urgency claim. | None | Intentionally absent |
| Price | Open-source/license truth only after source verification. | About/footer | Pending verification |

## Audit Findings

| Issue | Severity (0-4) | Fix | Status |
|---|---:|---|---|
| Task board is the homepage identity. | 4 | Move it to `/lab`; create architecture-first home. | In progress |
| Nested `<main>` landmarks. | 4 | Layout owns the only main landmark. | In progress |
| Contact implies an operational channel. | 4 | Label it a bounded D1 intake lab. | In progress |
| Generic hierarchy and incomplete nav styling. | 3 | Apply HORIZON system and shared shell. | In progress |
| Inline spacing forks and mismatched grids. | 3 | Parent-owned spacing and corrected responsive layouts. | In progress |
| No conversion baseline. | 0 | Await real traffic before CRO claims. | Awaiting evidence |

## Lead Capture

No lead-capture or nurture funnel is proposed. The repository adoption path is direct.
