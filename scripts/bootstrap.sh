#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
source "$ROOT_DIR/scripts/toolchain.sh"

log() {
  printf '[bootstrap] %s\n' "$1"
}

require_command() {
  local cmd="$1"
  local hint="$2"

  if ! command -v "$cmd" >/dev/null 2>&1; then
    printf '[bootstrap] %s\n' "$hint" >&2
    exit 1
  fi
}

require_command rustup "Rustup is required. Install it from https://rustup.rs/."
require_command cargo "Cargo is required. Install Rust from https://rustup.rs/."
require_command bun "Bun is required. Install it from https://bun.sh/."
require_command node "Node.js 22 or later is required by Wrangler. Install it from https://nodejs.org/."
EXPECTED_WRANGLER_VERSION="$(expected_wrangler_version)"

wrangler_cmd() {
  "$ROOT_DIR/scripts/wrangler.sh" "$@"
}

if rustup toolchain list | grep -q '^stable'; then
  log "Stable Rust toolchain already installed."
else
  log "Installing the stable Rust toolchain."
  rustup toolchain install stable
fi

if rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  log "wasm32-unknown-unknown target already installed."
else
  log "Installing the wasm32-unknown-unknown target."
  rustup target add wasm32-unknown-unknown
fi

if cargo leptos --version >/dev/null 2>&1; then
  current_cargo_leptos_version="$(cargo leptos --version | awk '{print $2}')"
else
  current_cargo_leptos_version=""
fi

if [ "$current_cargo_leptos_version" = "$EXPECTED_CARGO_LEPTOS_VERSION" ]; then
  log "cargo-leptos $EXPECTED_CARGO_LEPTOS_VERSION already installed."
else
  log "Installing cargo-leptos $EXPECTED_CARGO_LEPTOS_VERSION."
  cargo install cargo-leptos --locked --version "$EXPECTED_CARGO_LEPTOS_VERSION"
fi

repo_wasm_bindgen_version="$("$ROOT_DIR/scripts/with-wasm-bindgen-cli.sh" --version | awk '{print $2}')"
log "Ensured repo-local wasm-bindgen-cli $repo_wasm_bindgen_version from Cargo.lock."

if command -v worker-build >/dev/null 2>&1; then
  current_worker_build_version="$(worker-build --version | awk '{print $1}')"
else
  current_worker_build_version=""
fi

if [ "$current_worker_build_version" = "$EXPECTED_WORKER_BUILD_VERSION" ]; then
  log "worker-build $EXPECTED_WORKER_BUILD_VERSION already installed."
else
  log "Installing worker-build $EXPECTED_WORKER_BUILD_VERSION."
  cargo install worker-build --locked --version "$EXPECTED_WORKER_BUILD_VERSION"
fi

if ! command -v cargo-audit >/dev/null 2>&1; then
  log "cargo-audit is required for release verification. Install it with cargo install cargo-audit --locked, then rerun bootstrap."
  exit 1
fi

bun install --frozen-lockfile

log "Checking Wrangler $EXPECTED_WRANGLER_VERSION through the local launcher."
wrangler_cmd --version >/dev/null

log "Running dependency checks."
"$ROOT_DIR/scripts/check-deps.sh"

LEPTOS_LOCAL_D1_NAME="$(bun -e 'console.log(Bun.TOML.parse(await Bun.file("wrangler.toml").text()).d1_databases[0].database_name)')"
cat <<EOF

Bootstrap complete.

Local next steps:
1. CI=1 ./scripts/wrangler.sh d1 migrations apply $LEPTOS_LOCAL_D1_NAME --local
2. bash ./scripts/build-edge.sh
3. ./scripts/wrangler.sh dev --local --ip 127.0.0.1 --port 57581

Provider operations are separate from local setup:
- Choose standalone Wrangler or an owner-selected cfctl lane: docs/credentials.md.
- Workers deployment: docs/edge-deployment.md.
- Pages preparation and readback: docs/pages-production.md.
- Optional governed capability discovery: docs/cfctl.md.
Keep tracked IDs as placeholders; use verified identity for ignored production config.
A blocked governed capability is a hold, not permission to bypass it.
EOF
