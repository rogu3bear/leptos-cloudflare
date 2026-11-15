#!/usr/bin/env bun
import { strict as assert } from "node:assert";
import { cp, mkdir, mkdtemp, readFile, readdir, rm, stat, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const root = process.cwd();
const fixture = await mkdtemp(join(tmpdir(), "leptos-pages-package-"));
const run = (output) => Bun.spawnSync([process.execPath, resolve(root, "scripts/package-pages.mjs"), output], { cwd: fixture, stdout: "pipe", stderr: "pipe" });
try {
  await mkdir(join(fixture, "build"));
  await mkdir(join(fixture, "target/site/pkg"), { recursive: true });
  await cp(join(root, "Cargo.toml"), join(fixture, "Cargo.toml"));
  await writeFile(join(fixture, "build/index.js"), 'import wasm from "./index_bg.wasm"; export default wasm;');
  await writeFile(join(fixture, "build/index_bg.wasm"), "wasm fixture");
  await writeFile(join(fixture, "target/site/pkg/app.js"), "asset fixture");
  await writeFile(join(fixture, "target/site/logo.svg"), "adopter logo");
  const shim = Bun.spawnSync([process.execPath, join(root, "scripts/write-worker-shim.mjs")], { cwd: fixture, stderr: "pipe" });
  assert.equal(shim.exitCode, 0, new TextDecoder().decode(shim.stderr));
  const output = join(fixture, "target/output");
  let result = run(output);
  assert.equal(result.exitCode, 0, new TextDecoder().decode(result.stderr));
  assert.equal(await readFile(join(output, "_server/index_bg.wasm"), "utf8"), "wasm fixture");
  assert.equal(await readFile(join(output, "pkg/app.js"), "utf8"), "asset fixture");
  assert.equal(await readFile(join(output, "logo.svg"), "utf8"), "adopter logo");
  result = run(output);
  assert.notEqual(result.exitCode, 0);
  assert.equal(await readFile(join(output, "pkg/app.js"), "utf8"), "asset fixture");
  await writeFile(join(fixture, "target/site/_routes.json"), '{"version":1,"include":["/*"],"exclude":["/_server/*"]}');
  result = run(join(fixture, "target/routes-output"));
  assert.notEqual(result.exitCode, 0, "route exclusions must not bypass the Pages module guard");
  assert.match(new TextDecoder().decode(result.stderr), /reserved _routes\.json/);
  await rm(join(fixture, "target/site/_routes.json"));
  for (const name of ["_worker.js", "_server"]) {
    await writeFile(join(fixture, "target/site", name), "unowned bytes");
    result = run(join(fixture, `target/reserved-${name}`));
    assert.notEqual(result.exitCode, 0, `reserved ${name} must be rejected`);
    assert.equal(await readFile(join(fixture, "target/site", name), "utf8"), "unowned bytes");
    await rm(join(fixture, "target/site", name));
  }
  await symlink(join(fixture, "target/site"), join(fixture, "target/site-alias"));
  result = run(join(fixture, "target/site-alias/new-output"));
  assert.notEqual(result.exitCode, 0, "output symlink ancestor must not point into assets");
  await assert.rejects(stat(join(fixture, "target/site/new-output")), { code: "ENOENT" });
  await mkdir(join(fixture, "target/site/empty"));
  await symlink(join(fixture, "target/site/empty"), join(fixture, "target/output-link"));
  result = run(join(fixture, "target/output-link"));
  assert.notEqual(result.exitCode, 0, "an empty destination symlink into assets must be rejected");
  assert.deepEqual(await readdir(join(fixture, "target/site/empty")), []);
  await mkdir(join(fixture, "protected"));
  await symlink(join(fixture, "protected"), join(fixture, "target/protected-alias"));
  result = run(join(fixture, "target/protected-alias/new-output"));
  assert.notEqual(result.exitCode, 0, "output symlink ancestor must not point into source");
  await assert.rejects(stat(join(fixture, "protected/new-output")), { code: "ENOENT" });
  await symlink(join(fixture, "target/site/pkg/app.js"), join(fixture, "target/site/alias.js"));
  result = run(join(fixture, "target/symlink-source-output"));
  assert.notEqual(result.exitCode, 0, "asset symlinks must be rejected");
  assert.equal(await readFile(join(fixture, "target/site/pkg/app.js"), "utf8"), "asset fixture");
  console.log("[pages-package] asset routing, reserved names, symlink and output safety checks passed");
} finally {
  await rm(fixture, { recursive: true, force: true });
}
