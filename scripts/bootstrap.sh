#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXPECTED_WASM_BINDGEN_VERSION="0.2.114"

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
  log "cargo-leptos already installed."
else
  log "Installing cargo-leptos."
  cargo install cargo-leptos --locked
fi

if command -v wasm-bindgen >/dev/null 2>&1; then
  current_wasm_bindgen_version="$(wasm-bindgen --version | awk '{print $2}')"
else
  current_wasm_bindgen_version=""
fi

if [ "$current_wasm_bindgen_version" = "$EXPECTED_WASM_BINDGEN_VERSION" ]; then
  log "wasm-bindgen-cli $EXPECTED_WASM_BINDGEN_VERSION already installed."
else
  log "Installing wasm-bindgen-cli $EXPECTED_WASM_BINDGEN_VERSION."
  cargo install -f wasm-bindgen-cli --version "$EXPECTED_WASM_BINDGEN_VERSION"
fi

log "Checking Wrangler through bunx."
bunx wrangler --version >/dev/null

log "Running dependency checks."
"$ROOT_DIR/scripts/check-deps.sh"

cat <<'EOF'

Bootstrap complete.

Next steps:
1. bunx wrangler d1 create leptos-cf-db
2. Replace the placeholder database IDs in wrangler.toml
3. bunx wrangler d1 migrations apply leptos-cf-db --local
4. cargo leptos build --release
5. bunx wrangler dev --local --ip 127.0.0.1 --port 57581
EOF
