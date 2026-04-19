#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXPECTED_CARGO_LEPTOS_VERSION="0.3.5"
EXPECTED_WASM_BINDGEN_VERSION="0.2.114"
EXPECTED_WORKER_BUILD_VERSION="0.7.5"
EXPECTED_WRANGLER_VERSION="4.83.0"
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

wrangler_cmd() {
  bunx "wrangler@${EXPECTED_WRANGLER_VERSION}" "$@"
}

if cargo leptos --version >/dev/null 2>&1; then
  cargo_leptos_version="$(cargo leptos --version | awk '{print $2}')"
  if [ "$cargo_leptos_version" = "$EXPECTED_CARGO_LEPTOS_VERSION" ]; then
    pass "cargo-leptos ($cargo_leptos_version)"
  else
    fail "cargo-leptos is $cargo_leptos_version, expected $EXPECTED_CARGO_LEPTOS_VERSION. Run: cargo install cargo-leptos --locked --version $EXPECTED_CARGO_LEPTOS_VERSION"
  fi
else
  fail "cargo-leptos is not installed. Run: cargo install cargo-leptos --locked --version $EXPECTED_CARGO_LEPTOS_VERSION"
fi

if wrangler_cmd --version >/dev/null 2>&1; then
  pass "wrangler via bunx ($(wrangler_cmd --version | tail -n 1))"
else
  fail "Wrangler $EXPECTED_WRANGLER_VERSION is not available through bunx. Check your Bun installation and network access."
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

if command -v worker-build >/dev/null 2>&1; then
  worker_build_version="$(worker-build --version | awk '{print $1}')"
  if [ "$worker_build_version" = "$EXPECTED_WORKER_BUILD_VERSION" ]; then
    pass "worker-build ($worker_build_version)"
  else
    fail "worker-build is $worker_build_version, expected $EXPECTED_WORKER_BUILD_VERSION. Run: cargo install worker-build --locked --version $EXPECTED_WORKER_BUILD_VERSION"
  fi
else
  fail "worker-build is not installed. Run: cargo install worker-build --locked --version $EXPECTED_WORKER_BUILD_VERSION"
fi

if rustup target list --installed | grep -qx 'wasm32-unknown-unknown'; then
  pass "wasm32-unknown-unknown target installed"
else
  fail "Rust wasm target missing. Run: rustup target add wasm32-unknown-unknown"
fi

if grep -q '00000000-0000-0000-0000-000000000000' "$ROOT_DIR/wrangler.toml"; then
  warn "wrangler.toml still contains placeholder D1 IDs. Replace them after running: bunx wrangler@4.83.0 d1 create leptos-cf-db"
fi

if [ "$missing" -ne 0 ]; then
  exit 1
fi

pass "Dependency checks passed."
