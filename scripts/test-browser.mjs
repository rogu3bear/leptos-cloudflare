import assert from "node:assert/strict";
import { mkdtemp, readdir, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { projectMetadata } from "./project-metadata.mjs";
import { requireBaseline } from "./hydration-history.mjs";
import { withLocalPreview } from "./local-preview.mjs";
import processes from "./preserve-guard-fds.cjs";

export async function qualifyBrowser(target, pagesReleaseDirectory = null) {
  assert(["workers", "pages"].includes(target));
  const { referenceSite, leptos } = await projectMetadata();
  const readyPath = process.env.LEPTOS_BOUNDARY_DOCUMENT_PATH || "/";
  const results = await mkdtemp(join(tmpdir(), "leptos-browser-results-"));
  await withLocalPreview({ pages: target === "pages", pagesReleaseDirectory, readyPath, migrate: true }, async origin => {
    const history = resolve(process.env.LEPTOS_HYDRATION_HISTORY || "var/hydration-history", leptos["output-name"]);
    const baseline = process.env.LEPTOS_RELEASE_BASELINE;
    await requireBaseline(history, leptos["output-name"], baseline);
    const bundles = await readdir(history);
    if (baseline !== "first-release") assert(bundles.includes(baseline), "Outgoing bundle missing from HTTP checks");
    assert(bundles.length > 0, "Missing hydration history for the build");
    for (const id of bundles) {
      const archived = join(history, id);
      const manifest = JSON.parse(await readFile(join(archived, "asset-manifest.json"), "utf8"));
      for (const kind of ["js", "wasm", "css"]) {
        const path = manifest[kind];
        const response = await fetch(origin + path);
        assert.equal(response.status, 200, `Unavailable archived ${path} on ${target}`);
        assert.deepEqual(Buffer.from(await response.arrayBuffer()), await readFile(join(archived, path.slice(1))));
      }
    }
    const child = processes.spawn("node", [resolve("node_modules/playwright/cli.js"), "test",
      "--config", resolve("scripts/browser/playwright.config.cjs"), "--reporter=list"], {
      stdio: ["ignore", "inherit", "inherit"],
      env: { ...process.env, LEPTOS_BROWSER_ORIGIN: origin, LEPTOS_BROWSER_PATH: readyPath,
        LEPTOS_BROWSER_REFERENCE: referenceSite ? "1" : "0", LEPTOS_BROWSER_RESULTS: results },
    });
    const code = await new Promise((done, fail) => { child.once("error", fail); child.once("close", done); });
    assert.equal(code, 0, `WebKit failed on ${pagesReleaseDirectory || target}. Results: ${results}`);
    console.log(`[browser] ${pagesReleaseDirectory || target}: ${referenceSite ? "hydration and reference server functions" : "adopted hydration and application server function"} passed`);
  });
}

if (import.meta.main) await qualifyBrowser(process.argv[2]);
