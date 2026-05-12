# AGENTS.md

This repository is an agent-first public template. Agents should optimize for reproducibility, explicit boundaries, and truthful release evidence.

## Required Start

Before non-trivial edits:

```bash
pwd
git branch --show-current
git status --short
```

Treat dirty work as live operator intent. Classify it before editing; do not overwrite or discard it.

## Build and Verification

Use these gates for template changes:

```bash
./scripts/check-deps.sh
cargo fmt --check
cargo check --features ssr
bash ./scripts/build-edge.sh
bunx wrangler@4.83.0 deploy --dry-run
git diff --check
```

`./scripts/check-deps.sh` may warn about placeholder D1 IDs in `wrangler.toml`; that warning is expected before a project is initialized from the template.

## Runtime Boundaries

- `wrangler.toml` must keep `main = "build/_worker.js"`.
- `scripts/write-worker-shim.mjs` generates the Cloudflare entrypoint.
- Workers Assets serve static files from `target/site` through `env.ASSETS`.
- Non-asset requests fall through to Leptos SSR/server functions.
- `/realtime/socket` is the only template WebSocket capability route.
- Shared realtime state belongs in Durable Objects, not in Leptos components or request-local Worker state.

## Security Rules

- Never commit real Cloudflare account IDs, tokens, D1 IDs, tunnel tokens, Stripe keys, or `.dev.vars`.
- Keep `.env.example` placeholder-only.
- Keep CSP, anti-framing, body limits, session cookies, and D1 session scoping intact unless replacing them with stronger controls.
- Do not weaken `scripts/check-deps.sh` or the build verifiers to make a release pass.

## Documentation Rules

If you change routing, build tools, Cloudflare bindings, realtime behavior, D1 usage, or initialization behavior, update the matching docs:

- `README.md`
- `docs/agent-playbook.md`
- `docs/edge-deployment.md`
- `docs/building-features.md`
- `docs/realtime.md`
- `RELEASE.md`
