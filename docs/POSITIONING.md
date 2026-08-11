# Positioning & Messaging

## Competitive Alternatives

- Assemble Leptos, Wrangler, D1, asset serving, and security controls independently.
- Start from a generic Leptos example and discover Cloudflare constraints later.
- Use a framework-specific hosted platform with less runtime control.

## Unique Attributes → Value Themes

| Attribute | Value ("so what") | Proof |
|---|---|---|
| Explicit browser/Worker/SSR/data boundaries | Less architectural guesswork. | `NORTH_STAR.md`, runtime docs, Worker shim. |
| Checked-in release verifier | Claims can be reproduced locally against the deployment shape. | `scripts/verify.sh`. |
| Fail-closed starter controls | Safer defaults survive first use. | Security headers, body limits, cookie and token doctrine. |
| Real D1-backed lab | Evaluators can inspect server actions and persistence. | Existing task-board components and API. |

## Best-Fit Customer

An engineer who values transparent boundaries and local proof more than a one-click demo.

## Market Category

Existing category: open-source Leptos starter/reference implementation for Cloudflare Workers. Creating a new category would add language without evidence.

## One-Liner

See every boundary before you ship: leptos-cf is a source-derived field guide and starter for full-stack Leptos on Cloudflare Workers.

## Brand Script (StoryBrand)

- Character: an engineer who wants a credible edge-native Leptos foundation.
- External problem: the browser, Worker, SSR, assets, D1, and hydration path crosses several systems.
- Internal problem: hidden ownership makes every failure feel like guesswork.
- Philosophical problem: a starter should reveal the critical path, not conceal it.
- Guide: the repository provides explicit source, docs, labs, and local verification.
- Plan: trace the request, use the starter, verify the exact tree.
- Call to action: Use the starter.
- Failure: adopt opaque glue and debug runtime boundaries late.
- Success: start from a system whose responsibilities and proof planes are visible.

## Key Messages

| Surface | Message | Status |
|---|---|---|
| Home | See every boundary before you ship. | Selected |
| Architecture | Follow one request from browser intent to hydrated interaction. | Selected |
| Start | Clone, configure, run locally, then verify the exact tree. | Selected |
| Lab | A real local mutation path; not a hosted product or live trace. | Selected |
| Trust | Source-derived is not the same as deployed and observed. | Selected |
