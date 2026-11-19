import { strict as assert } from "node:assert";
import { hydrationAliases } from "./hydration-routes.mjs";

// A retained client keeps posting to the server-function URL it was built
// with. The Worker must deliver that request to the active function.
export async function testLegacyAlias(origin, { site, outputName }) {
  const aliases = await hydrationAliases(site, outputName);
  const legacy = Object.keys(aliases).find((path) => path.startsWith("/api/create_todo"));
  if (!legacy) {
    console.log("[legacy-alias] no retained client differs from the active one; nothing to route");
    return;
  }
  const page = await fetch(`${origin}/patterns/resource-explorer`);
  const html = await page.text();
  const cookie = page.headers.get("set-cookie")?.split(";")[0];
  assert.ok(cookie, "session cookie for the legacy request");
  assert.ok(html.includes(`action="${aliases[legacy]}"`), "the active document must publish the alias target");
  assert.ok(!html.includes(legacy), "the active document must not publish a retired endpoint");

  const response = await fetch(`${origin}${legacy}?legacy=1`, {
    method: "POST",
    headers: {
      origin,
      cookie,
      accept: "application/json",
      "content-type": "application/x-www-form-urlencoded",
      "cf-connecting-ip": "203.0.113.121",
    },
    body: new URLSearchParams({ title: "Legacy client" }),
    signal: AbortSignal.timeout(10_000),
  });
  const text = await response.text();
  assert.equal(response.status, 200, `retired server-function URL returned ${response.status}: ${text.slice(0, 500)}`);
  assert.equal(JSON.parse(text).title, "Legacy client");
  console.log("[legacy-alias] a retired server-function URL reached the active function");
}
