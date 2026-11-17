const { test, expect } = require('@playwright/test');

const baseURL = process.env.LEPTOS_BROWSER_ORIGIN;

test('public field-guide routes render and hydrate without console errors', async ({ page }) => {
  const errors = [];
  page.on('console', (message) => {
    if (message.type() === 'error') errors.push(message.text());
  });
  page.on('pageerror', (error) => errors.push(error.message));

  for (const path of ['/', '/start', '/architecture', '/patterns', '/lab', '/about', '/contact']) {
    await page.goto(`${baseURL}${path}`, { waitUntil: 'networkidle' });
    await expect(page.locator('main#content')).toHaveCount(1);
    await expect(page.locator('h1')).toBeVisible();
  }

  expect(errors).toEqual([]);
});

test('bounded intake validates, persists, and states its non-delivery scope', async ({ page }) => {
  await page.goto(`${baseURL}/contact`, { waitUntil: 'networkidle' });
  await expect(page.getByRole('button', { name: 'Send' })).toBeDisabled();
  await page.getByLabel('Name').fill('Local proof');
  await page.getByLabel('Email').fill('proof@example.test');
  await page.getByLabel('Topic').fill('Boundary review');
  await page.getByLabel('Message').fill('Exercise the bounded D1 intake path.');
  await page.getByRole('button', { name: 'Send' }).click();
  await expect(page.getByRole('status')).toContainText(/received|saved|accepted/i);
  await expect(page.getByText(/does not send email/i)).toBeVisible();
});

test('unknown routes return an SSR 404 with a hydrated recovery path', async ({ page }) => {
  const response = await page.goto(`${baseURL}/outside-field-guide`, { waitUntil: 'networkidle' });
  expect(response.status()).toBe(404);
  await expect(page.getByRole('heading', { name: 'This route is outside the field guide.' })).toBeVisible();
  await expect(page.locator('#content').getByRole('link', { name: 'Field guide' })).toBeVisible();
});

test('all public routes fit a 320 pixel viewport', async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 800 });

  for (const path of ['/', '/start', '/architecture', '/patterns', '/lab', '/about', '/contact', '/outside-field-guide']) {
    await page.goto(`${baseURL}${path}`, { waitUntil: 'networkidle' });
    const dimensions = await page.evaluate(() => ({
      viewport: document.documentElement.clientWidth,
      document: document.documentElement.scrollWidth,
      body: document.body.scrollWidth,
    }));
    expect(dimensions.document, `${path} document overflow`).toBeLessThanOrEqual(dimensions.viewport);
    expect(dimensions.body, `${path} body overflow`).toBeLessThanOrEqual(dimensions.viewport);
  }
});

test('keyboard entry exposes the skip link and visible focus', async ({ page }) => {
  await page.goto(`${baseURL}/`, { waitUntil: 'networkidle' });
  // WebKit on macOS uses Option+Tab for link traversal unless full keyboard
  // access is enabled in the host browser settings.
  await page.keyboard.press('Alt+Tab');

  const skipLink = page.getByRole('link', { name: 'Skip to content' });
  await expect(skipLink).toBeFocused();
  await expect(skipLink).toBeVisible();
  const focusStyle = await skipLink.evaluate((element) => {
    const style = getComputedStyle(element);
    return { outlineStyle: style.outlineStyle, outlineWidth: style.outlineWidth };
  });
  expect(focusStyle.outlineStyle).not.toBe('none');
  expect(focusStyle.outlineWidth).not.toBe('0px');

  await page.keyboard.press('Alt+Tab');
  await expect(page.getByRole('link', { name: 'Leptos CF home' })).toBeFocused();
});

test('D1 task records remain isolated between browser sessions', async ({ browser }) => {
  const ownerContext = await browser.newContext();
  const strangerContext = await browser.newContext();
  const owner = await ownerContext.newPage();
  const stranger = await strangerContext.newPage();
  const title = `Session boundary ${Date.now()}`;

  try {
    await owner.goto(`${baseURL}/lab`, { waitUntil: 'networkidle' });
    await owner.getByLabel('Create a task record').fill(title);
    await owner.getByRole('button', { name: 'Add task' }).click();
    await expect(owner.getByText(title)).toBeVisible();

    await stranger.goto(`${baseURL}/lab`, { waitUntil: 'networkidle' });
    await expect(stranger.getByText(title)).toHaveCount(0);

    await owner.locator('.todo-row').filter({ hasText: title }).getByRole('button', { name: 'Delete' }).click();
    await expect(owner.getByText(title)).toHaveCount(0);
  } finally {
    await ownerContext.close();
    await strangerContext.close();
  }
});

test('dynamic responses bind every script to the per-response CSP nonce', async ({ page }) => {
  const response = await page.goto(`${baseURL}/`, { waitUntil: 'networkidle' });
  const headers = await response.allHeaders();
  const csp = headers['content-security-policy'];
  expect(csp).toContain("default-src 'self'");
  expect(csp).toContain("frame-ancestors 'none'");
  expect(csp).not.toMatch(/script-src[^;]*'unsafe-inline'/);

  const nonce = csp.match(/'nonce-([^']+)'/)?.[1];
  expect(nonce).toBeTruthy();
  const scriptNonces = await page.locator('script').evaluateAll((scripts) =>
    scripts.map((script) => script.nonce)
  );
  expect(scriptNonces.length).toBeGreaterThan(0);
  expect(scriptNonces.every((value) => value === nonce)).toBe(true);
});

