# Prioritized Action Plan — `leptos-cf` production website

## Step 0. Source ledger

- **S1:** “spearhead to prod” (origin: user request)
- **S2:** “de-template it, make it beautiful” (origin: user request)
- **S3:** “make a new website for this, pages” (origin: user request)
- **S4:** “leptos router, cargo-leptos” (origin: user request)
- **S5:** “create images at your discression” (origin: user request)
- **S6:** “`leptos-cf` is a public starter template for production-grade Leptos applications on Cloudflare Workers.” (origin: `NORTH_STAR.md`)
- **S7:** “Every release claim is backed by local proof that matches the Cloudflare deployment surface.” (origin: `NORTH_STAR.md`)

## Section 0. Executive summary

- **Situation classification:** Complicated (Cynefin) — the product identity and technical contracts are knowable from source, but hierarchy, design, security policy, and release must be reconciled across several specialist domains.
- **The binding constraint:** The current website makes the todo demonstration the product, so a qualified visitor cannot quickly understand or adopt the reference implementation.
- **The critical next effort (P1):** Lock an architecture-first message, information hierarchy, and full-route design contract before changing production source.
- **Overall plan confidence:** Medium — repository truth is strong, while user behavior and conversion evidence are absent.
- **Time-to-value:** A reviewable full-design direction and test script can exist within the first implementation cycle; validated visitor signal requires recruited sessions.

## Section 1. Input mirror — what I understand

- **What you gave me:** A directive to turn the existing Leptos/Cloudflare template into a beautiful multi-page production website, create supporting imagery, harden its security posture, and deploy it.
- **What you appear to be trying to accomplish:** Make the template itself a credible flagship reference implementation and public adoption surface — confidence: Medium-High, based on repository doctrine and the absence of another product domain.
- **Adjacent intents I noticed but did not assume:** A new unrelated business, a paid conversion funnel, real-time telemetry, production contact delivery, and a custom domain.

## Section 2. Situation classification (Cynefin)

**Domain:** Complicated
**Source:** S1, S2, S3, S4, S6, S7

The target system and release path can be analyzed from checked-in contracts, so cause and effect is more knowable than a Complex-market bet. Expertise is still required across PM, Leptos SSR, visual design, AppSec, accessibility, and Cloudflare deployment. The plan commits to a bounded build while treating comprehension and conversion outcomes as hypotheses.

## Section 3. The binding constraint (Theory of Constraints)

- **System and goal:** Move a qualified developer from first visit to confident, reproducible adoption.
- **The constraint:** Product identity and adoption hierarchy are obscured by a demo-first homepage.
- **Source:** S2, S3, S6
- **Candidate constraints considered:** Cloudflare credentials block deployment but not local design and build; visual polish is downstream because polish cannot repair the wrong message.
- **Why P1 lifts it:** A locked semantic and hierarchy contract makes every route, component, image, and CTA serve the same adoption decision.

## Section 4. Prioritized questions, gaps, and open decisions

| Rank | Question / gap | Why it matters | Decision required? | How to resolve |
| --- | --- | --- | --- | --- |
| Q1 | Is the site for `leptos-cf` itself? | Changes every message and route | Resolved by evidence-backed operating assumption; owner may override | Preserve in HORIZON decision log |
| Q2 | Which design direction best clarifies the request lifecycle? | Locks layout and asset direction | Delegated to agent discretion by S5 | Compare three structural directions against persona and accessibility |
| Q3 | What is the production account, Worker, D1 database, and route? | Required for external mutation | Yes, blocks deploy execution | Governed cfctl discovery and plan review |
| Q4 | Is contact a real conversion channel? | Current route stores but does not deliver submissions | No for initial build; yes before making it primary | Keep as bounded lab or add an owner-approved delivery path later |
| Q5 | What analytics are acceptable? | Needed for KR baselines | Yes before instrumentation | Approve events, retention, and privacy policy |

## Section 5. The prioritized action plan

### P1. Lock product meaning and full-design hierarchy

- **Why:** This directly removes the demo-first positioning constraint.
- **What:** Persona, OKRs, risk review, journey, stories, acceptance criteria, and a selected HORIZON design contract covering every launch route and critical state.
- **How:** Ground claims in doctrine; quarantine old visual direction; compare three structural concepts; select against task clarity, truth, accessibility, and Leptos delivery cost; validate a five-session comprehension script.
- **Confidence:** Medium — product evidence is strong, behavior evidence is absent.
- **Source:** S2, S3, S5, S6
- **Expected outcome / success signal:** Every route and region traces to one user job and a reviewer can test the selected first-screen proposition.
- **Estimated effort:** One focused design and specification cycle.
- **Dependencies:** None for draft; user-testing recruitment for validation.

### P2. Implement one coherent SSR-first route system

- **Why:** Product meaning cannot change outcomes until it exists in the actual Leptos topology.
- **What:** Overview, Architecture, Patterns, Live Lab, About, Contact, and 404 with shared tokens, layout primitives, and control geometry.
- **How:** Preserve Worker and route invariants; remove nested landmarks and inline geometry forks; keep useful HTML in SSR; localize hydration; implement responsive and failure states.
- **Confidence:** Medium-High — installed Leptos topology and release gate are explicit.
- **Source:** S3, S4, S6
- **Expected outcome / success signal:** Focused builds, SSR readback, hydration, routes, keyboard flow, and matched responsive captures pass.
- **Estimated effort:** One to two implementation cycles.
- **Dependencies:** P1 selected direction.

