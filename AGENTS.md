# Agent instructions

A single-crate Leptos starter for Cloudflare Workers and Pages. The reference
application demonstrates SSR, hydration, server functions and session-scoped D1.
Change application behavior while preserving those runtime boundaries.

## Start here

1. Read [README.md](README.md), inspect `git status --short`, and preserve existing
   work. Follow enclosing instructions when applicable.
2. Use Workers unless the owner selected Pages. Local development needs no
   Cloudflare account, token or cfctl. Do not provision just to run the sample.
3. Run `./scripts/check-deps.sh`; use `./scripts/bootstrap.sh` for missing build
   tools. Bootstrap needs Rust/rustup, Bun, Node.js 22+ and `cargo-audit` already
   installed; the README Quick Start includes the audit-tool installation.
4. Follow [the playbook](docs/agent-playbook.md) for focused checks. Run
   `./scripts/verify.sh` before release-readiness claims. Missing tools or failed
   checks mean incomplete verification; never weaken a gate to get green.

## Choose control explicitly

| Context | Entry point | Boundary |
|---|---|---|
| Local development | README Quick Start; pinned `scripts/wrangler.sh` | No credentials or cfctl; use `--local` for D1 |
| Independent standalone account | [Credentials](docs/credentials.md), [Workers](docs/edge-deployment.md), [Pages](docs/pages-production.md) | Scoped credentials and approved remote operations |
| Account governed by cfctl | [cfctl control](docs/cfctl.md) | Discover installed capabilities, bind account/profile/targets, plan, obtain exact approval, execute, verify |

Installing cfctl grants neither provider authority nor account selection. An
unavailable governed capability is a hold, not permission to switch to raw API,
a dashboard or direct Wrangler writes. Local work can continue.
Checkout-specific guidance belongs in ignored `*.local.md` or `.agent/`; public
adopters need none. If `.agent/operator-instructions.local.md` exists, read it for checkout-specific
operator rules. That optional file is absent from the public template.

## Source ownership

| Surface | Owner |
|---|---|
| Routes and UI | `src/app.rs`, `src/components/`, `style/main.css` |
| Typed server functions and validation | `src/api.rs`, domain types, `src/server/` |
| Worker/SSR and request security | `src/lib.rs`, `src/server/`, `runtime/dispatch.mjs` |
| Generated Worker and asset manifest | `scripts/build-edge.sh`, `scripts/write-worker-shim.mjs`, `scripts/hash-assets.mjs` |
| Workers contract | `wrangler.toml`: keep `main = "build/_worker.js"`, Assets in `target/site` through `env.ASSETS` |
| Pages contract | `pages/wrangler.toml`, `scripts/package-pages.mjs`, `scripts/prepare-pages-release.mjs` |
| Schema and governed migration contract | Append-only `migrations/`, `.cfctl/operations/d1-migrations.toml` |

Keep CSP, anti-framing, body limits, cookies, D1 session scoping and private-module
denials intact. `/realtime/socket` is the sole template WebSocket route; shared
state belongs in Durable Objects. Read [realtime guidance](docs/realtime.md).

Never commit real account/D1 IDs, tokens, `.env`, `.dev.vars*`, or generated
`build/`, `target/`, `var/` and `.wrangler/` contents. Tracked configuration stays
placeholder-only; derive ignored production configuration from verified identity.
Use [adoption](docs/adopting.md) before binding a new application's identity.
Replace sample consumers before removing their code; never delete deployed data
or migration history because a sample page disappeared.

Update matching docs when changing runtime, routing, bindings or build tools.
Remote CI is intentionally absent: do not add or trigger hosted workflows without
an explicit owner decision. Build, local runtime, source publication, deployment
and live acceptance are separate claims. Report the candidate, changed behavior,
checks and limits. Provider mutation and publication need applicable approval.
