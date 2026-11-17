#!/usr/bin/env bun
// Explicit heavy proof; run alone, then rebuild the canonical reference.
import { strict as assert } from "node:assert";
import { cp, mkdir, mkdtemp, readFile, realpath, rm, symlink, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { basename, dirname, join, resolve } from "node:path";
import { adoptProject } from "./init-project.mjs";
import processes from "./preserve-guard-fds.cjs";

const root = process.cwd();
let fixture;
if (process.argv.length > 2) {
  assert.equal(process.argv[2], "--fixture");
  assert.equal(process.argv.length, 4);
  fixture = resolve(process.argv[3]);
  assert.equal(dirname(fixture), await realpath(tmpdir()), "explicit fixture must be directly under the system temporary directory");
  assert.match(basename(fixture), /^leptos-cf-adopted-runtime-[A-Za-z0-9-]+$/);
  await mkdir(fixture, { mode: 0o700 });
} else fixture = await mkdtemp(join(tmpdir(), "leptos-cf-adopted-runtime-"));
try {
  for (const file of ["Cargo.toml", "Cargo.lock", "file-limits.json", "package.json", "bun.lock", ".cargo", ".cfctl", "wrangler.toml", "pages", "runtime", "src", "scripts", "docs", "migrations", "style", "assets"]) {
    await cp(join(root, file), join(fixture, file), { recursive: true });
  }
  // Share only generated outputs/tool caches, never source or provider config.
  await mkdir(join(root, "target"), { recursive: true });
  await mkdir(join(root, "var"), { recursive: true });
  await symlink(join(root, "target"), join(fixture, "target"));
  await symlink(join(root, "var"), join(fixture, "var"));
  await symlink(join(root, "node_modules"), join(fixture, "node_modules"));
  await adoptProject(fixture, "acceptance-app");
  const pages = Bun.TOML.parse(await readFile(join(fixture, "pages/wrangler.toml"), "utf8"));
  assert.equal(pages.name, "acceptance-app");
  assert.equal(pages.d1_databases[0].database_name, "acceptance-app-db");
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
  const history = join(fixture, "hydration-history");
  await mkdir(history);
  for (const command of [
    [process.execPath, "scripts/verify-architecture-contract.mjs"],
    ["bash", "scripts/build-edge.sh"],
    [process.execPath, "scripts/test-worker-boundaries.mjs"],
  ]) {
    const env = { ...process.env, LEPTOS_BOUNDARY_DOCUMENT_PATH: "/adopted",
      LEPTOS_HYDRATION_HISTORY: history, LEPTOS_RELEASE_BASELINE: "first-release" };
    let launch = command;
    if (command[0] === "bash" && process.env.DISK_GUARD_RESERVATION) {
      const budget = process.env.LEPTOS_ACCEPTANCE_BUILD_BUDGET_GB;
      const request = budget ? ["--budget-gb", budget, "--volume", fixture, "--build-lane", await realpath(join(root, "target"))] : [];
      const explanation = processes.spawnSync("disk-guard", ["explain", "--format", "text", ...request, "--", ...command], { cwd: fixture, env, encoding: "utf8" });
      assert.equal(explanation.status, 0, explanation.stdout + explanation.stderr);
      launch = ["disk-guard", "run", ...request, "--", ...command];
    }
    const child = processes.spawn(launch[0], launch.slice(1), { cwd: fixture, env, stdio: ["ignore", "inherit", "inherit"] });
    const exit = await new Promise((resolve, reject) => { child.once("close", resolve); child.once("error", reject); });
    assert.equal(exit, 0, `${command.join(" ")} failed in the adopted application`);
  }
  console.log("[adopted-runtime] renamed application, replacement page, added route 200, and unknown route 404 passed");
} finally {
  // rm unlinks the fixture's target/var symlinks; shared caches remain owned by root.
  await rm(fixture, { recursive: true, force: true });
}
