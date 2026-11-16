# Credential profiles

A local clone, build, local D1 database, and Wrangler local server do not need
provider credentials. Select the production operating context explicitly.

Choose the standalone section for an independently operated account. Choose the
[cfctl control guide](cfctl.md) when the account owner selected that lane;
installation alone selects neither an account nor a control policy.

## Governed cfctl account

cfctl owns credential storage and lifecycle as well as Cloudflare live reads,
change preparation, approval, execution, and post-change verification. Check
current `cfctl doctor`, `cfctl auth --help`, and `cfctl keys --help` before a
release. Use the installed catalog guide
for the intended operation. Stop on an unavailable capability rather than using
a raw API or direct Wrangler provider write.

Obtain a purpose-scoped account child just before the approved release window.
The release needs Workers Scripts Write/Read and D1 Write/Read; include account
introspection only when the selected provider operation requires it. Review the
exact permission groups and targets in the control-plane plan. The account
minter credential never enters this repository, a deployment process, or CI.
The child expires within one day as a fail-safe and is revoked after its final
verified consumer. These bounds describe the operator lane, not a grant of
standing authority or permission to mint.

Import a child through protected input, for example:

```bash
cfctl auth import-api-token --account <account-id> --value-in <mode-0600-file>
```

Use the selected profile explicitly in provider calls. Do not print tokens or
put values in command arguments, logs, plans, or tracked files. Follow cfctl's
exact plan/approval/run/status lifecycle for credential mutations. There is no
repository rotation command and no dependency on `../scripts/cf-rotate.sh`.
Existing operator scheduling must be retired or cut over by its owner before
calling credential automation migrated; removal here does not prove that.

## Independent public adopter: standalone Wrangler

An independently operated clone may use Cloudflare's scoped API-token setup
and the pinned Wrangler commands in README and the edge-deployment guide.
No cfctl installation, maintainer-home files or shared scripts are required.
Keep the token account-scoped, least-privileged, and limited to your chosen
release window. Do not copy the token-minter credential into the project.
Supply `CLOUDFLARE_API_TOKEN` using your platform's protected secret injection
or a gitignored, mode-0600 local environment file. Never paste its value into
a tracked configuration or command line.

Read the target identity before provisioning, derive ignored production config,
apply migrations, deploy, and verify the actual result. A local build is not
provider evidence. Rotation belongs to the standalone operator's chosen secret
manager; this starter does not ship a second account controller.

This standalone profile is not a fallback inside an already governed account.
