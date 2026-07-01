# Langual

Langual is a single-crate Rust + Leptos CSR app for exploring phonological
divergence across human languages. It is built with Trunk, ships as a static
bundle, and keeps curated language facts in source-controlled Rust data with
citations.

The project is intentionally not a server app. There is no Worker runtime, D1
database, API route, telemetry, third-party font, or build-time data fetch.

## Product Shape

- `src/data/languages.rs` stores the curated language entries and citations.
- `src/data/ipa.rs` classifies IPA symbols for display only.
- `src/routes/` owns the app views: home, explorer, compare, language detail,
  about, and not found.
- `src/components/` owns reusable UI pieces for language cards, phoneme chips,
  and signature rows.
- `Trunk.toml`, `index.html`, `assets/_headers`, `assets/_redirects`, and
  `style/main.css` define the static deployment surface.

Primary data references include PHOIBLE 2.0, WALS Online, Maddieson (1984),
Ladefoged and Maddieson (1996), and per-language references embedded in each
entry.

## Requirements

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install -f wasm-bindgen-cli --version 0.2.105
```

## Local Development

```bash
trunk serve
```

The dev server listens on `127.0.0.1:8080`.

## Verification

```bash
cargo fmt --check
cargo check
trunk build --release
```

If the host exports `NO_COLOR=1`, unset it for Trunk 0.21.x:

```bash
env -u NO_COLOR trunk build --release
```

## Data Rules

Every language entry must carry its sources. New numeric claims need a source in
the entry being changed; UI code should not invent derived authority beyond the
fields exposed by `src/data/languages.rs`.

Additions should stay hand-verifiable. This project is a curated atlas, not an
auto-ingested copy of a dataset.
