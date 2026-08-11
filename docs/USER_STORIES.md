---
artifact: user-stories
version: "1.0"
created: 2026-08-05
status: draft
---

# User Stories — architecture-first `leptos-cf` website

## US-001 — Understand the reference at a glance

| Field | Value |
| --- | --- |
| Persona | Rowan, proof-oriented Rust builder |
| Priority | P0 |
| Epic | Evaluation and adoption |
| Estimate | M |

**As a** Rust-capable developer evaluating an edge stack, **I want** the first screen to explain what `leptos-cf` is and expose one clear start action, **so that** I can decide whether to inspect or adopt it without operating the demo first.

**Context.** The current homepage foregrounds a todo composer even though the repository is a public reference implementation.

**Acceptance criteria.** Given the homepage before hydration, when I scan the first screen, then I can identify the product category, intended audience, architecture-led value, and primary start action. Given a narrow viewport, when the same content reflows, then its DOM and task order remain promise → action → proof.

**Design notes.** The selected Edge Field Guide direction uses a request-path plate as proof, not a generic feature grid.

**Technical notes.** Meaningful SSR HTML; no browser-only branch for initial layout.

**Dependencies.** Selected HORIZON direction.
**Out of scope.** Validated conversion uplift.
**Open question.** Final comprehension threshold after the five-session pilot.

## US-002 — Trace the architecture

| Field | Value |
| --- | --- |
| Persona | Rowan |
| Priority | P0 |
| Epic | Architecture and trust |
| Estimate | L |

**As a** technical evaluator, **I want** to trace a request from browser through Worker, SSR, assets or D1, and hydration, **so that** I understand ownership boundaries before adopting the stack.

**Acceptance criteria.** Given JavaScript is unavailable, when I open `/architecture`, then the complete ordered path and text alternative are present. Given keyboard navigation, when I move through architectural references, then links and disclosures are reachable with visible focus and no contradictory reading order.

**Design notes.** Use an annotated field plate with source-derived labels; never call it live telemetry.

**Technical notes.** Ordinary SSR components and accessible SVG/HTML; route registered before wildcard.

**Dependencies.** Current runtime and documentation contracts.
**Out of scope.** Real POP locations, latency, or tracing.
**Open question.** Whether optional capabilities appear in the primary plate or a secondary legend.

## US-003 — Choose a core or pattern path

| Field | Value |
| --- | --- |
| Persona | Rowan |
| Priority | P1 |
| Epic | Learning and extension |
| Estimate | M |

**As a** builder extending the starter, **I want** the site to distinguish verified core behavior from optional patterns, **so that** I do not accidentally treat example complexity as a default production dependency.

**Acceptance criteria.** Given `/patterns`, when I inspect an entry, then I can identify its problem, ownership boundary, prerequisites, and proof path. Given an optional capability, when it is described, then the site does not imply it ships enabled in the core.

**Design notes.** Pattern entries use indexed editorial rows, not interchangeable marketing cards.

**Technical notes.** Content must trace to `patterns/README.md` and real paths.

**Dependencies.** Pattern inventory.
**Out of scope.** Adding new pattern implementations.
**Open question.** Whether pattern detail routes are needed at launch.

## US-004 — Inspect a bounded live proof

| Field | Value |
| --- | --- |
| Persona | Rowan |
| Priority | P1 |
| Epic | Live Lab |
| Estimate | L |

**As a** skeptical evaluator, **I want** to use the D1-backed demo and see its limits, **so that** I can verify server functions, session scoping, dynamic routes, pending states, and recovery without mistaking the demo for a complete application.

**Acceptance criteria.** Given `/lab`, when I create, toggle, open, or delete an item, then pending, success, error, empty, and disabled states remain observable and semantically announced. Given the lab copy, when I inspect its boundary, then it states that data is session-scoped and the capability is demonstrative.

**Design notes.** Keep native controls and make explanation subordinate but adjacent.

**Technical notes.** Preserve existing APIs, D1 queries, dynamic route compatibility, and keyed iteration.

**Dependencies.** Local D1 migration and existing server-function path.
**Out of scope.** Authentication, multi-user collaboration, and realtime shared state.
**Open question.** Compatibility URL for `/todo/:id` after visible navigation moves to `/lab`.

## US-005 — Verify the release boundary

| Field | Value |
| --- | --- |
| Persona | Rowan |
| Priority | P0 |
| Epic | Security and release evidence |
| Estimate | M |

**As a** production-minded adopter, **I want** security guarantees, non-guarantees, and proof planes stated separately, **so that** I can judge what the starter provides and what my application must add.

**Acceptance criteria.** Given `/about` or the release-proof region, when I read “production-grade,” then application-level auth, authorization, rate-limit, audit, and retention gaps are not hidden. Given the launch candidate, when release is claimed, then local verification, provider execution, provider readback, and runtime readback are independently evidenced.

**Design notes.** Use a release ledger with empty states labeled unproven.

**Technical notes.** Do not expose secret values; preserve CSP, anti-framing, body, origin, and session controls.

**Dependencies.** Approved security policy and threat model.
**Out of scope.** Certification or guarantee language.
**Open question.** Canonical production target and domain.

## INVEST review

Each story delivers an independently reviewable user outcome, remains negotiable in presentation, is valuable to the primary persona, is estimable within one implementation cycle, is bounded to a route or coherent flow, and has observable acceptance conditions. Cross-story runtime contracts are dependencies rather than duplicated scope.
