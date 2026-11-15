#!/usr/bin/env bun
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { lstat, readFile } from "node:fs/promises";
import { basename, join } from "node:path";

export function measureFile(path, bytes, policy = {}) {
  const instruction = /^(AGENTS|CLAUDE|GEMINI|SKILL|INSTRUCTIONS|CONSTITUTION|ANCHOR|NORTH_STAR)\.md$/i.test(basename(path)) || (policy.instructionPaths ?? []).includes(path);
  const limit = instruction ? 799 : 2000;
  let content;
  try { content = new TextDecoder("utf-8", { fatal: true }).decode(bytes); }
  catch { content = null; }
  const binary = content === null || bytes.includes(0);
  const lines = binary ? null : content.split("\n").length - Number(content.endsWith("\n") || content.length === 0);
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  const exception = (policy.exceptions ?? []).find((entry) => entry.path === path && entry.owner && entry.reason && entry.review && entry.sha256 === sha256 && (binary ? entry.kind === "binary" : entry.kind === "generated-text"));
  return { path, kind: binary ? "binary" : instruction ? "instruction" : /(^|\/)\w+\.lock$/.test(path) ? "lockfile" : "text", bytes: bytes.length, sha256, lines, limit, exception: exception ?? null, status: exception ? "excepted" : binary ? "unresolved-binary" : lines > limit ? "over-limit" : "pass" };
}

function gitPaths(root, args) {
  const result = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  if (result.status !== 0) throw new Error(result.stderr || "Git inventory failed");
  return result.stdout.split("\0").filter(Boolean);
}

export async function inspectFiles(root, policy = {}) {
  const files = [...new Set(gitPaths(root, ["ls-files", "--cached", "--others", "--exclude-standard", "-z"]))].sort();
  // Ignored contents are never opened. These three local guidance files are
  // explicit instruction owners, and only their counts enter this report.
  for (const path of ["AGENTS.md", "ANCHOR.md", "NORTH_STAR.md"]) {
    try { await lstat(join(root, path)); if (!files.includes(path)) files.push(path); }
    catch (error) { if (error.code !== "ENOENT") throw error; }
  }
  const rows = [];
  for (const path of files.sort()) {
    const name = basename(path);
    const sensitive = name !== ".env.example" && (/^\.env(?:$|\.)/.test(name) || /^\.dev\.vars(?:$|\.)/.test(name))
      || /\.(?:pem|key|p12|pfx)$/i.test(name) || /^(?:id_rsa|id_ed25519|wrangler\.production\.toml)$/.test(name);
    if (sensitive) {
      rows.push({ path, status: "unverified-sensitive", lines: null }); continue;
    }
    try {
      const stat = await lstat(join(root, path));
      if (!stat.isFile()) { rows.push({ path, status: "unverified-nonregular", lines: null }); continue; }
      rows.push(measureFile(path, await readFile(join(root, path)), policy));
    } catch (error) { rows.push({ path, status: "unverified-unreadable", reason: error.code, lines: null }); }
  }
  const ignored = gitPaths(root, ["ls-files", "--others", "--ignored", "--exclude-standard", "--directory", "-z"]).filter((path) => !files.includes(path));
  const ignoredException = policy.ignoredBoundary;
  const ignoredApproved = Boolean(ignoredException?.owner && ignoredException?.reason && ignoredException?.review && Array.isArray(ignoredException.paths) && ignored.every((path) => ignoredException.paths.includes(path)));
  const issues = rows.filter((row) => !["pass", "excepted"].includes(row.status));
  return { population: "tracked and non-ignored files, plus explicit top-level local guidance", files: rows, ignored: { paths: ignored, contentsRead: false, exception: ignoredApproved ? ignoredException : null }, issues, passed: issues.length === 0 && (ignored.length === 0 || ignoredApproved) };
}

if (import.meta.main) {
  const reportOnly = process.argv.includes("--report");
  let policy = {};
  try { policy = JSON.parse(await readFile("file-limits.json", "utf8")); }
  catch (error) { if (error.code !== "ENOENT") throw error; }
  const result = await inspectFiles(process.cwd(), policy);
  console.log(JSON.stringify(result, null, 2));
  if (!reportOnly && !result.passed) process.exitCode = 1;
}
