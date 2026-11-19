#!/usr/bin/env bun
import { access, mkdir, rm, writeFile } from "node:fs/promises";
import { projectMetadata } from "./project-metadata.mjs";
import { workerEntry } from "./worker-entry.mjs";

import { hydrationAliases } from "./hydration-routes.mjs";

const { assetPrefix, paths, leptos } = await projectMetadata();
await access("build/index.js");
await mkdir("build", { recursive: true });
// Earlier builds copied a JavaScript dispatch module here; it is not part of the entry.
await rm("build/dispatch.mjs", { force: true });
const aliases = await hydrationAliases(paths.site, leptos["output-name"]);
await writeFile("build/_worker.js", workerEntry({ assetPrefix, aliases }));
console.log("[write-worker-shim] wrote build/_worker.js");
