#!/usr/bin/env node
/**
 * Point the benchmark's `@lumis-sh/wasm-*` dependencies at the latest published
 * versions.
 *
 * Lumis 0.9 loads the parser package a project installs, and only one that
 * exports its language, which each package reached at a different patch
 * version. Pinning the latest keeps the comparison on the parsers a project
 * gets today, and keeps older ones out of the workspace lockfile, where pnpm
 * would reuse them for every other package that wants the same parser.
 *
 * Usage: node benchmarks/scripts/sync-language-deps.mjs [--check]
 */

import { execFile } from "node:child_process";
import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { promisify } from "node:util";

const root = resolve(import.meta.dirname, "../..");
const manifestPath = resolve(root, "benchmarks/javascript/package.json");
const run = promisify(execFile);

async function latest(name) {
  const { stdout } = await run("npm", ["view", name, "version"], { cwd: root });
  return stdout.trim();
}

const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
const names = Object.keys(manifest.dependencies).filter((name) =>
  name.startsWith("@lumis-sh/wasm-"),
);
const versions = await Promise.all(names.map((name) => latest(name)));
const drift = [];

names.forEach((name, index) => {
  const version = versions[index];
  if (manifest.dependencies[name] !== version) {
    drift.push(`${name}: ${manifest.dependencies[name]} -> ${version}`);
    manifest.dependencies[name] = version;
  }
});

if (process.argv.includes("--check")) {
  if (drift.length > 0) {
    console.error("benchmark language dependencies are behind the latest published versions:");
    for (const line of drift) console.error(`  ${line}`);
    console.error("\nRun: node benchmarks/scripts/sync-language-deps.mjs");
    process.exit(1);
  }
  console.log(`every benchmark language dependency is the latest (${names.length} packages)`);
} else if (drift.length > 0) {
  await writeFile(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);
  console.log(`updated ${drift.length} dependency/dependencies:`);
  for (const line of drift) console.log(`  ${line}`);
} else {
  console.log("already the latest published versions");
}
