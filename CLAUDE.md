# CLAUDE.md

Claude and Codex are peer coding agents in this repo. Use `AGENTS.md` as the shared operating contract.

Quick reminders:

- Start from live git state.
- Preserve unrelated dirty work.
- Build with `bash ./scripts/build-edge.sh`, not ad hoc `cargo build`, before release claims.
- Keep `build/_worker.js` generated from `scripts/write-worker-shim.mjs`.
- Keep WebSocket/Durable Object decisions aligned with `docs/realtime.md`.
- Do not commit secrets or real operator account identifiers.
