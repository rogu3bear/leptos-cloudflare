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