test('resource explorer supports review, failed save retry, persisted readback and filters', async ({ page }) => {
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.goto(`${baseURL}/patterns/resource-explorer`, { waitUntil: 'networkidle' });
  await page.getByText('Add a record', { exact: true }).click();
  const title = `Reviewed browser resource ${Date.now()}`;
  await page.getByLabel('New record title').fill(title);
  await page.getByRole('button', { name: 'Add record', exact: true }).click();
  await expect(page.getByRole('link', { name: title, exact: true })).toBeVisible();
  await page.getByLabel('Search records').fill(title);
  await page.getByRole('combobox').selectOption('open');
  await page.getByRole('button', { name: 'Apply filters' }).click();
  await expect(page).toHaveURL(/status=open/);
  await page.getByRole('link', { name: title, exact: true }).click();
  await expect(page.getByRole('heading', { name: title, exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Edit record' }).click();
  await page.getByLabel('Title', { exact: true }).fill(`${title} saved`);
  await page.getByRole('combobox').selectOption('completed');
  await page.getByRole('button', { name: 'Review changes', exact: true }).click();
  await expect(page.getByRole('heading', { name: title, exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Back to editing' }).click();
  await expect(page.getByLabel('Title', { exact: true })).toHaveValue(`${title} saved`);
  await page.getByRole('button', { name: 'Review changes', exact: true }).click();
  await page.route('**/api/update_todo', route => route.abort('failed'));
  await page.getByRole('button', { name: 'Save changes', exact: true }).click();
  await expect(page.getByRole('alert')).toContainText('draft is preserved');
  await page.unroute('**/api/update_todo');
  await page.getByRole('button', { name: 'Save changes', exact: true }).click();
  await expect(page.getByRole('heading', { name: `${title} saved`, exact: true })).toBeVisible();
  await page.reload({ waitUntil: 'networkidle' });
  await expect(page.getByRole('heading', { name: `${title} saved`, exact: true })).toBeVisible();
  await page.setViewportSize({width:320,height:800});
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(320);
  await page.getByRole('link', { name: '← Back to results' }).click();
  await expect(page).toHaveURL(/status=open/);
  await expect(page.getByRole('link', { name: `${title} saved`, exact: true })).toHaveCount(0);
  await page.getByRole('combobox').selectOption('completed');
  await page.getByRole('button', { name: 'Apply filters' }).click();
  await expect(page.getByRole('link', { name: `${title} saved`, exact: true })).toBeVisible();
  expect(errors).toEqual([]);
});

test('detail mutations preserve the record on network failure and allow retry', async ({ page }) => {
  await page.goto(`${baseURL}/lab`, { waitUntil: 'networkidle' });
  const title = `Detail retry ${Date.now()}`;
  await page.getByLabel('Create a task record').fill(title);
  await page.getByRole('button', { name: 'Add task' }).click();
  await page.getByRole('link', { name: title, exact: true }).click();
  await page.route('**/api/toggle_todo*', route => route.abort('failed'));
  await page.getByRole('button', { name: 'Mark done', exact: true }).click();
  await expect(page.getByRole('alert')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Mark done', exact: true })).toBeEnabled();
  await page.unroute('**/api/toggle_todo*');
  await page.getByRole('button', { name: 'Mark done', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Mark open', exact: true })).toBeEnabled();
  await page.route('**/api/delete_todo*', route => route.abort('failed'));
  await page.getByRole('button', { name: 'Delete task', exact: true }).click();
  await expect(page.getByRole('alert')).toBeVisible();
  await expect(page.getByRole('heading', { name: title, exact: true })).toBeVisible();
  await expect(page).toHaveURL(/\/lab\/\d+$/);
  await page.unroute('**/api/delete_todo*');
  await page.getByRole('button', { name: 'Delete task', exact: true }).click();
  await expect(page).toHaveURL(/\/lab$/);
  await expect(page.getByText(title)).toHaveCount(0);
});

test('rotated browser cookies cannot reset the shared client-network intake quota', async ({ browser }) => {
  async function submitFromFreshBrowser(expectAccepted) {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 }, extraHTTPHeaders: { "cf-connecting-ip": "203.0.113.77" } });
    const page = await context.newPage();
    try {
      await page.goto(`${baseURL}/contact`, { waitUntil: 'networkidle' });
      await expect(page.getByText(/shared client-network budgets/i)).toBeVisible();
      await page.getByLabel('Name').fill('Synthetic quota proof');
      await page.getByLabel('Email').fill('quota@example.test');
      await page.getByLabel('Topic').fill('Local quota');
      await page.getByLabel('Message').fill('Disposable browser security regression.');
      await page.getByRole('button', { name: 'Send' }).click();
      if (expectAccepted) await expect(page.getByRole('status')).toContainText(/accepted/i);
      else {
        await expect(page.getByRole('status')).toContainText(/limit reached/i);
        await expect(page.getByLabel('Message')).toHaveValue('Disposable browser security regression.');
        await expect(page.getByRole('button', { name: 'Send' })).toBeEnabled();
      }
    } finally { await context.close(); }
  }
  await submitFromFreshBrowser(true);
  await submitFromFreshBrowser(true);
  await submitFromFreshBrowser(true);
  await submitFromFreshBrowser(false);
});
