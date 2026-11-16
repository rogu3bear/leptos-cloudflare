#!/usr/bin/env bun
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { inventory } from "./build-provenance.mjs";
import { commandPlan, validateIdentity } from "./prepare-pages-release.mjs";

function require(condition, message) { if (!condition) throw new Error(message); }

export async function verifyReleaseFiles(directory) {
  const manifest = JSON.parse(await readFile(join(directory, "release.json"), "utf8"));
  require(manifest.schema === 1 && typeof manifest.source?.dirty === "boolean" && (manifest.source.commit === null || /^[0-9a-f]{40}$/.test(manifest.source.commit)), "Invalid release source identity");
  validateIdentity(manifest.identity);
  require(JSON.stringify(manifest.commands) === JSON.stringify(commandPlan(manifest.identity, manifest.source)), "Release command binding changed");
  const files = await inventory(directory, ["package.json", "wrangler.toml", "public", "migrations", "production.bundle", "preview.bundle", "production.metadata.json", "preview.metadata.json"]);
  require(JSON.stringify(files) === JSON.stringify(manifest.files), "Release files changed after qualification");
  return manifest;
}

export function verifyPagesProjectSafety(identity, { accountId, project }) {
  validateIdentity(identity);
  require(accountId === identity.accountId, "Readback account mismatch");
  require(project.name === identity.projectName && project.production_branch === identity.productionBranch && project.subdomain === `${identity.projectName}.pages.dev`, "Pages project identity mismatch");
  for (const environment of ["production", "preview"]) require(project.deployment_configs?.[environment]?.fail_open === false, `Pages ${environment} must fail closed before upload`);
  return { stage: "project-preflight", runtimeVerified: false };
}

// Consumes independently authenticated snapshots; it cannot authenticate JSON
// files or establish that a deployed endpoint actually serves this application.
export function verifyPagesReadback(manifest, snapshots) {
  verifyPagesProjectSafety(manifest.identity, snapshots);
  require(!manifest.source.dirty && /^[0-9a-f]{40}$/.test(manifest.source.commit), "Readback cannot qualify a dirty or unidentified source");
  const { project, deployment } = snapshots;
  const identity = manifest.identity;
  const configHash = manifest.files.find((file) => file.path === "wrangler.toml")?.sha256;
  for (const environment of ["production", "preview"]) {
    const config = project.deployment_configs?.[environment];
    require(config && config.compatibility_date === manifest.compatibilityDate && config.always_use_latest_compatibility_date === false
      && JSON.stringify(config.compatibility_flags) === JSON.stringify(manifest.compatibilityFlags)
      && config.wrangler_config_hash === configHash, `Pages ${environment} configuration mismatch`);
    require(Object.keys(config.d1_databases ?? {}).length === 1 && config.d1_databases.DB?.id === identity[`${environment}Database`].id, `Pages ${environment} D1 mismatch`);
  }
  require(typeof project.id === "string" && project.id.length > 0 && deployment.project_id === project.id && deployment.project_name === project.name, "Deployment belongs to another project");
  require(typeof deployment.id === "string" && deployment.id.length > 0 && project.canonical_deployment?.id === deployment.id, "Deployment is not the current production deployment");
  require(deployment.environment === "production" && deployment.is_skipped === false && deployment.latest_stage?.name === "deploy" && deployment.latest_stage.status === "success", "Production deployment is incomplete");
  const trigger = deployment.deployment_trigger?.metadata;
  require(trigger?.branch === identity.productionBranch && trigger.commit_hash === manifest.source.commit && trigger.commit_dirty === false, "Deployment source or branch mismatch");
  const url = new URL(deployment.url);
  require(url.protocol === "https:" && !url.username && !url.password && !url.port && url.pathname === "/" && !url.search && !url.hash
    && url.hostname.endsWith(`.${project.subdomain}`) && url.hostname !== project.subdomain, "Invalid immutable deployment URL");
  return { deploymentId: deployment.id, url: url.href, commit: manifest.source.commit, runtimeVerified: false };
}

if (import.meta.main) {
  const [directory, snapshotsPath, mode, ...extra] = process.argv.slice(2);
  require(directory && extra.length === 0 && (!mode || mode === "--project-only" && snapshotsPath), "Usage: bun scripts/verify-pages-release.mjs <release-directory> [authenticated-snapshots.json [--project-only]]");
  const manifest = await verifyReleaseFiles(directory);
  require(!manifest.source.dirty && manifest.source.commit, "Release requires a clean, commit-bound source build");
  if (snapshotsPath) {
    const snapshots = JSON.parse(await readFile(snapshotsPath, "utf8"));
    const result = mode === "--project-only" ? verifyPagesProjectSafety(manifest.identity, snapshots) : verifyPagesReadback(manifest, snapshots);
    console.log(JSON.stringify(result));
  } else console.log("[pages-release] Clean source and unchanged local release files verified; no provider or runtime verification claimed.");
}
