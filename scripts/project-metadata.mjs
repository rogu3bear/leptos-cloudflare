import { readFile } from "node:fs/promises";
import { join } from "node:path";

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
  return { cargo, leptos, referenceSite };
}
