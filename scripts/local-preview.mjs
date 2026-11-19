import processes from "./preserve-guard-fds.cjs";
const { spawn, spawnSync } = processes;
import { once } from "node:events";
import { cp, mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { strict as assert } from "node:assert";
import { projectMetadata } from "./project-metadata.mjs";

async function reservePort() {
  const server = createServer();
  server.unref();
  server.listen(0, "127.0.0.1");
  await once(server, "listening");
  const address = server.address();
  assert(address && typeof address === "object", "failed to reserve a local test port");
  const { port } = address;
  server.close();
  await once(server, "close");
  return port;
}

export async function fetchBoundary(url, init) {
  return fetch(url, {
    ...init,
    redirect: "manual",
    signal: AbortSignal.timeout(5_000),
  });
}

async function waitForWorker(origin, child, logs, documentPath, lifecycle) {
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    if (lifecycle.error || child.exitCode !== null || child.signalCode !== null) {
      throw new Error(`Wrangler exited before readiness.\n${logs()}`);
    }

    try {
      const response = await fetchBoundary(`${origin}${documentPath}`);
      if (response.status > 0) return;
    } catch {
      // Wrangler is still starting.
    }

    await Bun.sleep(250);
  }

  throw new Error(`timed out waiting for Wrangler.\n${logs()}`);
}

async function stopWorker(child, lifecycle) {
  if (lifecycle.closed || lifecycle.error) return;
  child.kill("SIGTERM");
  const closed = await Promise.race([lifecycle.settled.then(() => true), Bun.sleep(5_000).then(() => false)]);
  if (!closed) {
    child.kill("SIGKILL");
    await lifecycle.settled;
  }
}

