# Launch-window OKRs — `leptos-cf` public website

Mode: One-Shot

## Context

- **Scope:** Product initiative
- **Cycle:** Launch window from design lock through 30 days after production release
- **OKR type:** Learning, with committed safety and reliability guardrails
- **Strategic intent:** Make the repository's correct Leptos-on-Cloudflare path immediately understandable, credible, and adoptable without overstating unmeasured outcomes.
- **Empowerment signal:** Mixed — the production website outcome is committed, while hierarchy, creative direction, and implementation bets may change when evidence does not move.
- **Source of truth:** Owner must designate a GitHub issue or project before launch; this document is a planning input, not a live score tracker.

## Objective

Make a proof-oriented developer confident enough to understand the stack, choose a next step, and begin a reproducible adoption path.

## Key Results

### KR1 — First-screen comprehension

- **Metric definition:** Percentage of target-developer sessions in which the participant can state what the project is, who it is for, and the primary next action after a five-second homepage exposure.
- **Baseline:** `recommended-to-measure`; no study exists.
- **Target:** Owner-approved threshold to be set after a five-session pilot; not fabricated here.
- **Deadline:** Before production go/no-go.
- **Evidence source:** Moderated comprehension test notes.
- **Owner:** Product owner.
- **Indicator class:** `leading`
- **Confidence:** Low until sessions are recruited.

### KR2 — Adoption-path completion

- **Metric definition:** Percentage of qualified visitors who move from the homepage to an inspectable start or source action and complete the local quick-start sequence without undocumented intervention.
- **Baseline:** `recommended-to-measure`; analytics and test cohort are absent.
- **Target:** Owner-approved threshold after baseline instrumentation; not fabricated here.
- **Deadline:** 30 days after production release.
- **Evidence source:** Privacy-respecting click events plus observed setup sessions.
- **Owner:** Product owner and engineering owner.
- **Indicator class:** `lagging`
- **Confidence:** Low.

### KR3 — Evidence-backed learning

- **Metric definition:** Resolve the three highest-risk assumptions: evaluator identity, decision-driving proof, and the largest setup abandonment point.
- **Baseline:** All three are explicitly unresolved.
- **Target:** Each assumption is `validated`, `invalidated`, or `partially-validated` with a cited evidence source; completion count alone is not success.
- **Deadline:** 30 days after production release.
- **Evidence source:** Research notes, anonymized analytics, and support feedback.
- **Owner:** Product owner.
- **Indicator class:** `evidence_generation`
- **Confidence:** Medium that the questions are answerable; low on the outcomes.

### KR4 — Release integrity guardrail

- **Metric definition:** The exact release candidate passes `./scripts/verify.sh`, preserves security invariants, deploys through the governed Cloudflare lane, and returns expected live SSR and route readback with no unresolved P0–P2 accessibility or visual regressions.
- **Baseline:** Current template gate exists; the redesigned candidate has not been built or tested.
- **Target:** Pass, with no exceptions or weakened checks.
- **Deadline:** Production go/no-go.
- **Evidence source:** Exact-tree local verifier receipt, security review, deployment operation receipt, and live route checks.
- **Owner:** Engineering and release owner.
- **Indicator class:** `guardrail`
- **Confidence:** Medium; the repo already owns a release-shaped local gate.

## Initiatives as Bets

| Initiative | KRs | Assumption |
| --- | --- | --- |
| Reframe the homepage around a visible edge request path and one start action | KR1, KR2 | A system-first narrative reduces evaluation ambiguity |
| Publish focused Architecture, Patterns, Start, and Contact routes | KR1, KR2 | Progressive disclosure serves both scanners and implementers |
| Create a coherent visual system with generated atmospheric imagery used as support, not proof | KR1 | Distinctive craft improves attention without obscuring technical truth |
| Instrument minimal, privacy-respecting evaluation events after owner approval | KR2, KR3 | The chosen events can identify abandonment without collecting personal data |
| Preserve the repo's local verification, security, and provider gates | KR4 | Existing contracts can support the redesigned route set without weakening controls |

## Guardrails and Health Checks

- KR4 is reported separately and never averaged into learning or conversion results.
- No claim about global scale, latency, conversion, or developer demand ships without current evidence.
- The homepage must remain useful as SSR HTML before hydration.
- Contact intake keeps bounded bodies, same-origin server-function posts, session scoping, and no external send.

## Alignment Notes

- Parent strategy: public, agent-first reference implementation with explicit boundaries and reproducible local proof.
- Dependency: the owner must choose an analytics policy, production target, and canonical OKR tracker.
- Peer boundary: optional Durable Object and service patterns remain separate from the minimal core.

## Quality Audit

| Criterion | Rating | Rationale / correction |
| --- | --- | --- |
| Strategic fit | pass | Directly traces to repository strategy and North Star |
| Objective quality | pass | Describes a user state change, not a release task |
| KR outcome quality | pass | Comprehension, adoption behavior, learning, and release integrity are outcomes |
| Measurement quality | risk | Baselines and targets are intentionally unfilled; run the pilot and instrument before scoring |
| Product influence | pass | Hierarchy, content, proof, and setup path are controllable |
| Focus | pass | One objective and four KRs |
| Guardrails | pass | Release integrity is explicit and separate |
| Alignment | pass | Strategy, pattern boundary, and owner dependencies are named |
| Operating rhythm | risk | Check-in owner and calendar are not yet designated |
| Integrity | pass | No fabricated values or compensation coupling |
| Empowered-team Disclosure | pass | Included below because the signal is mixed |

## Disclosure

This OKR set frames the committed website and deployment as outcome bets. If the metrics do not move when the work ships, that is learning rather than permission to rewrite results. The build commitment remains; the evidence should change the next iteration.

## Open Questions

- Which GitHub issue or project will become the canonical tracker?
- What analytics events and retention window does the owner approve?
- Who are the five pilot participants and who owns recruitment?

## Suggested Next Step

Run the five-session comprehension pilot against the selected design preview, establish KR1's baseline, and designate the production go/no-go owner.
