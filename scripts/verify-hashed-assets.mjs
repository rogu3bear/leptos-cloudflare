#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { join } from "node:path";

function runOrThrow(cmd, args) {
  const proc = Bun.spawnSync([cmd, ...args], {
    stdout: "pipe",
    stderr: "pipe",
    cwd: process.cwd(),
  });

  if (proc.exitCode !== 0) {
    const stderr = new TextDecoder().decode(proc.stderr).trim();
    throw new Error(stderr || `${cmd} exited with code ${proc.exitCode}`);
  }

  return new TextDecoder().decode(proc.stdout);
}

async function main() {
  const metadata = JSON.parse(
    runOrThrow("cargo", ["metadata", "--no-deps", "--format-version", "1"]),
  );
  const workspaceRoot = metadata.workspace_root;
  const rootPackage = metadata.packages.find(
    (pkg) => pkg.manifest_path === join(workspaceRoot, "Cargo.toml"),
  );

  if (!rootPackage?.metadata?.leptos) {
    throw new Error("missing package.metadata.leptos");
  }

  const leptos = rootPackage.metadata.leptos;
  const outputName = leptos["output-name"];
  const siteRoot = join(workspaceRoot, leptos["site-root"]);
  const pkgDir = join(siteRoot, leptos["site-pkg-dir"]);

  const manifestPath = join(siteRoot, "asset-manifest.json");
  const generatedHashesPath = join(workspaceRoot, "target/asset-hashes.env");
  const headersPath = join(siteRoot, "_headers");

  if (!existsSync(manifestPath)) {
    throw new Error(`missing asset manifest: ${manifestPath}`);
  }
  if (!existsSync(generatedHashesPath)) {
    throw new Error(`missing generated asset hash env file: ${generatedHashesPath}`);
  }
  if (!existsSync(headersPath)) {
    throw new Error(`missing copied Cloudflare asset headers file: ${headersPath}`);
  }

  const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
  const generatedHashes = await readFile(generatedHashesPath, "utf8");
  const headersFile = await readFile(headersPath, "utf8");

  for (const [kind, href] of Object.entries({
    js: manifest.js,
    wasm: manifest.wasm,
    css: manifest.css,
  })) {
    if (typeof href !== "string" || !href.includes(`.${manifest.hashes[kind]}.`)) {
      throw new Error(`${kind} manifest entry is not hashed: ${href}`);
    }

    const filePath = join(siteRoot, href.replace(/^\//, ""));
    if (!existsSync(filePath)) {
      throw new Error(`${kind} manifest entry does not exist: ${filePath}`);
    }
  }

  for (const extension of ["js", "wasm", "css"]) {
    const unhashedPath = join(pkgDir, `${outputName}.${extension}`);
    if (existsSync(unhashedPath)) {
      throw new Error(`unhashed production asset still exists: ${unhashedPath}`);
    }
  }

  for (const [kind, hash] of Object.entries(manifest.hashes)) {
    if (!generatedHashes.includes(`"${hash}"`)) {
      throw new Error(`generated asset hash env values are out of sync for ${kind}`);
    }
  }

  for (const requiredSnippet of [
    "/pkg/*",
    "Cache-Control: public, max-age=31536000, immutable",
    "/asset-manifest.json",
    "Cache-Control: no-store",
  ]) {
    if (!headersFile.includes(requiredSnippet)) {
      throw new Error(`_headers is missing required cache rule snippet: ${requiredSnippet}`);
    }
  }

  console.log("[verify-hashed-assets] hashed assets and compile-time hash env values are in sync");
}

main().catch((error) => {
  console.error(`[verify-hashed-assets] ${error.message}`);
  process.exit(1);
});
