#!/usr/bin/env bun
import { strict as assert } from "node:assert";
import { mkdtemp, mkdir, readFile, writeFile, rm, symlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { renderPagesConfig, commandPlan } from "./prepare-pages-release.mjs";
import { verifyPagesProjectSafety, verifyPagesReadback } from "./verify-pages-release.mjs";
import { validateBuild } from "./build-provenance.mjs";

export const fixtureIdentity = {
  accountId: "1".repeat(32), projectName: "pages-release-fixture", productionBranch: "main",
  productionDatabase: { name: "pages-production-fixture", id: "12345678-1234-4abc-9def-1234567890ab" },
  previewDatabase: { name: "pages-preview-fixture", id: "23456789-1234-4abc-9def-1234567890ab" },
};

if (import.meta.main) {
  const template = await readFile("pages/wrangler.toml", "utf8");
  const output = renderPagesConfig(template, fixtureIdentity);
  const config = Bun.TOML.parse(output);
  assert.equal(config.pages_build_output_dir, "./public");
  assert.equal(config.d1_databases[0].database_id, fixtureIdentity.productionDatabase.id);
  assert.equal(config.env.preview.d1_databases[0].database_id, fixtureIdentity.previewDatabase.id);
  assert.equal(config.d1_databases[0].preview_database_id, fixtureIdentity.previewDatabase.id);
  assert.equal(renderPagesConfig(template, fixtureIdentity), output);
  for (const identity of [
    { ...fixtureIdentity, projectName: "../escape" },
    { ...fixtureIdentity, accountId: "" },
    { ...fixtureIdentity, productionBranch: "main\ninjected" },
    { ...fixtureIdentity, productionBranch: "main/../other" },
    { ...fixtureIdentity, token: "not-allowed-in-identity-metadata" },
    { ...fixtureIdentity, previewDatabase: fixtureIdentity.productionDatabase },
    { ...fixtureIdentity, productionDatabase: { name: "fixture", id: "00000000-0000-0000-0000-000000000000" } },
  ]) assert.throws(() => renderPagesConfig(template, identity));
  assert.throws(() => renderPagesConfig(template + '\n[vars]\nCUSTOM = "value"\n', fixtureIdentity));
  assert.throws(() => renderPagesConfig(template.replace('binding = "DB"', 'binding = "OTHER"'), fixtureIdentity));
  const manifest = { schema: 1, identity: fixtureIdentity, source: { commit: "a".repeat(40), dirty: false }, compatibilityDate: config.compatibility_date, compatibilityFlags: [], files: [{ path: "wrangler.toml", sha256: "b".repeat(64) }] };
  const plan = commandPlan(fixtureIdentity, manifest.source);
  assert.equal(plan.status, "requires-provider-approval");
  assert.equal(plan.production.deploy.includes("--config"), false, "Pages must discover a standard config");
  assert.equal(plan.production.deploy[plan.production.deploy.indexOf("--branch") + 1], "main");
  assert.ok(plan.production.migrate.includes("--remote"));
  assert.deepEqual(plan.preview.migrate.slice(-2), ["--env", "preview"]);
  assert.equal(commandPlan(fixtureIdentity, { commit: null, dirty: true }).status, "qualification-only");
  const snapshots = {
    accountId: fixtureIdentity.accountId,
    project: { id: "project-fixture", name: fixtureIdentity.projectName, production_branch: "main", subdomain: `${fixtureIdentity.projectName}.pages.dev`, canonical_deployment: { id: "deployment-fixture" }, deployment_configs: {} },
    deployment: { id: "deployment-fixture", project_id: "project-fixture", project_name: fixtureIdentity.projectName, environment: "production", is_skipped: false,
      latest_stage: { name: "deploy", status: "success" }, deployment_trigger: { metadata: { branch: "main", commit_hash: manifest.source.commit, commit_dirty: false } }, url: `https://abc123.${fixtureIdentity.projectName}.pages.dev` },
  };
  for (const environment of ["production", "preview"]) snapshots.project.deployment_configs[environment] = {
    fail_open: false, compatibility_flags: [], compatibility_date: manifest.compatibilityDate, always_use_latest_compatibility_date: false, wrangler_config_hash: manifest.files[0].sha256,
    d1_databases: { DB: { id: fixtureIdentity[`${environment}Database`].id } },
  };
  assert.equal(verifyPagesReadback(manifest, snapshots).runtimeVerified, false);
  assert.equal(verifyPagesProjectSafety(fixtureIdentity, snapshots).stage, "project-preflight");
  for (const change of [
    (s) => s.accountId = "2".repeat(32),
    (s) => s.project.name = "another-project",
    (s) => s.project.production_branch = "preview",
    (s) => s.project.canonical_deployment.id = "old-deployment",
    (s) => s.project.deployment_configs.preview.d1_databases.DB.id = fixtureIdentity.productionDatabase.id,
    (s) => s.project.deployment_configs.production.wrangler_config_hash = "changed",
    (s) => s.project.deployment_configs.production.compatibility_date = "2020-01-01",
    (s) => s.project.deployment_configs.production.compatibility_flags.push("nodejs_compat"),
    (s) => s.project.deployment_configs.production.fail_open = true,
    (s) => delete s.project.deployment_configs.preview.fail_open,
    (s) => s.deployment.project_id = "another-project",
    (s) => s.deployment.environment = "preview",
    (s) => s.deployment.latest_stage.status = "failure",
    (s) => s.deployment.is_skipped = true,
    (s) => s.deployment.deployment_trigger.metadata.commit_dirty = true,
    (s) => s.deployment.deployment_trigger.metadata.commit_hash = "c".repeat(40),
    (s) => s.deployment.url = `https://abc123.${fixtureIdentity.projectName}.pages.dev.attacker.example`,
  ]) {
    const altered = structuredClone(snapshots); change(altered);
    assert.throws(() => verifyPagesReadback(manifest, altered));
  }
  assert.throws(() => verifyPagesReadback({ ...manifest, source: { ...manifest.source, dirty: true } }, snapshots));
  console.log("[pages-production] config/identity isolation and synthetic readback rejection cases passed; no provider evidence claimed");

  const fixture = await mkdtemp(join(tmpdir(), "leptos-build-provenance-"));
  try {
    for (const directory of [".cargo", ".cfctl/operations", "migrations", "src", "runtime", "scripts", "style", "assets", "build", "target/site", "pages"]) await mkdir(join(fixture, directory), { recursive: true });
    for (const path of ["Cargo.toml", "Cargo.lock", ".cfctl/operations/d1-migrations.toml", "package.json", "bun.lock", "wrangler.toml", "pages/wrangler.toml", "pages/package.json", "src/app.rs", "build/_worker.js", "build/index.js", "build/index_bg.wasm", "build/dispatch.mjs", "target/site/logo.svg"]) await writeFile(join(fixture, path), "fixture");
    const script = resolve("scripts/build-provenance.mjs");
    const run = (phase) => Bun.spawnSync([process.execPath, script, phase], { cwd: fixture, stdout: "pipe", stderr: "pipe" });
    assert.equal(run("start").exitCode, 0);
    assert.equal(run("finish").exitCode, 0);
    await validateBuild(fixture);
    await writeFile(join(fixture, "build.rs"), "new implicit build input");
    await assert.rejects(validateBuild(fixture), /source changed/);
    await rm(join(fixture, "build.rs"));
    await writeFile(join(fixture, "build/index_bg.wasm"), "changed artifact");
    await assert.rejects(validateBuild(fixture), /artifacts changed/);
    await writeFile(join(fixture, "build/index_bg.wasm"), "fixture");
    await writeFile(join(fixture, "src/app.rs"), "changed source");
    await assert.rejects(validateBuild(fixture), /source changed/);
    assert.notEqual(run("finish").exitCode, 0, "source changes during a build must invalidate completion");
    await symlink(join(fixture, "Cargo.lock"), join(fixture, "src/link"));
    assert.notEqual(run("start").exitCode, 0, "source symlinks must not expand release input custody");
    console.log("[build-provenance] source/artifact drift, mid-build changes and symlink boundaries passed");
  } finally { await rm(fixture, { recursive: true, force: true }); }
}
