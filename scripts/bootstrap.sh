#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXPECTED_CARGO_LEPTOS_VERSION="0.3.5"
EXPECTED_WORKER_BUILD_VERSION="0.7.5"
EXPECTED_WRANGLER_VERSION="4.120.1"

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

wrangler_cmd() {
  bunx "wrangler@${EXPECTED_WRANGLER_VERSION}" "$@"
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

log "Checking Wrangler $EXPECTED_WRANGLER_VERSION through bunx."
wrangler_cmd --version >/dev/null

log "Running dependency checks."
"$ROOT_DIR/scripts/check-deps.sh"

cat <<'EOF'

Bootstrap complete.

Local next steps:
1. CI=1 bunx wrangler@4.120.1 d1 migrations apply leptos-cf-db --local
2. bash ./scripts/build-edge.sh
3. bunx wrangler@4.120.1 dev --local --ip 127.0.0.1 --port 57581

Production initialization is a separate provider transaction:
Choose the governed or independent standalone profile in docs/credentials.md.
In this operator workspace:
1. Acquire a short-lived, account-scoped child token through the governed credential flow.
2. Use cfctl to read D1 by name; if absent, prepare, approve, run, and verify d1-create-database.
3. Derive ignored wrangler.production.toml with scripts/write-production-config.mjs using only verified Worker/D1 names and the read-back D1 UUID. Never edit the tracked template identity.
4. Apply the exact committed migration set through the repository-bound governed D1 migration operation.
5. Prepare, approve, run, and read back the Workers deployment through cfctl.

If cfctl reports that a provider mutation is blocked, stop at that boundary. Do not bypass it with a direct Wrangler or raw API write.
EOF