### P3. Prove security and release integrity

- **Why:** The product promise depends on controls and evidence, not appearance.
- **What:** Approved `SECURITY.md`, grounded threat model, full local verifier, visual regression review, and launch go/no-go checklist.
- **How:** Resolve the policy chain; inventory trust boundaries; preserve same-origin/body/session/CSP controls; run exact-tree proof; close P0–P2 findings.
- **Confidence:** Medium — controls exist, policy and final candidate remain unreviewed.
- **Source:** S7
- **Expected outcome / success signal:** No weakened security gate and no unresolved release blocker.
- **Estimated effort:** One review cycle after implementation.
- **Dependencies:** P2 candidate and owner decisions required by policy/threat-model skills.

### P4. Governed Cloudflare release and learning baseline

- **Why:** Production availability and outcome measurement are the final evidence planes.
- **What:** Reviewed cfctl plan, explicit approval, execution receipt, provider readback, live route checks, and a 30-day evidence baseline.
- **How:** Resolve capability; bind artifact and target; plan and inspect; obtain approval; execute once; verify provider and runtime separately; instrument only approved events.
- **Confidence:** Low-Medium — current D1 IDs are placeholders and target identity is unresolved.
- **Source:** S1, S7
- **Expected outcome / success signal:** Verified production URL at an exact source revision, with unresolved metrics clearly baselined rather than claimed.
- **Estimated effort:** One release window after credentials and target are resolved.
- **Dependencies:** P3 pass, production resources, explicit provider-plan approval.

**Sequencing**

| Now | Next | Later |
| --- | --- | --- |
| P1 | P2, P3 | P4 |

**What to defer / what NOT to do**

- Do not add live telemetry, auth, billing, or a broad pattern marketplace to make the site feel complete.
- Do not claim global latency, scale, conversion, or security outcomes without current evidence.
- Do not weaken `./scripts/verify.sh`, run `scripts/init.sh`, or discard the existing D1/contact proof paths.

## Section 6. Risks and pre-mortem

| Risk | Likelihood | Impact | Early signal | Mitigation | Source |
| --- | --- | --- | --- | --- | --- |
| Architecture spectacle buries quick start | Medium | High | Pilot participants admire the page but cannot find start | Keep start action first-screen and test it | S2, S3 |
| “Production-grade” implies production-complete | Medium | High | Reviewers miss auth/rate-limit caveats | Put boundary beside the claim | S6 |
| Contact submissions disappear into D1 | High if primary | High | Form success has no operator delivery path | Keep contact labeled as bounded lab until delivery exists | Inferred |
| Deployment is blocked by placeholder resources | High | High | cfctl cannot bind a concrete target/artifact | Finish local prep, then resolve through governed plan | S1, S7 |
| Generated imagery harms performance or truth | Medium | Medium | Hero becomes LCP or implies real network data | Use optimized decorative assets with explicit labels | S5 |

## Section 7. Recommended pm-skill prompts

### To execute P1

**Skill:** `deliver-user-stories`
**Why this skill:** Converts the architecture-first adoption journey into bounded engineering slices.
**Source:** S2, S3, S4, S6

**Prompt:**
> Write sprint-sized user stories for a Rust-capable developer evaluating and adopting the public leptos-cf reference implementation. Cover overview comprehension, architecture inspection, patterns, the bounded live lab, security/release evidence, responsive access, and an honest start action. Preserve Leptos Router, SSR/hydration, Cloudflare Worker/Assets, D1/session, and local verification contracts. Do not invent analytics or production-complete claims.

### To execute P3

**Skill:** `deliver-acceptance-criteria`
**Why this skill:** Makes runtime, accessibility, and evidence boundaries objectively pass/fail.
**Source:** S4, S7

**Prompt:**
> Produce Given/When/Then acceptance criteria for the leptos-cf production website across meaningful SSR, hydration, deep links, responsive navigation, keyboard/focus, 320px reflow, reduced motion, live-lab states, contact validation, security headers, local verification, and separate deployment/live readback evidence.

### To execute P4

**Skill:** `deliver-launch-checklist`
**Why this skill:** Coordinates the cross-functional production gate and rollback boundary.
**Source:** S1, S7

**Prompt:**
> Create a launch checklist for the leptos-cf public website with owner and date placeholders, exact-tree local proof, security policy and threat-model approval, visual/accessibility QA, Cloudflare resource binding, governed plan approval, execution receipt, provider readback, live route health, rollback triggers, and a 30-day learning baseline.

## Section 8. Evidence and source map

| Claim / recommendation | Source ID | Exact quote |
| --- | --- | --- |
| Production is the requested outcome | S1 | “spearhead to prod” |
| The current template identity should be replaced | S2 | “de-template it, make it beautiful” |
| The surface should be multi-page | S3 | “make a new website for this, pages” |
| Leptos Router and cargo-leptos are required | S4 | “leptos router, cargo-leptos” |
| Creative selection is delegated | S5 | “create images at your discression” |
| The product is the public starter | S6 | “`leptos-cf` is a public starter template for production-grade Leptos applications on Cloudflare Workers.” |
| Release claims require matching proof | S7 | “Every release claim is backed by local proof that matches the Cloudflare deployment surface.” |

**Inferred (Low confidence) claims:** Visitor segment details, emotional states, conversion objections, traffic, and analytics policy. None drives the binding constraint or P1.
**Evidence gaps:** Customer research, conversion baseline, production target identity, custom domain, analytics consent, and a real contact-delivery requirement.
