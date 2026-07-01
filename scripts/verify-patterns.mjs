#!/usr/bin/env bun

import { existsSync } from "node:fs";
import { readFile, readdir } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";

const root = process.cwd();
const patternsDir = join(root, "patterns");
const rootReadmePath = join(patternsDir, "README.md");

const rootPathPrefixes = [
  "AGENTS.md",
  "ANCHOR.md",
  "NORTH_STAR.md",
  "README.md",
  "RELEASE.md",
  "SECURITY.md",
  "CONTRIBUTING.md",
  "STRATEGY.md",
  "Cargo.toml",
  "Cargo.lock",
  "wrangler.toml",
  "assets/",
  "docs/",
  "migrations/",
  "patterns/",
  "scripts/",
  "src/",
  "style/",
];

function requireSnippet(label, text, snippet) {
  if (!text.includes(snippet)) {
    throw new Error(`${label} is missing required snippet: ${snippet}`);
  }
}

function stripAnchor(pathText) {
  return pathText.split("#")[0].trim();
}

function isExternalLink(pathText) {
  return /^(https?:|mailto:|tel:)/.test(pathText) || pathText.startsWith("#");
}

function isRootPathReference(pathText) {
  const normalized = pathText.replace(/^\.\//, "");
  return rootPathPrefixes.some(
    (prefix) => normalized === prefix.replace(/\/$/, "") || normalized.startsWith(prefix),
  );
}

function resolveReference(fromFile, pathText, fromInlineCode = false) {
  const withoutAnchor = stripAnchor(pathText).replace(/^['"]|['"]$/g, "");
  if (!withoutAnchor) {
    return null;
  }

  if (fromInlineCode && isRootPathReference(withoutAnchor)) {
    return resolve(root, withoutAnchor.replace(/^\.\//, ""));
  }

  return resolve(dirname(fromFile), withoutAnchor);
}

function extractMarkdownLinks(text, fromFile) {
  const refs = [];
  const linkPattern = /\[[^\]]+\]\(([^)]+)\)/g;

  for (const match of text.matchAll(linkPattern)) {
    const rawTarget = match[1].trim().split(/\s+/)[0];
    if (!rawTarget || isExternalLink(rawTarget)) {
      continue;
    }

    refs.push({
      kind: "markdown link",
      original: rawTarget,
      resolved: resolveReference(fromFile, rawTarget),
    });
  }

  return refs;
}

function extractInlinePathReferences(text, fromFile) {
  const refs = [];
  const inlineCodePattern = /`([^`\n]+)`/g;

  for (const match of text.matchAll(inlineCodePattern)) {
    const token = match[1].trim();
    if (!token || token.includes(" ") || token.includes("<") || token.includes(">")) {
      continue;
    }

    if (!isRootPathReference(token)) {
      continue;
    }

    refs.push({
      kind: "inline path",
      original: token,
      resolved: resolveReference(fromFile, token, true),
    });
  }

  return refs;
}

function assertReferencesExist(label, references, errors) {
  for (const ref of references) {
    if (!ref.resolved || !existsSync(ref.resolved)) {
      const target = ref.resolved ? relative(root, ref.resolved) : "<empty>";
      errors.push(
        `${label} has a stale ${ref.kind}: ${ref.original} -> ${target}`,
      );
    }
  }
}

async function main() {
  if (!existsSync(rootReadmePath)) {
    throw new Error(`missing patterns index: ${relative(root, rootReadmePath)}`);
  }

  const rootReadme = await readFile(rootReadmePath, "utf8");
  requireSnippet("patterns/README.md", rootReadme, "second layer");
  requireSnippet("patterns/README.md", rootReadme, "core template");
  requireSnippet("patterns/README.md", rootReadme, "## Verification Contract");
  requireSnippet("patterns/README.md", rootReadme, "scripts/verify-patterns.mjs");
  requireSnippet("patterns/README.md", rootReadme, "./scripts/verify.sh");

  const entries = await readdir(patternsDir, { withFileTypes: true });
  const patternDirs = entries
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith("."))
    .map((entry) => entry.name)
    .sort();

  if (patternDirs.length === 0) {
    throw new Error("patterns/ must contain at least one pattern directory");
  }

  const errors = [];
  const rootReferences = [
    ...extractMarkdownLinks(rootReadme, rootReadmePath),
    ...extractInlinePathReferences(rootReadme, rootReadmePath),
  ];
  assertReferencesExist(
    "patterns/README.md",
    rootReferences,
    errors,
  );

  for (const patternName of patternDirs) {
    const readmePath = join(patternsDir, patternName, "README.md");
    const label = `patterns/${patternName}/README.md`;

    if (!existsSync(readmePath)) {
      errors.push(`patterns/${patternName}/ is missing README.md`);
      continue;
    }

    if (!rootReadme.includes(`./${patternName}/`)) {
      errors.push(`patterns/README.md does not link to patterns/${patternName}/`);
    }

    const readme = await readFile(readmePath, "utf8");
    if (!readme.includes("Problem")) {
      errors.push(`${label} must start from the problem it solves`);
    }

    const references = [
      ...extractMarkdownLinks(readme, readmePath),
      ...extractInlinePathReferences(readme, readmePath),
    ];
    assertReferencesExist(label, references, errors);

    const coreReferences = references.filter((ref) => {
      const repoRelative = relative(root, ref.resolved ?? "");
      return repoRelative && !repoRelative.startsWith("patterns/");
    });

    if (coreReferences.length === 0) {
      errors.push(`${label} must reference at least one core template contract or file`);
    }
  }

  if (errors.length > 0) {
    throw new Error(errors.map((error) => `- ${error}`).join("\n"));
  }

  console.log(
    `[verify-patterns] ${patternDirs.length} pattern READMEs are indexed and their core references resolve`,
  );
}

main().catch((error) => {
  console.error(`[verify-patterns] ${error.message}`);
  process.exit(1);
});
