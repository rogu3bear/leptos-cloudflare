# Pages production source qualification

The Pages production preparation path is qualified locally with the pinned
Wrangler. This is source/build evidence, not a deployed Pages project. Workers
remains the default. Live provider operations require their own owner, exact
plan approval, credentials, recovery evidence and post-change verification.

## Prepare an immutable release directory

Keep `pages/wrangler.toml` placeholder-only. Pages deployment discovers a
standard-named config in its working directory; it does not support an arbitrary
`--config` path. The generator creates a fresh `target/pages-release-NAME/`
containing a mode-0600 `wrangler.toml`, a private `package.json` boundary,
`public/`, copied ordered migrations,
production/preview multipart upload bundles, Wrangler build metadata and
`release.json`. The directory is private and ignored. Existing output is refused,
including partial prior attempts; inspect those and choose another name.

After local adoption, obtain the account/project/branch and both D1 identities
from authenticated provider readback. Production and preview database names and
IDs must differ. Save only identity metadata, never credentials, in an ignored
mode-0600 file such as `var/pages-identity.json` with this shape:

```json
{
  "accountId": "<verified-account-id>",
  "projectName": "my-app",
  "productionBranch": "main",
  "productionDatabase": { "name": "my-app-production-db", "id": "<verified-production-uuid>" },
  "previewDatabase": { "name": "my-app-preview-db", "id": "<verified-preview-uuid>" }
}
```

Commit the intended source, complete `./scripts/verify.sh`, then prepare:

```bash
bun scripts/prepare-pages-release.mjs var/pages-identity.json target/pages-release-candidate
bun scripts/verify-pages-release.mjs target/pages-release-candidate
```

These commands do not contact the provider or execute the recorded migration or
deployment commands. `prepare` checks identity syntax, not its provenance. It
allows dirty source for local qualification and labels it `qualification-only`;
the release verifier and readback validator refuse dirty or unidentified source.
Archive-based adopters can build without Git, but must establish a clean Git
revision before preparing a releasable candidate.

The edge build records its source inputs before compilation and checks them
again afterward. Preparation rejects stale source, changed artifacts, symlinked
inputs/output, or migrations differing from the repository operation pack.
It uses `pages functions build` with the isolated project directory, build
metadata, and the pinned CLI's `PAGES_ENVIRONMENT` selector for each environment.
The multipart bundles are compiler-qualification artifacts: a later `pages deploy public` recompiles the pinned public inputs; it does not upload these `.bundle` files directly. Retain the actual upload receipt rather than treating a qualification bundle hash as the deployed artifact hash.

It runs the pinned CLI's local asset validator (`pages project validate`) for
file-count/size limits, then inspects the resulting upload multipart: main module, server WASM, date and
the exact `DB` binding. Metadata must name `public` and the generated config hash.
This qualifies the upload compiler; it does not simulate a successful deployment.

The default config binds production D1; `env.preview` explicitly binds preview D1.
The local preview ID is the preview database ID. All local migration tests use
`--local` and a disposable persistence directory. Nothing is applied remotely by
the release gate. No real IDs, tokens or release artifacts belong in Git.

## Future authorized provider procedure

In a cfctl-governed account, follow the account owner’s governed
cfctl contract in [credentials](credentials.md). The local manifest is input to
that owner's exact plan, not permission to run Wrangler. If an owning capability
cannot bind the Pages config, migrations or upload, stop at that capability;
source qualification does not certify the installed control plane.

An independently operated public clone may use the standalone Wrangler profile.
Before any mutation, verify the existing project's production branch, bind the
account explicitly, record its current canonical deployment and D1 recovery
bookmarks, and review the manifest, ordered migration hashes and schema assertions.
Before uploading, require `fail_open = false` in both project deployment configs.
Server modules live in the reserved `functions/` directory, excluded from the
pinned uploader's static asset map and bundled into the advanced Worker. The
handler also denies private paths and decoded traversal aliases. Fail-closed
settings preserve request/security checks when the Functions quota is exhausted.
This project setting is provider state, not a supported Wrangler TOML field; apply it
through the owner's reviewed project-settings operation. Unknown or true values
block release. Check an authenticated project snapshot before any upload:

