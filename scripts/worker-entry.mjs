// The generated Cloudflare entry handles no request. It publishes the
// build-generated routing data, then re-exports the workers-rs module: the
// Rust `fetch` handler in src/lib.rs is the first code to see every request.
export const EDGE_MANIFEST_GLOBAL = "__LEPTOS_EDGE__";

// Field names are the contract read by `EdgeManifest` in src/edge/route.rs.
export function edgeManifest({ assetPrefix = "/pkg/", assetPaths = null, aliases = {}, blockServerModules = false } = {}) {
  return { assetPrefix, assetPaths, aliases, blockServerModules };
}

export function workerEntry({ serverModule = "./index.js", ...manifest } = {}) {
  const module = JSON.stringify(serverModule);
  return `globalThis.${EDGE_MANIFEST_GLOBAL} = Object.freeze(${JSON.stringify(edgeManifest(manifest))});\nexport * from ${module};\nexport { default } from ${module};\n`;
}
