import { strict as assert } from "node:assert";

export async function testTodoAdmission(origin, { query }) {
  const page = await fetch(`${origin}/patterns/resource-explorer`);
  const html = await page.text();
  const cookie = page.headers.get("set-cookie").split(";")[0];
  const endpoint = [...html.matchAll(/action="([^"]+)"/g)].map((match) => match[1]).find((path) => path.startsWith("/api/create_todo"));
  assert.ok(endpoint);
  async function create(session = cookie, ip = "203.0.113.40") {
    const response = await fetch(origin + endpoint, { method: "POST", headers: { origin, cookie: session,
      accept: "application/json", "content-type": "application/x-www-form-urlencoded", "cf-connecting-ip": ip },
      body: "title=Admission+fixture", signal: AbortSignal.timeout(10_000) });
    const text = await response.text(); return { status: response.status, text };
  }
  query("DELETE FROM todos");
  const first = await create(); assert.equal(first.status, 200, first.text);
  const { admission_key: admission, session_id: session } = query("SELECT admission_key,session_id FROM todos")[0];
  assert.match(admission, /^[0-9a-f]{64}$/); assert.match(session, /^[0-9a-f]{64}$/);
  query(`WITH RECURSIVE rows(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM rows WHERE n<198) INSERT INTO todos(session_id,admission_key,title) SELECT '${session}','${admission}','Capacity fixture' FROM rows`);
  const concurrent = await Promise.all(Array.from({ length: 10 }, () => create()));
  assert.equal(concurrent.filter((response) => response.status === 200).length, 1);
  assert.equal(query("SELECT COUNT(*) AS total FROM todos")[0].total, 200);
  assert.ok((await create(`leptos_cf_session=${"f".repeat(64)}`)).status >= 400, "new cookie cannot reset network capacity");
  assert.equal((await create(`leptos_cf_session=${"e".repeat(64)}`, "203.0.113.41")).status, 200);
  query("DELETE FROM todos; WITH RECURSIVE rows(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM rows WHERE n<5000) INSERT INTO todos(id,session_id,title) SELECT n,'victim-fixture','Preserved fixture' FROM rows");
  assert.ok((await create(cookie, "203.0.113.42")).status >= 400);
  assert.equal(query("SELECT COUNT(*) AS total FROM todos")[0].total, 5000);
  assert.deepEqual(query("SELECT id,title FROM todos WHERE id IN (1,5000) ORDER BY id"), [{ id: 1, title: "Preserved fixture" }, { id: 5000, title: "Preserved fixture" }]);
  console.log("[todo-admission] atomic session/network capacity and non-evicting global storage passed");
}
