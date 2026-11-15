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

- Use a short-lived, account-scoped child credential; the token-minter credential never enters the repo or deployment profile.
- Read D1 by exact name before preparing creation. Review, explicitly approve, run, and verify every provider mutation through `cfctl`.
- Apply remote schema only through the repository-owned operation that binds the clean root/HEAD, pack and ordered migration blobs, derived-config hash/identity, a fresh pre-change recovery bookmark, Wrangler ledger, and closed post-schema proof. Stop if it is blocked.
- Treat D1 creation, schema application, Worker deployment, provider configuration readback, and authenticated route/telemetry evidence as separate proof planes.

Dependency audit notes (recheck the advisory database for each candidate):

- `cargo audit` reports zero known vulnerabilities.
- `RUSTSEC-2024-0436` remains for transitive `paste 1.0.15` through
  Leptos/Tachys (`either_of`, `reactive_graph`, and related view machinery).
  RustSec classifies it as unmaintained and publishes no patched `paste`
  release. It is a compile-time proc-macro dependency, not a reported runtime
  vulnerability. Track the upstream Leptos/Tachys replacement and remove this
  exception when that dependency disappears.
- `RUSTSEC-2026-0173` remains for transitive `proc-macro-error2 2.0.1`
  through `leptos_macro`, `leptos_router_macro`, `reactive_stores_macro`, and
  `syn_derive`. RustSec classifies it as unmaintained and publishes no patched
  release. It is also a compile-time proc-macro dependency. Track the upstream
  macro crates rather than substituting a second local macro stack.
- `cargo audit` reports yanked `spin 0.9.8` through
  `multer 3.1.0 -> axum 0.8.9`. This is a registry-yank warning without a
  RustSec vulnerability advisory or patched `0.9.x` lockfile candidate.
  Continue tracking the current Axum/multer chain and remove the exception as
  soon as upstream stops selecting this version.

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

## Source publication candidate — 2026-11-15

The publication target is the GitHub starter source, as selected by the operator.
No live Workers or Pages deployment, provider identity, migration or telemetry
readback is included. Workers remains the default; Pages production deployment
remains experimental. The package version remains 0.1.3; the published commit
identifies this source revision.

The candidate contains shared Workers/Pages dispatch, request-body cancellation
isolation, original Disk Guard descriptor forwarding, pinned local tooling,
session-scoped Resource Explorer, and the qualified source/documentation
refactor. File-size policy retains the operator-approved generated-lock, PNG
and ignored-boundary exceptions; ignored contents remain unverified.

Cargo regenerated only the Tokio version and checksum, from 1.52.3 to 1.53.3,
which resolves [RUSTSEC-2026-0386](https://rustsec.org/advisories/RUSTSEC-2026-0386/).
The generated-lock exception hash was reviewed and refreshed for those bytes.

The local gate now proves contact intake against disposable D1 on both targets:
migration, normalized persistence, honeypot rejection, separate hourly/daily
session budgets, and oldest-first global trimming at 2,000 rows. The test seeds
only its own temporary store; it does not establish production delivery.

Open project obligations were inventoried before publication:

| Issue | Disposition for this candidate |
| --- | --- |
| #16 contact D1 proof | Addressed by `scripts/test-contact-intake.mjs`, run through both native boundary suites; close only after integration. |
| #19 Rust/UI demo | Separate optional component-stack integration; no Rust/UI support is claimed here. Retain open. |
| #11 Langual rewrite | Separate product and checkout; preserve its owner and local work. This template release does not resolve it. |
| #7 extension app shell | Separate optional extension contract; retain open. |
| #8 Rust-domain Worker adapter | Separate extension design contract; retain open. |
| #9 operator reply identity | Separate authenticated extension UX contract; retain open. |

No open PR competed with this candidate at inventory time. The existing local
topic branches have no commits outside `main`; they were retained. Publication
does not close the historical website's provider-launch obligations or establish
external adopter acceptance. Required merge approval remains with the operator.