export async function withLocalPreview({ root = process.cwd(), pages = false, migrate = false, readyPath = "/", pagesReleaseDirectory = null, wrapEntry = null }, use) {
  assert(!pagesReleaseDirectory || pages, "a Pages release directory requires Pages mode");
  assert(!wrapEntry || !pages, "a wrapped test entry requires Workers mode");
  const pagesMode = pages;
  const documentPath = readyPath;
  const { paths } = await projectMetadata(root);
  const workerEntrypoint = resolve(root, "build/_worker.js");
  const assetDirectory = paths.site;
  const assetManifestPath = paths.manifest;
  await Promise.all([
    readFile(workerEntrypoint),
    readFile(assetManifestPath),
  ]).catch(() => {
    throw new Error("missing release artifacts; run bash ./scripts/build-edge.sh first");
  });

  const testDirectory = await mkdtemp(join(tmpdir(), "leptos-cf-boundaries-"));
  let wrangler;
  let output = "";
  const lifecycle = { closed: false, error: null, settled: null };
  try {
  const pagesOutput = pagesReleaseDirectory ? join(pagesReleaseDirectory, "public") : join(testDirectory, "pages");
  if (pagesMode && !pagesReleaseDirectory) {
    const packageRoot = join(testDirectory, "fixture");
    await mkdir(join(packageRoot, "target"), { recursive: true });
    await mkdir(join(packageRoot, "build"));
    await cp(resolve(root, "Cargo.toml"), join(packageRoot, "Cargo.toml"));
    await cp(assetDirectory, join(packageRoot, "target/site"), { recursive: true });
    await writeFile(join(packageRoot, "target/site/__adopter_boundary_logo.svg"), "adopter asset fixture", { flag: "wx" });
    for (const file of ["_worker.js", "index.js", "index_bg.wasm"]) await cp(resolve(root, "build", file), join(packageRoot, "build", file));
    const packaged = spawnSync("bun", [resolve(root, "scripts/package-pages.mjs"), pagesOutput], { cwd: packageRoot, encoding: "utf8" });
    if (packaged.status !== 0) throw new Error(`Pages packaging failed: ${packaged.stderr}`);
  }
  // A wrapped entry lets a test observe the built Worker; it never ships.
  const mainEntrypoint = wrapEntry ? join(testDirectory, "entry.mjs") : workerEntrypoint;
  if (wrapEntry) await writeFile(mainEntrypoint, wrapEntry(workerEntrypoint), { flag: "wx" });
  const configPath = pagesReleaseDirectory ? join(pagesReleaseDirectory, "wrangler.toml") : join(testDirectory, pagesMode ? "wrangler.toml" : "wrangler.test.toml");
  const persistencePath = join(testDirectory, "state");
  const port = await reservePort();
  const inspectorPort = await reservePort();
  const origin = `http://127.0.0.1:${port}`;
  const config = [
    'name = "leptos-cf-boundary-test"',
    pagesMode ? `pages_build_output_dir = ${JSON.stringify(pagesOutput)}` : `main = ${JSON.stringify(mainEntrypoint)}`,
    'compatibility_date = "2026-08-10"',
    "",
    ...(pagesMode ? [] : ["[assets]", `directory = ${JSON.stringify(assetDirectory)}`, 'binding = "ASSETS"']),
    "",
    "[[d1_databases]]",
    'binding = "DB"',
    'database_name = "leptos-cf-boundary-test"',
    'database_id = "00000000-0000-0000-0000-000000000000"',
    `migrations_dir = ${JSON.stringify(resolve(root, "migrations"))}`,
    "",
  ].join("\n");
  if (!pagesReleaseDirectory) await writeFile(configPath, config);
  const databaseName = Bun.TOML.parse(await readFile(configPath, "utf8")).d1_databases[0].database_name;

  if (migrate) {
    const migration = spawnSync(resolve(root, "scripts/wrangler.sh"), [ "d1", "migrations", "apply", databaseName, "--local", "--config", configPath, "--persist-to", persistencePath], {
      cwd: root, env: { ...process.env, CI: "1", NO_COLOR: "1" }, encoding: "utf8",
    });
    if (migration.status !== 0) {
      throw new Error(`local test migrations failed: ${migration.stderr}`);
    }
  }

  wrangler = spawn(
    resolve(root, "scripts/wrangler.sh"),
    [
      ...(pagesMode ? ["pages", "dev", pagesOutput] : ["dev"]),
      ...(pagesMode ? [] : ["--config", configPath, "--local"]),
      "--ip",
      "127.0.0.1",
      "--port",
      String(port),
      "--inspector-port",
      String(inspectorPort),
      "--persist-to",
      persistencePath,
      "--log-level",
      "error",
      "--show-interactive-dev-session=false",
    ],
    {
      cwd: pagesMode ? pagesReleaseDirectory ?? testDirectory : root,
      env: { ...process.env, CI: "1", NO_COLOR: "1" },
      stdio: ["ignore", "pipe", "pipe"],
    },
  );

  lifecycle.settled = new Promise((resolve) => {
    wrangler.once("close", () => { lifecycle.closed = true; resolve(); });
    wrangler.once("error", (error) => { lifecycle.error = error; resolve(); });
  });

  const capture = (chunk) => {
    output = `${output}${chunk}`.slice(-20_000);
  };
  wrangler.stdout.on("data", capture);
  wrangler.stderr.on("data", capture);

    await waitForWorker(origin, wrangler, () => output, documentPath, lifecycle);
    return await use(origin, {
      query(sql) {
        const result = spawnSync(resolve(root, "scripts/wrangler.sh"), ["d1", "execute", databaseName, "--local", "--config", configPath, "--persist-to", persistencePath, "--command", sql, "--json"], {
          cwd: root, env: { ...process.env, CI: "1", NO_COLOR: "1" }, encoding: "utf8",
        });
        if (result.status !== 0) throw new Error(`disposable D1 query failed: ${result.stderr}`);
        return JSON.parse(result.stdout).flatMap((statement) => statement.results ?? []);
      },
    });
  } catch (error) {
    throw new Error(`${error.message}\nWrangler output:\n${output}`);
  } finally {
    if (wrangler) await stopWorker(wrangler, lifecycle);
    await rm(testDirectory, { recursive: true, force: true });
  }
}
