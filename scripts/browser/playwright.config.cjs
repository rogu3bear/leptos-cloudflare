module.exports = {
  testDir: __dirname,
  testMatch: process.env.LEPTOS_BROWSER_REFERENCE === '1' ? '*.spec.cjs' : 'runtime.spec.cjs',
  outputDir: process.env.LEPTOS_BROWSER_RESULTS,
  timeout: 30_000,
  workers: 1,
  retries: 0,
  use: { browserName: 'webkit', headless: true },
};
