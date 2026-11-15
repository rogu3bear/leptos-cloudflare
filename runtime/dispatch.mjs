
const REALTIME_SOCKET_PATH = "/realtime/socket";

const STATIC_ASSET_PATHS = [
  "/asset-manifest.json",
  "/app-icon.svg",
  "/app-icon-192.png",
  "/app-icon-512.png",
  "/apple-touch-icon.png",
  "/favicon.svg",
  "/site.webmanifest",
];

function shouldServeAsset(pathname, assetPrefix) {
  return STATIC_ASSET_PATHS.includes(pathname)
    || pathname.startsWith(assetPrefix);
}

function isWebSocketUpgrade(request) {
  return request.headers.get("Upgrade")?.toLowerCase() === "websocket";
}

function requestBoundary(pathname) {
  return pathname.startsWith("/api/") ? "server_function" : "ssr";
}

function handleRealtimeSocket() {
  const pair = new WebSocketPair();
  const [client, server] = Object.values(pair);

  server.accept();
  server.send(JSON.stringify({
    type: "ready",
    transport: "websocket",
    route: REALTIME_SOCKET_PATH,
    note: "Template endpoint only. Use Durable Objects for rooms, presence, collaboration, fanout, or long-lived state.",
  }));
  server.close(1000, "template capability check complete");

  return new Response(null, {
    status: 101,
    webSocket: client,
  });
}

export function withDispatch(LeptosWorker, { assetPaths = null, blockServerModules = false, assetPrefix = "/pkg/" } = {}) {
  const pagesAssets = assetPaths === null ? null : new Set(assetPaths);
  return class extends LeptosWorker {
  handleRealtimeSocket(_request) { return handleRealtimeSocket(); }

  async fetch(request) {
    const url = new URL(request.url);
    if (blockServerModules && (url.pathname === "/_server" || url.pathname.startsWith("/_server/"))) {
      return new Response("Not found.", { status: 404 });
    }

    if (isWebSocketUpgrade(request)) {
      if (url.pathname === REALTIME_SOCKET_PATH) {
        return this.handleRealtimeSocket(request);
      }

      return new Response("Unknown WebSocket route.", { status: 404 });
    }

    if (url.pathname === REALTIME_SOCKET_PATH) {
      return new Response("WebSocket upgrade required.", {
        status: 426,
        headers: { Upgrade: "websocket" },
      });
    }

    let assetPath;
    try { assetPath = decodeURIComponent(url.pathname); } catch { assetPath = null; }
    const applicationPath = assetPath === "/api" || assetPath?.startsWith("/api/") || assetPath === REALTIME_SOCKET_PATH;
    const exactPageAsset = pagesAssets?.has(assetPath) && !applicationPath;
    if (exactPageAsset || shouldServeAsset(url.pathname, assetPrefix)) {
      return this.env.ASSETS.fetch(request);
    }

    // Keep Rust's request-body cancellation isolated from the native request.
    // The API body limit remains in Rust; rejected bodies are never read.
    const leptosRequest = url.pathname.startsWith("/api/") && request.body ? request.clone() : request;
    const boundary = requestBoundary(url.pathname);
    if (!this.ctx.tracing?.enterSpan) {
      return super.fetch(leptosRequest);
    }
    return this.ctx.tracing.enterSpan(
      `leptos.${boundary}`,
      (span) => {
        span.setAttribute("app.telemetry.schema_version", 1);
        span.setAttribute("app.boundary", boundary);
        return super.fetch(leptosRequest);
      },
    );
  }
};
}
