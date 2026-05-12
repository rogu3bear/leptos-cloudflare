#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { mkdir, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const root = process.cwd();
const workerBundle = join(root, "build/index.js");
const shimPath = join(root, "build/_worker.js");

if (!existsSync(workerBundle)) {
  console.error(`[write-worker-shim] missing worker bundle: ${workerBundle}`);
  process.exit(1);
}

await mkdir(dirname(shimPath), { recursive: true });
await writeFile(
  shimPath,
  [
    'import LeptosWorker from "./index.js";',
    "",
    'const REALTIME_SOCKET_PATH = "/realtime/socket";',
    "",
    "const STATIC_ASSET_PATHS = [",
    '  "/asset-manifest.json",',
    '  "/app-icon.svg",',
    '  "/app-icon-192.png",',
    '  "/app-icon-512.png",',
    '  "/apple-touch-icon.png",',
    '  "/favicon.svg",',
    '  "/site.webmanifest",',
    "];",
    "",
    "const STATIC_ASSET_PREFIXES = [",
    '  "/pkg/",',
    "];",
    "",
    "function shouldServeAsset(pathname) {",
    "  return STATIC_ASSET_PATHS.includes(pathname)",
    "    || STATIC_ASSET_PREFIXES.some((prefix) => pathname.startsWith(prefix));",
    "}",
    "",
    "function isWebSocketUpgrade(request) {",
    '  return request.headers.get("Upgrade")?.toLowerCase() === "websocket";',
    "}",
    "",
    "function handleRealtimeSocket() {",
    "  const pair = new WebSocketPair();",
    "  const [client, server] = Object.values(pair);",
    "",
    "  server.accept();",
    "  server.send(JSON.stringify({",
    '    type: "ready",',
    '    transport: "websocket",',
    '    route: REALTIME_SOCKET_PATH,',
    '    note: "Template endpoint only. Use Durable Objects for rooms, presence, collaboration, fanout, or long-lived state.",',
    "  }));",
    '  server.close(1000, "template capability check complete");',
    "",
    "  return new Response(null, {",
    "    status: 101,",
    "    webSocket: client,",
    "  });",
    "}",
    "",
    "export default class extends LeptosWorker {",
    "  async fetch(request) {",
    "    const url = new URL(request.url);",
    "",
    "    if (isWebSocketUpgrade(request)) {",
    "      if (url.pathname === REALTIME_SOCKET_PATH) {",
    "        return handleRealtimeSocket();",
    "      }",
    "",
    "      return new Response(\"Unknown WebSocket route.\", { status: 404 });",
    "    }",
    "",
    "    if (url.pathname === REALTIME_SOCKET_PATH) {",
    "      return new Response(\"WebSocket upgrade required.\", {",
    "        status: 426,",
    '        headers: { Upgrade: "websocket" },',
    "      });",
    "    }",
    "",
    "    if (shouldServeAsset(url.pathname)) {",
    "      return this.env.ASSETS.fetch(request);",
    "    }",
    "",
    "    return super.fetch(request);",
    "  }",
    "}",
    "",
  ].join("\n"),
);

console.log("[write-worker-shim] wrote build/_worker.js");
