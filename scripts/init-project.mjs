#!/usr/bin/env bun

import { access, readFile, rename, rm, writeFile } from "node:fs/promises";
import { constants } from "node:fs";
import { join } from "node:path";
import { projectMetadata } from "./project-metadata.mjs";

const ZERO = "00000000-0000-0000-0000-000000000000";
const NAME = /^[a-z](?:[a-z0-9-]{0,58}[a-z0-9])?$/;

function replaceOnce(source, before, after, label) {
  const escaped = before.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const pattern = new RegExp(`^${escaped}`, "gm");
  if (Array.from(source.matchAll(pattern)).length !== 1) throw new Error(`expected exactly one ${label}; adopt this customized contract manually`);
  return source.replace(pattern, after);
}

function validateLocalDeployment(config, previous, pages) {
  const prefix = pages ? "Pages identity is customized or bound" : "project identity is already customized";
  if (config.name !== previous || config.d1_databases?.length !== 1 || config.d1_databases[0].database_name !== `${previous}-db`) {
    throw new Error(`${prefix}; reconcile it before adoption`);
  }
  if (config.account_id || config.route || config.routes || config.env) {
    throw new Error(pages ? `${prefix}; reconcile it before adoption` : "bound provider routing or environment cannot be renamed by local adoption");
  }
  const database = config.d1_databases[0];
  if ([database.database_id, database.preview_database_id].some((id) => id !== ZERO)) {
    throw new Error(pages ? `${prefix}; reconcile it before adoption` : "bound provider identity cannot be renamed by local adoption");
  }
}

export async function adoptProject(root, name) {
  if (!NAME.test(name ?? "")) throw new Error("project name must be 1–60 lowercase letters, digits, or interior hyphens, starting with a letter");
  const { cargo, leptos } = await projectMetadata(root);
  const previous = cargo.package.name;
  const files = { cargo: "Cargo.toml", lock: "Cargo.lock", worker: "wrangler.toml", migration: ".cfctl/operations/d1-migrations.toml", pages: "pages/wrangler.toml" };
  const originals = Object.fromEntries(await Promise.all(Object.entries(files).map(async ([key, file]) => [key, await readFile(join(root, file), "utf8")])));
  const wrangler = Bun.TOML.parse(originals.worker);
  const operation = Bun.TOML.parse(originals.migration).operation;
  const pages = Bun.TOML.parse(originals.pages);
  validateLocalDeployment(pages, previous, true);
  if (leptos["output-name"] !== previous) throw new Error("project identity is already customized; reconcile output-name before adoption");
  validateLocalDeployment(wrangler, previous, false);
  try {
    await access(join(root, "wrangler.production.toml"));
    throw new Error("production config exists; preserve its binding and perform a separately reviewed application cutover");
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
  }
  if (operation?.length !== 1 || operation[0].id !== `${previous}.d1-migrations-apply`) {
    throw new Error("migration operation identity is customized; reconcile it before adoption");
  }
  if (name === previous) return { changed: false, name };

  const next = { ...originals };
  next.cargo = replaceOnce(next.cargo, `name = "${previous}"`, `name = "${name}"`, "package name");
  next.cargo = replaceOnce(next.cargo, `output-name = "${previous}"`, `output-name = "${name}"`, "output-name");
  const referenceFlag = /^(\s*reference-site\s*=\s*)(?:true|false)(\s*(?:#.*)?)$/gm;
  if (Array.from(next.cargo.matchAll(referenceFlag)).length !== 1) {
    throw new Error("expected exactly one reference-site flag; reconcile metadata before adoption");
  }
  next.cargo = next.cargo.replace(referenceFlag, "$1false$2");
  if (Bun.TOML.parse(next.cargo).package.metadata["leptos-cf"]["reference-site"] !== false) {
    throw new Error("adoption must select application verification before identity changes");
  }
  next.lock = replaceOnce(next.lock, `[[package]]\nname = "${previous}"\n`, `[[package]]\nname = "${name}"\n`, "root lockfile package");
  const renameDeployment = (source, label) => replaceOnce(
    replaceOnce(source, `name = "${previous}"`, `name = "${name}"`, `${label} name`),
    `database_name = "${previous}-db"`, `database_name = "${name}-db"`, `${label} D1 name`,
  );
  next.worker = renameDeployment(next.worker, "Worker");
  next.pages = renameDeployment(next.pages, "Pages");
  next.migration = replaceOnce(next.migration, `id = "${previous}.d1-migrations-apply"`, `id = "${name}.d1-migrations-apply"`, "migration operation id");
  next.migration = next.migration
    .replace(`the ${previous} D1 migrations`, `the ${name} D1 migrations`)
    .replace(`append-only ${previous} D1 migration`, `append-only ${name} D1 migration`);
  const documents = Object.entries(files).map(([key, file]) => ({
    file: join(root, file), original: originals[key], next: next[key],
    temporary: join(root, `${file}.adopt-${process.pid}.tmp`),
  }));
  for (const document of documents) {
    Bun.TOML.parse(document.next);
    await access(document.file, constants.W_OK);
  }
  const installed = [];
  const ownedTemporary = new Set();
  try {
    for (const document of documents) {
      await writeFile(document.temporary, document.next, { flag: "wx", mode: 0o644 });
      ownedTemporary.add(document.temporary);
    }
    for (const document of documents) {
      if (await readFile(document.file, "utf8") !== document.original) throw new Error(`concurrent edit detected in ${document.file}`);
      await rename(document.temporary, document.file);
      ownedTemporary.delete(document.temporary);
      installed.push(document);
    }
  } catch (error) {
    for (const document of installed.reverse()) {
      await writeFile(document.temporary, document.original, { flag: "wx", mode: 0o644 });
      ownedTemporary.add(document.temporary);
      await rename(document.temporary, document.file);
      ownedTemporary.delete(document.temporary);
    }
    throw error;
  } finally {
    await Promise.all([...ownedTemporary].map((file) => rm(file, { force: true })));
  }
  return { changed: true, name };
}

if (import.meta.main) {
  try {
    if (process.argv.length !== 3) throw new Error("usage: ./scripts/init.sh <project-name>");
    const result = await adoptProject(process.cwd(), process.argv[2]);
    console.log(`[init] ${result.changed ? "Adopted" : "Already using"} local project identity ${result.name}; all application code and migrations are preserved.`);
    console.log("[init] No provider resources changed. Follow docs/adopting.md, then run ./scripts/verify.sh before release.");
  } catch (error) {
    console.error(`[init] ${error.message}`);
    process.exitCode = 1;
  }
}
