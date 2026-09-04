#!/usr/bin/env bun

import { strict as assert } from "node:assert";
import { mkdtemp, readFile, rm, stat, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { renderProductionConfig, writeProductionConfig } from "./write-production-config.mjs";

const source = await readFile(new URL("../wrangler.toml", import.meta.url), "utf8");
const identity = {
  workerName: "example-production-worker",
  databaseName: "example-production-db",
  databaseId: "12345678-1234-4abc-9def-1234567890ab",
};
const output = renderProductionConfig(source, identity);

const changedLines = source
  .split("\n")
  .map((line, index) => [line, output.split("\n")[index]])
  .filter(([before, after]) => before !== after);

assert.deepEqual(changedLines, [
  [source.match(/^name = .*$/m)[0], 'name = "example-production-worker"'],
  [source.match(/^database_name = .*$/m)[0], 'database_name = "example-production-db"'],
  ['database_id = "00000000-0000-0000-0000-000000000000"', 'database_id = "12345678-1234-4abc-9def-1234567890ab"'],
  ['preview_database_id = "00000000-0000-0000-0000-000000000000"', 'preview_database_id = "12345678-1234-4abc-9def-1234567890ab"'],
]);

for (const invariant of [
  'main = "build/_worker.js"',
  '[assets]\ndirectory = "./target/site"\nbinding = "ASSETS"',
  '[observability.logs]\nenabled = true\nhead_sampling_rate = 0.1',
  '[observability.traces]\nenabled = true\nhead_sampling_rate = 0.01',
  'binding = "DB"',
  'migrations_dir = "migrations"',
]) {
  assert.ok(output.includes(invariant), `production transform changed invariant: ${invariant}`);
}
assert.equal(output.includes("[env.production]"), false);
assert.equal(output.includes("pages_build_output_dir"), false);
assert.equal(output.includes("csr"), false);
assert.equal(renderProductionConfig(source, identity), output, "transform must be deterministic");

for (const invalid of [
  { ...identity, workerName: "Bad Worker" },
  { ...identity, databaseName: "-bad" },
  { ...identity, databaseId: "00000000-0000-0000-0000-000000000000" },
  { ...identity, databaseId: "not-a-uuid" },
]) {
  assert.throws(() => renderProductionConfig(source, invalid));
}
assert.throws(() => renderProductionConfig(source.replaceAll("00000000-0000-0000-0000-000000000000", identity.databaseId), identity));

console.log("[test-production-config] portable template boundary and deterministic production derivation passed");

const fixture = await mkdtemp(join(tmpdir(), "leptos-production-config-"));
try {
  const config = join(fixture, "wrangler.production.toml");
  const collision = `${config}.${process.pid}.tmp`;
  await writeFile(join(fixture, "wrangler.toml"), source);
  await writeFile(config, "existing-production-config");
  await writeFile(collision, "unowned-temporary-bytes");
  await assert.rejects(writeProductionConfig(fixture, identity), { code: "EEXIST" });
  assert.equal(await readFile(collision, "utf8"), "unowned-temporary-bytes");
  assert.equal(await readFile(config, "utf8"), "existing-production-config");
  await rm(collision);
  await writeProductionConfig(fixture, identity);
  assert.equal(await readFile(config, "utf8"), output);
  assert.equal((await stat(config)).mode & 0o777, 0o600);
  await assert.rejects(stat(collision), { code: "ENOENT" });
  console.log("[test-production-config] exclusive-temp collision preserves unowned files; success uses mode0600");
} finally {
  await rm(fixture, { recursive: true, force: true });
}
