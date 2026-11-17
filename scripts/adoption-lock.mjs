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
  const records = packages.map(block => {
    const [{ name, source, version }] = Bun.TOML.parse(block).package;
    return { name, source, version, block };
  });
  const roots = records.filter(pkg => pkg.name === previous && !pkg.source);
  assert.equal(roots.length, 1, "Expected one local package in Cargo.lock");
  const root = roots[0];
  root.name = name;
  root.block = root.block.replace(`name = "${previous}"`, `name = "${name}"`);
  // References to a registry dependency gain or lose qualifiers when its
  // name starts or stops matching the local package. Preserve its identity.
  function reference(pkg, siblings) {
    if (siblings.length === 1) return pkg.name;
    const versioned = `${pkg.name} ${pkg.version}`;
    return siblings.filter(s => s.version === pkg.version).length === 1
      ? versioned : `${versioned} (${pkg.source})`;
  }
  const replacements = new Map();
  for (const affected of [previous, name]) {
    const deps = records.filter(pkg => pkg !== root && pkg.name === affected);
    const before = affected === previous ? [...deps, root] : deps;
    const after = affected === name ? [...deps, root] : deps;
    for (const pkg of deps) replacements.set(reference(pkg, before), reference(pkg, after));
  }
  for (const pkg of records) {
    pkg.block = pkg.block.replace(/^ "([^"]+)",$/gm, (line, ref) =>
      replacements.has(ref) ? ` "${replacements.get(ref)}",` : line);
  }
  const others = records.filter(pkg => pkg !== root);
  const index = others.findIndex(pkg => pkg.name > name ||
    (pkg.name === name && Bun.semver.order(pkg.version, root.version) >= 0));
  others.splice(index < 0 ? others.length : index, 0, root);
  return [header, ...others.map(pkg => pkg.block)].join("\n\n") + trailer;
}

export function renamedPolicy(original, lock, nextLock) {
  const policy = JSON.parse(original);
  const exception = lockException(original, lock);
  policy.exceptions.find(entry => entry.path === exception.path && entry.kind === exception.kind).sha256 = digest(nextLock);
  return JSON.stringify(policy, null, 2) + "\n";
}

if (import.meta.main) lockException(await readFile("file-limits.json", "utf8"), await readFile("Cargo.lock", "utf8"));
