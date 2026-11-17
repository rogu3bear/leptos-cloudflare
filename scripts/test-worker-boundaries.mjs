#!/usr/bin/env bun

import { createConnection } from "node:net";
import { withLocalPreview, fetchBoundary } from "./local-preview.mjs";
import { testResourceExplorer } from "./test-resource-explorer.mjs";
import { testBodyLimit } from "./test-body-limit.mjs";
import { testTodoAdmission } from "./test-todo-admission.mjs";
import { testContactIntake } from "./test-contact-intake.mjs";
import { projectMetadata } from "./project-metadata.mjs";

const root = process.cwd();
const pagesMode = process.env.LEPTOS_TEST_PAGES === "1";
const { leptos, referenceSite } = await projectMetadata(root);
const documentPath = process.env.LEPTOS_BOUNDARY_DOCUMENT_PATH ?? (referenceSite ? "/architecture" : "/");
if (!documentPath.startsWith("/") || documentPath.startsWith("//")) throw new Error("boundary document path must be local");

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

async function runAssertions(origin) {
  const documentResponse = await fetchBoundary(`${origin}${documentPath}`);
  const documentHtml = await documentResponse.text();
  assert(documentResponse.status === 200, `${documentPath} returned ${documentResponse.status}`);
  assert(
    documentResponse.headers.get("content-type")?.includes("text/html"),
    "document did not return HTML",
  );
  assert(
    /<h1(?:\s[^>]*)?>[\s\S]*?[^\s<>][\s\S]*?<\/h1>/.test(documentHtml),
    "document lacks a useful server-rendered heading",
  );
  assert(
    /<main(?:\s[^>]*)?>/.test(documentHtml),
    "document lacks the server-rendered main landmark",
  );
  assert(
    documentResponse.headers.get("cache-control") === "no-store",
    "dynamic HTML is not marked no-store",
  );
  assert(
    documentResponse.headers.has("set-cookie"),
    "dynamic HTML did not establish the scoped session cookie",
  );
  assert(documentResponse.headers.get("x-frame-options") === "DENY", "dynamic HTML is missing anti-framing protection");

  const csp = documentResponse.headers.get("content-security-policy") ?? "";
  const nonce = csp.match(/'nonce-([^']+)'/)?.[1];
  assert(nonce, "dynamic HTML CSP is missing a script nonce");
  assert(
    documentHtml.includes(`nonce="${nonce}"`),
    "hydration script nonce does not match the response CSP",
  );

  const wasmPreload = documentHtml.match(/<link\b[^>]*rel="preload"[^>]*>/g)?.find((tag) => tag.includes('as="fetch"'));
  assert(wasmPreload?.includes('crossorigin="anonymous"'), "WASM preload must match the anonymous CORS/default fetch credentials mode");

  const manifestResponse = await fetchBoundary(`${origin}/asset-manifest.json`);
  assert(manifestResponse.status === 200, `asset manifest returned ${manifestResponse.status}`);
  assert(
    manifestResponse.headers.get("cache-control") === "no-store",
    "asset manifest is not marked no-store",
  );
  const manifest = await manifestResponse.json();
  assert(
    typeof manifest.js === "string" && new RegExp(`^/pkg/${leptos["output-name"]}\\.[a-f0-9]{16}\\.js$`).test(manifest.js),
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

  const missingResponse = await fetchBoundary(`${origin}/__runtime_boundary_missing_document__`);
  const missingHtml = await missingResponse.text();
  assert(missingResponse.status === 404, `unknown document route returned ${missingResponse.status}`);
  assert(
    /<main(?:\s[^>]*)?>/.test(missingHtml) && /<h1(?:\s[^>]*)?>/.test(missingHtml),
    "unknown document route lacks the server-rendered recovery page",
  );
  if (pagesMode) {
    const adopterAsset = await fetchBoundary(`${origin}/__adopter_boundary_logo.svg`);
    assert(adopterAsset.status === 200 && await adopterAsset.text() === "adopter asset fixture", "Pages did not serve an adopter asset outside the template whitelist");
    const absentAsset = await fetchBoundary(`${origin}/__adopter_boundary_missing.svg`);
    assert(absentAsset.status === 404, `missing adopter asset returned ${absentAsset.status}`);
    for (const path of ["/_server/index.js", "/_server/index_bg.wasm", "/functions/index.js", "/functions/index_bg.wasm", "/pkg/%2f..%2f_server/index_bg.wasm", "/pkg/%2f..%2ffunctions/index_bg.wasm"]) {
      const response = await fetchBoundary(`${origin}${path}`);
      assert(response.status === 404, `Pages server module ${path} returned ${response.status}`);
    }
  }

  if (referenceSite) {
    assert(documentHtml.includes("SSR + hydration") && documentHtml.includes("Cloudflare Pages"), "reference architecture page lost its decision content");
    assert(missingHtml.includes("This route is outside the field guide."), "reference recovery page lost its content");
  }

  const realtimeResponse = await fetchBoundary(`${origin}/realtime/socket`);
  const realtimeBody = await realtimeResponse.text();
  assert(realtimeResponse.status === 426, `non-upgrade realtime request returned ${realtimeResponse.status}`);
  assert(
    realtimeBody === "WebSocket upgrade required.",
    "realtime capability route did not return the expected upgrade-required response",
  );
  const socketOrigin = origin.replace(/^http/, "ws");
  const socket = new WebSocket(`${socketOrigin}/realtime/socket`);
  const message = await new Promise((resolveMessage, rejectMessage) => {
    const timer = setTimeout(() => rejectMessage(new Error("WebSocket capability timed out")), 5_000);
    socket.onmessage = (event) => { clearTimeout(timer); resolveMessage(event.data); };
    socket.onerror = () => { clearTimeout(timer); rejectMessage(new Error("WebSocket capability upgrade failed")); };
  });
  const ready = JSON.parse(String(message));
  assert(ready.type === "ready" && ready.route === "/realtime/socket", "WebSocket capability sent an unexpected message");
  socket.close();
  const upgradeStatus = await new Promise((resolveStatus, rejectStatus) => {
    const address = new URL(origin);
    const connection = createConnection({ host: address.hostname, port: Number(address.port) });
    connection.setTimeout(5_000, () => connection.destroy(new Error("unknown WebSocket route timed out")));
    connection.on("connect", () => connection.write(`GET /__unknown_websocket__ HTTP/1.1\r\nHost: ${address.host}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\n\r\n`));
    connection.once("data", (data) => { connection.destroy(); resolveStatus(data.toString().split("\r\n")[0]); });
    connection.once("error", rejectStatus);
  });
  assert(/^HTTP\/1\.1 404\b/.test(upgradeStatus), `unknown WebSocket route returned ${upgradeStatus}`);

  const rejectedApiResponse = await fetchBoundary(`${origin}/api/__runtime_boundary__`, {
    method: "DELETE",
  });
  assert(rejectedApiResponse.status === 405, `unsafe API method returned ${rejectedApiResponse.status}`);
  assert(
    (await rejectedApiResponse.text()) === "Method not allowed for server functions.",
    "unsafe API method did not cross the expected request guard",
  );
  for (const [label, headers, body, expected] of [
    ["cross-origin", { origin: "https://other.example", "content-type": "application/x-www-form-urlencoded" }, "x=1", 403],
    ["content-type", { origin, "content-type": "text/plain" }, "x=1", 415],
    ["body limit", { origin, "content-type": "application/x-www-form-urlencoded" }, `x=${"x".repeat(4096)}`, 413],
  ]) {
    const response = await fetchBoundary(`${origin}/api/__runtime_boundary__`, { method: "POST", headers, body });
    assert(response.status === expected, `${label} guard returned ${response.status}, expected ${expected}: ${(await response.text()).slice(0, 1000)}`);
  }
}

async function main() {
  await withLocalPreview({ root, pages: pagesMode, migrate: referenceSite, readyPath: documentPath }, async (origin, store) => {
    await runAssertions(origin);
    await testBodyLimit(origin, { referenceSite });
    if (referenceSite) {
      await testResourceExplorer(origin);
      await testContactIntake(origin, store);
      await testTodoAdmission(origin, store);
    }
    console.log(`[test-worker-boundaries] ${pagesMode ? "Pages" : "Workers"}: rendering, assets, guards and WebSocket boundaries passed; ${referenceSite ? "reference D1 workflows passed" : "sample-domain checks excluded by the adopter profile"}`);
  });
}

main().catch((error) => {
  console.error(`[test-worker-boundaries] ${error.message}`);
  process.exit(1);
});
