# Leptos on Cloudflare Workers

A full-stack [Leptos 0.8](https://leptos.dev/) starter deployed to Cloudflare Workers. SSR on the edge, hydration in the browser, D1 for persistence, and an agent-first setup workflow that lets your AI coding assistant provision the entire Cloudflare stack for you.

The included example is a D1-backed todo app with list, create, toggle, and delete flows implemented entirely through Leptos server functions.

## Why This Stack

Leptos compiles to WASM on both sides of the wire. The server renders HTML on Cloudflare's edge network, the client hydrates it with the same component code, and server functions give you a typed RPC boundary between the two. No JavaScript framework, no REST boilerplate, no separate API layer.

Cloudflare gives you the deployment surface: Workers for compute, D1 for SQL, Assets for static files, Tunnels for exposing local dev to the internet, and Containers for when you outgrow the Worker sandbox. All of it scales to zero and bills by usage.

**What you get out of the box:**

- Single Rust crate with feature-flagged SSR/hydration split
- Worker entrypoint using `workers-rs` + `axum`
- Leptos server functions as the full-stack boundary
- D1 access layer with prepared statements
- Client-side optimistic UI with loading and error states
- Bootstrap scripts that verify your toolchain
- A setup flow designed for AI coding agents

## Quick Start (New Project)

```bash
git clone https://github.com/user/leptos-cf.git my-app
cd my-app
./scripts/init.sh my-app        # strips the todo example, rewrites config
./scripts/bootstrap.sh           # installs Rust toolchain + cargo-leptos
bunx wrangler d1 create my-app-db
# paste the database_id into wrangler.toml
bunx wrangler d1 migrations apply my-app-db --local
cargo leptos build --release
bunx wrangler dev --local --ip 127.0.0.1 --port 57581
```

`init.sh` removes the todo domain (components, server functions, migration, styles) and leaves a clean "It works." skeleton with all the wiring intact. See [De-templating](#de-templating) for details.

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

The fastest path from clone to deploy is to hand the project to an AI coding agent (Claude Code, Codex, etc.) with a scoped Cloudflare API token. The agent can create your D1 database, patch `wrangler.toml` with real IDs, apply migrations, and deploy -- all without you touching the Cloudflare dashboard.

### What the agent needs

1. **A Cloudflare API token** with the permissions described in [Cloudflare API Tokens](#cloudflare-api-tokens)
2. **Your Cloudflare Account ID** (visible at the top of any zone's overview page, or under Workers & Pages)
3. **The tools installed** (Rust, `cargo-leptos`, Bun) -- or let the agent run `./scripts/bootstrap.sh`

### Give the agent these environment variables

```bash
export CLOUDFLARE_API_TOKEN="your-scoped-token"
export CLOUDFLARE_ACCOUNT_ID="your-account-id"
```

### Then tell it what to do

> Bootstrap this project for Cloudflare. Create the D1 database, update wrangler.toml
> with the real database ID, apply migrations locally and remotely, build, and deploy.

The agent will execute something like:

```bash
# 1. Install toolchain
./scripts/bootstrap.sh

# 2. Create the D1 database
bunx wrangler d1 create leptos-cf-db

# 3. Parse the database_id from the output and patch wrangler.toml

# 4. Apply migrations
bunx wrangler d1 migrations apply leptos-cf-db --local
bunx wrangler d1 migrations apply leptos-cf-db --remote

# 5. Build and deploy
bunx wrangler deploy
```

The placeholder `00000000-0000-0000-0000-000000000000` IDs in `wrangler.toml` are the signal -- the agent knows to replace them.

### Why this matters

Cloudflare's infrastructure is fully API-driven. Every dashboard action has a CLI or API equivalent. This means a coding agent with the right token can provision your entire stack -- databases, secrets, tunnels, DNS records -- without you context-switching to a browser. You stay in your editor, the agent does the ops.

---

## Manual Setup

### Required tools

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
cargo install -f wasm-bindgen-cli --version 0.2.114
```

This template uses `bunx wrangler`, so a global Wrangler install is not required. You do need [Bun](https://bun.sh/).

### Bootstrap scripts

```bash
./scripts/check-deps.sh    # verify all tools are present
./scripts/bootstrap.sh     # install missing tools, then verify
```

### Create your D1 database

```bash
bunx wrangler d1 create leptos-cf-db
```

Wrangler prints a `database_id`. Copy it into both `database_id` and `preview_database_id` in `wrangler.toml`.

### Apply the initial migration

```bash
# Local (for wrangler dev)
bunx wrangler d1 migrations apply leptos-cf-db --local

# Remote (for production)
bunx wrangler d1 migrations apply leptos-cf-db --remote
```

### Build and run locally

```bash
cargo leptos build --release
bunx wrangler dev --local --ip 127.0.0.1 --port 57581
```

### Deploy

```bash
bunx wrangler deploy
```

---

## Cloudflare API Tokens

Scoped tokens are the key to safe agent-assisted workflows. You create a token with exactly the permissions needed, set an expiration, and hand it to the agent. When the token expires, the agent loses access automatically.

### Creating a scoped token

Go to **My Profile > API Tokens > Create Token** in the Cloudflare dashboard, or use the API.

**Minimum permissions for this project:**

| Permission | Access | Why |
|---|---|---|
| Account Settings | Read | Wrangler needs account introspection |
| Workers Scripts | Edit | Deploy the Worker |
| D1 | Edit | Create databases, apply migrations |

**Additional permissions if you're using more of the platform:**

| Permission | Access | Why |
|---|---|---|
| Workers R2 Storage | Edit | Create/manage R2 buckets |
| Cloudflare Tunnel | Edit | Create and configure tunnels |
| Workers KV Storage | Edit | Create/manage KV namespaces |

### Token restrictions

Always apply these:

- **Expiration**: Set `expires_on` to 30-90 days. Tokens do not expire by default.
- **Account scope**: Restrict to a single account, not "All accounts."
- **IP filtering** (optional): Lock to your office/home IP or CI runner CIDR range.

### Rotation strategy

Cloudflare does not auto-rotate tokens, but the workflow is straightforward:

1. **Account-owned tokens** (recommended for CI/CD and agents) support **secret rolling** via the API -- the token value regenerates while permissions stay intact.
2. Create tokens with overlapping expiration windows: issue a new token before the old one expires, update your secrets, then let the old one die.
3. For CI/CD, store `CLOUDFLARE_API_TOKEN` as a repository secret and rotate on a schedule.
4. For agents, set short-lived tokens (7-30 days) and re-issue as needed. The agent only needs the token during active development sessions.

**Automated rotation via the API:**

```bash
# Roll an account-owned token's secret (preserves permissions, generates new value)
curl -X PUT "https://api.cloudflare.com/client/v4/accounts/{account_id}/tokens/{token_id}/value" \
  -H "Authorization: Bearer {current_token}"
```

### Account-owned vs user-owned tokens

- **User-owned** (`My Profile > API Tokens`): tied to your user account. If you leave the org, the token dies.
- **Account-owned** (`Manage Account > Account API Tokens`): tied to the account, survives employee changes. **Use this for CI/CD and persistent agent access.**

### Environment variables

Wrangler reads these automatically:

```bash
export CLOUDFLARE_API_TOKEN="..."   # the scoped token
export CLOUDFLARE_ACCOUNT_ID="..."  # your account ID
```

Set these before any `wrangler` command for non-interactive (agent) usage.

---

## Local Development

```bash
cargo leptos build --release
bunx wrangler dev --local --ip 127.0.0.1 --port 57581
```

Wrangler serves the Worker and the asset bundle from `target/site`. The todo UI talks to D1 only through Leptos server functions.

**Local secrets**: Create a `.dev.vars` file (gitignored) for local environment variables:

```
SECRET_KEY=dev-value-here
```

**Iterating**: Run `cargo leptos watch` in a second terminal for automatic client rebuilds. The Worker itself needs a `wrangler dev` restart to pick up server-side changes.

---

## Deployment

Once `wrangler.toml` has a real D1 database ID and the remote migration has been applied:

```bash
bunx wrangler deploy
```

Wrangler runs the configured build command:

1. `cargo leptos build --release` -- compiles the client WASM + CSS
2. `worker-build --release --features ssr` -- compiles the Worker bundle

That produces:

- Client assets in `target/site/`
- Worker bundle in `build/index.js`

### Setting secrets

```bash
# Interactive (one at a time)
bunx wrangler secret put SECRET_KEY

# Bulk (from JSON, good for agent use)
echo '{"SECRET_KEY":"value","OTHER_KEY":"value2"}' | bunx wrangler secret bulk
```

### Dry run

```bash
bunx wrangler deploy --dry-run
```

---

## De-templating

The included todo app is meant to be replaced. `scripts/init.sh` automates this:

```bash
./scripts/init.sh my-app
```

This does:

- Rewrites `Cargo.toml`, `wrangler.toml`, and `src/app.rs` with your project name
- Replaces the todo components, server functions, and D1 queries with a minimal "It works." scaffold
- Strips todo-specific CSS, keeps the design system foundation (variables, glass panels, typography, responsive breakpoints)
- Removes the todo migration (you'll create your own)
- Leaves the wiring intact: `AppState`, `server_error` helper, shell, router, feature flags

The script refuses to run if `wrangler.toml` already has real D1 IDs (indicating the project was already initialized).

After running `init.sh`, your project compiles and renders a working page — the SSR, hydration, and server function plumbing is all in place. Add your own domain by following [docs/building-features.md](docs/building-features.md).

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
│   └── 0001_init.sql         # todos table + index
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
│   │   └── todo_page.rs      # TodoPage, TodoBoard, TodoRow
│   └── server/
│       ├── mod.rs             # re-exports + server_error helper
│       ├── state.rs           # AppState (LeptosOptions + worker::Env)
│       └── todos.rs           # D1 query layer
├── style/
│   └── main.css              # hand-written CSS
└── assets/
    └── favicon.svg
```

### Why a single crate

- It matches the proven Cloudflare Workers Leptos deployment model
- Feature flags (`ssr`, `hydrate`) keep code paths explicit
- Server functions, shared types, and UI live together without workspace overhead
- `cargo-leptos` handles the dual compilation (server WASM + client WASM)

---

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

### Default todo schema

```sql
CREATE TABLE IF NOT EXISTS todos (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  completed INTEGER NOT NULL DEFAULT 0 CHECK (completed IN (0, 1)),
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

### Verification targets

```bash
./scripts/check-deps.sh                                    # toolchain
cargo check --features ssr                                 # server compilation
cargo leptos build --release                               # full client + server build
bunx wrangler deploy --dry-run                             # deployment structure
bunx wrangler dev --local --ip 127.0.0.1 --port 57581     # local smoke test
```

---

## Wrangler CLI Reference

Commands you'll use most with this project:

| Command | What it does |
|---|---|
| `bunx wrangler dev --local` | Start local dev server |
| `bunx wrangler deploy` | Deploy to production |
| `bunx wrangler deploy --dry-run` | Validate without deploying |
| `bunx wrangler d1 create <name>` | Create a D1 database |
| `bunx wrangler d1 migrations apply <db> --local` | Apply migrations locally |
| `bunx wrangler d1 migrations apply <db> --remote` | Apply migrations to production |
| `bunx wrangler d1 execute <db> --local --command "SQL"` | Run ad-hoc SQL locally |
| `bunx wrangler d1 execute <db> --remote --command "SQL"` | Run ad-hoc SQL in production |
| `bunx wrangler secret put <KEY>` | Set a secret (interactive) |
| `bunx wrangler secret list` | List all secrets |
| `bunx wrangler tail` | Stream live Worker logs |

---

## License

MIT
