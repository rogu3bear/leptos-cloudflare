import { strict as assert } from "node:assert";
import { request } from "node:http";

export async function testBodyLimit(origin, { referenceSite = false } = {}) {
  let path = "/api/__stream_limit_probe__";
  const page = await fetch(`${origin}/patterns/resource-explorer`);
  const html = await page.text();
  const cookie = page.headers.get("set-cookie")?.split(";")[0];
  if (referenceSite) {
    path = [...html.matchAll(/action="([^"]+)"/g)].map((match) => match[1])
      .find((action) => action.startsWith("/api/create_todo"));
    assert.ok(path, "discover the real generated mutation endpoint");
  }
  async function send(size, chunked) {
    const prefix = "title=Body+boundary&padding=";
    const body = prefix + "x".repeat(size - prefix.length);
    return new Promise((resolve, reject) => {
      const req = request(origin + path, { method: "POST", headers: {
        origin, ...(cookie ? { cookie } : {}), "content-type": "application/x-www-form-urlencoded",
        accept: "application/json", "cf-connecting-ip": "203.0.113.120",
        ...(chunked ? { "transfer-encoding": "chunked" } : { "content-length": size }),
      } }, (res) => {
        let text = "";
        res.setEncoding("utf8"); res.on("data", (part) => { text += part; });
        res.on("end", () => resolve({ status: res.statusCode, text, headers: res.headers }));
      });
      req.setTimeout(10_000, () => req.destroy(new Error("body limit probe timed out")));
      req.on("error", reject);
      req.write(body.slice(0, 31)); req.end(body.slice(31));
    });
  }
  for (const size of [4095, 4096]) {
    const response = await send(size, true);
    if (referenceSite) {
      assert.equal(response.status, 200, response.text);
      assert.equal(JSON.parse(response.text).title, "Body boundary");
    } else assert.notEqual(response.status, 413, "small requests must pass the body guard");
  }
  for (const chunked of [false, true]) {
    const response = await send(4097, chunked);
    assert.equal(response.status, 413, `${chunked ? "streamed" : "declared"} over-limit body accepted`);
    assert.equal(response.headers["cache-control"], "no-store");
    assert.ok(response.headers["content-security-policy"]);
  }
  console.log("[body-limit] exact 4096-byte boundary and streamed/declared rejection passed");
}
