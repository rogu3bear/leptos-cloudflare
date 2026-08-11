#!/usr/bin/env bash
# scripts/rotate-secrets.sh — standard Cloudflare child-token rotation.
#
# Delegates to the workspace-level shared rotator, discovered relative to this
# repo (../scripts/cf-rotate.sh under the workspace root) or via $CF_ROTATOR.
# The shared engine verifies the current child, clones its policies, mints a
# replacement from the account minter, verifies it, and updates this repo's
# .env. Raw curl, no cfctl/keychain — runs unattended.
#
# See AGENTS.md § Cloudflare Token Doctrine.
#
# Usage:  ./scripts/rotate-secrets.sh [--dry-run]
set -euo pipefail
REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
REPO="$(basename "$REPO_DIR")"
ROTATOR="${CF_ROTATOR:-$(cd "$REPO_DIR/.." && pwd)/scripts/cf-rotate.sh}"
[[ -x "$ROTATOR" ]] || {
  echo "ERROR: shared rotator not found or not executable at: $ROTATOR" >&2
  echo "       set \$CF_ROTATOR to your shared cf-rotate.sh path." >&2
  exit 1
}
exec "$ROTATOR" "$REPO" "$@"
