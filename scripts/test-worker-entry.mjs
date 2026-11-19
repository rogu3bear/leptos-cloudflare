#!/usr/bin/env bun
import { strict as assert } from "node:assert";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { EDGE_MANIFEST_GLOBAL, edgeManifest, workerEntry } from "./worker-entry.mjs";

// The generated entry must add nothing between Cloudflare and the workers-rs
// module: same default export, same named exports, and only data on the side.
const fixture = await mkdtemp(join(tmpdir(), "leptos-worker-entry-"));
try {
  const stub = 'export default class LeptosWorker { fetch() { return "rust"; } }\nexport class RealtimeRoom {}\nexport const marker = "workers-rs";\n';
  await mkdir(join(fixture, "functions"));
  await writeFile(join(fixture, "index.js"), stub);
  await writeFile(join(fixture, "functions/index.js"), stub);

  const aliases = { "/api/create_todo111": "/api/create_todo222" };
  const pages = { serverModule: "./functions/index.js", assetPrefix: "/pkg/", assetPaths: ["/logo.svg", "/pkg/app.js"], blockServerModules: true, aliases };
  const variants = { "defaults.mjs": {}, "workers.mjs": { assetPrefix: "/static/", aliases }, "pages.mjs": pages };
  // Write every entry before the first import: Bun caches directory listings.
  for (const [name, options] of Object.entries(variants)) {
    const source = workerEntry(options);
    assert.equal(source.split("\n").length, 4, "the entry is one data statement and two re-exports");
    await writeFile(join(fixture, name), source);
  }

  async function load(name) {
    delete globalThis[EDGE_MANIFEST_GLOBAL];
    const entry = await import(pathToFileURL(join(fixture, name)).href);
    const server = await import(pathToFileURL(join(fixture, variants[name].serverModule ?? "./index.js")).href);
    assert.equal(entry.default, server.default, "the default export must be the workers-rs class itself");
    assert.deepEqual(Object.keys(entry).sort(), Object.keys(server).sort(), "every workers-rs export must pass through unchanged");
    assert.equal(entry.RealtimeRoom, server.RealtimeRoom, "a Durable Object class must reach Wrangler through the entry");
    const manifest = globalThis[EDGE_MANIFEST_GLOBAL];
    assert.ok(Object.isFrozen(manifest), "routing data must be frozen");
    assert.deepEqual(Object.keys(manifest), ["assetPrefix", "assetPaths", "aliases", "blockServerModules"], "routing data fields are a closed contract with src/edge/route.rs");
    return manifest;
  }

  assert.deepEqual(await load("defaults.mjs"), { assetPrefix: "/pkg/", assetPaths: null, aliases: {}, blockServerModules: false });
  assert.deepEqual(await load("workers.mjs"), { assetPrefix: "/static/", assetPaths: null, aliases, blockServerModules: false });
  assert.deepEqual(await load("pages.mjs"), edgeManifest(pages));
  assert.equal(JSON.stringify(edgeManifest(pages)).includes("serverModule"), false, "the module path is not routing data");
  assert.throws(() => { "use strict"; globalThis[EDGE_MANIFEST_GLOBAL].blockServerModules = false; }, TypeError);

  // Quotes and line breaks in a path stay inside the string literal.
  const hostile = workerEntry({ serverModule: './index.js";\nthrow 1;//' });
  assert.equal(hostile.split("\n").length, 4);
  console.log("[worker-entry] the generated entry re-exports the workers-rs module unchanged and publishes frozen routing data");
} finally {
  delete globalThis[EDGE_MANIFEST_GLOBAL];
  await rm(fixture, { recursive: true, force: true });
}
