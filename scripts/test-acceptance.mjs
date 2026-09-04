#!/usr/bin/env bun

import { strict as assert } from "node:assert";
import { mkdtemp, mkdir, readFile, writeFile, rm, cp, readdir, symlink } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { adoptProject } from "./init-project.mjs";

const root = process.cwd();
const temporary = await mkdtemp(join(tmpdir(), "leptos-cf-acceptance-"));
const bun = process.execPath;
function run(command, args, options = {}) {
  return Bun.spawnSync([command, ...args], { cwd: root, stdout: "pipe", stderr: "pipe", ...options });
}
function success(result) {
  assert.equal(result.exitCode, 0, new TextDecoder().decode(result.stderr));
}
try {
  const fixture = join(temporary, "app");
  await mkdir(fixture);
  for (const path of ["Cargo.toml", "Cargo.lock", "wrangler.toml", ".cfctl", "src", "scripts", "docs", "migrations"]) {
    await cp(join(root, path), join(fixture, path), { recursive: true });
  }
  const identityFiles = ["Cargo.toml", "Cargo.lock", "wrangler.toml", ".cfctl/operations/d1-migrations.toml"];
  const snapshot = () => Promise.all(identityFiles.map((path) => readFile(join(fixture, path), "utf8")));
  const original = await snapshot();
  const oldName = Bun.TOML.parse(original[0]).package.name;
  const referencePage = join(fixture, "src/components/architecture_page.rs");
  const referenceBefore = await readFile(referencePage, "utf8");
  if (Bun.TOML.parse(original[0]).package.metadata["leptos-cf"]["reference-site"]) {
    await writeFile(referencePage, "// required reference explanation removed\n");
    assert.notEqual(run(bun, ["scripts/verify-architecture-contract.mjs"], { cwd: fixture }).exitCode, 0);
    await writeFile(referencePage, referenceBefore);
  }
  assert.equal((await adoptProject(fixture, oldName)).changed, false);
  await writeFile(join(fixture, "wrangler.toml"), original[2].replace(`name = "${oldName}"`, 'name = "incoherent-worker"'));
  const incoherent = await snapshot();
  await assert.rejects(adoptProject(fixture, "sample-app"), /already customized/);
  assert.deepEqual(await snapshot(), incoherent);
  await writeFile(join(fixture, "wrangler.toml"), original[2]);
  assert.deepEqual(await snapshot(), original);
  for (const name of ["", "Bad Name", "../outside", "trailing-", "a".repeat(61)]) {
    await assert.rejects(adoptProject(fixture, name));
    assert.deepEqual(await snapshot(), original, `invalid ${name} changed source`);
  }
  await writeFile(join(fixture, "wrangler.production.toml"), "preserve-existing-provider-binding\n");
  await assert.rejects(adoptProject(fixture, "sample-app"), /production config exists/);
  assert.deepEqual(await snapshot(), original);
  await rm(join(fixture, "wrangler.production.toml"));
  await writeFile(join(fixture, "wrangler.toml"), original[2].replaceAll("00000000-0000-0000-0000-000000000000", "12345678-1234-4abc-9def-1234567890ab"));
  const bound = await snapshot();
  await assert.rejects(adoptProject(fixture, "sample-app"), /bound provider identity/);
  assert.deepEqual(await snapshot(), bound);
  await writeFile(join(fixture, "wrangler.toml"), original[2]);
  await writeFile(join(fixture, "wrangler.toml"), 'account_id = "operator-account"\n' + original[2]);
  const routed = await snapshot();
  await assert.rejects(adoptProject(fixture, "sample-app"), /bound provider routing/);
  assert.deepEqual(await snapshot(), routed);
  await writeFile(join(fixture, "wrangler.toml"), original[2]);
  const collision = join(fixture, `Cargo.lock.adopt-${process.pid}.tmp`);
  await writeFile(collision, "existing-file-must-survive");
  await assert.rejects(adoptProject(fixture, "sample-app"), /EEXIST/);
  assert.deepEqual(await snapshot(), original);
  assert.equal(await readFile(collision, "utf8"), "existing-file-must-survive");
  await rm(collision);
  const homeBefore = await readFile(join(fixture, "src/components/home_page.rs"));
  const migrationsBefore = await Promise.all((await readdir(join(fixture, "migrations"))).map(async (name) => [name, await readFile(join(fixture, "migrations", name), "utf8")]));
  const adopted = await adoptProject(fixture, "sample-app");
  assert.equal(adopted.changed, true);
  assert.deepEqual(await readFile(join(fixture, "src/components/home_page.rs")), homeBefore);
  for (const [name, bytes] of migrationsBefore) assert.equal(await readFile(join(fixture, "migrations", name), "utf8"), bytes);
  const after = await snapshot();
  assert.equal(Bun.TOML.parse(after[0]).package.name, "sample-app");
  assert.equal(Bun.TOML.parse(after[0]).package.metadata.leptos["output-name"], "sample-app");
  assert.equal(Bun.TOML.parse(after[0]).package.metadata["leptos-cf"]["reference-site"], false);
  assert.ok(Bun.TOML.parse(after[1]).package.some((pkg) => pkg.name === "sample-app"));
  assert.equal(Bun.TOML.parse(after[2]).name, "sample-app");
  assert.equal(Bun.TOML.parse(after[2]).d1_databases[0].database_name, "sample-app-db");
  assert.equal(Bun.TOML.parse(after[3]).operation[0].id, "sample-app.d1-migrations-apply");
  // An adopter can replace public copy and remove the reference architecture page
  // without changing or disabling the runtime contract checks.
  await writeFile(join(fixture, "src/components/architecture_page.rs"), "// replaced application page\n");
  success(run(bun, ["scripts/verify-architecture-contract.mjs"], { cwd: fixture }));
  success(run(bun, ["scripts/test-production-config.mjs"], { cwd: fixture }));
  await writeFile(join(fixture, "wrangler.toml"), after[2].replace('main = "build/_worker.js"', 'main = "wrong.js"'));
  assert.notEqual(run(bun, ["scripts/verify-architecture-contract.mjs"], { cwd: fixture }).exitCode, 0);
  console.log("[acceptance] adoption, unchanged invalid/bound inputs, migration preservation, and application-independent runtime contract passed");

  await writeFile(join(fixture, "wrangler.toml"), after[2]);
  const migrationHashes = Bun.TOML.parse(after[3]).operation[0].migration;
  await adoptProject(fixture, "a");
  await adoptProject(fixture, "another-app");
  assert.deepEqual(Bun.TOML.parse((await snapshot())[3]).operation[0].migration, migrationHashes);
  assert.equal(Bun.TOML.parse((await snapshot())[3]).operation[0].title, "Apply the another-app D1 migrations");
  success(run("/bin/bash", ["scripts/init.sh", "cli-app"], { cwd: fixture }));
  assert.equal(Bun.TOML.parse((await snapshot())[0]).package.name, "cli-app");

  const commentedCargo = (await snapshot())[0].replace("reference-site = false", "reference-site = true # public template");
  await writeFile(join(fixture, "Cargo.toml"), commentedCargo);
  await adoptProject(fixture, "comment-app");
  assert.equal(Bun.TOML.parse((await snapshot())[0]).package.metadata["leptos-cf"]["reference-site"], false);
  assert.ok((await snapshot())[0].includes("reference-site = false # public template"));

  const bin = join(temporary, "bin");
  await mkdir(bin);
  await symlink("/bin/bash", join(bin, "bash"));
  await symlink("/usr/bin/dirname", join(bin, "dirname"));
  const fullGate = run("/bin/bash", [join(root, "scripts/verify.sh")], { env: { PATH: bin } });
  assert.notEqual(fullGate.exitCode, 0);
  assert.match(new TextDecoder().decode(fullGate.stderr), /Required cargo-audit is missing/);
  assert.doesNotMatch(new TextDecoder().decode(fullGate.stdout), /All release readiness checks passed/);
  const audit = join(root, "scripts/security-audit.sh");
  let result = run("/bin/bash", [audit], { env: { PATH: bin } });
  assert.notEqual(result.exitCode, 0);
  assert.match(new TextDecoder().decode(result.stderr), /Required cargo-audit is missing/);
  await writeFile(join(bin, "cargo-audit"), "#!/bin/sh\nexit 0\n", { mode: 0o755 });
  await writeFile(join(bin, "cargo"), '#!/bin/sh\n[ "$1" = "audit" ] || exit 98\nexit 0\n', { mode: 0o755 });
  success(run("/bin/bash", [audit], { env: { PATH: bin } }));
  await writeFile(join(bin, "cargo"), "#!/bin/sh\nexit 7\n", { mode: 0o755 });
  assert.equal(run("/bin/bash", [audit], { env: { PATH: bin } }).exitCode, 7);
  console.log("[acceptance] required security audit missing, success, and failure outcomes passed");
} finally {
  await rm(temporary, { recursive: true, force: true });
}
