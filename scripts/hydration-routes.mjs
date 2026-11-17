// Leptos's default endpoint suffix includes the absolute Cargo manifest directory.
// Preserve only the six established template contracts found in retained clients.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";

function routes(bytes) {
  const result = new Map();
  const paths = bytes.toString("latin1").matchAll(/\/api\/(list_todos|create_todo|toggle_todo|delete_todo|get_todo|submit_contact)([0-9]{1,20})(?![0-9])/g);
  for (const [path, name] of paths) {
    assert(!result.has(name) || result.get(name) === path, `Ambiguous server-function path for ${name}`);
    result.set(name, path);
  }
  return result;
}

export async function hydrationAliases(site, name) {
  const manifest = JSON.parse(await readFile(join(site, "asset-manifest.json")));
  const pattern = new RegExp(`^${name}\\.([0-9a-f]{16})\\.wasm$`);
  const clients = new Map();
  for (const entry of await readdir(join(site, "pkg"), { withFileTypes: true })) {
    const match = pattern.exec(entry.name);
    if (!match) continue;
    assert(entry.isFile(), "Expected regular client WASM");
    const bytes = await readFile(join(site, "pkg", entry.name));
    assert.equal(createHash("sha256").update(bytes).digest("hex").slice(0, 16), match[1]);
    clients.set("/pkg/" + entry.name, routes(bytes));
  }
  const active = clients.get(manifest.wasm);
  assert(active, "Missing active client WASM");
  const aliases = {};
  for (const client of clients.values()) {
    for (const [name, previous] of client) {
      const current = active.get(name);
      if (current && previous !== current) aliases[previous] = current;
    }
  }
  return aliases;
}
