const { expect } = require('@playwright/test');

// When replacing /lab, update this check to exercise the application's own mutation.
module.exports = async function checkApplicationMutation({ page }) {
  const baseURL = process.env.LEPTOS_BROWSER_ORIGIN;
  await page.goto(`${baseURL}/lab`, { waitUntil: 'networkidle' });
  const title = `Boundary proof ${Date.now()}`;
  await page.getByLabel('Create a task record').fill(title);
  await page.getByRole('button', { name: 'Add task' }).click();
  await expect(page.getByText(title)).toBeVisible();

  const row = page.locator('.todo-row').filter({ hasText: title });
  const toggled = page.waitForResponse(response =>
    new URL(response.url()).pathname.startsWith('/api/toggle_todo') && response.request().method() === 'POST');
  await row.getByRole('button', { name: 'Open' }).click();
  const response = await toggled;
  expect(response.status()).toBe(200);
  expect(await response.json()).toMatchObject({ title, completed: true });
  await expect(row).toHaveClass(/todo-row--done/);
  await row.getByRole('link', { name: title }).click();
  await expect(page).toHaveURL(/\/lab\/\d+$/);
  await expect(page.getByText(title)).toBeVisible();
  await expect(page.getByRole('button', { name: 'Mark open', exact: true })).toBeVisible();
  await page.goBack({ waitUntil: 'networkidle' });
  await page.locator('.todo-row').filter({ hasText: title }).getByRole('button', { name: /delete/i }).click();
  await expect(page.getByText(title)).toHaveCount(0);
};
