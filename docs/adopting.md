# Adopting the starter

The field guide is a working reference application. Adopt its identity first,
then replace one application surface while preserving the runtime contracts.
Local steps require no Cloudflare credential. The adoption tools use Bun
TOML parsing; the verified toolchain uses Bun 1.3.14.

## Name a fresh clone

```bash
git clone https://github.com/rogu3bear/leptos-cloudflare.git my-app
cd my-app
./scripts/init.sh my-app
```

The command changes the root Cargo package and lock entry, Leptos output name,
Worker name, D1 name, and repository migration-operation identity together.
It preserves every source module and migration byte. Names contain 1–60 lower
case letters, digits, or interior hyphens and begin with a letter. Repeating
the current name changes nothing. Invalid input, customized identity, a bound
D1 UUID, or an existing production config is rejected before source writes.
Ordinary write failures restore already-written identity files; interruption by
process termination is not a multi-file filesystem transaction, so inspect Git
and reconcile identity before retrying an interrupted invocation.

A new name also sets `package.metadata.leptos-cf.reference-site = false`.
This removes only field-guide wording assertions. SSR/hydration, Worker/Assets,
CSP, caching, 404 recovery, request guards, dependency audit, and build checks
remain in `./scripts/verify.sh`. Set it true only when maintaining this reference
site itself. Application-specific acceptance belongs to your application.

## Replace a page and exercise a mutation

1. Change the copy and layout in `src/components/home_page.rs`, retaining useful
   initial HTML with a heading and a `main` landmark from `AppLayout`. Keep the
   layout inside `Router`. Update title, description, icons, and manifest for
   your product; they are application identity, not provisioning identity.
2. Keep `/lab` while validating the first cutover. Apply the existing migrations
   locally with the adopted D1 name:

   ```bash
   CI=1 bunx wrangler@4.120.1 d1 migrations apply my-app-db --local
   bash ./scripts/build-edge.sh
   bunx wrangler@4.120.1 dev --local --ip 127.0.0.1 --port 57581
   ```

3. Create a record in `/lab`, reload and inspect it, toggle it, then delete it.
   Use a second browser session to check isolation. Contact remains stored
   intake only; adoption does not introduce delivery, authentication, or billing.
4. Run `./scripts/verify.sh`. Its Worker test uses the application's home page
   and output-name rather than requiring the field-guide architecture page or
   branding. Add direct tests for the domain behavior you change.
5. Replace one sample workflow with your own typed command/query and UI. Remove
   old routes, API functions, components, and migration assertions only after
   their consumers are replaced. Keep deployed migrations append-only; never
   delete deployed data because a demo disappeared from navigation.

The route tree is the sole page registry. Its not-found component sets the
404 status through Leptos response context, so adding a route does not require
a second Worker allowlist. Keep this response behavior when replacing recovery.
Run `bun scripts/test-adopted-runtime.mjs` for the heavier disposable renamed-app
exercise; it adds a new page, builds both WASM sides, and tests that page plus
unknown-route recovery. It reuses the local target sequentially and then must be
followed by the canonical build before release.

## Production cutover

Choose the context in [Credential profiles](credentials.md). In a governed
workspace, register the renamed repository and use the operation ID actually
recorded in `.cfctl/operations/d1-migrations.toml`. Obtain provider identity from
readback, derive `wrangler.production.toml`, and prepare/approve/run/verify the
migration and deployment separately. An existing provider-bound application
needs a reviewed migration plan; the local naming command refuses to rename it.

Independent public adopters may use the documented portable Wrangler lane.
Local proof, publication, deployment, and observed application behavior are
separate completion claims.
