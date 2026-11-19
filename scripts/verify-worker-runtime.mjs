#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { hydrationAliases } from "./hydration-routes.mjs";
import { projectMetadata } from "./project-metadata.mjs";
import { workerEntry } from "./worker-entry.mjs";

const root = process.cwd();
const expectedCompatibilityDate = process.env.EXPECTED_COMPATIBILITY_DATE ?? "2026-08-10";
const wrangler = await readFile(join(root, "wrangler.toml"), "utf8");
const shimPath = join(root, "build/_worker.js");

function requireSnippet(label, text, snippet) {
  if (!text.includes(snippet)) {
    throw new Error(`${label} is missing required snippet: ${snippet}`);
  }
}

requireSnippet("wrangler.toml", wrangler, 'main = "build/_worker.js"');
requireSnippet(
  "wrangler.toml",
  wrangler,
  `compatibility_date = "${expectedCompatibilityDate}"`,
);
requireSnippet("wrangler.toml", wrangler, "[assets]");
requireSnippet("wrangler.toml", wrangler, 'directory = "./target/site"');
requireSnippet("wrangler.toml", wrangler, 'binding = "ASSETS"');
requireSnippet("wrangler.toml", wrangler, "[observability.logs]");
requireSnippet("wrangler.toml", wrangler, "[observability.traces]");
requireSnippet("wrangler.toml", wrangler, "head_sampling_rate = 0.1");
requireSnippet("wrangler.toml", wrangler, "head_sampling_rate = 0.01");

if (!existsSync(shimPath)) {
  throw new Error(`missing generated Worker entry: ${shimPath}`);
}

// The entry must be exactly what the generator emits for this build: routing
// data plus a re-export of the workers-rs module, with no request handling of
// its own. Routing behaviour is proven by the Rust unit tests and by the local
// runtime boundary tests, not by the spelling of generated source.
const { assetPrefix, paths, leptos } = await projectMetadata(root);
const entry = await readFile(shimPath, "utf8");
const expected = workerEntry({ assetPrefix, aliases: await hydrationAliases(paths.site, leptos["output-name"]) });
if (entry !== expected) {
  throw new Error("build/_worker.js differs from the generated entry for this build; rerun bash ./scripts/build-edge.sh");
}
for (const module of ["build/index.js", "build/index_bg.wasm"]) {
  if (!existsSync(join(root, module))) throw new Error(`missing workers-rs module: ${module}`);
}

console.log("[verify-worker-runtime] generated entry (routing data and workers-rs re-export only), compatibility date, sampled observability and Assets binding are aligned");
