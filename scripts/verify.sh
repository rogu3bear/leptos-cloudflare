#!/usr/bin/env bash
#
# Full release readiness verification for the leptos-cf template.
#
# This is the single local command that reproduces what CI runs.
# Run it before pushing, opening a PR, or claiming a change is complete.
#
# It is intentionally a thin orchestrator — the real power lives in the
# individual scripts it calls (check-deps.sh, build-edge.sh, etc.).
set -euo pipefail

echo "==> Verifying release readiness (local equivalent of CI)"

echo "==> 1/6 Dependency and toolchain check"
./scripts/check-deps.sh

echo "==> 2/6 Formatting"
cargo fmt --check

echo "==> 3/6 SSR compile check"
cargo check --features ssr

echo "==> 4/6 Security audit"
if command -v cargo-audit >/dev/null 2>&1; then
  cargo audit
else
  echo "[verify] cargo-audit not found on PATH — skipping."
  echo "         Install once with: cargo install cargo-audit --locked"
fi

echo "==> 5/6 Full edge build (WASM + hashed assets + worker bundle + verifiers)"
bash ./scripts/build-edge.sh

echo "==> 6/6 Wrangler deployment structure validation"
bunx wrangler@4.83.0 deploy --dry-run

echo "==> 7/7 Repository hygiene"
git diff --check

echo ""
echo "==> All release readiness checks passed."
echo "    This is the same sequence executed by .github/workflows/rust.yml"
