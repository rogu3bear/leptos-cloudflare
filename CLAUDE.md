# CLAUDE.md

Langual is a single-crate Rust + Leptos (CSR) WebAssembly SPA — a curated,
citation-backed atlas of the vocal capability divergence across human languages.
It renders a deliberately small corpus (currently twenty languages in
`src/data/languages.rs`) where every count and feature traces to a published
phonological reference (PHOIBLE 2.0, Maddieson 1984, Ladefoged & Maddieson 1996,
WALS). It is built with Trunk and shipped as a static bundle (Cloudflare Pages,
SPA fallback via `assets/_redirects`). No server, no runtime database, no
telemetry, no third-party fonts.

Read `ANCHOR.md` (product/architecture boundaries) and `NORTH_STAR.md`
(strategy) first when a task could broaden scope.

> Truth note: the tracked git history and `README.md` still describe a former
> Leptos-on-Cloudflare-Workers SSR/D1 *template*. That is stale. The live working
> tree is this CSR Trunk atlas. Trust the working tree (`Cargo.toml`,
> `Trunk.toml`, `index.html`, `src/`), not the README, for current truth.

## Core Commands

```bash
# Local dev server (127.0.0.1:8080, see Trunk.toml)
trunk serve

# Static builds
trunk build              # debug, output in dist/
trunk build --release    # production bundle, output in dist/

# Checks
cargo check              # type-check against wasm32-unknown-unknown
cargo fmt

# One-time toolchain
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install -f wasm-bindgen-cli --version 0.2.105
```

`.cargo/config.toml` pins the default build target to
`wasm32-unknown-unknown`, so plain `cargo` commands run against WASM.

## Architecture

- `src/main.rs` — entry point; mounts `app::App` via
  `leptos::mount::mount_to_body`.
- `src/app.rs` — `App` + `Shell`: meta context, header/nav, footer, and the
  `leptos_router` route table (`/`, `/explore`, `/language/:id`, `/compare`,
  `/about`).
- `src/routes/` — page components: `home`, `explorer`, `language`, `compare`,
  `about`, `not_found`. `explorer.rs` holds search/filter/sort logic;
  `compare.rs` reads `?a=&b=` query params.
- `src/data/languages.rs` — static `&[Language]` corpus with citations; access
  via `all_languages` / `find_language`. This file is the source of truth.
- `src/data/ipa.rs` — `classify()` returns a presentational `PhonemeKind` for
  rendering only; it is not a phonological inventory analysis.
- `src/components/` — reusable views: `LanguageCard`, `PhonemeChip`,
  `SignatureRow`.
- `index.html` / `Trunk.toml` — Trunk build entry and config. `style/main.css` —
  hand-written CSS. `assets/_headers`, `assets/_redirects` — static-host caching,
  security headers, and SPA fallback.

## Guardrails

- Keep the data/presentation split: data + citations in `src/data/`, views in
  `src/routes/` and `src/components/`.
- No number without a source. Adding or correcting a value means editing its
  `Language` struct and keeping `sources` accurate.
- Do not promote `ipa.rs` heuristics into inventory truth.
- Preserve the CSR + Trunk + static-bundle model: no backend, API, runtime DB,
  build-time fetch, telemetry, or third-party fonts.
- Keep the corpus hand-verifiable; do not auto-ingest external datasets.

## Output Contract (required for every agent)

- Summary
- Files touched (no hidden edits)
- Patch / Contents
- Verification: commands run (`trunk build --release`, `cargo check`) and results
- For data changes: name the primary source backing each new/changed value
