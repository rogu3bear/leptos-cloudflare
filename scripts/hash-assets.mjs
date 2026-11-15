#!/usr/bin/env bun

import { createHash } from "node:crypto";
import { existsSync } from "node:fs";
import { readdirSync } from "node:fs";
import { readFile, rename, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { projectMetadata } from "./project-metadata.mjs";

function shortHash(buffer) {
  return createHash("sha256").update(buffer).digest("hex").slice(0, 16);
}

function hashedName(baseName, hash, extension) {
  return `${baseName}.${hash}.${extension}`;
}

export function fingerprintAssets(outputName, jsBuffer, wasmBuffer, cssBuffer) {
  const wasmHash = shortHash(wasmBuffer);
  const cssHash = shortHash(cssBuffer);
  const hashedWasmName = hashedName(outputName, wasmHash, "wasm");
  const hashedCssName = hashedName(outputName, cssHash, "css");
  const source = new TextDecoder().decode(jsBuffer);
  const wasmImport = /new URL\(\s*["']([^"']+\.wasm)["']\s*,\s*import\.meta\.url\s*\)/g;
  if (Array.from(source.matchAll(wasmImport)).length !== 1) {
    throw new Error("expected exactly one generated WASM import before fingerprinting JavaScript");
  }
  const rewrittenJs = source.replace(wasmImport, `new URL("${hashedWasmName}",import.meta.url)`);
  // Immutable filenames identify the bytes actually served, including the WASM
  // reference. A WASM-only change must therefore also change the JS URL.
  const jsHash = shortHash(rewrittenJs);
  return { rewrittenJs, jsHash, wasmHash, cssHash,
    hashedJsName: hashedName(outputName, jsHash, "js"), hashedWasmName, hashedCssName };
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
  const { root: workspaceRoot, leptos, assetPrefix, paths } = await projectMetadata();
  const outputName = leptos["output-name"];
  const siteRoot = paths.site;
  const pkgDir = paths.pkg;

  const jsPath = join(pkgDir, `${outputName}.js`);
  const wasmPath = join(pkgDir, `${outputName}.wasm`);
  const cssPath = join(pkgDir, `${outputName}.css`);

  for (const requiredPath of [jsPath, wasmPath, cssPath]) {
    if (!existsSync(requiredPath)) {
      throw new Error(`expected build artifact is missing: ${requiredPath}`);
    }
  }

  const jsBuffer = await readFile(jsPath);
  const wasmBuffer = await readFile(wasmPath);
  const cssBuffer = await readFile(cssPath);
  const { rewrittenJs, jsHash, wasmHash, cssHash, hashedJsName, hashedWasmName, hashedCssName } =
    fingerprintAssets(outputName, jsBuffer, wasmBuffer, cssBuffer);

  for (const extension of ["js", "wasm", "css"]) {
    await removeStaleHashedFiles(pkgDir, outputName, extension);
  }

  await writeFile(join(pkgDir, hashedJsName), rewrittenJs);
  await writeFile(join(pkgDir, hashedCssName), cssBuffer);
  await rename(wasmPath, join(pkgDir, hashedWasmName));

  await rm(jsPath, { force: true });
  await rm(cssPath, { force: true });

  const assetManifest = {
    js: `${assetPrefix}${hashedJsName}`,
    wasm: `${assetPrefix}${hashedWasmName}`,
    css: `${assetPrefix}${hashedCssName}`,
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

if (import.meta.main) {
  main().catch((error) => {
    console.error(`[hash-assets] ${error.message}`);
    process.exit(1);
  });
}
