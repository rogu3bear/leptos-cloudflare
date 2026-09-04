#!/usr/bin/env bun

import { readFile } from "node:fs/promises";
import { join } from "node:path";

const root = process.cwd();

async function read(relativePath) {
  return readFile(join(root, relativePath), "utf8");
}

function requireSnippets(label, text, snippets) {
  const missing = snippets.filter((snippet) => !text.includes(snippet));
  if (missing.length > 0) {
    throw new Error(`${label} is missing required architecture contract: ${missing.join(", ")}`);
  }
}

const [architecturePage, howLeptosWorks] = await Promise.all([
  read("src/components/architecture_page.rs"),
  read("docs/how-leptos-works.md"),
]);

requireSnippets("public architecture page", architecturePage, [
  "Rendering decision",
  "SSR + hydration",
  "Pure CSR",
  "Current template",
  "Platform decision",
  "Workers Static Assets",
  "Cloudflare Pages",
  "Platform asset router",
  "Exact asset match",
  "Worker entrypoint",
  "Leptos SSR",
]);

requireSnippets("docs/how-leptos-works.md", howLeptosWorks, [
  "browser WASM + JS glue",
  "worker-build",
  "write-worker-shim.mjs",
  "HomePage",
  "/architecture",
  "asset-manifest.json",
]);

console.log(
  "[verify-field-guide] reference application decisions and documentation are aligned",
);
