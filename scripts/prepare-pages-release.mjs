#!/usr/bin/env bun
import { mkdir, readFile, realpath, writeFile, cp } from "node:fs/promises";
import { basename, dirname, join, resolve } from "node:path";
import processes from "./preserve-guard-fds.cjs";
import { digest, inventory, validateBuild } from "./build-provenance.mjs";

const name = /^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/;
const uuid = /^(?!00000000-0000-0000-0000-000000000000$)[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$/;
function require(condition, message) { if (!condition) throw new Error(message); }

export function validateIdentity(identity) {
  require(identity && /^(?!0{32}$)[0-9a-f]{32}$/.test(identity.accountId), "A verified account identity is required");
  require(Object.keys(identity).every((key) => ["accountId", "projectName", "productionBranch", "productionDatabase", "previewDatabase"].includes(key)), "Identity metadata must not contain credentials or unknown fields");
  require(name.test(identity.projectName), "Invalid Pages project name");
  require(typeof identity.productionBranch === "string" && /^[A-Za-z0-9][A-Za-z0-9._/-]{0,99}$/.test(identity.productionBranch), "Invalid production branch");
  require(!identity.productionBranch.includes("..") && !identity.productionBranch.includes("//") && !identity.productionBranch.includes("/.") && !/[/.]$|\.lock$/.test(identity.productionBranch), "Invalid production branch");
  for (const key of ["productionDatabase", "previewDatabase"]) {
    require(name.test(identity[key]?.name) && uuid.test(identity[key]?.id), `Invalid ${key} identity`);
    require(Object.keys(identity[key]).every((field) => ["name", "id"].includes(field)), `Unexpected ${key} metadata`);
  }
  require(identity.productionDatabase.id !== identity.previewDatabase.id && identity.productionDatabase.name !== identity.previewDatabase.name, "Production and preview D1 identities must be distinct");
}

export function renderPagesConfig(source, identity) {
  validateIdentity(identity);
  const template = Bun.TOML.parse(source);
  require(Object.keys(template).every((key) => ["name", "pages_build_output_dir", "compatibility_date", "d1_databases"].includes(key)), "Reconcile customized Pages template before release");
  require(template.pages_build_output_dir === "../target/pages" && /^\d{4}-\d{2}-\d{2}$/.test(template.compatibility_date), "Unexpected Pages output or compatibility contract");
  const db = template.d1_databases;
  require(db?.length === 1 && db[0].binding === "DB" && db[0].migrations_dir === "../migrations"
    && db[0].database_id === "00000000-0000-0000-0000-000000000000" && db[0].preview_database_id === db[0].database_id, "Pages template must remain provider-neutral");
  require(Object.keys(db[0]).every((key) => ["binding", "database_name", "database_id", "preview_database_id", "migrations_dir"].includes(key)), "Reconcile customized Pages D1 binding before release");
  const binding = (database) => `binding = "DB"\ndatabase_name = ${JSON.stringify(database.name)}\ndatabase_id = ${JSON.stringify(database.id)}\npreview_database_id = ${JSON.stringify(identity.previewDatabase.id)}\nmigrations_dir = "./migrations"\n`;
  return `name = ${JSON.stringify(identity.projectName)}\npages_build_output_dir = "./public"\ncompatibility_date = ${JSON.stringify(template.compatibility_date)}\n\n[[d1_databases]]\n${binding(identity.productionDatabase)}\n[[env.preview.d1_databases]]\n${binding(identity.previewDatabase)}`;
}

export function commandPlan(identity, source) {
  return {
    status: source.dirty || !source.commit ? "qualification-only" : "requires-provider-approval",
    accountId: identity.accountId,
    environment: { CLOUDFLARE_ACCOUNT_ID: identity.accountId },
    requiredProjectSettings: { production: { fail_open: false }, preview: { fail_open: false } },
    production: {
      migrate: ["d1", "migrations", "apply", identity.productionDatabase.name, "--remote", "--config", "wrangler.toml"],
      deploy: ["pages", "deploy", "public", "--project-name", identity.projectName, "--branch", identity.productionBranch, "--commit-hash", source.commit, "--commit-dirty=false"],
    },
    preview: {
      migrate: ["d1", "migrations", "apply", identity.previewDatabase.name, "--remote", "--config", "wrangler.toml", "--env", "preview"],
    },
  };
}

export async function readBundleMetadata(path) {
  const bytes = await readFile(path);
  const boundary = bytes.subarray(0, bytes.indexOf("\r\n")).toString().slice(2);
  require(boundary.length > 0 && boundary.length < 200, "Invalid Pages upload multipart boundary");
  const form = await new Response(bytes, { headers: { "content-type": `multipart/form-data; boundary=${boundary}` } }).formData();
  const metadata = JSON.parse(form.get("metadata"));
  require([...form.values()].some((value) => typeof value === "object" && value.type === "application/wasm"), "Pages upload lost its server WASM module");
  require(form.has(metadata.main_module), "Pages upload lost its main module");
  return metadata;
}

export async function preparePagesRelease(root, output, identity) {
  root = await realpath(root);
  validateIdentity(identity);
  const build = await validateBuild(root);
  const target = join(root, "target");
  require(await realpath(target) === target, "Release target must not traverse a symlink");
  output = resolve(root, output);
  require(dirname(output) === target && /^pages-release-[a-z0-9-]+$/.test(basename(output)), "Use a fresh target/pages-release-NAME directory");
  if (build.source.commit) require(processes.spawnSync("git", ["check-ignore", "-q", join(output, "wrangler.toml")], { cwd: root }).status === 0, "Pages provider configuration must remain ignored");
  const config = renderPagesConfig(await readFile(join(root, "pages/wrangler.toml"), "utf8"), identity);
  const packageBoundary = await readFile(join(root, "pages/package.json"), "utf8");
  require(JSON.stringify(JSON.parse(packageBoundary)) === JSON.stringify({ private: true }), "Reconcile the private Pages package boundary before release");
  const packBytes = await readFile(join(root, ".cfctl/operations/d1-migrations.toml"), "utf8");
  const pack = Bun.TOML.parse(packBytes);
  const packageName = Bun.TOML.parse(await readFile(join(root, "Cargo.toml"), "utf8")).package.name;
  require(pack.operation?.length === 1 && pack.operation[0].id === `${packageName}.d1-migrations-apply`, "Reconcile the owned migration operation before release");
  const operation = pack.operation[0];
  const migrations = await inventory(root, ["migrations"]);
  require(JSON.stringify(migrations.map((file) => [file.path, `sha256:${file.sha256}`])) === JSON.stringify(operation.migration.map((file) => [file.path, file.sha256])), "Migration set differs from the owned operation pack");
  // Exclusive creation preserves every existing output and any incomplete prior attempt.
  await mkdir(output, { mode: 0o700 });
  await writeFile(join(output, "wrangler.toml"), config, { flag: "wx", mode: 0o600 });
  await writeFile(join(output, "package.json"), packageBoundary, { flag: "wx", mode: 0o600 });
  await cp(join(root, "migrations"), join(output, "migrations"), { recursive: true, force: false, errorOnExist: true });
  require(JSON.stringify(await inventory(output, ["migrations"])) === JSON.stringify(migrations), "Migration bytes changed while copying the release");
  const run = (command, args, env = {}) => {
    const result = processes.spawnSync(command, args, { cwd: output, env: { ...process.env, WRANGLER_SEND_METRICS: "false", ...env }, encoding: "utf8", timeout: 120_000 });
    if (result.status !== 0) throw new Error(`Pages local preparation failed: ${result.error?.message ?? result.stderr}`);
  };
  // package-pages judges paths relative to the canonical source checkout.
  const packaged = processes.spawnSync(process.execPath, [join(root, "scripts/package-pages.mjs"), join(output, "public")], { cwd: root, encoding: "utf8", timeout: 120_000 });
  require(packaged.status === 0, `Pages packaging failed: ${packaged.stderr}`);
  run(join(root, "scripts/wrangler.sh"), ["pages", "project", "validate", "public"]);
  for (const environment of ["production", "preview"]) {
    run(join(root, "scripts/wrangler.sh"), ["pages", "functions", "build", "--project-directory", output, "--build-metadata-path", `${environment}.metadata.json`, "--outfile", `${environment}.bundle`], { PAGES_ENVIRONMENT: environment });
    const metadata = await readBundleMetadata(join(output, `${environment}.bundle`));
    const expected = identity[`${environment}Database`].id;
    require(metadata.compatibility_date === Bun.TOML.parse(config).compatibility_date && Array.isArray(metadata.compatibility_flags) && metadata.compatibility_flags.length === 0 && metadata.bindings?.filter((binding) => binding.type === "d1").length === 1
      && metadata.bindings.find((binding) => binding.name === "DB")?.id === expected, `Pages ${environment} upload has the wrong D1 or compatibility binding`);
    const buildMetadata = JSON.parse(await readFile(join(output, `${environment}.metadata.json`), "utf8"));
    require(buildMetadata.wrangler_config_hash === digest(config) && buildMetadata.build_output_directory === "public", "Wrangler did not consume the isolated standard config");
  }
  require(JSON.stringify(build) === JSON.stringify(await validateBuild(root)), "Build changed during Pages preparation");
  const files = await inventory(output, ["package.json", "wrangler.toml", "public", "migrations", "production.bundle", "preview.bundle", "production.metadata.json", "preview.metadata.json"]);
  const manifest = { schema: 1, source: build.source, identity, compatibilityDate: Bun.TOML.parse(config).compatibility_date, compatibilityFlags: [],
    wranglerVersion: JSON.parse(await readFile(join(root, "package.json"), "utf8")).devDependencies.wrangler,
    buildReceipt: digest(JSON.stringify(build)), files, migrationOperationId: operation.id, migrationPackHash: digest(packBytes), migrationAssertions: operation.assertion,
    commands: commandPlan(identity, build.source) };
  await writeFile(join(output, "release.json"), JSON.stringify(manifest, null, 2) + "\n", { flag: "wx", mode: 0o600 });
  return manifest;
}

if (import.meta.main) {
  const [identityPath, output, ...extra] = process.argv.slice(2);
  require(identityPath && output && extra.length === 0, "Usage: bun scripts/prepare-pages-release.mjs <identity.json> target/pages-release-NAME");
  const manifest = await preparePagesRelease(process.cwd(), output, JSON.parse(await readFile(identityPath, "utf8")));
  console.log(`[pages-release] Prepared locally: ${output}; ${manifest.commands.status}. No provider operation executed.`);
}
