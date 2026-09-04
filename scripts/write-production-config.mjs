#!/usr/bin/env bun

import { chmod, open, readFile, rename, unlink } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ZERO_UUID = "00000000-0000-0000-0000-000000000000";
const NAME_PATTERN = /^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/;
const UUID_PATTERN = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;

function requireIdentity(label, value) {
  if (!NAME_PATTERN.test(value)) {
    throw new Error(`${label} must be 1-63 lowercase letters, digits, or interior hyphens`);
  }
}

function requireDatabaseId(value) {
  if (!UUID_PATTERN.test(value) || value === ZERO_UUID) {
    throw new Error("database id must be a non-placeholder lowercase canonical UUID");
  }
}

function replaceUniqueLine(source, pattern, replacement, label) {
  const matches = source.match(pattern);
  if (matches?.length !== 1) {
    throw new Error(`tracked wrangler.toml must contain exactly one ${label} line`);
  }
  return source.replace(pattern, replacement);
}

export function renderProductionConfig(source, { workerName, databaseName, databaseId }) {
  requireIdentity("worker name", workerName);
  requireIdentity("database name", databaseName);
  requireDatabaseId(databaseId);

  for (const invariant of [
    'main = "build/_worker.js"',
    '[assets]',
    'binding = "ASSETS"',
    '[observability.logs]',
    '[observability.traces]',
    'binding = "DB"',
    'migrations_dir = "migrations"',
  ]) {
    if (!source.includes(invariant)) {
      throw new Error(`tracked wrangler.toml is missing invariant: ${invariant}`);
    }
  }
  if (source.includes("[env.production]") || source.includes("pages_build_output_dir")) {
    throw new Error("tracked wrangler.toml must remain the Workers SSR template, not a Pages or environment overlay");
  }

  const placeholderCount = source.split(ZERO_UUID).length - 1;
  if (placeholderCount !== 2) {
    throw new Error("tracked wrangler.toml must contain exactly two placeholder D1 UUIDs");
  }

  let output = source;
  output = replaceUniqueLine(output, /^name = "[^"]+"$/gm, `name = "${workerName}"`, "Worker name");
  output = replaceUniqueLine(output, /^database_name = "[^"]+"$/gm, `database_name = "${databaseName}"`, "D1 database name");
  output = replaceUniqueLine(output, /^database_id = "[^"]+"$/gm, `database_id = "${databaseId}"`, "D1 database id");
  output = replaceUniqueLine(output, /^preview_database_id = "[^"]+"$/gm, `preview_database_id = "${databaseId}"`, "D1 preview database id");
  return output;
}

function parseArguments(args) {
  const values = new Map();
  for (let index = 0; index < args.length; index += 2) {
    const flag = args[index];
    const value = args[index + 1];
    if (!["--worker", "--database", "--database-id"].includes(flag) || value === undefined || values.has(flag)) {
      throw new Error("usage: bun scripts/write-production-config.mjs --worker <name> --database <name> --database-id <uuid>");
    }
    values.set(flag, value);
  }
  if (values.size !== 3) {
    throw new Error("usage: bun scripts/write-production-config.mjs --worker <name> --database <name> --database-id <uuid>");
  }
  return {
    workerName: values.get("--worker"),
    databaseName: values.get("--database"),
    databaseId: values.get("--database-id"),
  };
}

export async function writeProductionConfig(root, identity) {
  const sourcePath = join(root, "wrangler.toml");
  const outputPath = join(root, "wrangler.production.toml");
  const temporaryPath = `${outputPath}.${process.pid}.tmp`;
  const source = await readFile(sourcePath, "utf8");
  const output = renderProductionConfig(source, identity);

  let ownsTemporary = false;
  try {
    const temporary = await open(temporaryPath, "wx", 0o600);
    ownsTemporary = true;
    try {
      await temporary.writeFile(output, "utf8");
    } finally {
      await temporary.close();
    }
    await rename(temporaryPath, outputPath);
    ownsTemporary = false;
    await chmod(outputPath, 0o600);
  } catch (error) {
    if (ownsTemporary) await unlink(temporaryPath).catch(() => {});
    throw error;
  }
}

async function main() {
  const root = dirname(fileURLToPath(new URL("../package.json", import.meta.url)));
  await writeProductionConfig(root, parseArguments(process.argv.slice(2)));
  console.log("[production-config] wrote ignored wrangler.production.toml from the tracked Workers SSR template");
}

if (import.meta.main) {
  await main();
}
