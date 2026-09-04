---
artifact: acceptance-criteria
version: "1.0"
created: 2026-09-04
status: verified-local
---

# Acceptance Criteria: starter adoption and truthful release verification

## Story Context

A developer can adopt a different project name and replace a page without
breaking routing, infrastructure identity, or mandatory runtime checks. An
operator can distinguish complete release proof from an unavailable check and
choose the proper credential context. This is the bounded source repair;
publication and provider deployment require their separate receipts. Historical
website criteria remain in `reference-site-acceptance.md`.

## Happy Path

### AC-1: Adopt a new local identity

**Given** a provider-neutral clone with coherent project identity

**When** the developer adopts a valid different name

**Then** its package, generated asset identity, Worker, D1, and migration operation
use the new name while all application code and migration contents remain intact.

### AC-2: Replace and extend application pages

**Given** an adopted application with different page content and an added route

**When** its release-shaped local Worker receives requests for the new page and
an unknown document

**Then** the new page returns useful SSR HTML with status 200 and the unknown
page returns useful recovery HTML with status 404.

### AC-3: Complete required verification

**Given** all mandatory release tools are installed and all checks succeed

**When** the operator runs the full local release verification

**Then** it reports success only after every required stage has completed.

## Edge Cases

### AC-4: Repeat an adoption without changing its name

**Given** a provider-neutral project already using the requested name

**When** adoption is repeated

**Then** all identity files remain byte-for-byte unchanged.

### AC-5: Preserve domain history through repeated renaming

**Given** an adopted project with existing migration contents and hashes

**When** it is renamed again, including a one-letter project name

**Then** migration contents and hashes remain unchanged.

## Error States

### AC-6: Reject unsafe adoption inputs before alteration

**Given** an invalid name, incoherent identity, provider binding, existing
production configuration, or a conflicting temporary file

**When** adoption is requested

**Then** it fails with an actionable reason and preserves existing source and
conflicting files without claiming readiness.

### AC-7: Refuse incomplete or failing security verification

**Given** the mandatory security-audit tool is absent or reports failure

**When** release verification reaches that requirement

**Then** verification exits unsuccessfully, identifies the missing prerequisite
or failure, and never reports that all release checks passed.

## Non-Functional Criteria

### AC-8: Preserve runtime controls after adoption

**Given** an application with a different identity and public page content

**When** its runtime boundary checks execute

**Then** SSR, CSP nonce agreement, asset separation and immutable caching,
missing-route recovery, server-function rejection, and the realtime upgrade
boundary remain enforced independently of field-guide wording.

### AC-9: Keep reference application checks explicit

**Given** this repository is configured as the public reference site

**When** a required reference architecture explanation is removed

**Then** reference verification fails while application adopters continue to be
checked against the shared runtime contract.

### AC-10: Use a portable, explicit credential context

**Given** a local-only clone, an independently operated standalone deployment,
or this operator's governed workspace

**When** the operator follows its documented credential profile

**Then** local work requires no provider credential, standalone setup requires
no operator-home script, and governed account operations remain with cfctl
without falling back to an external raw-API rotator.

### AC-11: Recover from a failed detail deletion

**Given** a loaded task detail and a delete request that fails

**When** the failure reaches the browser

**Then** the detail stays visible, an error is announced, retry remains available,
and neither deletion success nor navigation to the list is reported.

### AC-12: Recover from a failed detail toggle

**Given** a loaded task detail and a toggle request that fails

**When** the failure reaches the browser

**Then** the prior canonical completion state is restored, an error is announced,
and the user can retry without losing the detail.

### AC-13: Fingerprint the final asset bytes

**Given** a build where only the hydration WASM bytes change

**When** assets are fingerprinted and verified

**Then** both the WASM URL and its importing JavaScript URL change, every manifest
hash matches its final file bytes, and unchanged CSS retains its URL.

### AC-14: Classify generated functions without exposing paths

**Given** a known server function with its generated decimal suffix or an unknown path

**When** request telemetry is emitted

**Then** known functions retain their closed function labels and unknown paths
remain `unknown` without exposing raw paths or suffixes.

### AC-15: Preload hydration assets with matching credentials

**Given** the final local Worker build

**When** a browser hydrates its document

**Then** preload and fetch credential modes agree, the application becomes
interactive, and no preload credential mismatch warning appears.

### AC-16: Preserve unowned production-configuration files

**Given** a production-config temporary path already exists

**When** local production derivation attempts exclusive creation

**Then** it fails without deleting the existing temporary file or changing the
existing production config; successful derivation retains mode 0600.

## Notes

- The account, deployed Worker/D1 identity, production runtime, publication, and
  user adoption outcomes are not established by local verification.
- The minter credential never enters the repository. Current authentication
  health must be observed; historical Keychain notes have no operational force.
- Ordinary write errors are rolled back. Abrupt process or machine termination
  is not an atomic multi-file filesystem transaction; inspect Git and reconcile
  identity before retrying after such an interruption.
- No external adoption, staffing, timing, or market result is claimed.

## Direct evidence

| Criteria | Verification | Status |
|---|---|---|
| AC-1, AC-4, AC-5, AC-6 | `bun scripts/test-acceptance.mjs`: identity, byte preservation, repeated names, invalid/bound input and collision cases | PASS |
| AC-2, AC-8 | `bun scripts/test-adopted-runtime.mjs`: renamed build, replaced page, added route, missing route and shared runtime assertions | PASS (final 0.1.3 source) |
| AC-3 | `./scripts/verify.sh` against the canonical candidate | PASS (all 11 stages on final 0.1.3 source) |
| AC-7 | Direct missing/success/failing tool cases in `scripts/test-acceptance.mjs`, plus full gate missing-tool refusal | PASS |
| AC-9 | Reference architecture check and deliberately damaged reference fixture | PASS |
| AC-16 | Production-config exclusive-temp collision and successful mode check | PASS |
| AC-13 | Fingerprint regression including WASM-only changes and final manifest verification | PASS |
| AC-14 | Rust generated-suffix and malformed-path regression tests | PASS (15 Rust tests total) |
| AC-15 | HTTP preload attributes and final browser console | PASS (no unexpected warnings) |
| AC-11, AC-12 | Real-browser failed mutation and recovery checks | PASS (final 0.1.3 build) |
| AC-10 | Source review of `docs/credentials.md`, README, local AGENTS; removed rotation wrapper and external scope file | PASS (source/documentation only) |

Local raw logs are written under ignored `var/acceptance/`. The final handoff
binds the candidate SHA and observed results. Provider and publication receipts
must be added by their owning release task; they are not inferred here.

Final local proof on 2026-09-04 used `var/acceptance/adopted-runtime-v013.log`
and `var/acceptance/verify-v013.log`. The independent browser run covered all
eight checks: hydration/create, reload persistence, session isolation, failed
toggle with retry, persisted toggle, failed delete retaining detail and concise
error, successful retry/delete, and mobile overflow. There were no page errors
or unexpected console warnings; two resource errors were intentionally injected
503 responses. The verifier completed the mandatory dependency audit with the
three warnings documented in `RELEASE.md`.

The pre-gate source manifest SHA-256 was
`1ef44820f1b1ab3c3123b6b4de5feba8a21bcee5c75e2e1b1131596f6ff960f2`;
the browser-tested asset manifest SHA-256 was
`43851da171c12816ab4abe778117920978c622b64e1e01e0b2ff33c7415725e3`.
Only this evidence record changed after those checks. The final candidate
handoff binds the commit tree and records that documentation-only difference.
