# Build Risk Review — public `leptos-cf` website

**Mode:** Feature-change

## Decision

**Verdict: Build small.** The current public surface demonstrably presents the todo example as the product while repository doctrine defines a production-minded reference implementation; a bounded architecture-first website corrects that mismatch, but all conversion claims remain hypotheses until real developer testing exists.

## Biggest risk

- **R1 `positioning`:** The redesign could make the site more beautiful while still failing to communicate the wedge: explicit, verifiable ownership across Leptos SSR, hydration, Cloudflare bindings, security, and release proof.
- **R2 `trust`:** “Production-grade” could be misread as a production-complete application despite explicit missing application-level auth, rate limits, audit, and retention controls.
- **R3 `feature-fit`:** A broad cinematic site could bury the quick-start path and turn architecture explanation into spectacle.
- **R4 `distribution`:** No evidence identifies how qualified developers currently discover or evaluate the repository.

## Demand level

**L0 — owner directive with no external demand evidence.** The operator has explicitly requested the production website, but no repeated customer asks, behavior proof, adoption analytics, retention signal, or revenue effect was supplied.

## Evidence ledger

| Signal | Strength | What it does or does not prove |
| --- | --- | --- |
| Repository North Star defines a public, agent-first production-grade starter | Medium | Proves intended product identity, not visitor demand |
| Current `/` foregrounds a D1 todo form and “Todos” navigation | Medium | Proves a source-level positioning mismatch, not its conversion impact |
| Exact user request to de-template, beautify, and deploy | Strong authority, not demand | Authorizes work but does not establish market pull |
| No interviews, analytics, support corpus, or adoption funnel | Counter-signal | Prevents evidence-backed conversion targets or emotional claims |

## Validation plan

1. Put the selected homepage preview in front of five Rust-capable developers; pass only if at least four can state what the project is, who it serves, and the primary next action without prompting. This is a proposed bar, not a benchmark.
2. Ask each participant to locate the quick start, architecture boundary, and production caveat; record the first failure point and revise before production go/no-go.
3. After launch, instrument only owner-approved, privacy-respecting events for start-action selection and documented setup completion; use the first 30 days as baseline, not as proof of success.

## Routing

→ `define-problem-statement`, then `deliver-user-stories`: bind the architecture-first site to an explicit user problem and a small, testable delivery slice.

## Sources

- `NORTH_STAR.md`, `STRATEGY.md`, current route and component source at `2ee33f2930a60228024ee868b0414cf0fc0e526c`.
- Operator request dated 2026-08-05.
