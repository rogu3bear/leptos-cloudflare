#!/usr/bin/env bun
import { strict as assert } from "node:assert";
import { withDispatch } from "../runtime/dispatch.mjs";

let received;
const spans = [];
class Application {
  constructor() {
    this.env = { ASSETS: { fetch: async () => new Response("asset") } };
    this.ctx = { tracing: { enterSpan(name, callback) {
      spans.push(name);
      return callback({ setAttribute() {} });
    } } };
  }
  async fetch(request) { received = request; return new Response("application"); }
}
const Worker = withDispatch(Application);
const worker = new Worker();
assert.equal(await (await worker.fetch(new Request("https://test/pkg/app.js"))).text(), "asset");
assert.deepEqual(spans, []);
const apiRequest = new Request("https://test/api/save", { method: "POST", body: "title=record" });
assert.equal(await (await worker.fetch(apiRequest)).text(), "application");
assert.notEqual(received, apiRequest);
assert.equal(await received.text(), "title=record");
assert.equal(apiRequest.bodyUsed, false);
assert.deepEqual(spans, ["leptos.server_function"]);
await worker.fetch(new Request("https://test/new-route"));
assert.equal(spans.at(-1), "leptos.ssr");
assert.equal((await worker.fetch(new Request("https://test/realtime/socket"))).status, 426);
assert.equal((await worker.fetch(new Request("https://test/wrong", { headers: { Upgrade: "websocket" } }))).status, 404);
class RoomWorker extends Worker {
  handleRealtimeSocket() { return new Response("room owner"); }
}
assert.equal(await (await new RoomWorker().fetch(new Request("https://test/realtime/socket", { headers: { Upgrade: "websocket" } }))).text(), "room owner");
const Pages = withDispatch(Application, { assetPaths: ["/logo.svg", "/api/save"], blockServerModules: true });
const pages = new Pages();
assert.equal(await (await pages.fetch(new Request("https://test/logo.svg"))).text(), "asset");
assert.equal(await (await pages.fetch(new Request("https://test/api/save"))).text(), "application");
assert.equal(await (await pages.fetch(new Request("https://test/missing.svg"))).text(), "application");
assert.equal((await pages.fetch(new Request("https://test/_server/dispatch.mjs"))).status, 404);
for (const path of ["/functions/index_bg.wasm", "/pkg/%2f..%2f_server/index_bg.wasm", "/pkg/%2f..%2ffunctions/index_bg.wasm"]) {
  assert.equal((await pages.fetch(new Request(`https://test${path}`))).status, 404, "private modules must never reach ASSETS through encoded traversal");
}
console.log("[dispatch] application/asset routing, API stream isolation, spans and socket override passed");
