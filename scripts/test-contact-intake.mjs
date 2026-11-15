import { strict as assert } from "node:assert";
import { fetchBoundary } from "./local-preview.mjs";

// Only the harness's disposable local D1 store is seeded or queried.
export async function testContactIntake(origin, { query }) {
  const manifest = await (await fetchBoundary(`${origin}/asset-manifest.json`)).json();
  const wasm = await (await fetchBoundary(`${origin}${manifest.wasm}`)).arrayBuffer();
  const endpoint = new TextDecoder("latin1").decode(wasm).match(/\/api\/submit_contact\d+/)?.[0];
  assert.ok(endpoint, "served client must expose its generated contact endpoint");
  async function session() {
    const response = await fetchBoundary(`${origin}/contact`);
    assert.equal(response.status, 200);
    await response.text();
    const cookie = response.headers.get("set-cookie")?.split(";")[0];
    assert.ok(cookie);
    return cookie;
  }
  const cookie = await session();
  async function submit(active = cookie, extra = {}) {
    const response = await fetchBoundary(`${origin}${endpoint}`, {
      method: "POST",
      headers: { origin, cookie: active, accept: "application/json", "content-type": "application/x-www-form-urlencoded" },
      body: new URLSearchParams({ name: "  Ada   Lovelace  ", email: " ADA@Example.COM ", topic: "  Local   proof ", message: "  A synthetic contact message for local qualification.  ", website: "", ...extra }),
    });
    return { status: response.status, body: await response.text() };
  }
  const count = () => query("SELECT COUNT(*) AS total FROM contact_submissions")[0].total;
  assert.equal(count(), 0, "contact migration must create an empty local table");
  const accepted = await submit();
  assert.equal(accepted.status, 200, accepted.body);
  assert.equal(JSON.parse(accepted.body).accepted, true);
  assert.deepEqual(query("SELECT name,email,topic,message FROM contact_submissions"), [{ name: "Ada Lovelace", email: "ada@example.com", topic: "Local proof", message: "A synthetic contact message for local qualification." }]);
  assert.ok((await submit(cookie, { website: "https://spam.example" })).status >= 400);
  assert.equal(count(), 1, "honeypot rejection must not insert");
  for (let index = 0; index < 2; index++) assert.equal((await submit()).status, 200);
  const hourly = await submit();
  assert.ok(hourly.status >= 400);
  assert.match(hourly.body, /Try again later/);
  assert.equal(count(), 3, "hourly rejection must not insert");
  for (let index = 3; index < 8; index++) {
    query("UPDATE contact_submissions SET created_at = datetime('now','-2 hours')");
    assert.equal((await submit()).status, 200);
  }
  query("UPDATE contact_submissions SET created_at = datetime('now','-2 hours')");
  const daily = await submit();
  assert.ok(daily.status >= 400);
  assert.match(daily.body, /Try again tomorrow/);
  assert.equal(count(), 8, "daily rejection must not insert");
  assert.equal((await submit(await session())).status, 200, "another session must retain its own budget");
  assert.equal(count(), 9);
  query("DELETE FROM contact_submissions; WITH RECURSIVE rows(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM rows WHERE n<2000) INSERT INTO contact_submissions(id,session_id,name,email,topic,message,created_at) SELECT n,'retention-fixture','Fixture','fixture@example.com','Retention','Synthetic retention fixture',datetime('now','-2 days') FROM rows");
  assert.equal(count(), 2000);
  assert.equal((await submit(await session())).status, 200);
  assert.equal(count(), 2000, "global trimming must preserve the configured cap");
  assert.equal(query("SELECT id FROM contact_submissions WHERE id=1").length, 0, "oldest row must be trimmed");
  assert.equal(query("SELECT id FROM contact_submissions WHERE id=2000").length, 1, "newer prior rows must survive");
  assert.equal(query("SELECT name FROM contact_submissions ORDER BY id DESC LIMIT 1")[0].name, "Ada Lovelace", "new submission must survive trimming");
  console.log("[contact-intake] migrated local D1, normalized insert, honeypot, hourly/daily session caps and global retention passed");
}
