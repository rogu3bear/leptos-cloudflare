import { strict as assert } from "node:assert";
import { activeServerFunctions } from "./hydration-routes.mjs";
import { fetchBoundary, withLocalPreview } from "./local-preview.mjs";
import { EDGE_MANIFEST_GLOBAL } from "./worker-entry.mjs";

const PROBE_PATH = "/__span_probe__";
const RETIRED_PATH = "/api/create_todo1";

// Test-only entry. It hands the built Worker a recording `ctx.tracing`, so the
// span contract is observed on the real request handler instead of a mock of it.
// `?head=<path>` returns the handler's own status and headers, which the local
// HTTP proxy would otherwise filter (it drops `Upgrade` from a 426). `aliases`
// extends the routing data before the first request reads it, so a retired
// server-function URL is exercised even when no retained client needs one.
function probeEntry(buildEntry, aliases) {
  return `import Worker from ${JSON.stringify(buildEntry)};
const routing = globalThis.${EDGE_MANIFEST_GLOBAL};
globalThis.${EDGE_MANIFEST_GLOBAL} = Object.freeze({ ...routing, aliases: { ...routing.aliases, ...${JSON.stringify(aliases)} } });
const spans = [];
let enabled = true;
const tracing = {
  enterSpan(name, callback, ...args) {
    const attributes = {};
    spans.push({ name, attributes });
    return callback({ setAttribute(key, value) { attributes[key] = value; } }, ...args);
  },
};
export default class extends Worker {
  constructor(ctx, env) {
    super(ctx, env);
    Object.defineProperty(this, "ctx", { value: new Proxy(ctx, { get(target, key) {
      if (key === "tracing") return enabled ? tracing : undefined;
      const value = Reflect.get(target, key);
      return typeof value === "function" ? value.bind(target) : value;
    } }) });
  }
  async fetch(request) {
    const url = new URL(request.url);
    if (url.pathname !== ${JSON.stringify(PROBE_PATH)}) return super.fetch(request);
    if (url.searchParams.has("head")) {
      const response = await super.fetch(new Request(new URL(url.searchParams.get("head"), url)));
      return Response.json({ status: response.status, headers: Object.fromEntries(response.headers) });
    }
    if (url.searchParams.has("tracing")) enabled = url.searchParams.get("tracing") === "on";
    return Response.json(spans.splice(0));
  }
}
`;
}

// Without routing data the handler must refuse every lane rather than guess:
// the Pages private-module denial depends on that data being present.
function withoutRoutingData(buildEntry) {
  return `import Worker from ${JSON.stringify(buildEntry)};
delete globalThis.${EDGE_MANIFEST_GLOBAL};
export default Worker;
`;
}

export async function testMissingRoutingData(root) {
  await withLocalPreview({ root, wrapEntry: withoutRoutingData }, async (origin) => {
    for (const path of ["/", "/pkg/__missing__.js", "/realtime/socket", "/api/__missing__", "/_server/index.js"]) {
      const response = await fetchBoundary(`${origin}${path}`);
      await response.arrayBuffer();
      assert.equal(response.status, 500, `${path} was answered without routing data (${response.status})`);
    }
  });
  console.log("[handler-probe] every lane fails closed when the generated routing data is absent");
}

const span = (boundary) => ({
  name: `leptos.${boundary}`,
  attributes: { "app.telemetry.schema_version": 1, "app.boundary": boundary },
});

export async function testHandlerProbe(root, documentPath, { site, referenceSite = false } = {}) {
  // The sample domain owns `create_todo`; an adopted application has no such function to reach.
  const active = referenceSite ? (await activeServerFunctions(site)).get("create_todo") : null;
  assert.ok(!referenceSite || active, "the active client must publish its create_todo endpoint");
  const aliases = active ? { [RETIRED_PATH]: active } : {};
  await withLocalPreview({ root, readyPath: documentPath, migrate: referenceSite, wrapEntry: (entry) => probeEntry(entry, aliases) }, async (origin) => {
    const drain = async (query = "") => (await fetchBoundary(`${origin}${PROBE_PATH}${query}`)).json();
    const observe = async (label, path, init, status, expected) => {
      const response = await fetchBoundary(`${origin}${path}`, init);
      await response.arrayBuffer();
      assert.equal(response.status, status, `${label} returned ${response.status}`);
      assert.deepEqual(await drain(), expected, `${label} recorded unexpected spans`);
    };

    await drain(); // discard the readiness probes
    await observe("document", documentPath, undefined, 200, [span("ssr")]);
    await observe("unknown document", "/__span_probe_missing__", undefined, 404, [span("ssr")]);
    await observe("server function", "/api/__span_probe__", { method: "DELETE" }, 405, [span("server_function")]);
    await observe("missing asset", "/pkg/__span_probe__.js", undefined, 404, []);
    await observe("realtime without upgrade", "/realtime/socket", undefined, 426, []);

    const upgradeRequired = await drain("?head=/realtime/socket");
    assert.equal(upgradeRequired.status, 426);
    assert.equal(upgradeRequired.headers.upgrade, "websocket", "upgrade-required response must name the protocol");
    assert.deepEqual(Object.keys(upgradeRequired.headers).filter((name) => name === "set-cookie" || name === "content-security-policy"), []);
    assert.deepEqual(await drain(), [], "the socket lane must stay outside the span contract");

    if (active) {
      const page = await fetchBoundary(`${origin}${documentPath}`);
      await page.arrayBuffer();
      const cookie = page.headers.get("set-cookie")?.split(";")[0];
      assert.ok(cookie, "session cookie for the retired request");
      await drain();
      const retired = await fetchBoundary(`${origin}${RETIRED_PATH}?retired=1`, {
        method: "POST",
        headers: { origin, cookie, accept: "application/json", "content-type": "application/x-www-form-urlencoded", "cf-connecting-ip": "203.0.113.122" },
        body: new URLSearchParams({ title: "Retired client" }),
      });
      const text = await retired.text();
      assert.equal(retired.status, 200, `retired server-function URL returned ${retired.status}: ${text.slice(0, 500)}`);
      assert.equal(JSON.parse(text).title, "Retired client");
      assert.deepEqual(await drain(), [span("server_function")], "a retired URL stays inside the server-function boundary");
    }

    await drain("?tracing=off");
    await observe("document without tracing", documentPath, undefined, 200, []);
    await observe("server function without tracing", "/api/__span_probe__", { method: "DELETE" }, 405, []);
    await drain("?tracing=on");
  });
  console.log("[handler-probe] closed ssr/server_function spans on the built handler, none for assets or the socket lane, the 426 protocol offer, retired server-function URLs and the no-tracing fallback passed");
}
