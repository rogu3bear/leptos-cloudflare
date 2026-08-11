---
artifact: launch-checklist
version: "1.0"
created: 2026-08-05
status: in-progress
---

# Launch Checklist: The Edge Field Guide

## Launch Overview

| Field | Value |
|---|---|
| What | Replace the generic starter UI with a public, multi-page leptos-cf product and learning site. |
| Launch Date | Not scheduled; provider mutation requires a reviewed cfctl plan and owner approval. |
| Launch Type | Major public-site revision |
| Launch Owner | Repository owner/operator |
| Go/No-Go Decision Maker | Repository owner/operator |

### Key Stakeholders

| Role | Name | Contact |
|---|---|---|
| Product | Repository owner/operator | Existing task |
| Engineering | Codex, with owner approval for protected actions | Existing task |
| Design | Codex under HORIZON; owner retains final authority | Existing task |
| Security | Repository owner/operator | Existing task |
| Operations | Repository owner/operator | Existing task |

## Engineering Readiness

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Implement selected route tree and shared shell | Codex | 2026-08-05 | Complete | `AppLayout` is inside `Router`; wildcard is last and returns the SSR recovery page with HTTP `404`. |
| Pass focused source and route checks | Codex | 2026-08-05 | Complete | 13 Rust unit tests and 8 WebKit route, interaction, boundary, and security tests pass. |
| Pass `./scripts/verify.sh` on the exact tree | Codex | 2026-08-05 | Complete | Passed after the HTTP 404 and repo-scoped `worker-build` corrections. |
| Review exact diff | Owner/operator | Before launch | Pending — blocker | No merge or deploy claim substitutes for review. |
| Prepare repository-bound D1 migration operation | Codex | 2026-08-10 | Prepared locally | `.cfctl/operations/d1-migrations.toml` binds the exact append-only migrations and closed readback assertions; no remote apply has occurred. |
| Update route/build/runtime documentation | Codex | 2026-08-05 | Complete | README, playbook, feature guide, edge deployment guide, and release guide reflect the current route and build contracts. |

## QA & Testing

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Execute route and interaction test plan | Codex | 2026-08-05 | Complete | WebKit covers all public routes, task mutations, contact persistence, session isolation, and SSR 404 recovery. |
| Complete wide/narrow visual regression | Codex | 2026-08-05 | Complete | 1440 and 390 matched review plus an explicit 320-pixel overflow pass and screenshot. |
| Complete keyboard and landmark review | Codex | 2026-08-05 | Complete | Skip link and visible focus are keyboard-reachable; every route has exactly one `main`. |
| Complete Safari smoke test | Owner/operator or Codex via existing Safari session | Before launch | Pending — blocker | Local or preview route set. |
| Confirm console/page-error-free hydration | Codex | 2026-08-05 | Complete | All seven public routes hydrate without captured console or page errors. |
| Complete security review | Owner/operator | Before launch | Pending — blocker | Policy and threat model have protected checkpoints. |

## Design & UX

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Select full-design direction | Codex under delegated discretion | 2026-08-05 | Complete | HORIZON Direction A. |
| Persist concept asset and provenance | Codex | 2026-08-05 | Complete | `docs/design/edge-field-guide-concept.png`. |
| Review full route/state visualization | Codex | 2026-08-05 | Complete | Visualize model reviewed as design evidence, separately from rendered runtime proof. |
| Complete rendered design QA | Codex | 2026-08-05 | Complete | Generated concept, Visualize artifact, and the rendered wide/narrow Leptos routes were compared. |
| Finalize truthful copy | Codex | 2026-08-05 | Complete | The site makes no telemetry, customer, hosted-delivery, or deployment claims without evidence. |
| Verify loading, empty, error, disabled, and not-found states | Codex | 2026-08-05 | Complete | Loading, empty, disabled, validation, SSR 404, and injected no-D1 recovery states are covered. |

## Marketing & Communications

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Lock primary action and message | Codex under delegated discretion | 2026-08-05 | Complete | “Use the starter”; “See every boundary before you ship.” |
| Update website and route copy | Codex | 2026-08-05 | Complete | Multi-page architecture-first field guide is implemented. |
| Prepare release notes | Owner/operator | Before launch | Deferred | Not requested; create only when release scope is accepted. |
| Create truthful screenshots | Codex | 2026-08-05 | Complete | Wide, 390-pixel, and 320-pixel screenshots are bound by SHA-256 in the visual receipt. |
| Confirm repository and documentation links | Codex | 2026-08-05 | Complete | Public source URL matches the configured git remote; internal route links are covered by WebKit navigation. |

