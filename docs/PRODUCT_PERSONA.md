# Rowan — The Proof-Oriented Rust Builder

**Rowan is an experienced developer choosing an edge stack under delivery pressure; they need a starter whose architecture, security posture, and release evidence can survive the move from experiment to production.**

| Field | Value |
| --- | --- |
| Persona ID | PU-001 |
| Type | Primary |
| Product scope | Evaluation, adoption, extension, local verification, and Cloudflare release of `leptos-cf` |
| Valid for | Rust-capable solo developers and small product teams building server-rendered web applications on Cloudflare Workers |
| Not valid for | No-code site builders, teams committed to a JavaScript-only stack, or large enterprises requiring a complete platform and support contract |
| Confidence | Proto — grounded in repository intent and workflow contracts, with no customer interviews or analytics |
| Last validated | 2026-08-05 |
| Owner | Repository maintainer |

## Persona Card

**Rowan — The Proof-Oriented Rust Builder**

Rowan can build the application logic but does not want to rediscover the edge runtime's routing, asset, hydration, data, and deployment boundaries. They evaluate a starter by reading source and running proof, not by trusting a polished claim.

**Key quote:** No verbatim customer quote is available; this persona is operating in assumption mode.

**Goals.** Reach a correct first deploy quickly, understand the ownership boundaries well enough to extend them safely, and retain a local proof path that another developer or agent can reproduce.

**Frustrations.** Starters that are only demos, configuration that hides production assumptions, documentation that diverges from code, and a “green” build that does not resemble the hosted runtime.

**Design rules — always.** Show the deployment model before feature inventory; connect each promise to inspectable proof; preserve a clear path from evaluation to local start.

**Design rules — never.** Hide prerequisites behind spectacle; imply production readiness from screenshots; bury security and runtime constraints in footnotes.

## 1. Demographics & Identity

| Attribute | Detail |
| --- | --- |
| Age | Unknown; not decision-relevant |
| Location | Distributed; Cloudflare deployment implies no single geography |
| Education | Unknown; demonstrated Rust and web-platform fluency matters more than credentials |
| Role | Senior software engineer, technical founder, or hands-on technical lead |
| Company size | Solo to roughly 50 people, assumption |
| Team | One to eight product engineers, assumption |
| Reports to | Self, founder, or engineering lead |
| Stakeholders | Product collaborators, future maintainers, and end users of the deployed application |
| Purchasing role | Technical evaluator and implementation owner; the template is open source |
| Accessibility | May work keyboard-first, use zoom or reduced motion, and evaluate on narrow laptop or mobile viewports |

**Career stage and trajectory.** Rowan is experienced enough to distrust unexplained abstractions and wants leverage without surrendering architectural understanding. Their trajectory rewards shipping a trustworthy system, not merely demonstrating framework fluency.

**Organizational leverage.** Rowan's stack choice determines the team's build, deploy, and incident surface. A wrong choice becomes recurring integration tax; a good choice makes every later feature easier to reason about.

## 2. Technology & Environment Context

| Tool | Role |
| --- | --- |
| Rust and Cargo | Application language, dependency graph, and local verification |
| Leptos and cargo-leptos | SSR, hydration, routing, and frontend build |
| Cloudflare Workers, Assets, and D1 | Edge compute, static delivery, and relational persistence |
| Git and local shell tooling | Source review, reproducible builds, and release proof |

**Digital fluency level.** Rowan is highly fluent in typed systems and web architecture, but may be new to one of Leptos, worker-rs, or Cloudflare's binding model. They can follow exact commands and source maps; they will not tolerate unexplained magic.

**Adoption and abandonment patterns.** They inspect the route tree, configuration, dependency versions, security controls, and release command before committing. They leave when the docs and live source disagree, when setup depends on ambient machine state, or when the starter's “production” path is unproven.

**Work environment.** Evaluation happens in short, high-focus sessions with a terminal, editor, browser, and documentation open together. The site must support scanning first and deep inspection second.

## 3. Jobs to Be Done

**Functional.** When starting a Rust web product for the edge, Rowan needs to understand, run, and extend a complete SSR-to-deploy path so that infrastructure plumbing does not consume the first product sprint.

**Emotional.** Rowan wants to feel oriented and in control, so that adopting the starter feels like a reversible technical decision rather than a leap of faith.

**Social.** They want collaborators to see a deliberate architecture with reproducible evidence, not a fashionable stack chosen from a landing page.

**Underlying.** Rowan is hiring the template to collapse uncertainty: one coherent path should connect source, local proof, provider configuration, and live runtime behavior.

## 4. Goals & Motivations

**Build domain value sooner.** The starter should absorb edge-specific setup while leaving product decisions visible and editable.

**Keep one understandable system.** Rust types, route behavior, and server functions should form a coherent model across server and browser.

**Make release claims defensible.** Local gates, artifact identity, provider state, and runtime readback must remain distinct and reproducible.

**Feel oriented quickly.** The first screen should answer what this is, why it is credible, and where to start.

**Feel respected technically.** Copy should be specific and inspectable, avoiding inflated performance or scale claims.

**Feel safe to explore.** Examples should invite inspection without disguising optional patterns as production defaults.

## 5. Behavioral Patterns & Mental Models

