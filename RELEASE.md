# Template Release Checklist

Use this checklist before tagging or announcing a public template release.

## Source Hygiene

- `git status --short` contains only intentional source/doc/config changes.
- `.env.example` contains placeholders only.
- `build/`, `target/`, `var/`, `.wrangler/`, `.env`, and `.dev.vars*` are ignored.
- Tracked `wrangler.toml` keeps placeholder D1 IDs permanently; `wrangler.production.toml` is gitignored.
- `scripts/test-production-config.mjs` proves that production derivation changes only Worker/D1 identity and preserves Workers SSR, Assets, and observability.
- Browser identity assets stay in sync: `assets/favicon.svg`, `assets/app-icon.svg`, generated PNG app icons, and `assets/site.webmanifest`.
- The field-guide route tree and the compatibility-only `/todo/:id` path match `src/app.rs` and the public documentation.

## Required Gates

Set [the release baseline](#hydration-assets-during-release-switches), then run
the full local gate.

```bash
./scripts/verify.sh
```

The script requires cargo-audit and fails before compilation if it is absent.
An audit failure propagates; there is no skipped-audit success path.

The script executes the complete sequence, including the production-config, `/patterns/`, and architecture contracts, SSR unit tests, a local Worker network-boundary test, and `git diff --check` as the final hygiene step.

Expected template boundary:

- `./scripts/check-deps.sh` passes only while tracked `wrangler.toml` retains placeholder D1 IDs and `wrangler.production.toml` remains ignored. Production identity comes from deterministic derivation after live readback, never from plan intent.

Provider initialization contract:

Select the account owner's [credential profile](docs/credentials.md) before any
provider operation. Local qualification needs neither credentials nor cfctl.
The following plan/operation requirements apply to cfctl-governed accounts;
independent standalone accounts use the target-specific
[Workers](docs/edge-deployment.md) or [Pages](docs/pages-production.md) procedure.
Both lanes require owner authorization, verified target identity, recovery before
schema changes, ordered reviewed migrations, and independent provider readback.
A governed failure never authorizes switching lanes.

- Use a short-lived, account-scoped child credential; the token-minter credential never enters the repo or deployment profile.
- Read D1 by exact name before preparing creation. Review, explicitly approve, run, and verify every provider mutation through `cfctl`.
- Apply remote schema only through the repository-owned operation that binds the clean root/HEAD, pack and ordered migration blobs, derived-config hash/identity, a fresh pre-change recovery bookmark, Wrangler ledger, and closed post-schema proof. Stop if it is blocked.
- Treat D1 creation, schema application, Worker deployment, provider configuration readback, and authenticated route/telemetry evidence as separate proof planes.

The dependency audit allows two documented upstream maintenance warnings.
Refresh the audit for every release candidate.

- `RUSTSEC-2026-0543` flags `chrono 0.4.44`, required by `worker 0.7.5`.
  RustSec lists no patched version. Recheck when the Worker SDK changes its
  date/time dependency.
- `RUSTSEC-2024-0436` flags `paste 1.0.15`, used at compile time by Leptos,
  Tachys and their reactive dependencies. RustSec lists no patched version.
  Recheck when these crates replace the macro and it leaves the lockfile.

## Runtime Contract

- `wrangler.toml` points to `build/_worker.js`.
- The Workers Static Assets router owns exact asset matches before user Worker code; `scripts/write-worker-shim.mjs` preserves WebSocket, explicit asset-fallback, and SSR dispatch when the Worker is invoked.
- `scripts/verify-worker-runtime.mjs` checks the Worker shim, current compatibility date, sampled observability, Assets binding, WebSocket lane, and SSR fallback.
- The Worker shim emits only closed `ssr`/`server_function` custom-span dimensions after Assets/WebSocket exits; Rust emits one versioned completion record with an allowlisted schema and no raw request, identity, body, D1, or internal-error material.
- `scripts/verify-architecture-contract.mjs` keeps the public SSR/CSR and Workers/Pages decisions aligned with the checked-in runtime.
- `scripts/test-worker-boundaries.mjs` boots the exact release artifacts in local Wrangler and proves useful initial SSR HTML, CSP/hydration nonce agreement, static asset separation, rendered 404 status, API rejection, and the realtime upgrade boundary.
- `scripts/verify-hashed-assets.mjs` checks immutable hashed assets and no-store manifest behavior.
- `/contact` remains a same-origin, D1-backed intake demo; production delivery still requires an explicit email, Queue, Turnstile, or rate-limit integration.
- Dynamic HTML carries a per-response CSP nonce shared by Leptos hydration and streamed resource scripts; browser proof must show no CSP or hydration errors.
- `/lab` resource output remains inside a Suspense boundary and its create, toggle, detail, and delete flow passes in a real browser.
- Shared realtime state remains outside the core runtime until adopted through `patterns/realtime-durable-object/`.

## Hydration assets during release switches

A document can finish fetching its JS, WASM and CSS after the active server
changes. Each edge build archives its current hashed bundle and restores all
complete bundles for this application to the public `pkg/` directory. The active
manifest still names only the current build. Workers Assets and the Pages upload
both serve the retained bytes.

The default archive is `var/hydration-history/<output-name>/`. Back up the
history outside the checkout for clean builders. Set `LEPTOS_HYDRATION_HISTORY`
to the restored parent directory before running the normal build and full gate.
When a history directory is explicitly selected, the build imports earlier
assets only from that history or an explicit capture. Keep separate directories
per trusted application and environment. Only import
public client artifacts from releases you intend to continue serving. A first
build in an empty checkout cannot reconstruct earlier binaries from Git.

For the first release only, export `LEPTOS_RELEASE_BASELINE=first-release`.
For each upgrade or rollback, record the outgoing bundle ID alongside the
outgoing release, independently of the history backup. To obtain the ID and
seed history, point the capture command at the outgoing verified Workers
site directory or its prepared Pages `public/` directory:

```sh
export LEPTOS_HYDRATION_HISTORY=/path/to/restored-history
bun scripts/hydration-history.mjs capture target/pages-release-previous/public > /path/to/outgoing-release/bundle-id
# On the clean builder, use the ID recorded by the outgoing release.
export LEPTOS_RELEASE_BASELINE="$(cat /path/to/outgoing-release/bundle-id)"
```

For a rollback, rebuild the previous application and server with the retention
tooling and the same history **including the outgoing release**. For a rollback
to earlier application code, retain the current build scripts, browser tooling
and `EdgeHydrationScripts` bootstrap in `src/edge/shell.rs`. Its performance
mark signals that the browser returned from `hydrate()`. Prepare and
qualify that new rollback package before switching traffic. An unmodified
historical deployment cannot contain assets first published after it was built.
Leptos includes the build directory in generated server-function paths. The
Workers and Pages shims forward the six template functions’ exact retained
client paths to their matching active paths, preserving the request guards.
For new application functions, declare stable endpoint names and preserve
their input and output contracts across releases.

The full gate checks archived bytes over HTTP on Workers, Pages and the derived
Pages directory. The gate rejects a missing or corrupt outgoing bundle before
building. Pinned WebKit executes hydration and the application mutation in
`scripts/browser/application-check.cjs` on every profile. The reference
application also exercises server-function responses, persistence, retry and
session isolation. Install the browser with bootstrap or
`node node_modules/playwright/cli.js install webkit` after a frozen Bun install.

## Adoption Contract

- `scripts/test-acceptance.mjs` verifies safe local identity adoption, preserved
  migration bytes, rejection without writes, and required security-audit outcomes.
- `package.metadata.leptos-cf.reference-site` selects reference application text
  checks. It is true for this public field guide and false after name adoption.
- Every profile still verifies SSR/hydration, Worker/Assets ownership, CSP,
  hashed assets, rendered missing-route recovery, and request guards.
- `bun scripts/test-adopted-runtime.mjs` is the separate heavy adoption proof:
  run it before the final canonical verifier when changing adoption or routing.
  It builds a disposable renamed application with a replaced and added page,
  reusing the target sequentially; rebuild the canonical artifact afterward.
- Asset verification compares manifest hashes with final served bytes. The
  WASM-only fingerprint fixture requires its importing JavaScript URL to change.
- Browser acceptance covers hydration, isolated persisted CRUD, failed detail
  mutations with retry, and preload credential agreement.
- Follow `docs/adopting.md` before removing a sample domain.
- Use the profile in `docs/credentials.md`; no external rotator is shipped.

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

## Pages production preparation follow-up

Follow [Pages production qualification](docs/pages-production.md) for the
isolated standard-config directory, separate production and preview D1
identities, ordered migrations, native upload bundles and readback. Build from
clean committed source for provider-facing release files. Dirty builds qualify
locally, and preparation rejects stale source or modified artifacts. Verify
fail-closed project settings for both environments before upload and again
during final readback.

## Release Decision

Release only when every applicable gate has fresh output and residual dependency
audit warnings are documented with their upstream paths. For a GitHub template
source release, match the archive to the independently reviewed integrated
commit. Exclude credentials, generated runtime output, private qualification
records and provider identities. Cloudflare changes require their own approval.
