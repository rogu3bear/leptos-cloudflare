# Source-size inventory

```bash
bun scripts/check-file-limits.mjs
bun scripts/check-file-limits.mjs --report
```

The first command fails for limits or unresolved exceptions. `--report` emits
the same inventory without failing. The check scans tracked/non-ignored files
and explicit root instruction owners. Text is limited to 2,000 physical lines;
instructions to 799. Blank lines and an unterminated final line count.

The report includes paths, byte/line counts, hashes and unresolved conditions.
Ignored contents are listed but not read. Sensitive files, unreadable files and
symlinks remain unverified. Do not claim their compliance from a source-only pass.

`file-limits.json` binds each binary or generated-text exception to its exact
path, kind, SHA-256, owner, reason and review condition. Changed hashes invalidate
exceptions. Ignored-boundary approval names exact collapsed paths; new boundaries
remain unresolved. Additional instruction-purpose paths use `instructionPaths`.
Do not hand-edit generated dependency resolution merely to meet a line cap.

This inventory is a separate manual command. Its regression tests run through
`./scripts/verify.sh`; that does not make the inventory itself automatic.
