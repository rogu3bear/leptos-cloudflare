# Changelog

## Unreleased

- Make the workers-rs `fetch` handler the first request handler on Workers and
  Pages. Routing for the WebSocket lane, asset fallback, Pages private modules,
  retained server-function URLs and the trace span moved from
  `runtime/dispatch.mjs` into `src/edge/`. The generated `build/_worker.js` now
  only re-exports the workers-rs module and publishes build-generated routing data.
- Replace the JavaScript `handleRealtimeSocket` override with one Rust extension
  point, `upgrade` in `src/edge/realtime.rs`, and a Rust Durable Object example.
- Prove routing with Rust unit tests and local-runtime checks of the built
  handler instead of matching generated JavaScript source.

## 0.1.9

- Require the outgoing hydration bundle before qualifying upgrades and rollbacks.
- Exercise a persisted application server-function mutation on Workers and Pages after adoption.
- Preserve Cargo lockfile ordering and update its reviewed inventory digest during a rename.

## 0.1.8

- Preserve verified, content-addressed client bundles across upgrades and rebuilt
  rollback releases for Workers and Pages.
- Preserve the six reference server-function URLs used by retained clients when
  a rebuild in a different directory changes Leptos’s generated route suffix.
- Run tracked WebKit hydration and server-function checks in the local release
  gate, including the mutation response behind the optimistic todo control.

## 0.1.7

- Update `spin` to 0.9.9 to clear the registry yank warning.
- Update Leptos to 0.8.23 and the router macro to 0.8.7, removing
  `proc-macro-error2` from the dependency graph. The two remaining upstream
  maintenance warnings are documented in [RELEASE.md](RELEASE.md).

## 0.1.6

- Validate todo titles before accessing D1 so a storage failure cannot mask an input error.
- Check that malformed client network identity returns the contact identity error and leaves submissions empty on Workers and Pages.

## 0.1.5

- Enforce the 4 KiB server-function limit on streamed bytes, including chunked
  requests, before deserialization.
- Atomically enforce contact hourly/daily network budgets and todo capacity.
- Preserve anonymous intake with keyed per-IP digests; cookie rotation no longer
  resets the network budget, and NAT users share limits.
- Reject writes at storage capacity without evicting another session’s records.
- Add the append-only admission migration and native boundary/concurrency/privacy
  regressions across Workers, Pages and the derived Pages release directory.
- Apply migration 0004 before running this version. Preserve its database-local
  key during backup and recovery.
- Source patch only; no live Cloudflare migration or deployment.

## 0.1.4

- Ship public agent instructions, concise onboarding and distinct credential-free,
  standalone Wrangler and optional governed cfctl paths.
- Exclude internal planning records and personal machine references from the
  current source; preserve license attribution and canonical repository links.
- Qualify Pages production preparation with isolated configuration, separate
  production/preview D1 identities, exact build provenance and readback contracts.
- Normalize private paths and keep server modules out of Pages static uploads.
- Preserve Workers oversized-body rejection and genuine resource descriptors
  through nested builds and adopter qualification.
- Update Tokio and xxhash-rust to patched versions selected in Cargo.lock.
  The xxhash-rust update to 0.8.16 addresses
  [GHSA-6g2r-675j-hx59](https://github.com/advisories/GHSA-6g2r-675j-hx59).
- Distribute GitHub template source only; provider deployment and live acceptance
  remain separate authorized operations.

## 0.1.3

- Present the public reference as an SSR-first field guide with Start,
  Architecture, Patterns, and bounded D1 labs. Keep stored Contact intake
  distinct from delivery.
- Preserve a provider-neutral tracked Wrangler template and derive ignored
  production configuration from verified Worker/D1 identity. Bind governed
  migrations to the repository operation pack and recovery/schema contract.
- Preserve unowned temporary files when production configuration derivation
  encounters an exclusive-write collision.
- Replace destructive initialization with local identity adoption that preserves
  application code and migrations and refuses invalid or provider-bound inputs.
- Separate reference-site wording checks from reusable runtime verification.
  Added application routes now receive their router's status; the not-found
  component owns 404 recovery without a second Worker route allowlist.
- Fingerprint the final JavaScript bytes after rewriting the WASM reference,
  so WASM-only changes invalidate immutable JavaScript caches. Verify all
  manifest hashes against served bytes and align preload credentials with fetch.
- Recognize known generated numeric server-function suffixes in closed telemetry
  labels without logging raw paths or suffixes.
- Require the security audit for complete release verification; missing or failing
  tools cannot produce a successful release-readiness summary.
- Keep task detail visible after failed toggle/delete commands, announce errors,
  and navigate back only after a successful deletion.
- Remove the parent-directory token-rotation dependency and distinguish governed
  cfctl credentials from independently operated standalone Wrangler usage.
- Add executable adoption/failure checks and a disposable renamed application
  build with a replaced page and added route.

### Earlier runtime work carried by this release

- Added a generated Cloudflare `_worker.js` router that separates Workers Assets, WebSocket upgrades, and Leptos SSR fallback.
- Pinned `compatibility_date` to `2026-08-10` with Wrangler `4.120.1` for the checked-in runtime contract.
- Added Leptos CF logo, favicon, app icons, and web manifest assets.
- Added repo-local `wasm-bindgen-cli` resolution from `Cargo.lock`.
- Added release, security, contributing, realtime, and agent operating docs.
- Updated release verification to exercise the real template gates.
- Updated compatible Rust dependencies in `Cargo.lock`, including Leptos 0.8.19 and wasm-bindgen 0.2.121.
- Updated `anyhow` to 1.0.103 for RUSTSEC-2026-0190 on the Leptos hot-reload path
  and `event-listener` to 5.4.2 for RUSTSEC-2026-0221 on the reactive graph path.
