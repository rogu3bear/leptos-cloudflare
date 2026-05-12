#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO_LOCK_PATH="$ROOT_DIR/Cargo.lock"
TOOLS_ROOT="$ROOT_DIR/var/cargo-tools"

resolve_wasm_bindgen_version() {
  awk '
    $0 == "name = \"wasm-bindgen\"" { in_package = 1; next }
    in_package && $1 == "version" {
      gsub(/"/, "", $3)
      print $3
      exit
    }
  ' "$CARGO_LOCK_PATH"
}

version="$(resolve_wasm_bindgen_version)"

if [ -z "$version" ]; then
  printf '[wasm-bindgen] unable to resolve wasm-bindgen version from Cargo.lock\n' >&2
  exit 1
fi

install_root="$TOOLS_ROOT/wasm-bindgen-$version"
binary="$install_root/bin/wasm-bindgen"

if [ ! -x "$binary" ]; then
  mkdir -p "$TOOLS_ROOT"
  cargo install \
    --root "$install_root" \
    wasm-bindgen-cli \
    --version "$version" \
    --locked
fi

if [ $# -eq 0 ] || [[ "${1:-}" == -* ]]; then
  exec "$binary" "$@"
fi

PATH="$install_root/bin:$PATH" exec "$@"
