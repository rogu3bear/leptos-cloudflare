import { strict as assert } from "node:assert";
import { fetchBoundary } from "./local-preview.mjs";

// Every query below targets the harness's disposable local D1 store.
export async function testContactIntake(origin, { query }) {
  const manifest = await (await fetchBoundary(`${origin}/asset-manifest.json`)).json();
  const wasm = await (await fetchBoundary(`${origin}${manifest.wasm}`)).arrayBuffer();
  const endpoint = new TextDecoder("latin1").decode(wasm).match(/\/api\/submit_contact\d+/)?.[0];
  assert.ok(endpoint);
  async function session() {
    const response = await fetchBoundary(`${origin}/contact`);
    assert.equal(response.status, 200); await response.text();
    const cookie = response.headers.get("set-cookie")?.split(";")[0];
    assert.ok(cookie); return cookie;
  }
  const cookie = await session();
  async function submit(active = cookie, extra = {}, ip = "198.51.100.10", headers = {}) {
    const response = await fetchBoundary(`${origin}${endpoint}`, {
      method: "POST", headers: { origin, cookie: active, accept: "application/json",
        "content-type": "application/x-www-form-urlencoded", "cf-connecting-ip": ip, ...headers },
      body: new URLSearchParams({ name: "  Ada   Lovelace  ", email: " ADA@Example.COM ",
        topic: "  Local   proof ", message: "  A synthetic contact message for local qualification.  ", website: "", ...extra }),
    });
    return { status: response.status, body: await response.text() };
  }
  const count = () => query("SELECT COUNT(*) AS total FROM contact_submissions")[0].total;
  assert.equal(count(), 0);
  const invalidNetwork = await submit(await session(), {}, "not-an-ip");
  assert.ok(invalidNetwork.status >= 400);
  assert.match(invalidNetwork.body, /Client network identity is unavailable/);
  assert.equal(count(), 0, "invalid network identity must leave the submissions table empty");
  assert.equal((await submit()).status, 200);
  assert.deepEqual(query("SELECT name,email,topic,message FROM contact_submissions"), [{ name: "Ada Lovelace", email: "ada@example.com", topic: "Local proof", message: "A synthetic contact message for local qualification." }]);
  const admission = query("SELECT admission_key FROM contact_submissions")[0].admission_key;
  assert.match(admission, /^[0-9a-f]{64}$/);
  assert.ok((await submit(cookie, { website: "https://spam.example" })).status >= 400);
  assert.equal(count(), 1);
  for (let index = 0; index < 2; index++) assert.equal((await submit()).status, 200);
  assert.ok((await submit()).status >= 400);
  assert.ok((await submit(await session())).status >= 400, "cookie rotation must not reset the IP budget");
  assert.ok((await submit(cookie, {}, "198.51.100.10", { "x-forwarded-for": "203.0.113.99" })).status >= 400);
  assert.equal(count(), 3);
  for (let index = 3; index < 8; index++) {
    query("UPDATE contact_submissions SET created_at = datetime('now','-2 hours')");
    assert.equal((await submit()).status, 200);
  }
  query("UPDATE contact_submissions SET created_at = datetime('now','-2 hours')");
  assert.ok((await submit(await session())).status >= 400, "daily IP budget survives cookie rotation");
  assert.equal(count(), 8);
  assert.equal((await submit(await session(), {}, "198.51.100.11")).status, 200, "another IP has an independent budget");
  assert.equal(count(), 9);
  query("DELETE FROM contact_submissions; DELETE FROM anonymous_admission_key");
  const concurrent = await Promise.all(Array.from({ length: 10 }, () => submit()));
  assert.equal(concurrent.filter((response) => response.status === 200).length, 3, "atomic hourly budget");
  assert.equal(count(), 3);
  const rotated = await session();
  assert.ok((await submit(rotated)).status >= 400);
  const concurrentKeys = query("SELECT DISTINCT admission_key FROM contact_submissions");
  assert.equal(concurrentKeys.length, 1, "racing first writes use one winning database key");
  assert.match(concurrentKeys[0].admission_key, /^[0-9a-f]{64}$/);
  assert.notEqual(concurrentKeys[0].admission_key, admission, "explicit fixture key rotation changes the digest");
  query("UPDATE contact_submissions SET created_at = datetime('now','-2 hours')");
  const dailyRace = await Promise.all(Array.from({ length: 10 }, () => submit()));
  assert.equal(dailyRace.filter((response) => response.status === 200).length, 3);
  query("UPDATE contact_submissions SET created_at = datetime('now','-2 hours')");
  const finalRace = await Promise.all(Array.from({ length: 10 }, () => submit()));
  assert.equal(finalRace.filter((response) => response.status === 200).length, 2, "atomic daily budget");
  assert.equal(count(), 8);
  query("DELETE FROM contact_submissions");
  const sessionId = cookie.split("=")[1];
  assert.match(sessionId, /^[0-9a-f]{64}$/);
  query(`INSERT INTO contact_submissions(session_id,name,email,topic,message) VALUES ('${sessionId}','Legacy','fixture@example.com','Upgrade','Legacy session fixture'),('${sessionId}','Legacy','fixture@example.com','Upgrade','Legacy session fixture'),('${sessionId}','Legacy','fixture@example.com','Upgrade','Legacy session fixture')`);
  assert.ok((await submit(cookie, {}, "198.51.100.13")).status >= 400, "legacy session quotas survive the migration's empty admission key");
  assert.equal(count(), 3);
  query("DELETE FROM contact_submissions; WITH RECURSIVE rows(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM rows WHERE n<2000) INSERT INTO contact_submissions(id,session_id,name,email,topic,message,created_at) SELECT n,'retention-fixture','Fixture','fixture@example.com','Retention','Synthetic retention fixture',datetime('now','-2 days') FROM rows");
  assert.ok((await submit(await session(), {}, "198.51.100.12")).status >= 400, "full storage rejects without eviction");
  assert.equal(count(), 2000);
  assert.equal(query("SELECT id FROM contact_submissions WHERE id=1").length, 1);
  assert.equal(query("SELECT id FROM contact_submissions WHERE id=2000").length, 1);
  console.log("[contact-intake] validation, atomic hourly/daily IP quotas, cookie rotation and non-evicting capacity passed");
}
