import assert from "node:assert/strict";
import { mkdtemp, mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { fingerprintAssets } from "./hash-assets.mjs";
import { saveBundle, restoreBundles } from "./hydration-history.mjs";

const tmp = await mkdtemp(join(tmpdir(), "hydration-history-"));
const history = join(tmp, "history");
const app = "demo";
async function site(label) {
  const root = join(tmp, label);
  const pkg = join(root, "pkg");
  await mkdir(pkg, { recursive: true });
  const wasm = Buffer.from(label);
  const css = Buffer.from("body{}");
  const f = fingerprintAssets(app, Buffer.from('new URL("demo.wasm",import.meta.url)'), wasm, css);
  const manifest = { hashes: {} };
  for (const [kind, filename, bytes, hash] of [
    ["js", f.hashedJsName, f.rewrittenJs, f.jsHash], ["wasm", f.hashedWasmName, wasm, f.wasmHash],
    ["css", f.hashedCssName, css, f.cssHash],
  ]) {
    await writeFile(join(pkg, filename), bytes);
    manifest[kind] = "/pkg/" + filename;
    manifest.hashes[kind] = hash;
  }
  await writeFile(join(root, "asset-manifest.json"), JSON.stringify(manifest));
  return {root, manifest};
}
try {
  const old = await site("old");
  const next = await site("next");
  await saveBundle(old.root, history, app);
  await Promise.all([saveBundle(next.root, history, app), saveBundle(next.root, history, app)]);
  for (const [active, peer] of [[next, old], [old, next]]) {
    await restoreBundles(active.root, history, app);
    assert.deepEqual(JSON.parse(await readFile(join(active.root, "asset-manifest.json"))), active.manifest);
    for (const kind of ["js", "wasm", "css"]) {
      const path = peer.manifest[kind].slice(1);
      assert.deepEqual(await readFile(join(active.root, path)), await readFile(join(peer.root, path)));
    }
  }
  const oldWasm = join(old.root, old.manifest.wasm.slice(1));
  await writeFile(oldWasm, "corrupt");
  await assert.rejects(saveBundle(old.root, history, app), /bytes changed/);
  await assert.rejects(restoreBundles(old.root, history, app), /collision/);
  await rename(oldWasm, oldWasm + ".saved");
  await restoreBundles(old.root, history, app);
  await assert.rejects(saveBundle(old.root, history, "other"));
  console.log("[hydration-history] forward and rollback bundles, concurrent capture, integrity and isolation passed");
} finally { await rm(tmp, { recursive: true, force: true }); }
