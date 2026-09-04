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

export async function adoptProject(root, name) {
  if (!NAME.test(name ?? "")) throw new Error("project name must be 1–60 lowercase letters, digits, or interior hyphens, starting with a letter");
  const { cargo, leptos } = await projectMetadata(root);
  const previous = cargo.package.name;
  const files = ["Cargo.toml", "Cargo.lock", "wrangler.toml", ".cfctl/operations/d1-migrations.toml"];
  const originals = await Promise.all(files.map((file) => readFile(join(root, file), "utf8")));
  const wrangler = Bun.TOML.parse(originals[2]);
  const operation = Bun.TOML.parse(originals[3]).operation;
  if (wrangler.name !== previous || leptos["output-name"] !== previous || wrangler.d1_databases?.length !== 1 || wrangler.d1_databases[0].database_name !== `${previous}-db`) {
    throw new Error("project identity is already customized; reconcile Cargo, output-name, Worker, and D1 names before adoption");
  }
  if (wrangler.account_id || wrangler.route || wrangler.routes || wrangler.env) {
    throw new Error("bound provider routing or environment cannot be renamed by local adoption");
  }
  if ([wrangler.d1_databases[0].database_id, wrangler.d1_databases[0].preview_database_id].some((id) => id !== ZERO)) {
    throw new Error("bound provider identity cannot be renamed by local adoption");
  }
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

  const next = [...originals];
  next[0] = replaceOnce(next[0], `name = "${previous}"`, `name = "${name}"`, "package name");
  next[0] = replaceOnce(next[0], `output-name = "${previous}"`, `output-name = "${name}"`, "output-name");
  const referenceFlag = /^(\s*reference-site\s*=\s*)(?:true|false)(\s*(?:#.*)?)$/gm;
  if (Array.from(next[0].matchAll(referenceFlag)).length !== 1) {
    throw new Error("expected exactly one reference-site flag; reconcile metadata before adoption");
  }
  next[0] = next[0].replace(referenceFlag, "$1false$2");
  if (Bun.TOML.parse(next[0]).package.metadata["leptos-cf"]["reference-site"] !== false) {
    throw new Error("adoption must select application verification before identity changes");
  }
  next[1] = replaceOnce(next[1], `[[package]]\nname = "${previous}"\n`, `[[package]]\nname = "${name}"\n`, "root lockfile package");
  next[2] = replaceOnce(next[2], `name = "${previous}"`, `name = "${name}"`, "Worker name");
  next[2] = replaceOnce(next[2], `database_name = "${previous}-db"`, `database_name = "${name}-db"`, "D1 name");
  next[3] = replaceOnce(next[3], `id = "${previous}.d1-migrations-apply"`, `id = "${name}.d1-migrations-apply"`, "migration operation id");
  next[3] = next[3]
    .replace(`the ${previous} D1 migrations`, `the ${name} D1 migrations`)
    .replace(`append-only ${previous} D1 migration`, `append-only ${name} D1 migration`);
  // Validate every transformed document and destination before the first write.
  for (let index = 0; index < files.length; index++) {
    Bun.TOML.parse(next[index]);
    await access(join(root, files[index]), constants.W_OK);
  }
  const temporary = files.map((file) => join(root, `${file}.adopt-${process.pid}.tmp`));
  let installed = 0;
  const ownedTemporary = new Set();
  try {
    for (let index = 0; index < files.length; index++) {
      await writeFile(temporary[index], next[index], { flag: "wx", mode: 0o644 });
      ownedTemporary.add(temporary[index]);
    }
    for (let index = 0; index < files.length; index++) {
      // Detect intervening edits rather than overwriting another writer.
      if (await readFile(join(root, files[index]), "utf8") !== originals[index]) throw new Error(`concurrent edit detected in ${files[index]}`);
      await rename(temporary[index], join(root, files[index]));
      ownedTemporary.delete(temporary[index]);
      installed++;
    }
  } catch (error) {
    for (let index = installed - 1; index >= 0; index--) {
      await writeFile(temporary[index], originals[index], { flag: "wx", mode: 0o644 });
      ownedTemporary.add(temporary[index]);
      await rename(temporary[index], join(root, files[index]));
      ownedTemporary.delete(temporary[index]);
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
