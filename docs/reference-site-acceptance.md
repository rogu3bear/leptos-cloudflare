---
artifact: acceptance-criteria
version: "1.0"
created: 2026-08-05
status: draft
---

# Acceptance Criteria — production website launch slice

## Story Context

These criteria bind US-001 through US-005: an architecture-first, multi-page public website for `leptos-cf` that preserves the installed Leptos Router, SSR/hydration, Cloudflare Worker/Assets, D1 demo, contact safety, and exact release-proof contracts.

## Happy Path

### AC-1 — Meaningful server-rendered first visit

**Given** a first request to any public route before browser WASM executes
**When** the Worker renders the route
**Then** the response contains route-specific title/description, one `main`, one clear `h1`, useful page content, navigation, and a reachable next action.

### AC-2 — Architecture and patterns are discoverable

**Given** a visitor is evaluating the stack
**When** they use the primary navigation or homepage actions
**Then** they can reach Architecture, Patterns, Live Lab, About/Security evidence, and Start guidance without depending on hydration.

### AC-3 — Live Lab preserves behavior

**Given** local or production D1 is correctly bound
**When** a visitor creates, opens, toggles, or deletes a lab item
**Then** the correct pending, success, empty, error, disabled, and dynamic-route behaviors are observable and data remains scoped to the session contract.

## Edge Cases

### AC-4 — Narrow and stressed content reflow

**Given** a 320 CSS-pixel viewport, 200% zoom, WCAG text spacing, long headings, or long route text
**When** any public route renders
**Then** content reflows without page-level horizontal overflow, clipped focus, contradictory DOM order, or obscured primary action.

### AC-5 — Motion and imagery remain optional

**Given** reduced motion is requested or generated imagery fails to load
**When** the homepage and architecture route render
**Then** all meaning, actions, and architectural relationships remain available without animation or decorative media.

### AC-6 — Deep links and unknown routes

**Given** a hard refresh on a static route, lab detail route, or unknown path
**When** Cloudflare dispatches the request
**Then** known routes receive full SSR and unknown routes receive a useful SSR 404 with recovery links; the wildcard remains last.

## Error States

### AC-7 — Contact validation and recovery

**Given** invalid, oversized, cross-origin, honeypot, rate-limited, or storage-failure contact input
**When** submission is attempted
**Then** the server fails closed, no sensitive internal detail is exposed, and the user receives an actionable state without losing unrelated page context.

### AC-8 — D1 or server-function failure

**Given** the Live Lab dependency fails
**When** the route or action resolves
**Then** an error state identifies the failed task, offers recovery where safe, and does not misrepresent the application as verified or live.

## Non-Functional Criteria

### AC-9 — Keyboard and landmark integrity

**Given** keyboard-only navigation
**When** the visitor traverses the site and forms
**Then** a skip link, linked brand, navigation, actions, lab controls, validation states, and footer are reached in logical order with visible focus and no nested `main` landmarks.

### AC-10 — Security controls remain at least as strong

**Given** the redesigned candidate
**When** security tests and source inspection run
**Then** CSP, anti-framing, `nosniff`, referrer policy, no-store dynamic responses, body limits, same-origin API policy, prepared D1 operations, session cookies, and secret exclusions remain intact or stronger.

### AC-11 — Exact-tree release proof

**Given** a release candidate at a full SHA and classified dirty state
**When** readiness is claimed
**Then** `./scripts/verify.sh` passes on that exact tree and visual/accessibility P0–P2 findings are closed without weakening a verifier.

### AC-12 — Production evidence remains separated

**Given** a reviewed Cloudflare deployment plan
**When** the owner approves and execution occurs
**Then** plan, approval, execution, provider readback, deployment URL, and live runtime readback are reported as separate states bound to the artifact identity.

## Notes

- Conversion outcomes remain unverified until research and analytics exist.
- Contact is a bounded D1 intake demonstration unless a real delivery/retrieval and retention workflow is owner-approved.
