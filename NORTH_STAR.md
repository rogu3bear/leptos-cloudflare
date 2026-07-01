# Langual North Star

## Intent

Langual exists to make the vocal capability divergence across human languages
legible at a glance. It is a small, citation-backed atlas: a reading-room sized
set of languages chosen to span the extremes of the world's sound systems, where
every number on screen traces back to a published phonological reference. The
goal is not coverage — it is trustworthy contrast.

## Core Promise

- Every count, feature, and signature phoneme is sourced from a primary
  reference (PHOIBLE 2.0, Maddieson 1984, Ladefoged & Maddieson 1996, WALS, or a
  named per-language citation). Nothing is invented.
- The corpus stays deliberately small so each entry can carry its own citation
  and commentary rather than dissolving into a database dump.
- The site is a static, client-rendered explorer: fast, offline-capable once
  loaded, no runtime database, no telemetry, no third-party fonts, no network
  dependency for the core experience.
- When sources disagree on an inventory analysis, Langual gives one published
  count, links to PHOIBLE for canonical inventory data, and says analyses vary
  rather than implying false precision.

## Product Shape Today

A single-crate Rust + Leptos (CSR) WebAssembly SPA built with Trunk and shipped
as a static bundle (Cloudflare Pages, with `assets/_redirects` SPA fallback).

Surfaces, all under `src/routes/`:

- `Home` (`/`) — framing and entry into the atlas.
- `Explorer` (`/explore`) — the twenty languages with search (name/family/ISO),
  macroarea filter, phonological-feature filter, and sort by segment count.
- `LanguageDetail` (`/language/:id`) — a single language's counts, signature
  phonemes, features, and sources.
- `Compare` (`/compare?a=&b=`) — any two languages side by side.
- `About` (`/about`) — method, sources, and stated limits.

The data lives in `src/data/languages.rs` as a static, hand-curated `&[Language]`
(currently twenty entries), with presentational IPA classification in
`src/data/ipa.rs`. The data file is the product; the UI renders it.

## What "Good" Looks Like

- A reader opens `/explore`, sorts by segment count, and immediately sees the
  full range from the smallest inventory to the largest.
- They open a language page and find a real citation behind every number, with a
  link out to PHOIBLE for the canonical inventory.
- They compare two languages and the *shape* of the difference — segment count,
  tone, signature sounds — is the visible point.
- Adding or correcting a language means editing one `Language` struct with its
  sources attached; the UI needs no change.

## Scope Boundaries

- Curated data and its citations live in `src/data/`. Presentation logic lives in
  `src/routes/` and `src/components/`. Keep that split.
- `ipa.rs::classify` is *presentational* (layout/coloring cues), not a
  phonological analysis. It must not become a source of inventory truth.
- This is a static client app. There is no server, no API, no database, no
  build-time data fetch.

## Decision Filter

Prefer changes that increase trust and clarity:

- a real citation behind every claim
- contrast that is easy to read at the extremes
- a corpus small enough to stay hand-verifiable
- a build that stays a single static bundle

Avoid work that trades sourced truth for breadth, invents inventory data, or
adds runtime dependencies the static-bundle promise rules out.

## Anti-Goals

- Becoming a "complete" phoneme database or auto-ingesting PHOIBLE wholesale.
- Reproducing a full phoneme chart per language and implying it is canonical.
- Treating `ipa.rs` heuristics as phonological authority.
- Adding a backend, runtime DB, analytics, trackers, or third-party fonts.
- Growing the corpus past what can be hand-checked against primary sources.
