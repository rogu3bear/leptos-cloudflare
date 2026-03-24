#!/usr/bin/env bun

import { createHash } from "node:crypto";
import { existsSync } from "node:fs";
import { readdirSync } from "node:fs";
import { readFile, rename, rm, writeFile } from "node:fs/promises";
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

function shortHash(buffer) {
  return createHash("sha256").update(buffer).digest("hex").slice(0, 16);
}

function hashedName(baseName, hash, extension) {
  return `${baseName}.${hash}.${extension}`;
}

async function removeStaleHashedFiles(pkgDir, outputName, extension) {
  const escaped = outputName.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const pattern = new RegExp(`^${escaped}\\.[a-f0-9]{16}\\.${extension}$`);

  for (const entry of readdirSync(pkgDir)) {
    if (pattern.test(entry)) {
      await rm(join(pkgDir, entry), { force: true });
    }
  }
}

async function main() {
  const metadata = JSON.parse(
    runOrThrow("cargo", ["metadata", "--no-deps", "--format-version", "1"]),
  );
  const workspaceRoot = metadata.workspace_root;
  const manifestPath = join(workspaceRoot, "Cargo.toml");
  const rootPackage = metadata.packages.find(
    (pkg) => pkg.manifest_path === manifestPath,
  );

  if (!rootPackage) {
    throw new Error("failed to resolve root package metadata");
  }

  const leptos = rootPackage.metadata?.leptos;
  if (!leptos) {
    throw new Error("missing package.metadata.leptos");
  }

  const outputName = leptos["output-name"];
  const siteRoot = join(workspaceRoot, leptos["site-root"]);
  const pkgDir = join(siteRoot, leptos["site-pkg-dir"]);

  const jsPath = join(pkgDir, `${outputName}.js`);
  const wasmPath = join(pkgDir, `${outputName}.wasm`);
  const cssPath = join(pkgDir, `${outputName}.css`);

  for (const requiredPath of [jsPath, wasmPath, cssPath]) {
    if (!existsSync(requiredPath)) {
      throw new Error(`expected build artifact is missing: ${requiredPath}`);
    }
  }

  for (const extension of ["js", "wasm", "css"]) {
    await removeStaleHashedFiles(pkgDir, outputName, extension);
  }

  const jsBuffer = await readFile(jsPath);
  const wasmBuffer = await readFile(wasmPath);
  const cssBuffer = await readFile(cssPath);

  const jsHash = shortHash(jsBuffer);
  const wasmHash = shortHash(wasmBuffer);
  const cssHash = shortHash(cssBuffer);

  const hashedJsName = hashedName(outputName, jsHash, "js");
  const hashedWasmName = hashedName(outputName, wasmHash, "wasm");
  const hashedCssName = hashedName(outputName, cssHash, "css");

  const rewrittenJs = new TextDecoder().decode(jsBuffer).replace(
    /new URL\("([^"]+\.wasm)",import\.meta\.url\)/,
    `new URL("${hashedWasmName}",import.meta.url)`,
  );

  await writeFile(join(pkgDir, hashedJsName), rewrittenJs);
  await writeFile(join(pkgDir, hashedCssName), cssBuffer);
  await rename(wasmPath, join(pkgDir, hashedWasmName));

  await rm(jsPath, { force: true });
  await rm(cssPath, { force: true });

  const assetManifest = {
    js: `/pkg/${hashedJsName}`,
    wasm: `/pkg/${hashedWasmName}`,
    css: `/pkg/${hashedCssName}`,
    hashes: {
      js: jsHash,
      wasm: wasmHash,
      css: cssHash,
    },
  };

  await writeFile(
    join(siteRoot, "asset-manifest.json"),
    `${JSON.stringify(assetManifest, null, 2)}\n`,
  );

  await writeFile(
    join(workspaceRoot, "target/asset-hashes.env"),
    [
      `export LEPTOS_EDGE_JS_HASH="${jsHash}"`,
      `export LEPTOS_EDGE_WASM_HASH="${wasmHash}"`,
      `export LEPTOS_EDGE_CSS_HASH="${cssHash}"`,
      "",
    ].join("\n"),
  );
}

main().catch((error) => {
  console.error(`[hash-assets] ${error.message}`);
  process.exit(1);
});
