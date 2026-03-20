# Leptos on Cloudflare Workers

This repository is a public starter for a full-stack [Leptos](https://leptos.dev/) app deployed to Cloudflare Workers with:

- a single Rust crate
- SSR on Workers through `workers-rs` + `axum`
- client hydration built by `cargo-leptos`
- static assets served from Workers Assets
- Leptos server functions as the default full-stack boundary
- Cloudflare D1 as the default database

The included example is a D1-backed todo app with list, create, toggle, and delete flows implemented entirely through Leptos server functions.

## Required Tools

Install these before running the starter:

```bash
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --locked
bun install -g wrangler
```

If you prefer not to install Wrangler globally, run it through `bunx wrangler`.

## Project Structure

```text
.
├── .cargo/config.toml
├── Cargo.toml
├── README.md
├── assets/
├── migrations/
├── src/
│   ├── api.rs
│   ├── app.rs
│   ├── components/
│   ├── lib.rs
│   ├── main.rs
│   └── server/
├── style/
└── wrangler.toml
```

Why a single crate:

- it matches the proven Cloudflare Workers Leptos deployment model
- feature flags keep hydration and SSR code paths explicit
- server functions, shared DTOs, and UI can live together without workspace overhead

## First-Time Setup

### 1. Create your D1 database

```bash
bunx wrangler d1 create leptos-cf-db
```

Wrangler will print a `database_id`. Copy that value into both `database_id` and `preview_database_id` in `wrangler.toml`.

### 2. Apply the initial migration locally

```bash
bunx wrangler d1 migrations apply leptos-cf-db --local
```

To apply the same migration to the remote database later:

```bash
bunx wrangler d1 migrations apply leptos-cf-db --remote
```

## Local Development

Build the client bundle and start the Worker locally:

```bash
cargo leptos build --release
bunx wrangler dev --local --ip 127.0.0.1 --port 57581
```

During development, Wrangler serves the Worker and the asset bundle from `target/site`. The todo UI talks to D1 only through Leptos server functions mounted under `/api`.

If you want to rebuild more often while iterating, `cargo leptos watch` is useful in a second terminal, but the template itself does not depend on any custom local tooling.

## Deployment

Once `wrangler.toml` has a real D1 database ID and the remote migration has been applied:

```bash
bunx wrangler deploy
```

Wrangler will run the configured build command:

1. `cargo leptos build --release`
2. `worker-build --release --features ssr`

That produces:

- the client assets in `target/site`
- the Worker bundle in `build/index.js`

## What the Starter Includes

- Worker entrypoint using `#[worker::event(fetch)]`
- Axum router setup with `leptos_axum`
- server-function powered todo CRUD flow
- a small D1 access layer with prepared statements
- intentional default styling with loading, empty, and error states
- a migration that creates the `todos` table

## Default Todo Schema

The initial migration creates this table:

- `id INTEGER PRIMARY KEY AUTOINCREMENT`
- `title TEXT NOT NULL`
- `completed INTEGER NOT NULL DEFAULT 0`
- `created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP`

## Verification Targets

The starter is intended to satisfy these commands:

```bash
cargo check --features ssr
cargo leptos build --release
bunx wrangler deploy --dry-run
```

If `deploy --dry-run` is not available in your environment, `bunx wrangler dev --local --ip 127.0.0.1 --port 57581` is the fallback structural check.
