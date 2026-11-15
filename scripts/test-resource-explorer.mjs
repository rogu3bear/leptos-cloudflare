import { strict as assert } from "node:assert";

// Runs inside the existing local Worker harness and its disposable D1 store.
// No mock persistence, provider credentials, or independent server lifecycle.
export async function testResourceExplorer(origin) {
  const route = "/patterns/resource-explorer";
  const first = await fetch(`${origin}${route}`);
  assert.equal(first.status, 200);
  const cookie = first.headers.get("set-cookie")?.split(";")[0];
  assert.ok(cookie, "explorer must establish a session");
  const initialHtml = await first.text();
  const createPath = [...initialHtml.matchAll(/action="([^"]+)"/g)]
    .map((match) => match[1]).find((path) => path.startsWith("/api/create_todo"));
  assert.ok(createPath, "native create form must expose its generated action");

  async function call(path, body, session = cookie) {
    const response = await fetch(`${origin}${path}`, {
      method: "POST",
      headers: { "content-type": "application/x-www-form-urlencoded", accept: "application/json", origin, cookie: session },
      body: new URLSearchParams(Object.entries(body).map(([key, value]) => [key, String(value)])),
      signal: AbortSignal.timeout(10_000),
    });
    const text = await response.text();
    // ServerFnError has its own wire encoding; only successful results use the
    // JSON output codec. Keep HTTP failures available to validation assertions.
    if (!response.ok) return { status: response.status, body: text };
    let decoded;
    try { decoded = JSON.parse(text); }
    catch { throw new Error(`${path} returned non-JSON success (${response.headers.get("content-type")}): ${text.slice(0, 160)}`); }
    return { status: response.status, body: decoded };
  }
  async function ok(path, body, session) {
    const result = await call(path, body, session);
    assert.equal(result.status, 200, JSON.stringify(result.body));
    return result.body;
  }
  async function rawDocument(path) {
    const response = await fetch(`${origin}${path}`, { headers: { cookie }, signal: AbortSignal.timeout(10_000) });
    const html = await response.text();
    assert.equal(response.status, 200, `${path} returned ${response.status}: ${html.slice(0, 160)}`);
    return html;
  }
  async function document(path) {
    // Verify useful HTML independently of hydration/stream insertion scripts.
    return (await rawDocument(path)).replace(/<script\b[^>]*>[\s\S]*?<\/script>/g, "").replace(/<!--[\s\S]*?-->/g, "");
  }

  const original = await ok(createPath, { title: "Older resource needle" });
  for (let index = 0; index < 101; index += 1) {
    await ok(createPath, { title: `Recent resource ${index}` });
  }
  const all = await ok("/api/search_todos", { query: "", status: "all" });
  assert.equal(all.stats.total, 102);
  assert.equal(all.items.length, all.visible_limit);
  assert.equal(all.is_truncated, true);

  const filtered = await ok("/api/search_todos", { query: "older RESOURCE", status: "open" });
  assert.equal(filtered.stats.total, 1, "search must include matches beyond the unfiltered first page");
  assert.equal(filtered.items[0].id, original.id);
  assert.equal(filtered.is_truncated, false);
  const list = await document(`${route}?q=Older&status=open`);
  assert.match(list, /Older resource needle/);
  assert.match(list, /1 matching record/);
  assert.ok(list.includes(`${route}/${original.id}?q=Older&amp;status=open`), "detail links must carry collection filters");
  const detail = await document(`${route}/${original.id}?q=Older&status=open`);
  assert.match(detail, /Older resource needle/);
  assert.match(detail, /Related records/);
  assert.match(detail, /Recent resource 100/);
  const serialized = [...(await rawDocument(`${route}/${original.id}`)).matchAll(/__RESOLVED_RESOURCES\[\d+\]\s*=\s*("(?:\\.|[^"\\])*")\s*;/g)]
    .map((match) => JSON.parse(JSON.parse(match[1])));
  assert.equal(serialized.filter((value) => value.Ok?.items).length, 1,
    "detail must serialize related records once; duplicate snapshots can seed unrelated resources during client navigation");
  assert.ok(detail.includes(`${route}?q=Older&amp;status=open`), "back link must preserve filters");
  assert.match(await document(`${route}?q=absent-needle&status=all`), /No matching records/);
  assert.match(await document(`${route}/invalid`), /Record unavailable/);

  const requested = { id: original.id, title: "Reviewed resource", completed: true, expected_title: original.title, expected_completed: false };
  const saved = await ok("/api/update_todo", requested);
  assert.equal(saved.Saved.title, requested.title);
  assert.equal(saved.Saved.completed, true);
  assert.match(await document(`${route}?q=Reviewed&status=open`), /No matching records/);
  assert.deepEqual(await ok("/api/update_todo", requested), saved, "lost-response retry must recognize the already-saved values");
  const conflict = await ok("/api/update_todo", { ...requested, title: "Stale draft" });
  assert.equal(conflict.Conflict.title, "Reviewed resource", "stale review must not overwrite current saved values");
  const rebased = await ok("/api/update_todo", { ...requested, title: "Re-reviewed resource", expected_title: "Reviewed resource", expected_completed: true });
  assert.equal(rebased.Saved.title, "Re-reviewed resource");
  assert.match(await document(`${route}/${original.id}`), /Re-reviewed resource/);

  const second = await fetch(`${origin}${route}`);
  const otherCookie = second.headers.get("set-cookie")?.split(";")[0];
  await second.text();
  assert.ok(otherCookie && otherCookie !== cookie);
  const denied = await ok("/api/update_todo", requested, otherCookie);
  assert.equal(denied, "Unavailable", "another session receives no record data");
  const empty = await ok("/api/search_todos", { query: "", status: "all" }, otherCookie);
  assert.equal(empty.stats.total, 0);
  assert.equal(empty.items.length, 0);
  const invalid = await call("/api/update_todo", { ...requested, title: "   " });
  assert.ok(invalid.status >= 400, "server validation must reject an empty draft");
  const tooLong = await call("/api/update_todo", { ...requested, title: "x".repeat(121) });
  assert.ok(tooLong.status >= 400, "server validation must enforce the existing title limit");
  assert.ok((await call("/api/search_todos", { query: "", status: "invalid" })).status >= 400);
  const literal = await ok(createPath, { title: "Literal 50%_ resource" });
  const literalMatch = await ok("/api/search_todos", { query: "50%_", status: "all" });
  assert.deepEqual(literalMatch.items.map((item) => item.id), [literal.id], "search punctuation must remain literal");
  const complete = await ok("/api/search_todos", { query: "", status: "completed" });
  assert.equal(complete.stats.total, 1);
  assert.equal(complete.items[0].title, "Re-reviewed resource");
  console.log("[resource-explorer] SSR filters/detail/related/empty, full-collection search, reviewed saves, conflict/retry, validation, and session boundaries passed");
}
