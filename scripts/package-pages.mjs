#!/usr/bin/env bun

// Package an already verified edge build without changing the Workers output.
import { constants } from "node:fs";
import { workerEntry } from "./worker-entry.mjs";
import { projectMetadata } from "./project-metadata.mjs";
import { access, cp, mkdir, readdir, realpath, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, dirname, join, relative, resolve, sep } from "node:path";

const output = process.argv[2];
if (!output) throw new Error("Usage: bun scripts/package-pages.mjs <empty-output-directory>");
const project = await projectMetadata();
const root = await realpath(project.root);
const site = await realpath(project.paths.site);
const target = await realpath("target");
const destination = resolve(output);
const within = (child, parent) => child === parent || (relative(parent, child) !== "" && !relative(parent, child).startsWith(`..${sep}`) && relative(parent, child) !== ".." && !relative(parent, child).startsWith(sep));

// Resolve existing ancestors before creating anything. A lexical target can be
// redirected by a symlink; judge its actual destination, not its spelling.
async function canonicalDestination(path) {
  let cursor = path;
  const remaining = [];
  while (true) {
    try { return resolve(await realpath(cursor), ...remaining); }
    catch (error) {
      if (error.code !== "ENOENT") throw error;
      const parent = dirname(cursor);
      if (parent === cursor) throw error;
      remaining.unshift(basename(cursor));
      cursor = parent;
    }
  }
}
function validateDestination(actual) {
  if (within(actual, site) || within(site, actual) || within(root, actual)) {
    throw new Error("Pages output must not overlap the source asset directory or project root");
  }
  // Within this checkout, only generated target output is an admissible sink.
  if (within(actual, root) && !within(actual, target)) {
    throw new Error("Pages output inside the project must be under target");
  }
  // A target symlink into another source path does not create output authority.
  if (within(target, root) && target !== join(root, "target")) {
    throw new Error("target resolves into a protected source location");
  }
}
const actualDestination = await canonicalDestination(destination);
const temporaryRoot = await realpath(tmpdir());
const lexicalTemporaryRoot = resolve(tmpdir());
const expectedDestination = within(destination, lexicalTemporaryRoot)
  ? resolve(temporaryRoot, relative(lexicalTemporaryRoot, destination))
  : destination;
if (actualDestination !== expectedDestination) throw new Error("Pages output must not traverse a symlink");
if (target !== join(root, "target") || site !== join(target, "site")) throw new Error("Pages build inputs must not traverse a target or site symlink");
if (!within(actualDestination, target) && !within(actualDestination, temporaryRoot)) {
  throw new Error("Pages output must be under the project target directory or the system temporary directory");
}
validateDestination(actualDestination);

const reserved = new Set(["_worker.js", "_server", "_routes.json"]);
const metadata = new Set(["_headers", "_redirects"]);
const assets = [];
async function collectAssets(directory, prefix = "") {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const name = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (!prefix && reserved.has(entry.name)) throw new Error(`Pages source assets contain reserved ${entry.name}`);
    if (entry.isSymbolicLink()) throw new Error(`Pages source assets contain symlink: ${name}`);
    if (entry.isDirectory()) await collectAssets(join(directory, entry.name), name);
    else if (entry.isFile() && !(!prefix && metadata.has(entry.name))) assets.push(`/${name}`);
    else if (!entry.isFile()) throw new Error(`Pages source asset is not a regular file: ${name}`);
  }
}
await collectAssets(site);
const protectedShim = workerEntry({
  serverModule: "./_server/index.js",
  runtimeModule: "./_server/dispatch.mjs",
  assetPrefix: project.assetPrefix,
  assetPaths: assets.sort(),
  blockServerModules: true,
});
await Promise.all([access("build/index.js", constants.R_OK), access("build/index_bg.wasm", constants.R_OK), access("build/dispatch.mjs", constants.R_OK)]);
await mkdir(destination, { recursive: true });
if (await realpath(destination) !== actualDestination) throw new Error("Pages output path changed during preparation");
if ((await readdir(destination)).length) throw new Error("Pages output directory must be empty");
await cp(site, destination, { recursive: true, errorOnExist: true, force: false });
const workerDir = join(destination, "_server");
await mkdir(workerDir);
await writeFile(join(destination, "_worker.js"), protectedShim, { flag: "wx" });
await cp("build/index.js", join(workerDir, "index.js"), { errorOnExist: true, force: false });
await cp("build/index_bg.wasm", join(workerDir, "index_bg.wasm"), { errorOnExist: true, force: false });

await cp("build/dispatch.mjs", join(workerDir, "dispatch.mjs"), { errorOnExist: true, force: false });
console.log(`[package-pages] Pages advanced-mode output: ${destination}`);
