// Keep complete, content-verified client bundles outside cargo-leptos's site output.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { lstat, mkdir, mkdtemp, readFile, readdir, rename, rm, writeFile } from "node:fs/promises";
import { join, dirname, resolve } from "node:path";
import { projectMetadata } from "./project-metadata.mjs";

const kinds = ["js", "wasm", "css"];
const hash = bytes => createHash("sha256").update(bytes).digest("hex").slice(0, 16);

async function regular(path) {
  assert((await lstat(path)).isFile(), `Expected regular hydration asset: ${path}`);
  return readFile(path);
}

async function bundle(site, name) {
  assert((await lstat(site)).isDirectory(), "Expected regular bundle directory");
  assert((await lstat(join(site, "pkg"))).isDirectory(), "Expected regular pkg directory");
  const manifest = JSON.parse(await regular(join(site, "asset-manifest.json")));
  const files = {};
  for (const kind of kinds) {
    const digest = manifest.hashes?.[kind];
    assert(/^[a-f0-9]{16}$/.test(digest), `Invalid ${kind} hash`);
    const filename = `${name}.${digest}.${kind}`;
    assert.equal(manifest[kind], `/pkg/${filename}`);
    const bytes = await regular(join(site, "pkg", filename));
    assert.equal(hash(bytes), digest, `Hydration bytes changed: ${filename}`);
    files[kind] = { filename, bytes };
  }
  assert(files.js.bytes.includes(`new URL("${files.wasm.filename}",import.meta.url)`), "JS/WASM pair mismatch");
  return { manifest, files, id: kinds.map(k => manifest.hashes[k]).join("-") };
}

export async function requireBaseline(history, name, baseline) {
  if (baseline === "first-release") return;
  assert(typeof baseline === "string" && /^[a-f0-9]{16}(-[a-f0-9]{16}){2}$/.test(baseline),
    "Set LEPTOS_RELEASE_BASELINE to the outgoing bundle ID, or first-release for an initial release");
  const archived = await bundle(join(history, baseline), name).catch(error => {
    throw new Error(`Required outgoing hydration bundle ${baseline} is missing or invalid: ${error.message}`);
  });
  assert.equal(archived.id, baseline, "Outgoing bundle identity mismatch");
}

export async function saveBundle(site, history, name) {
  const input = await bundle(site, name);
  await mkdir(history, { recursive: true });
  const dest = join(history, input.id);
  const tmp = await mkdtemp(join(dirname(history), ".hydration-save-"));
  try {
    await mkdir(join(tmp, "pkg"));
    for (const {filename, bytes} of Object.values(input.files)) {
      await writeFile(join(tmp, "pkg", filename), bytes);
    }
    await writeFile(join(tmp, "asset-manifest.json"), JSON.stringify(input.manifest));
    try { await rename(tmp, dest); }
    catch (error) {
      if (!["ENOTEMPTY", "EEXIST"].includes(error.code)) throw error;
      assert((await lstat(dest)).isDirectory(), "Invalid hydration bundle directory");
      const existing = await bundle(dest, name);
      for (const kind of kinds) assert(existing.files[kind].bytes.equals(input.files[kind].bytes), "Immutable asset collision");
    }
  } finally { await rm(tmp, { recursive: true, force: true }); }
  return input.id;
}

export async function restoreBundles(site, history, name) {
  assert((await lstat(join(site, "pkg"))).isDirectory(), "Expected regular pkg directory");
  const entries = await readdir(history, { withFileTypes: true });
  for (const entry of entries) {
    assert(entry.isDirectory() && /^[a-f0-9]{16}(-[a-f0-9]{16}){2}$/.test(entry.name), "Invalid hydration history entry");
    const input = await bundle(join(history, entry.name), name);
    assert.equal(input.id, entry.name, "Hydration history identity mismatch");
    for (const {filename, bytes} of Object.values(input.files)) {
      const path = join(site, "pkg", filename);
      try { await writeFile(path, bytes, { flag: "wx" }); }
      catch (error) {
        if (error.code !== "EEXIST") throw error;
        assert((await regular(path)).equals(bytes), `Immutable asset collision: ${filename}`);
      }
    }
  }
}

export async function capturePrevious(site, history, name, explicitHistory) {
  // Explicit histories can share a build target across environments.
  // Only an explicit capture may import another build into that history.
  if (explicitHistory) return;
  let manifest;
  try { manifest = JSON.parse(await regular(join(site, "asset-manifest.json"))); }
  catch (error) { if (error.code === "ENOENT") return; throw error; }
  if (!manifest.js.startsWith(`/pkg/${name}.`)) return;
  try { await saveBundle(site, history, name); }
  catch (error) {
    if (error.code !== "ENOENT") throw error;
    // Cargo may have cleared pkg before a previous build was interrupted.
    const hashes = kinds.map(k => manifest.hashes[k]);
    assert(hashes.every(h => /^[a-f0-9]{16}$/.test(h)), "Invalid previous manifest");
    const id = hashes.join("-");
    const archived = await bundle(join(history, id), name);
    assert.equal(archived.id, id);
    for (const kind of kinds) assert.equal(manifest[kind], archived.manifest[kind]);
  }
}

if (import.meta.main) {
  const project = await projectMetadata();
  const name = project.leptos["output-name"];
  const base = process.env.LEPTOS_HYDRATION_HISTORY;
  if (base) assert((await lstat(base)).isDirectory(), "Restore the selected history directory before building");
  const history = join(resolve(base || "var/hydration-history"), name);
  const [mode, site, ...extra] = process.argv.slice(2);
  assert(!extra.length && ((mode === "capture" && site) || (["check-baseline", "capture-previous", "publish-assets"].includes(mode) && !site)),
    "Usage: hydration-history.mjs capture <trusted-site> | check-baseline | capture-previous | publish-assets");
  if (mode === "check-baseline" || (process.env.LEPTOS_RELEASE_BASELINE && mode !== "capture")) {
    await requireBaseline(history, name, process.env.LEPTOS_RELEASE_BASELINE);
  }
  if (mode === "capture") console.log(await saveBundle(resolve(site), history, name));
  else if (mode === "capture-previous") await capturePrevious(project.paths.site, history, name, Boolean(base));
  else if (mode === "publish-assets") {
    await saveBundle(project.paths.site, history, name);
    await restoreBundles(project.paths.site, history, name);
  }
}
