# Optional cfctl control

Use this lane when the account owner selected cfctl as the control plane.
Independent adopters can use the standalone profile in [credentials](credentials.md).
No cfctl installation is needed to build, test, rename or preview this starter.

cfctl provides capability discovery, account/profile selection, workspace impact,
hash-bound plans, approval, execution and verification receipts. Installing it
grants neither credentials nor permission to change infrastructure.

## Discover the installed contract

For authorized operator work, start with installed guidance rather than assuming
an old command example matches the installed version:

```bash
cfctl version --json
cfctl doctor --json
cfctl agents doctor --json
cfctl resolve "deploy this Cloudflare Worker" --json
# Use the returned capability ID:
cfctl catalog show <capability-id> --json
cfctl guide <capability-id> --json
```

Discovery describes capabilities; it proves no provider read or deployment.
If the installed version lacks these entry points, use `cfctl --help` and its
versioned guide. Report the missing capability and retain the hold. Do not switch
control planes, upgrade the controller or change policy to make an operation pass.

## Bind, plan, approve, verify

1. Select the exact profile, account, workspace and target. Follow the credential
   owner's protected-input procedure; never put tokens in arguments or logs.
   Register only authorized checkout roots, not unrelated workspaces.
2. Read current resource identity and schema through bounded capabilities.
   Derive ignored production configuration from verified identity.
3. Read the capability schema and guide. Prepare its plan and inspect targets,
   source/artifact identity, cost, workspace effects, permissions, verification
   and recovery. Plan creation is not execution approval.
4. Obtain approval for the exact operation and hash. Use the returned execution
   commands, preserve the operation ID and reconcile partial effects before retry.
5. Read the result envelope and independent provider readback. Exercise deployed
   workflows before reporting live acceptance. A completed plan or local build
   proves neither deployed bytes nor correct application behavior.

An unavailable capability or denied operation stays held until its owner repairs
it. Keep doing local work. Never bypass the hold with raw API, Cloudflare MCP,
a dashboard or direct Wrangler writes unless the governing owner granted an
applicable target-specific exception.

## Repository contracts remain in force

- Workers: keep `wrangler.toml` placeholders, derive `wrangler.production.toml`,
  retain `build/_worker.js` and `target/site`, run the full local gate.
- D1: inspect `.cfctl/operations/d1-migrations.toml`. Adoption changes the operation
  ID; use the actual pack's ordered migrations, hashes and assertions. Never
  substitute unreviewed SQL or delete migration history.
- Pages: follow [production preparation](pages-production.md), distinct
  production/preview D1 identities and fail-closed settings. Resolve installed
  Pages capabilities; Workers support does not establish Pages support.
- Release: follow [RELEASE.md](../RELEASE.md). Credentials, backup, migration,
  deployment, readback and live acceptance retain their own owners.

Use the installed catalog for current CLI inputs, capability IDs and exact
execution commands. Workers deployment must bind the selected profile, account,
Worker name, derived configuration and qualified source/artifact identity. Read
the deployment capability’s required message binding rather than inventing one.
Repository examples and local manifests grant no permission to execute.
