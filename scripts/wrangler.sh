#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [ ! -f "$ROOT_DIR/node_modules/wrangler/bin/wrangler.js" ]; then
  printf 'Run bun install --frozen-lockfile before invoking Wrangler.\n' >&2
  exit 1
fi
NODE_OPTIONS="${NODE_OPTIONS:+$NODE_OPTIONS }--require=$ROOT_DIR/scripts/preserve-guard-fds.cjs" exec node "$ROOT_DIR/node_modules/wrangler/bin/wrangler.js" "$@"
