# Exact versions qualified by the local release gate.
EXPECTED_CARGO_LEPTOS_VERSION="0.3.7"
EXPECTED_WORKER_BUILD_VERSION="0.7.5"
expected_wrangler_version() {
  bun -e 'console.log((await Bun.file(process.argv[1]).json()).devDependencies.wrangler)' "$ROOT_DIR/package.json"
}
