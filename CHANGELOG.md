# Changelog

## Unreleased

- Added a generated Cloudflare `_worker.js` router that separates Workers Assets, WebSocket upgrades, and Leptos SSR fallback.
- Pinned `compatibility_date` to `2026-04-22`, the newest date supported by the local Wrangler runtime used for template smoke tests.
- Added Leptos CF logo, favicon, app icons, and web manifest assets.
- Added repo-local `wasm-bindgen-cli` resolution from `Cargo.lock`.
- Added release, security, contributing, realtime, and agent operating docs.
- Updated CI to exercise the real template release gates.
- Updated compatible Rust dependencies in `Cargo.lock`, including Leptos 0.8.19 and wasm-bindgen 0.2.121.
