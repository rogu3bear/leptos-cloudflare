import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";

const digest = text => createHash("sha256").update(text).digest("hex");

export function lockException(policy, lock) {
  const matches = JSON.parse(policy).exceptions.filter(entry => entry.path === "Cargo.lock" && entry.kind === "generated-text");
  assert.equal(matches.length, 1, "Expected one generated Cargo.lock exception");
  assert.equal(matches[0].sha256, digest(lock), "Review Cargo.lock changes before updating its inventory hash");
  return matches[0];
}

export function renameLock(lock, previous, name) {
  // Keep Cargo's existing version/source order for every dependency. Move only
  // the renamed local package to the position Cargo's serializer will use.
  const trailer = lock.match(/\n\n\[metadata\][\s\S]*$|\s+$/)?.[0] ?? "";
  const body = lock.slice(0, lock.length - trailer.length);
  const [header, ...packages] = body.split(/\n\n(?=\[\[package\]\]\n)/);
  const packageName = block => /^\[\[package\]\]\nname = "([^"]+)"/.exec(block)?.[1];
  const roots = packages.filter(block => packageName(block) === previous && !/^source =/m.test(block));
  assert.equal(roots.length, 1, "Expected one local package in Cargo.lock");
  assert(!packages.some(block => packageName(block) === name), "Project name collides with a locked dependency");
  const others = packages.filter(block => block !== roots[0]);
  const index = others.findIndex(block => packageName(block) > name);
  others.splice(index < 0 ? others.length : index, 0, roots[0].replace('name = "' + previous + '"', 'name = "' + name + '"'));
  return [header, ...others].join("\n\n") + trailer;
}

export function renamedPolicy(original, lock, nextLock) {
  const policy = JSON.parse(original);
  const exception = lockException(original, lock);
  policy.exceptions.find(entry => entry.path === exception.path && entry.kind === exception.kind).sha256 = digest(nextLock);
  return JSON.stringify(policy, null, 2) + "\n";
}

if (import.meta.main) lockException(await readFile("file-limits.json", "utf8"), await readFile("Cargo.lock", "utf8"));
