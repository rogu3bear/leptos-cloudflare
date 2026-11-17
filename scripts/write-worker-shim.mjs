#!/usr/bin/env bun
import { access, copyFile, mkdir, writeFile } from "node:fs/promises";
import { projectMetadata } from "./project-metadata.mjs";
import { workerEntry } from "./worker-entry.mjs";

import { hydrationAliases } from "./hydration-routes.mjs";

const { assetPrefix, paths, leptos } = await projectMetadata();
await access("build/index.js");
await mkdir("build", { recursive: true });
await copyFile(new URL("../runtime/dispatch.mjs", import.meta.url), "build/dispatch.mjs");
const aliases = await hydrationAliases(paths.site, leptos["output-name"]);
await writeFile("build/_worker.js", workerEntry({ assetPrefix, aliases }));
console.log("[write-worker-shim] wrote build/_worker.js");
