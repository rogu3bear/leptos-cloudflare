#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXPECTED_CARGO_LEPTOS_VERSION="0.3.5"
EXPECTED_WORKER_BUILD_VERSION="0.7.5"

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

cargo leptos build --release
bun ./scripts/hash-assets.mjs
source "$ROOT_DIR/target/asset-hashes.env"
worker-build --release --features ssr
bun ./scripts/verify-hashed-assets.mjs
