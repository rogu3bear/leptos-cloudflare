# Template Release Checklist

Use this checklist before tagging or announcing a public template release.

## Source Hygiene

- `git status --short` contains only intentional source/doc/config changes.
- `.env.example` contains placeholders only.
- `build/`, `target/`, `var/`, `.wrangler/`, `.env`, and `.dev.vars*` are ignored.
- `wrangler.toml` keeps placeholder D1 IDs for template initialization.
- Browser identity assets stay in sync: `assets/favicon.svg`, `assets/app-icon.svg`, generated PNG app icons, and `assets/site.webmanifest`.

## Required Gates

```bash
./scripts/check-deps.sh
cargo fmt --check
cargo check --features ssr
cargo audit
bash ./scripts/build-edge.sh
bunx wrangler@4.83.0 deploy --dry-run
git diff --check
```

Expected template warning:

- `./scripts/check-deps.sh` warns that `wrangler.toml` still contains placeholder D1 IDs.

Current dependency audit note:

- `cargo audit` reports `RUSTSEC-2024-0436` for transitive `paste 1.0.15` through Leptos/Tachys. This is an unmaintained-crate warning, not a known exploit in this template path. Re-check before each release and remove this note when upstream no longer pulls `paste`.

## Runtime Contract

- `wrangler.toml` points to `build/_worker.js`.
- `scripts/write-worker-shim.mjs` routes WebSocket upgrades before static assets and SSR.
- `scripts/verify-worker-runtime.mjs` checks the Worker shim, compatibility date, Assets binding, WebSocket lane, and SSR fallback.
- `scripts/verify-hashed-assets.mjs` checks immutable hashed assets and no-store manifest behavior.

## Documentation Contract

Update these files when their surfaces change:

- `README.md`
- `docs/README.md`
- `docs/agent-playbook.md`
- `docs/building-features.md`
- `docs/edge-deployment.md`
- `docs/hybrid-deployment.md`
- `docs/realtime.md`
- `SECURITY.md`
- `CONTRIBUTING.md`

## Release Decision

Release only when every applicable gate above has fresh output and any residual dependency audit warnings are either fixed or documented with the upstream dependency path.
