#!/usr/bin/env bun

import { strict as assert } from "node:assert";
import { readFile } from "node:fs/promises";
import { renderProductionConfig } from "./write-production-config.mjs";

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
  ['name = "leptos-cf"', 'name = "example-production-worker"'],
  ['database_name = "leptos-cf-db"', 'database_name = "example-production-db"'],
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
