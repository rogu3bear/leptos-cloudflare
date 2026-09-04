#!/usr/bin/env bun
// Explicit heavy proof; run alone, then rebuild the canonical reference.
import { strict as assert } from "node:assert";
import { cp, mkdir, mkdtemp, readFile, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { adoptProject } from "./init-project.mjs";

const root = process.cwd();
const fixture = await mkdtemp(join(tmpdir(), "leptos-cf-adopted-runtime-"));
try {
  for (const file of ["Cargo.toml", "Cargo.lock", ".cargo", ".cfctl", "wrangler.toml", "src", "scripts", "docs", "migrations", "style", "assets"]) {
    await cp(join(root, file), join(fixture, file), { recursive: true });
  }
  // Share only generated outputs/tool caches, never source or provider config.
  await mkdir(join(root, "target"), { recursive: true });
  await mkdir(join(root, "var"), { recursive: true });
  await symlink(join(root, "target"), join(fixture, "target"));
  await symlink(join(root, "var"), join(fixture, "var"));
  await adoptProject(fixture, "acceptance-app");
  await writeFile(join(fixture, "src/components/home_page.rs"), `use leptos::prelude::*;
#[component]
pub fn HomePage() -> impl IntoView {
    view! { <section><h1>"Application acceptance page"</h1><p>"A new app route rendered on the Worker."</p></section> }
}
`);
  const app = join(fixture, "src/app.rs");
  const source = await readFile(app, "utf8");
  assert.ok(source.includes('<Route path=WildcardSegment("any")'));
  await writeFile(app, source.replace('<Route path=WildcardSegment("any")', '<Route path=StaticSegment("adopted") view=HomePage ssr=SsrMode::OutOfOrder/>\n                    <Route path=WildcardSegment("any")'));
  for (const command of [
    [process.execPath, "scripts/verify-architecture-contract.mjs"],
    ["bash", "scripts/build-edge.sh"],
    [process.execPath, "scripts/test-worker-boundaries.mjs"],
  ]) {
    const process = Bun.spawn(command, { cwd: fixture, env: { ...globalThis.process.env, LEPTOS_BOUNDARY_DOCUMENT_PATH: "/adopted" }, stdout: "inherit", stderr: "inherit" });
    const exit = await process.exited;
    assert.equal(exit, 0, `${command.join(" ")} failed in the adopted application`);
  }
  console.log("[adopted-runtime] renamed application, replacement page, added route 200, and unknown route 404 passed");
} finally {
  // rm unlinks the fixture's target/var symlinks; shared caches remain owned by root.
  await rm(fixture, { recursive: true, force: true });
}