## Customer Support

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Make support/contact capability truthful | Codex | 2026-08-05 | Complete | Contact is explicitly a bounded D1 intake lab with no email or operator-delivery promise. |
| Verify getting-started documentation | Codex | 2026-08-05 | Complete | Commands use pinned Wrangler and the lockfile-resolved repo-local `wasm-bindgen` path. |
| Name issue-reporting path | Owner/operator | Before launch | Pending | Use repository-native channel only if verified. |
| Define escalation path | Owner/operator | Before launch | Pending | Required only for an operational public service. |

## Legal & Compliance

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Confirm license presentation | Owner/operator | Before launch | Pending — blocker | Preserve repository license truth. |
| Confirm privacy representation | Owner/operator | Before launch | Pending — blocker | D1 intake storage and session behavior need truthful disclosure. |
| Complete accessibility conformance review | Codex, owner signs off | 2026-08-05 | Automated review complete; owner sign-off pending | WCAG-oriented keyboard, landmark, focus, and 320-pixel reflow evidence; not certification. |
| Confirm generated concept-image use | Owner/operator | Before launch | Pending | Concept is original generated material; no third-party marks intended. |

## Operations & Infrastructure

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Derive ignored production config from authenticated provider readback | Owner/operator | Just before release | Pending — blocker | Tracked `wrangler.toml` intentionally remains provider-neutral; create mode-0600 `wrangler.production.toml` only from verified Worker and D1 identity. |
| Review hash-bound cfctl plan | Owner/operator | Before deploy | Pending — blocker | Approval binds exact plan and expiry. |
| Apply provider change with cfctl | Codex after approval | Before launch | Pending — blocker | No direct Wrangler deployment mutation. |
| Read back provider state | Codex | Immediately after apply | Pending — blocker | Distinct from live HTTP readback. |
| Verify live routes and security headers | Codex | Immediately after apply | Pending — blocker | Authenticated where the surface requires it. |
| Document rollback target | Codex | Before deploy | Pending — blocker | Exact prior deployment/version and source SHA. |

## Analytics & Monitoring

| Item | Owner | Due | Status | Notes |
|---|---|---|---|---|
| Define a privacy-respecting adoption metric | Owner/operator | Before optimization | Pending | No analytics stack is currently evidenced. |
| Establish baseline | Owner/operator | After launch | Awaiting evidence | Do not fabricate conversion data. |
| Define uptime/error observation | Owner/operator | Before operational SLA | Pending | Provider and application signals are separate. |
| Record local proof receipt | Codex | 2026-08-05 | Complete | Ignored receipt binds HEAD, tracked patch, untracked candidate manifest, gate, browser, and visual evidence without claiming a commit. |

## Go/No-Go Criteria

### Must Have (Blockers)

- [x] Full local verification passes on the exact tree.
- [ ] Every in-scope route passes wide/narrow, keyboard, hydration, and error checks.
- [ ] Security policy and threat-model decisions are approved without weakening repository controls.
- [ ] Production bindings are non-placeholder and the cfctl plan is explicitly approved.
- [ ] Provider-state and live-route readbacks match the approved deployment.
- [ ] Rollback target and command are recorded before mutation.

### Should Have

- [ ] Safari smoke evidence from the existing browser session.
- [x] Source-bound public screenshots.
- [ ] Release notes after release scope is accepted.

### Nice to Have

- [ ] Privacy-respecting adoption instrumentation after the launch is stable.
- [ ] A later tested conversion experiment once real traffic exists.

## Rollback Plan

### Trigger Conditions

- Security headers, SSR, hydration, D1 session isolation, or static assets regress.
- Any primary route returns an unexpected error or the deployed version differs from the approved plan.
- The new shell prevents keyboard navigation or creates horizontal page overflow.

### Rollback Steps

1. Stop new promotion and record provider/live evidence without exposing secrets.
2. Use the approved cfctl rollback plan to restore the pre-recorded deployment/version.
3. Read back provider state and all primary routes, then open a source-local repair lane.

### Rollback Owner

Repository owner/operator, with Codex executing only an explicitly approved cfctl plan.

### Rollback Time Estimate

Unknown until the actual Cloudflare target and prior deployment are resolved in the plan.

## Check-in Schedule

| Checkpoint | Date | Attendees |
|---|---|---|
| Local implementation review | 2026-08-05 | Owner/operator and Codex |
| Security and deploy-plan approval | Before provider mutation | Owner/operator and Codex |
| Launch sync | At approved deploy window | Owner/operator and Codex |
| Post-launch readback | Immediately after deploy | Owner/operator and Codex |

## Open Issues

| Issue | Owner | Status | Impact |
|---|---|---|---|
| `wrangler.toml` contains placeholder D1 identifiers. | Owner/operator | Open | Launch blocker. |
| Security policy text requires explicit approval before write. | Owner/operator | Open | Launch blocker. |
| Threat-model context and assumptions require owner validation. | Owner/operator | Open | Launch blocker. |
| No live traffic or conversion evidence exists. | Owner/operator | Awaiting evidence | Optimization limitation, not a local-build blocker. |
