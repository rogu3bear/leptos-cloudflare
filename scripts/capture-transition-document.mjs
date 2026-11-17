import assert from "node:assert/strict";
import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { withLocalPreview } from "./local-preview.mjs";
import { projectMetadata } from "./project-metadata.mjs";

const [checkout, path, destination] = process.argv.slice(2);
assert(checkout && path?.startsWith("/") && !path.startsWith("//") && destination,
  "Usage: bun scripts/capture-transition-document.mjs <outgoing-checkout> <app-path> <new-json-file>");
const root = resolve(checkout);
const { paths } = await projectMetadata(root);
const manifest = JSON.parse(await readFile(paths.manifest, "utf8"));
await withLocalPreview({ root, migrate: true, readyPath: path }, async origin => {
  const response = await fetch(origin + path);
  assert.equal(response.status, 200);
  const body = await response.text();
  const headers = Object.fromEntries(["content-type", "content-security-policy"].map(key => [key, response.headers.get(key)]));
  assert(headers["content-security-policy"] && headers["content-type"]?.includes("text/html"));
  for (const kind of ["js", "css"]) assert(body.includes(manifest[kind]), `Outgoing document lacks ${kind} URL`);
  const baseline = ["js", "wasm", "css"].map(kind => manifest.hashes[kind]).join("-");
  await writeFile(destination, JSON.stringify({ path, baseline, headers, body }), { flag: "wx" });
});
