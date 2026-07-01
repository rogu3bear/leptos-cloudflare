# Template Release Checklist

Use this checklist before tagging or announcing a public template release.

## Source Hygiene

- `git status --short` contains only intentional source/doc/config changes.
- `.env.example` contains placeholders only.
- `build/`, `target/`, `var/`, `.wrangler/`, `.env`, and `.dev.vars*` are ignored.
- `wrangler.toml` keeps placeholder D1 IDs for template initialization.
- Browser identity assets stay in sync: `assets/favicon.svg`, `assets/app-icon.svg`, generated PNG app icons, and `assets/site.webmanifest`.

## Required Gates

Run the single local verification command:

```bash
./scripts/verify.sh
```

The script executes the complete sequence, including the `/patterns/` contract check and `git diff --check` as the final hygiene step.

Expected template warning:

- `./scripts/check-deps.sh` warns that `wrangler.toml` still contains placeholder D1 IDs.

Current dependency audit notes:

- `cargo audit` reports `RUSTSEC-2024-0436` for transitive `paste 1.0.15` through Leptos/Tachys. This is an unmaintained-crate warning, not a known exploit in this template path. Re-check before each release and remove this note when upstream no longer pulls `paste`.
- `cargo audit` reports `RUSTSEC-2026-0173` for transitive `proc-macro-error2 2.0.1` through Leptos macro dependencies. This is an unmaintained-crate warning, not a known exploit in this template path. Re-check before each release and remove this note when upstream no longer pulls `proc-macro-error2`.

## Runtime Contract

- `wrangler.toml` points to `build/_worker.js`.
- `scripts/write-worker-shim.mjs` routes WebSocket upgrades before static assets and SSR.
- `scripts/verify-worker-runtime.mjs` checks the Worker shim, compatibility date, Assets binding, WebSocket lane, and SSR fallback.
- `scripts/verify-hashed-assets.mjs` checks immutable hashed assets and no-store manifest behavior.
- `/contact` remains a same-origin, D1-backed intake demo; production delivery still requires an explicit email, Queue, Turnstile, or rate-limit integration.
- Shared realtime state remains outside the core runtime until adopted through `patterns/realtime-durable-object/`.

## Pattern Layer Contract

- `patterns/` is the second layer: examples and docs may extend the starter, but the core template must remain independently usable.
- `scripts/verify-patterns.mjs` checks that every pattern directory is indexed, has a README, and references existing core template files or contracts.
- New pattern docs must keep local links and inline core path references resolvable so examples fail locally when the core shape changes.
- `patterns/realtime-durable-object/` is the first shared-state realtime example; it documents the Durable Object binding without adding it to the default `wrangler.toml`.

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
