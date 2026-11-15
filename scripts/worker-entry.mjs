export function workerEntry({ serverModule = "./index.js", runtimeModule = "./dispatch.mjs", assetPrefix = "/pkg/", assetPaths = null, blockServerModules = false } = {}) {
  return `import LeptosWorker from ${JSON.stringify(serverModule)};\nimport { withDispatch } from ${JSON.stringify(runtimeModule)};\nexport default withDispatch(LeptosWorker, ${JSON.stringify({ assetPrefix, assetPaths, blockServerModules })});\n`;
}
