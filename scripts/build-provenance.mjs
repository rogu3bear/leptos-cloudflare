import { createHash } from "node:crypto";
import { lstat, readFile, readdir, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

export const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");

export async function inventory(root, paths) {
  const files = [];
  async function visit(path) {
    const info = await lstat(join(root, path));
    if (info.isDirectory()) {
      for (const child of (await readdir(join(root, path))).sort()) await visit(`${path}/${child}`);
    } else {
      if (!info.isFile()) throw new Error(`Release input is not a regular file: ${path}`);
      files.push({ path, sha256: digest(await readFile(join(root, path))), executable: Boolean(info.mode & 0o111) });
    }
  }
  for (const path of [...paths].sort()) await visit(path);
  return files;
}

export async function sourceSnapshot(root) {
  const paths = ["Cargo.toml", "Cargo.lock", ".cargo", ".cfctl/operations/d1-migrations.toml", "migrations", "package.json", "bun.lock", "src", "scripts", "style", "assets", "wrangler.toml", "pages/wrangler.toml", "pages/package.json"];
  for (const optional of ["build.rs", "rust-toolchain", "rust-toolchain.toml"]) {
    try { await lstat(join(root, optional)); paths.push(optional); }
    catch (error) { if (error.code !== "ENOENT") throw error; }
  }
  const files = await inventory(root, paths);
  const git = (...args) => spawnSync("git", ["-C", root, ...args], { encoding: "utf8" });
  const top = git("rev-parse", "--show-toplevel");
  const isRoot = top.status === 0 && resolve(top.stdout.trim()) === resolve(root);
  const head = isRoot ? git("rev-parse", "HEAD") : null;
  const status = isRoot ? git("status", "--porcelain") : null;
  if (isRoot && (head.status !== 0 || status.status !== 0)) throw new Error("Cannot establish build Git identity");
  return {
    commit: head?.stdout.trim() ?? null,
    dirty: status ? Boolean(status.stdout) : true,
    inputs: digest(JSON.stringify(files)),
    files,
  };
}

export async function validateBuild(root) {
  const receipt = JSON.parse(await readFile(join(root, "build/provenance.json"), "utf8"));
  if (JSON.stringify(receipt.source) !== JSON.stringify(await sourceSnapshot(root))) throw new Error("Build source changed; rebuild the edge artifacts");
  const artifacts = await inventory(root, ["build/_worker.js", "build/index.js", "build/index_bg.wasm", "target/site"]);
  if (JSON.stringify(receipt.artifacts) !== JSON.stringify(artifacts)) throw new Error("Build artifacts changed; rebuild the edge artifacts");
  return receipt;
}

if (import.meta.main) {
  const root = process.cwd();
  const snapshotPath = join(root, "target/build-source-start.json");
  if (process.argv[2] === "start") {
    const { mkdir } = await import("node:fs/promises");
    await mkdir(join(root, "target"), { recursive: true });
    await writeFile(snapshotPath, JSON.stringify(await sourceSnapshot(root)));
  } else if (process.argv[2] === "finish") {
    const source = await sourceSnapshot(root);
    const before = JSON.parse(await readFile(snapshotPath, "utf8"));
    if (JSON.stringify(before) !== JSON.stringify(source)) {
      const old = new Map(before.files.map((file) => [file.path, file.sha256]));
      const changed = source.files.filter((file) => old.get(file.path) !== file.sha256).map((file) => file.path);
      throw new Error(`Source changed during edge build (${changed.join(", ") || "Git identity or file modes"}); rebuild before release`);
    }
    const artifacts = await inventory(root, ["build/_worker.js", "build/index.js", "build/index_bg.wasm", "target/site"]);
    await writeFile(join(root, "build/provenance.json"), JSON.stringify({ schema: 1, source, artifacts }, null, 2) + "\n");
  } else throw new Error("Usage: bun scripts/build-provenance.mjs start|finish");
}
