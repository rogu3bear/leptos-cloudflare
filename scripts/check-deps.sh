#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXPECTED_WASM_BINDGEN_VERSION="0.2.114"
missing=0

pass() {
  printf '[ok] %s\n' "$1"
}

warn() {
  printf '[warn] %s\n' "$1"
}

fail() {
  printf '[missing] %s\n' "$1" >&2
  missing=1
}

check_command() {
  local cmd="$1"
  local install_hint="$2"

  if command -v "$cmd" >/dev/null 2>&1; then
    pass "$cmd ($(command -v "$cmd"))"
  else
    fail "$cmd is not installed. $install_hint"
  fi
}

check_command rustup "Install Rust from https://rustup.rs/."
check_command cargo "Install Rust from https://rustup.rs/."
check_command bun "Install Bun from https://bun.sh/."

if cargo leptos --version >/dev/null 2>&1; then
  pass "cargo-leptos ($(cargo leptos --version | head -n 1))"
else
  fail "cargo-leptos is not installed. Run: cargo install cargo-leptos --locked"
fi

if bunx wrangler --version >/dev/null 2>&1; then
  pass "wrangler via bunx ($(bunx wrangler --version | tail -n 1))"
else
  fail "Wrangler is not available through bunx. Check your Bun installation and network access."
fi

if command -v wasm-bindgen >/dev/null 2>&1; then
  wasm_bindgen_version="$(wasm-bindgen --version | awk '{print $2}')"
  if [ "$wasm_bindgen_version" = "$EXPECTED_WASM_BINDGEN_VERSION" ]; then
    pass "wasm-bindgen-cli ($wasm_bindgen_version)"
  else
    warn "wasm-bindgen-cli is $wasm_bindgen_version, expected $EXPECTED_WASM_BINDGEN_VERSION for the verified toolchain."
    warn "Fix with: cargo install -f wasm-bindgen-cli --version $EXPECTED_WASM_BINDGEN_VERSION"
  fi
else
  fail "wasm-bindgen-cli is not installed. Run: cargo install -f wasm-bindgen-cli --version $EXPECTED_WASM_BINDGEN_VERSION"
fi

if rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  pass "wasm32-unknown-unknown target installed"
else
  fail "Rust wasm target missing. Run: rustup target add wasm32-unknown-unknown"
fi

if grep -q '00000000-0000-0000-0000-000000000000' "$ROOT_DIR/wrangler.toml"; then
  warn "wrangler.toml still contains placeholder D1 IDs. Replace them after running: bunx wrangler d1 create leptos-cf-db"
fi

if [ "$missing" -ne 0 ]; then
  exit 1
fi

pass "Dependency checks passed."
