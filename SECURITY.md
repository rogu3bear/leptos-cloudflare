# Security Policy

## Supported Versions

This template is released from `main`. Security fixes are expected to land on `main` first.

## Reporting a Vulnerability

Open a private security advisory on GitHub if available, or contact the maintainer through the repository owner profile. Do not publish exploit details in a public issue before the maintainer has had a reasonable chance to respond.

## Template Security Model

The starter intentionally ships with:

- placeholder Cloudflare and D1 identifiers
- no committed secrets
- `HttpOnly` session cookies for demo data ownership
- D1 queries scoped by browser session
- bounded server-function request bodies
- CSP, anti-framing, `nosniff`, referrer policy, and no-store dynamic responses
- hashed immutable static assets served through Workers Assets
- repo-local `wasm-bindgen-cli` resolved from `Cargo.lock`

New applications built from the template should add their own authentication, authorization, rate limits, audit logging, and production data retention rules before handling real users.

## Secret Handling

Use `.dev.vars` for local Wrangler secrets and `wrangler secret put` for production secrets. Both `.env` and `.dev.vars*` are ignored. `.env.example` must remain placeholder-only.

## Release Security Gate

Before a public template release, run:

```bash
cargo audit
rg -n --hidden --glob '!target/**' --glob '!build/**' --glob '!var/**' --glob '!.git/**' \
  --glob '!SECURITY.md' \
  '(sk_live_|sk_test_|AKIA|ghp_|github_pat_|ca30e922|CLOUDFLARE_API_TOKEN="[A-Za-z0-9_-]{20,}"|[0-9a-f]{32})' .
bash ./scripts/build-edge.sh
bunx wrangler@4.83.0 deploy --dry-run
```
