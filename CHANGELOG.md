# Changelog

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
