#!/usr/bin/env bun
import { strict as assert } from "node:assert";
import { createHash } from "node:crypto";
import { fingerprintAssets } from "./hash-assets.mjs";

const glue = Buffer.from('const source = new URL("sample.wasm",import.meta.url); export default source;');
const css = Buffer.from("body { color: black; }");
const first = fingerprintAssets("sample", glue, Buffer.from("first wasm payload"), css);
const second = fingerprintAssets("sample", glue, Buffer.from("second wasm payload"), css);
assert.notEqual(first.hashedWasmName, second.hashedWasmName);
assert.notEqual(first.hashedJsName, second.hashedJsName, "WASM-only change must invalidate immutable JS");
assert.equal(first.hashedCssName, second.hashedCssName);
for (const item of [first, second]) {
  assert.equal(item.jsHash, createHash("sha256").update(item.rewrittenJs).digest("hex").slice(0, 16));
  assert.ok(item.rewrittenJs.includes(item.hashedWasmName));
}
assert.deepEqual(fingerprintAssets("sample", glue, Buffer.from("first wasm payload"), css), first);
assert.throws(() => fingerprintAssets("sample", Buffer.from("no import"), Buffer.from("wasm"), css), /exactly one/);
assert.throws(() => fingerprintAssets("sample", Buffer.concat([glue, glue]), Buffer.from("wasm"), css), /exactly one/);
console.log("[asset-fingerprints] final served bytes, WASM-only invalidation, determinism, and malformed imports passed");
