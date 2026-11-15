# Untangle decision log

Scope: existing checkout, behavior-preserving refactor; Comments and Tests selected. Baseline: main at c9054258839db3a2d85ecc5d2776922b8d717887, including the preceding locally qualified dirty starter changes and unrelated resource-explorer work. The baseline patch and file hashes are retained under ignored `var/untangle/`. No publication or provider effect is included.

## Redesign and inventory

1. Own dispatch in executable JavaScript, with explicit deployment options and one socket override.
   - Hidden links: `write-worker-shim.mjs` string-array runtime; `package-pages.mjs` exact source surgery; optional realtime example duplicates dispatch and loses clone isolation/telemetry.
   - Needless links: runtime/package tests assert those internal spellings.
   - Preserve generated Workers entrypoint, Pages exact-asset inventory/private namespace protections, API clone isolation, telemetry and socket behavior.
2. Give pinned tools one contract and one local Wrangler launcher.
   - Hidden links: three copies of Rust tool versions; package pin vs bunx vs local checks; docs use inconsistent launch paths.
   - Surplus: wasm-bindgen check compares its observed value to itself.
   - Keep exact approved versions, independently compare Cargo.lock to the selected binary, and preserve original validated Guard descriptors.
3. Resolve project/asset paths once, while keeping independent served-byte verification.
   - Hidden links: duplicated cargo-metadata subprocesses/root lookup; metadata paths versus hardcoded pkg paths in hash output, verification and packaging.
   - Retain the generated asset-hash environment producer/SSR compiler consumer contract.
4. Separate preview ownership from assertions.
   - Needless link: boundary tests own ports, migration/config/package lifecycle, generic security and application assertions together.
   - Hidden link: setup failures occur outside cleanup ownership.
   - Preserve isolated local stores, bounded readiness, request timeouts and owned TERM/KILL cleanup.
5. Name adoption documents and share deployment identity handling.
   - Hidden links: originals/next array indexes encode each file's responsibility; Worker/Pages duplicate local identity transformations.
   - Keep preflight, exclusive files, concurrency detection, rollback and distinct adoption/production identity restrictions.
6. Give the Rust edge boundary and SSR document distinct owners.
   - Needless links: `lib.rs` mixes wiring, guards, sessions, headers and policy tests; `app.rs` combines routes with build hashes/hydration shell.
   - Preserve request guard order, cookie rules, nonces, 404 ownership and SSR/hydrate separation.
7. Own server context/D1 and the Send bridge once.
   - Hidden links: contact/todos duplicate context/database/session and numeric conversions; eight API functions repeat transport bridging.
   - Keep domain SQL in each domain and API macro names/locations/wire identities stable. No generic repository framework.
8. Share domain limits and status contracts with UI consumers.
   - Hidden links: contact limits repeated in HTML, todo title limit repeated in inputs, search status interpreted separately.
   - Preserve byte/character/HTML length semantics and distinct permissive URL versus strict command parsing.
9. Remove code without an independent reason to change.
   - Surplus: immutable completion-rate Memo, static nested Suspense, discarded internal-error string payloads, comments narrating implementation.
   - Keep reactive optimistic action state, conflict/idempotency, privacy and runtime workaround explanations.
10. Let documentation depend on stable interfaces.
    - Hidden links: pattern checker keeps a root-tree registry; pattern README repeats its inventory; feature guide duplicates palette/glass details and router inventory.
    - Derive path families from the tree; keep missing-link checks and one linked pattern index; link to styling owner and show only added route.
11. Clean tests last.
    - Remove mapping-only UI tests; prefer public dispatch behavior and generated endpoint/asset/runtime proof over internal variable names.
    - Preserve all release/security/config checks and meaningful privacy, escaping, invalid-input, collision, symlink, isolation and fingerprint tests.
    - Field-guide editorial assertions are a protected existing release contract: retain their required content while decoupling renamed component/source plumbing where equivalent proof is available; do not silently retire a guard property.

## Necessary special cases

Pages standard-config directory, reserved namespaces, symlink refusal and route-exclusion refusal; reference-site versus adopted app; single-threaded non-Send bridge; legacy `/todo/:id`; route-owned 404; closed telemetry allowlists and generated hash normalization; async SSR resource allocation order; reviewed-save retry/conflict; honeypot/rate/data caps; best-effort editor focus; external preview readiness and shutdown; authoritative Guard descriptor validation; request clone isolation. These represent actual runtime, compatibility, privacy or ownership requirements.

## Progress

Inventory and redesign completed before source edits. Items 1–10 are implemented. Test cleanup followed the structural work: mapping-only UI assertions were removed; executable shared-dispatch tests and stale-root-link regression fixtures were added. Existing editorial field-guide requirements remain an intentional protected release contract, while runtime composition checks now follow the shared module rather than generated implementation spelling.

Three independent read-only review passes (Rust, tooling, consumers) found no remaining material structural issue after their final corrections. Focused adoption, dispatch, package safety, production derivation, asset fingerprints, pattern-link deletion/rename, architecture, formatting and diff checks pass. Current source SSR check passed during implementation. The final complete local gate passed in `var/untangle/verify-final.log`: 15 Rust tests, both WASM builds, native Workers and Pages request/asset/WebSocket/security/D1/session suites, nested-build Wrangler dry run, and hygiene. Browser read-only hydration was observed on both targets. Shared admission-lock waits were allowed to settle without stopping another task or bypassing admission. Four allowed dependency-audit warnings remain; no new compiler warning remains. Production Pages and provider deployment remain separate, deferred endpoints.

The refactor's exact changed-file list and dependency inventory are retained under ignored `var/untangle/`, alongside the baseline. No application data, migrations or provider configuration was removed.

Final comment/test cleanup removed tutorial/restatement comments, redundant cfg branches and mapping-only UI assertions. Safety/runtime cases now check executable dispatch and packaged asset behavior rather than generated variable spelling. The retained special cases above continue to serve real requirements.
