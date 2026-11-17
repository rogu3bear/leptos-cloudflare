# Leptos CF

A full-stack Rust starter for Cloudflare Workers and Pages, built with
[Leptos 0.8](https://leptos.dev/). One crate renders HTML on the server and
hydrates it in the browser. Typed server functions connect the UI to D1.

The field guide explains the architecture. `/lab` and the Resource Explorer
show session-scoped persistence, validation and retry; `/contact` stores bounded
intake without sending email or webhooks. Keep these examples while adopting
the runtime, then replace their consumers with your application.

## Start with an agent

Give a coding agent this repository and your application goal:

> Read AGENTS.md and follow the local Quick Start. Use Workers unless I selected
> Pages. Preserve the runtime security boundaries, make the requested change,
> and run the required verification. Prepare provider changes separately;
> do not deploy or publish without my approval.

[AGENTS.md](AGENTS.md) maps owners, commands and control boundaries. The
[agent playbook](docs/agent-playbook.md) covers feature work and troubleshooting.
No agent plugin, Cloudflare credential or cfctl installation is required locally.

## Quick Start (New Project)

Use GitHub's **Use this template** button to create your repository, then clone
it. To try the reference source directly:

```bash
git clone https://github.com/rogu3bear/leptos-cloudflare.git my-app
cd my-app
cargo install cargo-audit --locked
./scripts/bootstrap.sh
./scripts/check-deps.sh
CI=1 ./scripts/wrangler.sh d1 migrations apply leptos-cf-db --local
bash ./scripts/build-edge.sh
./scripts/wrangler.sh dev --local --ip 127.0.0.1 --port 57581
```

Open `http://127.0.0.1:57581`. Bootstrap requires Rust/rustup, Bun and Node.js
22+ on PATH. Install the required `cargo-audit` as shown above; bootstrap checks
it and installs the other pinned or lockfile-resolved build tools.
Wrangler is repository-local; no global Wrangler is needed.

To name your application, run `./scripts/init.sh my-app` before adding provider
identity. Then substitute `my-app-db` for `leptos-cf-db` in local commands.
Adoption preserves source and migrations and switches off reference-site wording
checks while retaining runtime checks. See [adoption](docs/adopting.md).

## Choose a deployment target

Workers is the default. `wrangler.toml` combines generated `build/_worker.js`
with static assets in `target/site`. Exact assets may be served before the Worker;
other requests reach Leptos SSR or server functions.

Pages uses the same application through advanced mode, isolated configuration
and a separately packaged module graph. To try it after the edge build:

```bash
bun ./scripts/package-pages.mjs target/pages
cd pages
CI=1 ../scripts/wrangler.sh d1 migrations apply leptos-cf-db --local
../scripts/wrangler.sh pages dev
```

Packaging requires an empty output directory. Use a fresh directory or deliberately
remove only your own obsolete output. Use the adopted database name after renaming.
Pages receives all requests and forwards public assets to `ASSETS`; server modules
stay private. Production/preview identities are separate.

## Choose a control lane

| Need | Without cfctl | With cfctl in a governed account |
|---|---|---|
| Build, test, local D1 and preview | Repository scripts; no credentials | Same local scripts |
| Discover provider capabilities | Standalone Wrangler help and Cloudflare documentation | Installed catalog, resolve and capability guides |
| Create resources, migrate or deploy | Scoped credentials and approved standalone procedure | Account/profile-bound plan, exact approval, execution and verification |
| Establish deployed behavior | Provider readback and live workflow checks | Governed readback and live workflow checks |

cfctl adds account control, workspace impact, plan review and operational receipts;
it is optional for independent adopters. Its presence alone authorizes nothing.
An already governed account must not switch lanes to bypass a failed capability.
Start with [credential profiles](docs/credentials.md), then use
[cfctl control](docs/cfctl.md) when applicable.

Tracked IDs stay placeholders. Derive ignored production configuration from
verified provider identity. Local success does not establish a live deployment.
Follow [Workers deployment](docs/edge-deployment.md) or
[Pages production preparation and readback](docs/pages-production.md).

## Change and verify

Before the full gate, set `LEPTOS_RELEASE_BASELINE` to the recorded outgoing
bundle ID, or `first-release` for a new application’s initial release.
See [release switches](RELEASE.md#hydration-assets-during-release-switches).

```bash
cargo check --features ssr       # focused server iteration
cargo fmt --check
./scripts/verify.sh              # complete local release-readiness gate
```

The full gate checks adoption/configuration contracts, formatting, Rust tests,
dependency audit, both WASM builds, asset integrity, WebKit hydration and
server-function checks, Workers and Pages runtime boundaries, the prepared
Pages production bundle and a deployment dry run.
It performs no live deployment. Missing mandatory tools fail visibly. Current
dependency warnings are documented in [RELEASE.md](RELEASE.md).
Bootstrap installs pinned Playwright WebKit. For focused checks, use the
[agent playbook](docs/agent-playbook.md). Remote CI is intentionally absent.
Adding hosted workflows requires an explicit operator decision.

| Change | Start here |
|---|---|
| Route, page, server function or D1 domain | [Building features](docs/building-features.md) |
| Adopt name, branding and application content | [Adopting the starter](docs/adopting.md) |
| Runtime and build ownership | [Agent playbook](docs/agent-playbook.md) |
| Resource Explorer and extension examples | [Patterns](patterns/README.md) |
| WebSocket vs shared Durable Object state | [Realtime](docs/realtime.md) |
| Containers or Tunnels | [Hybrid deployment](docs/hybrid-deployment.md) |
| Release procedure and proof boundaries | [RELEASE.md](RELEASE.md) |

## Source map

```text
src/app.rs           Leptos routes
src/components/      application UI
src/api.rs           typed server functions
src/server/          request security, session-scoped D1 and SSR
runtime/             shared asset/request dispatch
style/               CSS
assets/              public icons and manifest
migrations/          append-only schema
scripts/             pinned build, adoption and verification
pages/               isolated Pages configuration
patterns/            documented extensions and executable examples
.cfctl/operations/   optional governed migration contract
docs/                guides, acceptance and design records
```

Generated output and local credentials stay ignored. Core security controls
include bounded request bodies, nonce-based CSP, anti-framing, session cookies,
session-scoped sample data and private-module denial. Preserve them when adopting.
See [SECURITY.md](SECURITY.md) and [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT

Anonymous writes use per-IP admission budgets shared across cookies and users behind one NAT.
Only a keyed digest is stored; ownership remains session-scoped. Full storage rejects
new writes without deleting existing records. Apply all local migrations, including
`0004_anonymous_admission.sql`, before running the updated application.
