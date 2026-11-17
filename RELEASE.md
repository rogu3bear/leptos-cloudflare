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

Run the single local verification command:

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

The v0.1.7 lockfile has two upstream maintenance warnings. Refresh the audit
for every release candidate.

- `RUSTSEC-2026-0543` flags `chrono 0.4.44`, required by `worker 0.7.5`.
  RustSec lists no patched version. Recheck when the Worker SDK changes its
  date/time dependency.
- `RUSTSEC-2024-0436` flags `paste 1.0.15`, used at compile time by Leptos,
  Tachys and their reactive dependencies. RustSec lists no patched version.
  Recheck when these crates replace the macro and it leaves the lockfile.

Resolved in the 2026-08-10 candidate:

- `RUSTSEC-2026-0190`: locked `anyhow` was advanced from `1.0.102` to patched
  `1.0.103`; the transitive path is `leptos_hot_reload -> leptos`.
- `RUSTSEC-2026-0221`: locked `event-listener` was advanced from `5.4.1` to
  patched `5.4.2`; the transitive path is
  `async-lock -> reactive_graph -> leptos`.

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

## Release Decision

Release only when every applicable gate above has fresh output and any residual dependency audit warnings are either fixed or documented with the upstream dependency path.


Pages is a separate release target. `pages/wrangler.toml` retains placeholder D1 identity; advanced-mode output is generated by `scripts/package-pages.mjs`. Local Pages boundary proof and provider deployment/readback are separate from Workers proof.


Local tooling uses the exact Wrangler dependency in `package.json` and `bun.lock`; run `bun install --frozen-lockfile` (also done by bootstrap). The verification dry run launches this local CLI with a Node preload that validates and forwards the original Disk Guard descriptors. It still executes the configured build. API requests with bodies use an isolated request clone so cancellation by the Rust body adapter cannot reset the local Workers proxy; the Rust 4 KiB rejection remains authoritative.

## Pages production preparation follow-up

The earlier source candidate's deferred preparation path is now covered by
[Pages production qualification](docs/pages-production.md). The gate verifies
placeholder template boundaries, distinct production/preview identities, a fresh
private standard-config directory, source/artifact provenance, exact ordered
migrations, both native multipart upload bundles and config hashes, plus local
D1/SSR/security/persistence on the derived directory. The build fails on source
changes during compilation; preparation fails on stale source or artifact drift.

Readback contract fixtures reject mismatched account, project, branch, D1,
config/date, canonical deployment, stage, commit or URL. They are synthetic tests,
not provider evidence. A clean committed build is required by the release-file
CLI and readback validator; dirty builds are qualification-only. No live Pages
creation, migration, deployment or acceptance is established by this follow-up.
The source update leaves those effects with their explicitly authorized owner.

Qualification also found and repaired an encoded asset-path traversal that
returned server WASM. Pages now normalizes decoded private paths before ASSETS
dispatch and places its server graph in the uploader-excluded `functions/`
namespace. Native regression checks cover both old and new private paths.
Both environments must additionally report fail-closed project settings before
upload and during final readback; source tests do not establish provider quota
behavior.

The final source follow-up advances transitive `xxhash-rust` from 0.8.15 to
patched 0.8.16 for [GHSA-6g2r-675j-hx59](https://github.com/advisories/GHSA-6g2r-675j-hx59).
Cargo changed only that package version/checksum; the generated-lock exception
hash was reviewed and refreshed. The three existing audit warnings remain.

## Source release 0.1.4

This release distributes the agent-ready GitHub template source. It includes
Workers and Pages local qualification, private-module/body-boundary repairs,
portable onboarding, optional governed cfctl guidance and removal of internal
planning records from the public tree. See [CHANGELOG.md](CHANGELOG.md).

The source archive must match the independently reviewed integrated commit.
Publish no credentials, generated runtime output, private qualification packets
or provider identities. Local Workers/Pages proof is not deployment or live
acceptance; no Cloudflare deployment is included in this source release.

## Security patch 0.1.5

Enforce the 4 KiB body limit on actual streamed bytes before server-function
collection, including chunked requests. Native regressions test the exact boundary,
413 rejection and retained security headers on Workers, Pages and derived Pages.

Anonymous contact admission uses an atomic conditional insert for three/hour and
eight/day per keyed client-IP digest. Todo creation atomically bounds session,
network and global capacity. Cookie rotation cannot reset the network budget;
full contact/todo storage rejects without evicting unrelated sessions.

Apply the append-only fourth migration before using this code. Preserve its
database-local key during backup/recovery; never treat key deletion as quota
recovery. No provider migration/deployment is included in the source release.
