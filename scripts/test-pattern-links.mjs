#!/usr/bin/env bun
import { strict as assert } from "node:assert";
import { mkdir, mkdtemp, rename, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const root = await mkdtemp(join(tmpdir(), "pattern-links-"));
const verifier = resolve("scripts/verify-patterns.mjs");
const run = () => Bun.spawnSync([process.execPath, verifier], { cwd: root, stdout: "pipe", stderr: "pipe" });
try {
  for (const directory of ["patterns/sample", "scripts", "src"]) await mkdir(join(root, directory), { recursive: true });
  for (const file of ["STRATEGY.md", "Cargo.toml", "wrangler.toml", "scripts/verify-patterns.mjs", "scripts/verify.sh", "src/owned.rs"]) await writeFile(join(root, file), "fixture");
  await writeFile(join(root, "patterns/README.md"), "second layer core template\n## Verification Contract\n`scripts/verify-patterns.mjs` `./scripts/verify.sh`\n[Sample](./sample/) `STRATEGY.md` `Cargo.toml` `wrangler.toml`\n");
  await writeFile(join(root, "patterns/sample/README.md"), "Problem\n`src/owned.rs`\n");
  assert.equal(run().exitCode, 0);
  await rename(join(root, "STRATEGY.md"), join(root, "STRATEGY-renamed.md"));
  let result = run();
  assert.notEqual(result.exitCode, 0);
  assert.match(new TextDecoder().decode(result.stderr), /stale inline path: STRATEGY.md/);
  await rename(join(root, "STRATEGY-renamed.md"), join(root, "STRATEGY.md"));
  for (const name of ["Cargo.toml", "wrangler.toml"]) {
    await rename(join(root, name), join(root, `${name}.renamed`));
    const missingConfig = run();
    assert.notEqual(missingConfig.exitCode, 0);
    assert.ok(new TextDecoder().decode(missingConfig.stderr).includes(`stale inline path: ${name}`));
    await rename(join(root, `${name}.renamed`), join(root, name));
  }
  await rename(join(root, "src"), join(root, "source"));
  result = run();
  assert.notEqual(result.exitCode, 0);
  assert.match(new TextDecoder().decode(result.stderr), /stale inline path: src\/owned.rs/);
  console.log("[pattern-links] deleted root documents and renamed source directories remain checked");
} finally {
  await rm(root, { recursive: true, force: true });
}
