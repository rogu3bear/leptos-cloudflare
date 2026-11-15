# Repository cleanup and file limits

The local size check is:

```bash
bun scripts/check-file-limits.mjs
bun scripts/check-file-limits.mjs --report
```

The first command fails for limits or unresolved exceptions. The second emits the same inventory without failing. Both scan every tracked and non-ignored working file, plus the local AGENTS, ANCHOR and NORTH_STAR instruction owners when present. Text is limited to 2,000 physical lines; instruction files to 799. Blank lines and a final unterminated line count. The report includes path, kind, byte/line counts, status and bounded exception evidence. Symlinks, unreadable files and sensitive environment inputs are unverified; the check does not follow/open them. Ignored trees are listed without reading their contents.

The check is a manual source-size command. Its regression tests run through the existing local verification gate; that does not mean the size command runs automatically.

## Exception contract

No file is exempt merely because it is generated, a lockfile or binary. Optional `file-limits.json` entries must identify exact paths, kind (`binary` or `generated-text`), SHA-256, owner, reason and review condition. A changed file hash invalidates its exception. An ignored-boundary decision must name its exact collapsed paths, owner, reason and review condition; newly ignored paths remain unresolved. Extra instruction-purpose paths can be listed in `instructionPaths`.

At the start of this pass, the 145 tracked/non-ignored files contained one over-cap file: Cargo.lock, 2,551 lines. Four PNG files had no meaningful text-line count. No source, lockfile, image or ignored output was deleted or hand-edited to make the count pass. The generated lock belongs to Cargo; changing its dependency resolution only to reduce lines would alter application behavior and integrity.

Applicable instruction counts measured separately were 179 lines for the host AGENTS, 61 for the enclosing dev AGENTS, 97 for this checkout's AGENTS, 21 for ANCHOR and 12 for NORTH_STAR. These all fit the instruction cap. Ignored dependency, build/runtime, private agent and environment contents were not inspected; they are not represented as universally compliant.

## Bounded corrections

- The public start page used a second Wrangler launch route and incorrectly said local adoption was unverified. It now uses the qualified local launcher and distinguishes local naming from provider initialization.
- The task detail page claimed independent metadata loading despite using the already loaded record. The copy now describes its actual source.
- The agent playbook referred to removed database helpers and repeated a transport bridge at each caller. It now points to the domain type owners, request-scoped D1 helper and shared Send bridge.
- The new check has boundary fixtures for line counts, instruction purpose, binary decisions, hash invalidation, tracked/untracked population, ignored privacy, symlinks and ignored-boundary expansion.

Retained intentionally: the prior qualified refactor, historical planning/decision documents, compatibility route, explicit telemetry/privacy rules, runtime cancellation and descriptor repairs, placeholder provider config and deferred Pages production path. No missing text reference was treated as proof of dead code.

The operator approved all three bounded exceptions on 2026-11-15: Cargo-generated dependency resolution, four PNG assets, and the named ignored boundaries. The active `file-limits.json` pins file hashes and review conditions. Ignored contents remain unread and unverified; the approval does not establish their compliance. Raw inventories and the pre-edit byte hashes are under ignored `var/de-slop/`; `accepted-inventory.json` records the passing source-cap check with these exceptions.
