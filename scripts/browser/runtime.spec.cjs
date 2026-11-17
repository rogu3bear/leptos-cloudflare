const { test, expect } = require('@playwright/test');

test('SSR shell completes browser hydration', async ({ page }) => {
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  const response = await page.goto(process.env.LEPTOS_BROWSER_ORIGIN + process.env.LEPTOS_BROWSER_PATH);
  expect(response.status()).toBe(200);
  await expect(page.locator('main')).toBeVisible();
  await page.waitForFunction(() => performance.getEntriesByName('leptos:hydrated', 'mark').length > 0);
  expect(errors).toEqual([]);
});

test('hydrated application server-function mutation persists', require('./application-check.cjs'));

if (process.env.LEPTOS_TRANSITION_DOCUMENT) {
  test('outgoing document hydrates and writes against the active release', async ({ page }) => {
    const document = JSON.parse(require('node:fs').readFileSync(process.env.LEPTOS_TRANSITION_DOCUMENT, 'utf8'));
    expect(document.baseline).toBe(process.env.LEPTOS_RELEASE_BASELINE);
    const origin = process.env.LEPTOS_BROWSER_ORIGIN;
    let served = false;
    await page.route(origin + document.path, async route => {
      if (served || !route.request().isNavigationRequest()) return route.continue();
      served = true;
      await route.fulfill({ status: 200, headers: document.headers, body: document.body });
    });
    await require('./application-check.cjs')({ page });
    expect(served).toBe(true);
  });
}
