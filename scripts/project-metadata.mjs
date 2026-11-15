import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";

export async function projectMetadata(root = process.cwd()) {
  if (typeof Bun.TOML?.parse !== "function") {
    throw new Error("Bun with TOML.parse support is required; the verified toolchain uses Bun 1.3.14");
  }
  const cargo = Bun.TOML.parse(await readFile(join(root, "Cargo.toml"), "utf8"));
  const leptos = cargo.package?.metadata?.leptos;
  const referenceSite = cargo.package?.metadata?.["leptos-cf"]?.["reference-site"];
  if (!leptos || typeof referenceSite !== "boolean") {
    throw new Error("Cargo.toml must declare Leptos metadata and package.metadata.leptos-cf.reference-site");
  }
  if (!/^[a-z][a-z0-9-]*$/.test(leptos["output-name"])) {
    throw new Error("Leptos output-name must contain lowercase letters, digits, and hyphens");
  }
  if (leptos["site-root"] !== "target/site" || leptos["site-pkg-dir"] !== "pkg") {
    throw new Error("The edge runtime requires site-root target/site and site-pkg-dir pkg; routing and asset headers share this contract");
  }
  root = resolve(root);
  const assetPrefix = `/${leptos["site-pkg-dir"].replace(/^\/+|\/+$/g, "")}/`;
  const site = resolve(root, leptos["site-root"]);
  return { root, cargo, leptos, referenceSite, assetPrefix, paths: {
    site, pkg: resolve(site, leptos["site-pkg-dir"]),
    manifest: join(site, "asset-manifest.json"),
    hashes: join(root, "target/asset-hashes.env"),
    worker: join(root, "build"),
  } };
}
