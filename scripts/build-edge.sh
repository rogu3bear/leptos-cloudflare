#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"

cargo leptos build --release
bun ./scripts/hash-assets.mjs
source "$ROOT_DIR/target/asset-hashes.env"
if ! command -v worker-build >/dev/null 2>&1; then
  cargo install -q "worker-build@^0.7"
fi
worker-build --release --features ssr
bun ./scripts/verify-hashed-assets.mjs
