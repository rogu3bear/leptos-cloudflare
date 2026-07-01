#!/usr/bin/env bash
#
# Full release readiness verification for the leptos-cf template.
#
# This is the single local command for release readiness.
# Run it before pushing, opening a PR, or claiming a change is complete.
#
# It is intentionally a thin orchestrator — the real power lives in the
# individual scripts it calls (check-deps.sh, build-edge.sh, etc.).
set -euo pipefail

echo "==> Verifying local release readiness"

echo "==> 1/8 Dependency and toolchain check"
./scripts/check-deps.sh

echo "==> 2/8 Pattern layer contract"
bun ./scripts/verify-patterns.mjs

echo "==> 3/8 Formatting"
cargo fmt --check

echo "==> 4/8 SSR compile check"
cargo check --features ssr

echo "==> 5/8 Security audit"
if command -v cargo-audit >/dev/null 2>&1; then
  cargo audit
else
  echo "[verify] cargo-audit not found on PATH — skipping."
  echo "         Install once with: cargo install cargo-audit --locked"
fi

echo "==> 6/8 Full edge build (WASM + hashed assets + worker bundle + verifiers)"
bash ./scripts/build-edge.sh

echo "==> 7/8 Wrangler deployment structure validation"
bunx wrangler@4.83.0 deploy --dry-run

echo "==> 8/8 Repository hygiene"
git diff --check

echo ""
echo "==> All release readiness checks passed."
echo "    Remote CI is intentionally absent; this local gate is authoritative."
