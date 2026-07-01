# Langual Anchor

## Purpose

This file captures the truths that should stay stable while the code evolves.
If a proposal conflicts with these anchors, the burden is on the proposal.

## Product Anchors

- Langual is a curated, citation-backed phoneme atlas, not a complete linguistic
  database and not a general data-viz framework.
- The corpus is intentionally small (currently twenty languages) so each entry
  carries its own primary-source citation and commentary.
- Every visible number — consonant count, vowel count, tone count, segment total
  — comes from a published reference and stays attributable to it.
- Where analyses disagree, give one cited count and link to PHOIBLE; never invent
  or average an inventory into false precision.

## Architectural Anchors

- Single Rust crate, Leptos 0.8 in **CSR** mode (`features = ["csr"]`), compiled
  to `wasm32-unknown-unknown` and bundled by **Trunk** (`Trunk.toml`,
  `index.html`). There is no SSR, no Worker, no `cargo-leptos`.
- Entry point is `src/main.rs` → `leptos::mount::mount_to_body(app::App)`.
  Routing lives in `src/app.rs` via `leptos_router` (`Router`/`Routes`).
- Curated data lives in `src/data/languages.rs` as a static `&[Language]`;
  lookups go through `find_language` / `all_languages`. The data file is the
  source of truth the UI renders.
- `src/data/ipa.rs::classify` is presentational only — articulatory-class cues
  for rendering, explicitly not a phonological inventory analysis.
- Routes (`src/routes/`) are page components; reusable view pieces
  (`LanguageCard`, `PhonemeChip`, `SignatureRow`) live in `src/components/`.
- Deployment is a static bundle. `assets/_redirects` provides the SPA fallback
  (`/* /index.html 200`) and `assets/_headers` sets caching and security headers.

## Safety Anchors

- Never add invented phonological data. A new or changed count must arrive with
  its citation in the same `Language` struct.
- Never promote `ipa.rs` heuristics into a claim of inventory truth.
- Never add telemetry, trackers, surprise network calls, third-party fonts, or a
  runtime database — the About page states none exist.
- Never break the single-static-bundle deployment model by introducing a server,
  API layer, or build-time data fetch.

## Operational Anchors

- Local dev: `trunk serve` (configured to 127.0.0.1:8080). Production build:
  `trunk build --release`, output in `dist/`.
- The `.cargo/config.toml` pins the default target to `wasm32-unknown-unknown`;
  plain `cargo check` runs against the WASM target.
- The tracked git history still reflects an older Leptos-on-Cloudflare-Workers
  *template*; the live working tree is the Langual atlas. Trust the working
  tree, not the stale README/history, for current truth.

## Decision Questions

Before changing code, ask:

1. Does every new or changed number carry a primary-source citation?
2. Does this keep the data/presentation split (`src/data/` vs
   `src/routes/`+`src/components/`) intact?
3. Does it preserve the CSR + Trunk + static-bundle model with no backend?
4. Does it keep the corpus hand-verifiable rather than auto-ingested?
5. Does it avoid telemetry, runtime DB, network calls, and third-party fonts?

If the answer to any of those is "no", the change probably needs to be smaller
or differently shaped.
