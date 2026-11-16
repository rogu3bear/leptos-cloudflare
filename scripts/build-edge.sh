#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/toolchain.sh"

cd "$ROOT_DIR"

if ! cargo leptos --version >/dev/null 2>&1; then
  printf '[build-edge] cargo-leptos %s is required. Run ./scripts/bootstrap.sh first.\n' "$EXPECTED_CARGO_LEPTOS_VERSION" >&2
  exit 1
fi

if [ "$(cargo leptos --version | awk '{print $2}')" != "$EXPECTED_CARGO_LEPTOS_VERSION" ]; then
  printf '[build-edge] cargo-leptos %s is required. Run ./scripts/bootstrap.sh first.\n' "$EXPECTED_CARGO_LEPTOS_VERSION" >&2
  exit 1
fi

if ! command -v worker-build >/dev/null 2>&1; then
  printf '[build-edge] worker-build %s is required. Run ./scripts/bootstrap.sh first.\n' "$EXPECTED_WORKER_BUILD_VERSION" >&2
  exit 1
fi

if [ "$(worker-build --version | awk '{print $1}')" != "$EXPECTED_WORKER_BUILD_VERSION" ]; then
  printf '[build-edge] worker-build %s is required. Run ./scripts/bootstrap.sh first.\n' "$EXPECTED_WORKER_BUILD_VERSION" >&2
  exit 1
fi

# Settle Cargo's lockfile ordering after local name adoption before freezing inputs.
cargo metadata --format-version 1 > /dev/null
bun ./scripts/build-provenance.mjs start
./scripts/with-wasm-bindgen-cli.sh cargo leptos build --release
bun ./scripts/hash-assets.mjs
source "$ROOT_DIR/target/asset-hashes.env"
./scripts/with-wasm-bindgen-cli.sh worker-build --release --features ssr
bun ./scripts/write-worker-shim.mjs
bun ./scripts/verify-hashed-assets.mjs
bun ./scripts/verify-worker-runtime.mjs
bun ./scripts/build-provenance.mjs finish
