# Leptos CF — Edge Field Guide

A source-derived field guide and full-stack [Leptos 0.8](https://leptos.dev/) starter for Cloudflare Workers. The public site maps the request path, names runtime ownership, and keeps source, local verification, provider state, and live behavior visibly separate.

The field guide lives at `/`, `/start`, `/architecture`, `/patterns`, and `/about`. Two bounded implementation labs remain intentionally inspectable: `/lab` demonstrates session-scoped D1 mutations through Leptos server functions, while `/contact` demonstrates validated D1 intake without claiming email, webhook, Queue, or operator delivery.

## Why This Stack

Leptos compiles to WASM on both sides of the wire. The server renders HTML on Cloudflare's edge network, the client hydrates it with the same component code, and server functions give you a typed RPC boundary between the two. No JavaScript framework, no REST boilerplate, no separate API layer.

Cloudflare gives you the deployment surface: Workers for compute, D1 for SQL, Workers Static Assets for exact files, Tunnels for exposing local dev to the internet, and Containers for when you outgrow the Worker sandbox. The template deploys the Worker and `target/site` as one Workers unit. It does not create a Pages project; add that separate lane only when a named consumer requires its own Pages workflow and proof boundary.

**What you get out of the box:**

- Single Rust crate with feature-flagged SSR/hydration split
- Worker entrypoint using `workers-rs` + `axum`
- Leptos server functions as the full-stack boundary
- D1 access layer with prepared statements
- Public contact intake with server-side validation and session-scoped abuse caps
- Client-side optimistic UI with loading and error states
- Sampled Workers Logs (10%) and traces (1%) for production visibility without capturing every invocation
- Bootstrap scripts that verify your toolchain
- A setup flow designed for AI coding agents

## Quick Start (New Project)

```bash
git clone https://github.com/rogu3bear/leptos-cloudflare.git my-app
cd my-app
./scripts/bootstrap.sh
./scripts/check-deps.sh
CI=1 bunx wrangler@4.120.1 d1 migrations apply leptos-cf-db --local
bash ./scripts/build-edge.sh
bunx wrangler@4.120.1 dev --local --ip 127.0.0.1 --port 57581
```

This path uses the checked-in placeholder D1 binding only for local development. Production initialization is a separate, reviewed provider change. To adopt a different name before adding provider identity, run `./scripts/init.sh my-app`. It preserves the working example and migrations; see [Adopting the starter](docs/adopting.md).

## Table of Contents

- [Quick Start (New Project)](#quick-start-new-project)
- [Agent-First Setup](#agent-first-setup)
- [Manual Setup](#manual-setup)
- [Cloudflare API Tokens](#cloudflare-api-tokens)
- [Local Development](#local-development)
- [Deployment](#deployment)
- [De-templating](#de-templating)
- [Cloudflare Tunnels](#cloudflare-tunnels)
- [Cloudflare Containers](#cloudflare-containers)
- [Project Structure](#project-structure)
- [Architecture Notes](#architecture-notes)

---

## Agent-First Setup

The fastest path from clone to deploy is to hand the project to an AI coding agent (Claude Code, Codex, etc.) that can use a governed Cloudflare control plane. The agent should prepare and verify the source without a provider credential, receive a just-in-time short-lived child only for the approved release window, inventory the account, create only missing resources through reviewed plans, derive the ignored production config from provider readback, apply the repository-bound migration operation, deploy, and read the resulting provider state back.

### What the agent needs

1. **A governed short-lived Cloudflare child token** with the permissions described in [Cloudflare API Tokens](#cloudflare-api-tokens); do not give the deployment process the token-minter credential
2. **Your Cloudflare Account ID** (visible at the top of any zone's overview page, or under Workers & Pages)
3. **The tools installed** (Rust, `cargo-leptos`, Bun) -- or let the agent run `./scripts/bootstrap.sh`

### Give the agent the account identity

```bash
export CLOUDFLARE_ACCOUNT_ID="your-account-id"
export LEPTOS_CF_PROFILE="your-short-lived-profile"
```

Install the short-lived child in the control plane's secret store or repo-local gitignored `.env` without printing it. Secret values must not appear in prompts, argv, logs, or committed files.

### Then tell it what to do

> Bootstrap this project for Cloudflare. Bind the exact checkout and account, use
> a governed short-lived child credential, read or create the named D1 database
> through a reviewed plan, derive ignored wrangler.production.toml from provider readback, apply the
> reviewed migration, build and deploy through reviewed plans, then read back the
> Worker version, bindings, assets, routes, and observability configuration.

In a `cfctl`-governed workspace, the lifecycle is:

```bash
# 1. Install and prove the local toolchain and candidate
./scripts/bootstrap.sh
./scripts/verify.sh

# 2. Bind and audit the exact checkout/account
cfctl doctor
cfctl workspace add "$PWD" --account "$CLOUDFLARE_ACCOUNT_ID" --json
cfctl workspace audit --json

# 3. Read adopted identities from this checkout, then query with the pinned profile
LEPTOS_D1_NAME=$(bun -e 'console.log(Bun.TOML.parse(await Bun.file("wrangler.toml").text()).d1_databases[0].database_name)')
LEPTOS_MIGRATION_ID=$(bun -e 'console.log(Bun.TOML.parse(await Bun.file(".cfctl/operations/d1-migrations.toml").text()).operation[0].id)')
cfctl call d1-list-databases \
  --selector "account_id=$CLOUDFLARE_ACCOUNT_ID" \
  --query "name=$LEPTOS_D1_NAME" \
  --profile "$LEPTOS_CF_PROFILE" \
  --json

# 4. If absent, create a preview plan (this does not mutate Cloudflare)
printf '{"name":"%s","read_replication":{"mode":"disabled"}}' "$LEPTOS_D1_NAME" | \
  cfctl call d1-create-database \
    --selector "account_id=$CLOUDFLARE_ACCOUNT_ID" \
    --profile "$LEPTOS_CF_PROFILE" \
    --body-stdin \
    --json

# 5. Review the returned operation, request explicit approval, then use
#    cfctl plans approve/run/status. Accept identity only from verified readback.
# 6. Derive ignored wrangler.production.toml from those verified identities.
bun ./scripts/write-production-config.mjs \
  --worker <verified-worker-name> \
  --database <verified-d1-name> \
  --database-id <verified-d1-uuid>

# 7. Prepare the repository-owned migration operation and Worker deployment.
cfctl call "$LEPTOS_MIGRATION_ID" \
  --selector "account_id=$CLOUDFLARE_ACCOUNT_ID" \
  --selector "database_id=<verified-d1-uuid>" \
  --query config=wrangler.production.toml \
  --profile "$LEPTOS_CF_PROFILE" \
  --json
cfctl call wrangler.deploy \
  --account "$CLOUDFLARE_ACCOUNT_ID" \
  --profile "$LEPTOS_CF_PROFILE" \
  --query "config=$PWD/wrangler.production.toml" \
  --query "name=<verified-worker-name>" \
  --query "message=<exact-source-and-artifact-identity-from-cfctl>" \
  --json
```

Use the exact Worker name from provider readback and the exact source/artifact
identity required by cfctl for this clean candidate and build; do not invent that
message or reuse it after source or artifact changes. A mismatched message fails
plan preparation and reports the required value; review that value and repeat
preparation with it. Each returned plan retains
its pinned profile and account through `plans approve/run/status`; do not switch
credentials between preparation and execution. The migration ID above is read
from the operation pack because `init.sh` changes it along with the project name.

The placeholder `00000000-0000-0000-0000-000000000000` IDs in tracked `wrangler.toml` are a permanent fail-closed template boundary. Do not replace them. `scripts/write-production-config.mjs` validates provider-read identities and changes only the Worker name, D1 name, and the two D1 UUID fields while preserving the Workers SSR, Assets, compatibility, and observability contract. Its output, `wrangler.production.toml`, is root-level, mode `0600`, and gitignored. A plan preview, local build, or Wrangler dry run is not provider proof.

### Local control-plane note

In this `~/dev` workspace, live Cloudflare account reads, standards audits,
mutation planning, execution, and post-change verification use `cfctl` from
`PATH`, backed by `/Users/star/dev/cloudflare`. Direct Wrangler provider writes
are not a fallback when a `cfctl` capability fails closed:

```bash
cfctl doctor
cfctl workspace audit --json
```

The generic `wrangler.d1` and `d1-import-database` catalog entries remain intentionally unsuitable for this lifecycle. The repository-owned operation binds the canonical clean Git root and HEAD, operation-pack and migration blobs, ignored production-config hash, exact database identity, fresh pre-change recovery bookmark, Wrangler migration ledger, and closed post-schema assertions. A blocked operation is a control-plane stop condition, not permission to apply a remote migration out of band.

### Why this matters

Cloudflare's infrastructure is fully API-driven. Every dashboard action has a CLI or API equivalent. This means a coding agent with the right token can provision your entire stack -- databases, secrets, tunnels, DNS records -- without you context-switching to a browser. You stay in your editor, the agent does the ops.

---

## Manual Setup

### Required tools

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked --version 0.3.5
cargo install worker-build --locked --version 0.7.5
```

This template uses `bunx wrangler@4.120.1`, so a global Wrangler install is not required. You do need [Bun](https://bun.sh/). `wasm-bindgen-cli` is installed into `var/cargo-tools/` from the version resolved in `Cargo.lock`, so the build does not depend on whichever global `wasm-bindgen` happens to be on `PATH`.

### Bootstrap scripts

```bash
./scripts/check-deps.sh    # verify all tools are present
./scripts/bootstrap.sh     # install missing tools, then verify
```

### Create your D1 database

The following is the portable Wrangler path for operators who are not using a governed control plane:

```bash
bunx wrangler@4.120.1 d1 create leptos-cf-db
```

Wrangler prints a `database_id`. Use it to derive `wrangler.production.toml` with `scripts/write-production-config.mjs`; never patch the tracked template. In this operator workspace, use the governed `cfctl` lifecycle above instead.

### Apply the initial migration

```bash
# Local (for wrangler dev)
bunx wrangler@4.120.1 d1 migrations apply leptos-cf-db --local

# Remote (for production)
bunx wrangler@4.120.1 d1 migrations apply leptos-cf-db --remote --config wrangler.production.toml
```

### Build and run locally

```bash
bash ./scripts/build-edge.sh
bunx wrangler@4.120.1 dev --local --ip 127.0.0.1 --port 57581
```

### Deploy

```bash
bunx wrangler@4.120.1 deploy --config wrangler.production.toml
```

---

## Cloudflare API Tokens

Local builds and local Wrangler development require no Cloudflare credentials.
For a provider operation, use an account-scoped token limited to that operation's
release window. In this operator workspace, cfctl owns token storage, lifecycle,
approval, execution, and verification. The repository never receives the minter
credential and no longer delegates rotation to an unshipped parent-directory
script.

Independent public adopters can use Cloudflare's scoped-token setup with the
portable Wrangler commands below; installing cfctl is not required for that
standalone path. Read [Credential profiles](docs/credentials.md) before choosing
a lane. A failed governed operation never authorizes switching to the standalone
lane in an already governed account.

---

## Local Development

```bash
bash ./scripts/build-edge.sh
bunx wrangler@4.120.1 dev --local --ip 127.0.0.1 --port 57581
```

Wrangler serves the Worker and the asset bundle from `target/site`. The `/lab` UI talks to D1 only through Leptos server functions.
Each browser session gets its own cookie-scoped lab queue and contact submission budget. The starter only renders the newest 100 lab rows from that queue so the public demo stays bounded.

**Local secrets**: Create a `.dev.vars` file (gitignored) for local environment variables:

```
SECRET_KEY=dev-value-here
```

**Iterating**: Run `cargo leptos watch` in a second terminal for automatic client rebuilds. The Worker itself needs a `wrangler dev` restart to pick up server-side changes.

---

## Deployment

Once ignored `wrangler.production.toml` has been derived from provider-read identity and the remote migration has been applied:

```bash
bunx wrangler@4.120.1 deploy --config wrangler.production.toml
```

That command documents the portable starter path. In this operator workspace, production changes use the repository's governed `cfctl` plan/approval/run/status/readback lane; a successful local build or Wrangler dry run is not deployment proof. Use the complete account/profile/name/config/message-pinned deployment example in [the governed lifecycle](#then-tell-it-what-to-do). It prepares a plan and does not authorize or execute it.

Wrangler runs the configured build command:

1. `cargo leptos build --release` -- compiles the client WASM + CSS
2. `bun ./scripts/hash-assets.mjs` -- fingerprints the client JS/CSS/WASM and updates the SSR asset constants
3. `scripts/with-wasm-bindgen-cli.sh worker-build --release --features ssr` -- compiles the Worker bundle against those hashed asset names with the same lockfile-matched CLI
4. `bun ./scripts/write-worker-shim.mjs` -- emits `build/_worker.js`, a Module Worker router that sends static assets to `env.ASSETS` and all SSR/server-function traffic to the compiled Leptos Worker handler

That produces:

- Client assets in `target/site/`
- Asset manifest in `target/site/asset-manifest.json`
- Cloudflare cache header rules in `target/site/_headers`
- Compiled Leptos Worker module in `build/index.js`, generated by `worker-build`
- Worker shim/router in `build/_worker.js`, which is the `main` entrypoint in `wrangler.toml`

Cache behavior is split cleanly:

- Cloudflare Workers Assets serves matching files from `target/site` directly through the `ASSETS` binding, without invoking the Rust SSR router
- The generated `_worker.js` keeps the same separation when it receives asset-prefixed requests directly: `/pkg/*`, app icons, `site.webmanifest`, and `/asset-manifest.json` go to `env.ASSETS.fetch(request)`, while deep routes and `/api/*` fall through to Leptos SSR/server functions
- WebSocket upgrades have one explicit template route, `/realtime/socket`; all production realtime work should either keep request-scoped behavior there or graduate shared state to Durable Objects
- The Rust route list includes an explicit Leptos catch-all route, so an unknown document request receives the useful SSR recovery shell with an HTTP `404` status instead of a bare platform response
- Hashed `/pkg/*` assets ship with `Cache-Control: public, max-age=31536000, immutable`
- Dynamic Worker responses (`/`, route HTML, server functions) ship with `Cache-Control: no-store`
- `asset-manifest.json` is also `no-store`, so deploys never strand old asset pointers

Observability uses two deliberately small contracts. The generated Worker shim creates a custom span only after asset and WebSocket routing has exited, using the closed boundary value `ssr` or `server_function`. The Rust Worker emits one versioned JSON completion event containing only route/server-function enums, method family, outcome, status, and duration. It never records raw URLs, query strings, route parameters, headers, cookies, session or database identifiers, request bodies, submitted fields, IP/user-agent values, or internal error text. Provider logs remain sampled at 10% and traces at 1%; after an authorized deploy, read back those settings and prove representative SSR and server-function signals without treating sampling as an exhaustive audit log.

See [Realtime and WebSockets](docs/realtime.md) before adding chat, presence, collaboration, live dashboards, or any other WebSocket-backed feature. The short rule: request-scoped upgrades can stay in the Worker; shared state belongs in a Durable Object. The first shared-state example lives in [Realtime Durable Object](patterns/realtime-durable-object/).

### Setting secrets

```bash
# Interactive (one at a time)
bunx wrangler@4.120.1 secret put SECRET_KEY
```

### Dry run

```bash
bunx wrangler@4.120.1 deploy --dry-run
```

---

## De-templating

Run `./scripts/init.sh <project-name>` in a fresh, provider-neutral clone to
adopt Cargo, Worker, D1, and migration-operation identity together. It preserves
all routes, domain code, and migration bytes, and selects application verification
instead of reference-site copy checks. Invalid, customized, or provider-bound
inputs are rejected before changes. Repeating the same name is a no-op.

Then follow [Adopting the starter](docs/adopting.md) to replace page content and
exercise a real mutation. Runtime verification remains mandatory after branding
and page changes. Removing a sample domain is a separate application change:
update its routes, API, consumers, schema contract, and application tests together.
The initializer does not delete those contracts or claim the application is ready
for production. Run `./scripts/verify.sh` after the complete cutover.

---

## Cloudflare Tunnels

[cloudflared](https://github.com/cloudflare/cloudflared) creates outbound-only connections from your machine to Cloudflare's edge. This is useful for:

- Exposing your local dev server to the internet (webhook testing, OAuth callbacks, mobile testing)
- Giving teammates access to your local branch without deploying
- Connecting private services that Workers can route to in production

### Quick dev tunnel (no account needed)

```bash
# Expose your local wrangler dev server instantly
cloudflared tunnel --url http://localhost:57581
```

This gives you a random `*.trycloudflare.com` URL. No login, no config. Good for quick testing.

### Named tunnel (persistent, custom domain)

```bash
# Install
brew install cloudflare/cloudflare/cloudflared

# Authenticate (opens browser once)
cloudflared tunnel login

# Create a named tunnel
cloudflared tunnel create leptos-cf-dev

# Route a subdomain to it
cloudflared tunnel route dns leptos-cf-dev dev.yourdomain.com

# Run it
cloudflared tunnel run --url http://localhost:57581 leptos-cf-dev
```

### Agent-managed tunnels

An agent with a `Cloudflare Tunnel: Edit` permission on its token can create and configure tunnels programmatically. For non-interactive use (CI, agents), use **remotely-managed tunnels** with a tunnel token instead of `cloudflared tunnel login` (which opens a browser):

```bash
# The tunnel token is available after creating a tunnel in the dashboard
# or via the API -- it bypasses the browser login flow
cloudflared tunnel run --token <TUNNEL_TOKEN>
```

---

## Cloudflare Containers

> **Status: Public Beta** (launched June 2025). Workers Paid plan required ($5/month).

Containers run alongside Workers, built on Durable Objects. A Worker acts as the gateway; containers handle workloads that need a full Linux environment -- long-running processes, native binaries, GPU access, or anything that doesn't fit in the Worker sandbox.

### Why this matters for Leptos

Workers are perfect for the Leptos SSR + hydration model -- fast edge rendering, scale to zero. But as your app grows, you might need:

- Background job processing (PDF generation, image processing)
- Services that need native Linux dependencies
- Long-running WebSocket connections beyond Worker limits
- Sidecar services (Redis, Postgres, custom daemons)

Containers let you keep the Worker as your fast edge frontend while offloading heavier work to a container, all on the same platform.

### How it works

1. Your Worker handles HTTP requests (Leptos SSR, server functions)
2. The Worker talks to a Container via Durable Object bindings
3. The Container runs your Docker image with full Linux capabilities
4. Containers scale to zero when idle -- you only pay for active time

### Configuration

Add to your `wrangler.toml`:

```toml
[[containers]]
class_name = "MyContainer"
image = "./Dockerfile"
max_instances = 10

[[durable_objects.bindings]]
name = "MY_CONTAINER"
class_name = "MyContainer"

[[migrations]]
tag = "v1"
new_sqlite_classes = ["MyContainer"]
```

Note: containers use `new_sqlite_classes`, not `new_classes`.

### Instance types

| Type | vCPU | Memory | Disk |
|------|------|--------|------|
| lite | 1/16 | 256 MiB | -- |
| basic | 1/4 | 1 GiB | -- |
| standard-1 | 1/2 | 4 GiB | 8 GB |
| standard-2 | 1 | 8 GiB | 10 GB |
| standard-3 | 2 | 12 GiB | 15 GB |
| standard-4 | 4 | 12 GiB | 20 GB |

Pricing is scale-to-zero: memory, CPU (active usage only), and disk are billed per second with free tier included.

### Beta limitations

- No autoscaling or load balancing yet (manual scaling only)
- Cold starts typically 2-3 seconds
- Container images deploy gradually (not atomic with Worker code)
- Images must target `linux/amd64`
- Docker must be running locally at deploy time

---

## Project Structure

```text
.
├── .cargo/config.toml       # WASM target rustflags
├── Cargo.toml               # single-crate config with feature flags
├── wrangler.toml             # Worker + D1 + Assets config
├── migrations/
│   ├── 0001_init.sql         # todos table + index
│   ├── 0002_session_scope.sql
│   └── 0003_contact_submissions.sql
├── scripts/
│   ├── bootstrap.sh          # install missing tools
│   └── check-deps.sh         # verify toolchain
├── src/
│   ├── main.rs               # stub binary (entrypoint is in lib.rs)
│   ├── lib.rs                # Worker fetch handler + hydrate()
│   ├── app.rs                # Leptos App component, shell, router
│   ├── api.rs                # shared types + #[server] functions
│   ├── components/
│   │   ├── mod.rs
│   │   ├── contact_page.rs   # Public contact intake
│   │   └── todo_page.rs      # TodoPage, TodoBoard, TodoRow
│   └── server/
│       ├── mod.rs             # re-exports + server_error helper
│       ├── state.rs           # AppState (LeptosOptions + worker::Env)
│       ├── contact.rs         # Contact validation persistence + caps
│       └── todos.rs           # D1 query layer
├── style/
│   └── main.css              # hand-written CSS
└── assets/
    ├── favicon.svg
    ├── app-icon.svg
    ├── apple-touch-icon.png
    ├── app-icon-192.png
    ├── app-icon-512.png
    └── site.webmanifest
```

### Template identity assets

The starter ships with a small Leptos CF mark in `assets/favicon.svg` and
`assets/app-icon.svg`. The shell links `favicon.svg`, `apple-touch-icon.png`,
and `site.webmanifest`, and the visible starter page reuses `app-icon.svg` in
the hero lockup. Replace those files together when turning the template into a
product so browser tabs, installed app icons, and the in-app header stay in
sync.

### Why a single crate

- It matches the proven Cloudflare Workers Leptos deployment model
- Feature flags (`ssr`, `hydrate`) keep code paths explicit
- Server functions, shared types, and UI live together without workspace overhead
- `cargo-leptos` handles the dual compilation (server WASM + client WASM)

---

## Patterns Library

Real applications need more than the minimal starter. See the [`patterns/`](./patterns/) directory for battle-tested, well-documented solutions to common problems (dynamic entity detail, shared layouts, realtime Durable Objects, etc.). These are designed to be composed on top of the core template while keeping the starter itself lean.

## Architecture Notes

### Server function flow

```
Browser → Worker (Leptos SSR via axum) → server function → D1
                                       ↓
                              HTML response (first load)
                              or JSON response (after hydration)
```

Server functions are defined in `src/api.rs` with the `#[server]` macro. On the server, they execute inside a `SendWrapper` (required because Workers are single-threaded but Leptos server fns need `Send`). On the client, the macro generates an HTTP call to the server function endpoint.

### Feature flags

| Feature | What it enables |
|---|---|
| `ssr` | `axum`, `leptos_axum`, `worker`, server-side Leptos rendering |
| `hydrate` | `console_error_panic_hook`, client-side Leptos hydration |

These are mutually exclusive at compile time. `cargo-leptos` builds the lib with `hydrate` (for the client WASM) and the bin with `ssr` (for the Worker).

### D1 access pattern

The Worker's `Env` (which holds D1 bindings) is wrapped in `Arc` inside `AppState`, provided as axum state, and extracted in server functions via `use_context::<AppState>()`. All queries use prepared statements with `bind_refs` for parameterized SQL.

### Default D1 schema

```sql
CREATE TABLE IF NOT EXISTS todos (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  session_id TEXT NOT NULL,
  title TEXT NOT NULL,
  completed INTEGER NOT NULL DEFAULT 0 CHECK (completed IN (0, 1)),
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

The contact route adds a `contact_submissions` table keyed by the same browser-session cookie. It stores normalized form fields only; it does not store client IP addresses and it does not send email or webhooks. Add a Queue, email provider binding, Turnstile, or Cloudflare Rate Limiting rule before using it as a production intake path.

### Verification targets

The complete local release readiness check:

```bash
./scripts/verify.sh
```

For the individual pieces that the script orchestrates, see the script itself and `AGENTS.md`.

---

## Wrangler CLI Reference

Portable Wrangler commands are listed below for local development and non-governed environments. In this operator workspace, rows that change production state are explanatory only and must be routed through `cfctl`.

| Command | What it does |
|---|---|
| `bunx wrangler@4.120.1 dev --local` | Start local dev server |
| `bunx wrangler@4.120.1 deploy --config wrangler.production.toml` | Deploy to production |
| `bunx wrangler@4.120.1 deploy --dry-run --config wrangler.production.toml` | Validate the derived production config without deploying |
| `bunx wrangler@4.120.1 d1 create <name>` | Create a D1 database |
| `bunx wrangler@4.120.1 d1 migrations apply <db> --local` | Apply migrations locally |
| `bunx wrangler@4.120.1 d1 migrations apply <db> --remote --config wrangler.production.toml` | Apply migrations to production |
| `bunx wrangler@4.120.1 d1 execute <db> --local --command "SQL"` | Run ad-hoc SQL locally |
| `bunx wrangler@4.120.1 d1 execute <db> --remote --command "SQL"` | Run ad-hoc SQL in production |
| `bunx wrangler@4.120.1 secret put <KEY>` | Set a secret (interactive) |
| `bunx wrangler@4.120.1 secret list` | List all secrets |
| `bunx wrangler@4.120.1 tail` | Stream live Worker logs |

---

## License

MIT
