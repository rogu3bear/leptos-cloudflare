#!/usr/bin/env bun

import { spawn } from "node:child_process";
import { once } from "node:events";
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const root = process.cwd();
const workerEntrypoint = resolve(root, "build/_worker.js");
const assetDirectory = resolve(root, "target/site");
const assetManifestPath = join(assetDirectory, "asset-manifest.json");

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

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

async function fetchBoundary(url, init) {
  return fetch(url, {
    ...init,
    redirect: "manual",
    signal: AbortSignal.timeout(5_000),
  });
}

async function waitForWorker(origin, child, logs) {
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    if (child.exitCode !== null) {
      throw new Error(`Wrangler exited before readiness.\n${logs()}`);
    }

    try {
      const response = await fetchBoundary(`${origin}/architecture`);
      if (response.status > 0) return;
    } catch {
      // Wrangler is still starting.
    }

    await Bun.sleep(250);
  }

  throw new Error(`timed out waiting for Wrangler.\n${logs()}`);
}

async function stopWorker(child) {
  if (child.exitCode !== null) return;
  child.kill("SIGTERM");

  const closed = await Promise.race([
    once(child, "close").then(() => true),
    Bun.sleep(5_000).then(() => false),
  ]);

  if (!closed && child.exitCode === null) {
    child.kill("SIGKILL");
    await once(child, "close");
  }
}

async function runAssertions(origin) {
  const documentResponse = await fetchBoundary(`${origin}/architecture`);
  const documentHtml = await documentResponse.text();
  assert(documentResponse.status === 200, `/architecture returned ${documentResponse.status}`);
  assert(
    documentResponse.headers.get("content-type")?.includes("text/html"),
    "/architecture did not return HTML",
  );
  assert(
    documentHtml.includes("SSR + hydration") && documentHtml.includes("Cloudflare Pages"),
    "/architecture lacks useful server-rendered decision content",
  );
  assert(
    documentHtml.includes('<main id="content"'),
    "/architecture lacks the server-rendered main landmark",
  );
  assert(
    documentResponse.headers.get("cache-control") === "no-store",
    "dynamic HTML is not marked no-store",
  );
  assert(
    documentResponse.headers.has("set-cookie"),
    "dynamic HTML did not establish the scoped session cookie",
  );

  const csp = documentResponse.headers.get("content-security-policy") ?? "";
  const nonce = csp.match(/'nonce-([^']+)'/)?.[1];
  assert(nonce, "dynamic HTML CSP is missing a script nonce");
  assert(
    documentHtml.includes(`nonce="${nonce}"`),
    "hydration script nonce does not match the response CSP",
  );

  const manifestResponse = await fetchBoundary(`${origin}/asset-manifest.json`);
  assert(manifestResponse.status === 200, `asset manifest returned ${manifestResponse.status}`);
  assert(
    manifestResponse.headers.get("cache-control") === "no-store",
    "asset manifest is not marked no-store",
  );
  const manifest = await manifestResponse.json();
  assert(
    typeof manifest.js === "string" && /^\/pkg\/leptos-cf\.[a-f0-9]{16}\.js$/.test(manifest.js),
    `asset manifest contains an invalid JS path: ${manifest.js}`,
  );

  const assetResponse = await fetchBoundary(`${origin}${manifest.js}`);
  assert(assetResponse.status === 200, `hashed client JS returned ${assetResponse.status}`);
  assert(
    assetResponse.headers.get("cache-control")?.includes("immutable"),
    "hashed client JS is not immutable",
  );
  assert(!assetResponse.headers.has("set-cookie"), "static asset unexpectedly ran session setup");
  assert(
    !assetResponse.headers.has("content-security-policy"),
    "static asset unexpectedly received the dynamic SSR header set",
  );

  const missingResponse = await fetchBoundary(`${origin}/definitely-not-a-field-guide-route`);
  const missingHtml = await missingResponse.text();
  assert(missingResponse.status === 404, `unknown document route returned ${missingResponse.status}`);
  assert(
    missingHtml.includes("This route is outside the field guide."),
    "unknown document route lacks the server-rendered recovery page",
  );

  const realtimeResponse = await fetchBoundary(`${origin}/realtime/socket`);
  const realtimeBody = await realtimeResponse.text();
  assert(realtimeResponse.status === 426, `non-upgrade realtime request returned ${realtimeResponse.status}`);
  assert(
    realtimeBody === "WebSocket upgrade required.",
    "realtime capability route did not return the expected upgrade-required response",
  );

  const rejectedApiResponse = await fetchBoundary(`${origin}/api/list_todos`, {
    method: "DELETE",
  });
  assert(rejectedApiResponse.status === 405, `unsafe API method returned ${rejectedApiResponse.status}`);
  assert(
    (await rejectedApiResponse.text()) === "Method not allowed for server functions.",
    "unsafe API method did not cross the expected request guard",
  );
}

async function main() {
  await Promise.all([
    readFile(workerEntrypoint),
    readFile(assetManifestPath),
  ]).catch(() => {
    throw new Error("missing release artifacts; run bash ./scripts/build-edge.sh first");
  });

  const testDirectory = await mkdtemp(join(tmpdir(), "leptos-cf-boundaries-"));
  const configPath = join(testDirectory, "wrangler.test.toml");
  const persistencePath = join(testDirectory, "state");
  const port = await reservePort();
  const inspectorPort = await reservePort();
  const origin = `http://127.0.0.1:${port}`;
  const config = [
    'name = "leptos-cf-boundary-test"',
    `main = ${JSON.stringify(workerEntrypoint)}`,
    'compatibility_date = "2026-08-10"',
    "",
    "[assets]",
    `directory = ${JSON.stringify(assetDirectory)}`,
    'binding = "ASSETS"',
    "",
    "[[d1_databases]]",
    'binding = "DB"',
    'database_name = "leptos-cf-boundary-test"',
    'database_id = "00000000-0000-0000-0000-000000000000"',
    `migrations_dir = ${JSON.stringify(resolve(root, "migrations"))}`,
    "",
  ].join("\n");
  await writeFile(configPath, config);

  let output = "";
  const wrangler = spawn(
    "bunx",
    [
      "wrangler@4.120.1",
      "dev",
      "--config",
      configPath,
      "--local",
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
      cwd: root,
      env: { ...process.env, CI: "1", NO_COLOR: "1" },
      stdio: ["ignore", "pipe", "pipe"],
    },
  );

  const capture = (chunk) => {
    output = `${output}${chunk}`.slice(-20_000);
  };
  wrangler.stdout.on("data", capture);
  wrangler.stderr.on("data", capture);

  try {
    await waitForWorker(origin, wrangler, () => output);
    await runAssertions(origin);
    console.log(
      "[test-worker-boundaries] SSR HTML, nonce hydration, asset routing, 404, API guard, and realtime boundary passed",
    );
  } finally {
    await stopWorker(wrangler);
    await rm(testDirectory, { recursive: true, force: true });
  }
}

main().catch((error) => {
  console.error(`[test-worker-boundaries] ${error.message}`);
  process.exit(1);
});
