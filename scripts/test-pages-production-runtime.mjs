#!/usr/bin/env bun
import { strict as assert } from "node:assert";
import { readFile, writeFile, rm, stat } from "node:fs/promises";
import { join } from "node:path";
import { randomUUID } from "node:crypto";
import { fixtureIdentity } from "./test-pages-production.mjs";
import { preparePagesRelease, readBundleMetadata } from "./prepare-pages-release.mjs";
import { verifyReleaseFiles } from "./verify-pages-release.mjs";
import { withLocalPreview, fetchBoundary } from "./local-preview.mjs";
import { testResourceExplorer } from "./test-resource-explorer.mjs";
import { testContactIntake } from "./test-contact-intake.mjs";

const root = process.cwd();
const output = join(root, "target", `pages-release-test-${randomUUID()}`);
let owned = false;
try {
  const manifest = await preparePagesRelease(root, output, fixtureIdentity);
  owned = true;
  assert.deepEqual(await verifyReleaseFiles(output), manifest);
  assert.equal((await stat(join(output, "wrangler.toml"))).mode & 0o777, 0o600);
  assert.equal((await stat(output)).mode & 0o777, 0o700);
  await assert.rejects(preparePagesRelease(root, output, fixtureIdentity), { code: "EEXIST" });
  const configPath = join(output, "wrangler.toml");
  const config = await readFile(configPath, "utf8");
  await writeFile(configPath, config + "\n# changed after qualification\n");
  await assert.rejects(verifyReleaseFiles(output), /files changed/);
  await writeFile(configPath, config);
  for (const environment of ["production", "preview"]) {
    const metadata = await readBundleMetadata(join(output, `${environment}.bundle`));
    assert.equal(metadata.bindings.find((binding) => binding.name === "DB").id, fixtureIdentity[`${environment}Database`].id);
  }
  await withLocalPreview({ root, pages: true, migrate: true, pagesReleaseDirectory: output }, async (origin, store) => {
    const response = await fetchBoundary(`${origin}/architecture`);
    assert.equal(response.status, 200);
    assert.match(await response.text(), /SSR \+ hydration/);
    assert.match(response.headers.get("content-security-policy"), /nonce-/);
    assert.equal((await fetchBoundary(`${origin}/_server/index_bg.wasm`)).status, 404);
    for (const path of ["/functions/index_bg.wasm", "/pkg/%2f..%2f_server/index_bg.wasm", "/pkg/%2f..%2ffunctions/index_bg.wasm"]) assert.equal((await fetchBoundary(`${origin}${path}`)).status, 404);
    await testResourceExplorer(origin);
    await testContactIntake(origin, store);
  });
  assert.deepEqual(await verifyReleaseFiles(output), manifest, "local qualification must not change release files");
  console.log("[pages-production] isolated standard config, both native upload bundles, local migrations, SSR/CSP, private modules and persisted workflows passed; no provider operation executed");
} finally {
  if (owned) await rm(output, { recursive: true, force: true });
}
