# Langual Agent Contract

Langual is a single-crate Rust + Leptos (CSR) WebAssembly SPA — a curated,
citation-backed atlas of phonological divergence across human languages. It is
built with Trunk and shipped as a static bundle. This file is repo-local and
inherits the workspace contract at `/Users/star/dev/AGENTS.md`; it does not
restate it.

Read `ANCHOR.md` (product/architecture boundaries) and `NORTH_STAR.md`
(strategy) before any non-trivial change.

## First Moves

- Report target repo (`langual`), live branch, and dirty-tree state before edits.
- Trust the current static-app files for product truth. Read `Cargo.toml`,
  `Trunk.toml`, `index.html`, `src/main.rs`, and `src/app.rs` before making
  architecture assumptions.
- Before adding or editing language data, open `src/data/languages.rs` and follow
  the existing citation pattern. No number ships without a source.

## Hard Boundaries

- Curated data + citations live in `src/data/` (`languages.rs`, `ipa.rs`).
  Page/view logic lives in `src/routes/` and `src/components/`. Do not smear data
  into views or views into data.
- `ipa.rs::classify` is presentational only. Do not turn it into an inventory
  authority.
- CSR + Trunk + static bundle. No server, no API, no runtime database, no
  build-time fetch, no telemetry, no third-party fonts.
- Leptos 0.8 with `features = ["csr"]`; default build target is
  `wasm32-unknown-unknown` (see `.cargo/config.toml`).

## Canonical Commands

```bash
trunk serve                 # local dev at 127.0.0.1:8080 (see Trunk.toml)
trunk build                 # debug static build into dist/
trunk build --release       # production static bundle into dist/
cargo check                 # type-check against the wasm32 target
cargo fmt                   # format
```

Tooling, if missing: `rustup target add wasm32-unknown-unknown`,
`cargo install trunk`, `cargo install -f wasm-bindgen-cli --version 0.2.105`
(pinned in `Cargo.toml`).

## Scope Discipline

- Adding a language = one `Language` struct in `src/data/languages.rs` with its
  `sources` populated; the UI should not need changes.
- Correcting a number = change the value and confirm its citation; if the source
  changed, update `sources`.
- Do not expand the corpus past what stays hand-verifiable against primary
  references, and do not auto-ingest external datasets.
- Keep `index.html`, `Trunk.toml`, `assets/_headers`, and `assets/_redirects`
  aligned with the static-bundle deployment model.

## Output Contract (required for every agent)

- Summary: what changed and why.
- Files touched: every edited path, no hidden edits.
- Patch / Contents: the concrete diff or content.
- Verification: exact commands run (e.g. `trunk build --release`, `cargo check`)
  and their results, plus anything still pending.
- For any data change: name the primary source that backs the new/changed value.
