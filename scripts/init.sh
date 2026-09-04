#!/usr/bin/env bash
# Adopt portable project identity without deleting application code or data.
set -euo pipefail
cd "$(dirname "$0")/.."
exec bun ./scripts/init-project.mjs "$@"
