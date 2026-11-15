#!/usr/bin/env bun
import { access, copyFile, mkdir, writeFile } from "node:fs/promises";
import { projectMetadata } from "./project-metadata.mjs";
import { workerEntry } from "./worker-entry.mjs";

const { assetPrefix } = await projectMetadata();
await access("build/index.js");
await mkdir("build", { recursive: true });
await copyFile(new URL("../runtime/dispatch.mjs", import.meta.url), "build/dispatch.mjs");
await writeFile("build/_worker.js", workerEntry({ assetPrefix }));
console.log("[write-worker-shim] wrote build/_worker.js");
