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

cd "$(dirname "$0")/.."

echo "==> Verifying local release readiness"
# Refuse an incomplete release before expensive compilation.
if ! command -v cargo-audit >/dev/null 2>&1; then
  ./scripts/security-audit.sh
fi
bun ./scripts/test-acceptance.mjs
bun ./scripts/test-asset-fingerprints.mjs

echo "==> 1/11 Dependency and toolchain check"
./scripts/check-deps.sh

echo "==> 2/11 Production configuration boundary"
bun ./scripts/test-production-config.mjs

echo "==> 3/11 Pattern layer contract"
bun ./scripts/verify-patterns.mjs

echo "==> 4/11 Architecture decision contract"
bun ./scripts/verify-architecture-contract.mjs

echo "==> 5/11 Formatting"
cargo fmt --check

echo "==> 6/11 SSR compile and unit tests"
cargo check --features ssr
cargo test --features ssr

echo "==> 7/11 Security audit"
./scripts/security-audit.sh

echo "==> 8/11 Full edge build (WASM + hashed assets + worker bundle + verifiers)"
bash ./scripts/build-edge.sh

echo "==> 9/11 Local Worker rendering and network boundaries"
bun ./scripts/test-worker-boundaries.mjs

echo "==> 10/11 Wrangler deployment structure validation"
bunx wrangler@4.120.1 deploy --dry-run

echo "==> 11/11 Repository hygiene"
git diff --check

echo ""
echo "==> All release readiness checks passed."
echo "    Remote CI is intentionally absent; this local gate is authoritative."
