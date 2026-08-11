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

function rejectSnippets(label, text, snippets) {
  const present = snippets.filter((snippet) => text.includes(snippet));
  if (present.length > 0) {
    throw new Error(`${label} contains a deferred architecture lane: ${present.join(", ")}`);
  }
}

const [architecturePage, cargoToml, wranglerToml, howLeptosWorks] = await Promise.all([
  read("src/components/architecture_page.rs"),
  read("Cargo.toml"),
  read("wrangler.toml"),
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

rejectSnippets("Cargo.toml", cargoToml, ["leptos/csr"]);
rejectSnippets("wrangler.toml", wranglerToml, [
  "pages_build_output_dir",
  'not_found_handling = "single-page-application"',
]);

console.log(
  "[verify-architecture-contract] public decisions, build truth, and deferred CSR/Pages lanes are aligned",
);