**The stack is a chain of contracts.** Rowan models the system as request → Worker → assets or SSR → server function → binding, with a build artifact and proof lane around it. The site should expose that chain visually and let every deeper page elaborate one contract without changing the mental model.

**Primary work pattern.** They alternate between scanning documentation and testing exact commands. The preferred ratio is less configuration archaeology and more domain implementation.

**Accuracy and quality approach.** Rowan trusts checked-in versions, source citations, failing-closed verifiers, and live readback. “Good enough” for exploration is a local build; “good enough” for adoption includes security and deployment boundaries.

**Tolerance thresholds.** They will give a landing page seconds to establish relevance and a setup guide minutes to prove coherence. Unexplained prerequisites or broken copy-to-code links sharply reduce trust.

## 6. Decision-Making & Trust Patterns

**How trust is built and broken.** Trust accumulates when the public claims match `Cargo.toml`, `wrangler.toml`, route source, and the release script. One stale quick-start or hidden security assumption can outweigh several polished sections.

**Adoption filter.** What is the actual runtime? What remains server-rendered? Where does state live? Can I verify it locally? What must change before production? Can I remove the demo domain cleanly?

**Risk profile.** Rowan accepts framework novelty in an experiment but is conservative about credentials, data scope, request boundaries, and release tooling.

**Feature discovery behavior.** They discover capabilities through architecture diagrams, source links, and runnable patterns rather than promotional feature grids.

## 7. Workflow & Collaboration Context

**Work rhythm.** Deep technical work happens in one- to three-hour blocks, interrupted by product delivery. The site should preserve context between overview, architecture, patterns, and start instructions.

**Collaboration model.** Rowan is both evaluator and builder, then becomes the explainer for teammates or coding agents. Outputs must stand alone when handed off.

**Key collaboration friction.** A teammate may treat a demo feature as the architecture or repeat a marketing claim without its constraint. The site must make boundaries and evidence easy to quote accurately.

**Dependencies.** Rowan depends on compatible Rust/Leptos/worker-rs versions, Cloudflare account configuration, D1 bindings, and the repository's local gate.

## 8. Current Alternatives & Workarounds

**Primary alternative.** Assemble the stack from official framework examples, Cloudflare docs, and a custom deployment shim. This offers control but creates integration and upgrade work.

**Where the product enters.** `leptos-cf` is the reference implementation that connects those pieces in one checked-in system.

**The firing trigger.** Rowan abandons it if setup fails from a fresh checkout, the public site obscures the template identity, or release proof cannot be tied to the deployed source.

## 9. Pain Points & Unmet Needs

**Architecture before confidence.** Existing template copy describes many capabilities but makes visitors assemble the complete request path themselves.

**Demo mistaken for product.** A todo-first homepage can make a production reference implementation look like a tutorial sample.

**Proof is scattered.** Build, security, deployment, and runtime evidence live across several documents and commands.

**Adoption path competes with explanation.** Long-form README material is valuable but does not provide a concise evaluation-to-start journey.

## 10. Success Definition & Quality Bar

**Accuracy standard.** Every technical claim must match checked-in source or be labeled as an intended or optional pattern.

**Timeliness standard.** A visitor should understand the product category and primary action on the first screen; exact timing targets remain to be measured.

**Self-sufficiency standard.** A page should provide enough context to choose the next action and link directly to the deeper proof or source.

**Quality bar by context.** Marketing copy must be clear and honest; architecture pages must be precise; release claims require exact local and live evidence; experimental patterns must be explicitly separated from the core.

## 11. Design Principles & Tradeoff Heuristics

**Proof over promise.** Prefer inspectable architecture and exact commands over unsupported superlatives.

**Path over inventory.** Show the adoption sequence before enumerating platform capabilities.

**System map over card grid.** Use spatial relationships to explain the runtime rather than unrelated feature tiles.

**Meaningful SSR over decorative hydration.** Server-known content arrives as useful HTML; client code enhances only real interactions.

**Explicit boundary over apparent simplicity.** Name D1, session, asset, and provider constraints when hiding them would create downstream risk.

**Focused core over bundled breadth.** Optional realtime and service patterns remain clearly outside the minimal default.

## Evidence & Confidence

| Source | Type | Detail |
| --- | --- | --- |
| E1 | Repository doctrine | `NORTH_STAR.md`, `ANCHOR.md`, and `STRATEGY.md`, inspected 2026-08-05 |
| E2 | Product documentation | `README.md`, `docs/how-leptos-works.md`, and `docs/edge-deployment.md`, inspected 2026-08-05 |
| E3 | Implementation | Route, component, Worker, and verification source in the current checkout at `2ee33f2930a60228024ee868b0414cf0fc0e526c` |

**Validated.** The system identity, runtime boundaries, local verification contract, and intended agent-first adoption path are supported by multiple repository sources.

**Assumed.** Role, team size, emotional state, evaluation behavior, abandonment thresholds, and demand are hypotheses. No interview, survey, support, or analytics dataset was provided.

**Open questions.** Which visitor segment reaches the repository today? Which three facts most influence adoption? Where do evaluators abandon the setup path?

**Governance.** Review after five target-developer sessions or 90 days of live analytics, whichever comes first. Retire or split the persona if evidence shows materially different evaluator and implementer journeys.
