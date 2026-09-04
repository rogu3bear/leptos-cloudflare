#!/usr/bin/env bun

import { readFile } from "node:fs/promises";
import { projectMetadata } from "./project-metadata.mjs";

const { cargo, leptos, referenceSite } = await projectMetadata();
const wrangler = Bun.TOML.parse(await readFile("wrangler.toml", "utf8"));
function require(condition, message) {
  if (!condition) throw new Error(message);
}
require(cargo.features?.ssr?.includes("leptos/ssr"), "SSR must remain enabled through the ssr feature");
require(cargo.features?.hydrate?.includes("leptos/hydrate"), "hydration must remain enabled through the hydrate feature");
require(!Object.values(cargo.features ?? {}).flat().includes("leptos/csr"), "pure CSR requires a separate runtime contract");
require(leptos["bin-features"]?.includes("ssr") && leptos["lib-features"]?.includes("hydrate"), "Leptos build must retain the SSR/hydration split");
require(wrangler.main === "build/_worker.js", "Worker entrypoint must be the generated shim");
require(wrangler.assets?.binding === "ASSETS" && wrangler.assets.directory.replace(/^\.\//, "") === leptos["site-root"], "Workers Assets must bind the Leptos site root");
require(!wrangler.pages_build_output_dir && wrangler.assets.not_found_handling !== "single-page-application", "Pages and SPA fallbacks require a separate runtime contract");
if (referenceSite) await import("./verify-field-guide.mjs");
console.log("[verify-architecture-contract] SSR/hydration and Worker/Assets ownership passed");
