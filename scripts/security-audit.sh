#!/usr/bin/env bash
set -euo pipefail
if ! command -v cargo-audit >/dev/null 2>&1; then
  printf '[security-audit] Required cargo-audit is missing. Install with: cargo install cargo-audit --locked\n' >&2
  exit 1
fi
cargo audit