```bash
bun scripts/verify-pages-release.mjs target/pages-release-candidate var/pages-project.json --project-only
```

That snapshot contains `accountId` and the project result, without a deployment.
Recheck release files immediately before using them. For local preview of the
release directory, run the pinned launcher from that directory with `pages dev
public`. The private package boundary keeps Wrangler's temporary dev entrypoint
inside this project, preventing discovery of the parent Workers build. The pinned
Pages dev/deploy commands both reject custom `--config` paths.

From the release directory, the manifest's production migration command uses
`d1 migrations apply <production-db> --remote --config wrangler.toml`. The preview
counterpart adds `--env preview` and targets the separate preview database.
Apply only the approved environment's migrations, verify the migration ledger
and every repository schema assertion, then use the manifest's Pages deployment
arguments. The deployment explicitly pins `--project-name`, `--branch`,
`--commit-hash` and `--commit-dirty=false`, with no `--config` flag. The selected
credential's account must equal `release.json`'s account. Do not let branch
inference select the environment, or auto-create a project as a fallback.
The manifest records `CLOUDFLARE_ACCOUNT_ID` explicitly for the standalone
executor. Project setup must apply the declared preview configuration as well as
production before the strict two-environment readback contract can pass; a
production-only upload does not establish that preview settings were applied.

Configuration can replace dashboard-managed settings. The generator refuses
customized templates; reconcile additional bindings or variables before extending
this contract. Rollback to the recorded prior Pages deployment is a separate
authorized operation and does not roll back D1; retain the D1 recovery bookmark
and obtain the owner's recovery decision before data restoration.

## Readback and acceptance

After an authorized release, independently fetch the exact project and deployment
from the authenticated account. Save a protected local JSON document with
`accountId` (the authenticated request target), `project` (the project result),
and `deployment` (the deployment result), then run:

```bash
bun scripts/verify-pages-release.mjs target/pages-release-candidate var/pages-readback.json
```

It checks fail-closed settings, account/project/production branch, both environment D1 bindings and
config hashes, compatibility date/flags, canonical production deployment identity,
successful deployment stage, clean exact source commit and immutable Pages URL.
It cannot authenticate caller-supplied JSON or prove the uploaded artifact bytes
from commit metadata alone. Keep the trusted preparation/upload receipt with
the authenticated readback. The test suite uses synthetic readback fixtures and
never treats them as provider evidence.

Finally exercise the immutable deployment URL and intended public domain:
SSR HTML, nonce/hydration agreement, static immutable assets, rendered 404,
API method/origin/type/body limits, private-module denial and WebSockets. With
authorized synthetic inputs, exercise session-isolated persisted CRUD, reviewed
save/conflict/retry, and contact intake. Provider state and runtime acceptance
are separate from the local checks and remain unproved until executed.

References: [Pages config and environments](https://developers.cloudflare.com/pages/functions/wrangler-configuration/),
[advanced mode](https://developers.cloudflare.com/pages/functions/advanced-mode/),
[Pages CLI](https://developers.cloudflare.com/workers/wrangler/commands/pages/),
[project readback](https://developers.cloudflare.com/api/resources/pages/subresources/projects/methods/get/),
[deployment readback](https://developers.cloudflare.com/api/resources/pages/subresources/projects/subresources/deployments/methods/get/).

The [Pages fail-open/closed contract](https://developers.cloudflare.com/pages/functions/routing/#fail-open--closed) is a required project safety boundary; local Miniflare does not simulate provider quota exhaustion.

The 0.1.5 admission patch requires migration `0004_anonymous_admission.sql` on each
selected D1 database before using the new code. The owned migration pack binds its
hash and schema assertions. The database-local key is initialized on first write;
per-IP quotas survive cookie changes, NAT users share a budget, and full storage
rejects new writes without eviction. Follow the [admission prerequisite](edge-deployment.md#anonymous-admission-prerequisite)
for trusted-header, privacy and key-rotation limits. No remote migration is implied.
