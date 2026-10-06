#!/usr/bin/env node

/**
 * Point the benchmark's `@lumis-sh/wasm-*` dependencies at the versions the
 * catalog pins.
 *
 * The benchmark stages a local store from these installed packages, so a version
 * the catalog does not expect is not merely stale: the runtime asks for the
 * pinned one, misses, and reaches for the network, and the comparison stops
 * measuring the parser Lumis actually ships. Generating them keeps one source of
 * truth instead of a list somebody has to remember to bump.
 *
 * Usage: node benchmarks/scripts/sync-language-deps.mjs [--check]
 */

import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "../..");
const manifestPath = resolve(root, "benchmarks/javascript/package.json");
const catalogPath = resolve(root, "crates/lumis-wasm-runtime/src/catalog.rs");

const catalog = await readFile(catalogPath, "utf8");
const pinned = new Map(
  [...catalog.matchAll(/package_name:\s*"([^"]+)",\s*\n\s*version:\s*"([^"]+)"/gu)].map((match) => [
    match[1],
    match[2],
  ]),
);

if (pinned.size === 0) {
  throw new Error(`no pinned versions in ${catalogPath}; run mise run langs-gen-catalog`);
}

const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
const drift = [];

for (const name of Object.keys(manifest.dependencies)) {
  if (!name.startsWith("@lumis-sh/wasm-")) continue;
  const version = pinned.get(name);
  if (version === undefined) {
    throw new Error(`${name} is a benchmark dependency but is not in the catalog`);
  }
  if (manifest.dependencies[name] !== version) {
    drift.push(`${name}: ${manifest.dependencies[name]} -> ${version}`);
    manifest.dependencies[name] = version;
  }
}

if (process.argv.includes("--check")) {
  if (drift.length > 0) {
    console.error("benchmark language dependencies drifted from the catalog:");
    for (const line of drift) console.error(`  ${line}`);
    console.error("\nRun: node benchmarks/scripts/sync-language-deps.mjs");
    process.exit(1);
  }
  console.log(`every benchmark language dependency matches the catalog (${pinned.size} pinned)`);
} else if (drift.length > 0) {
  await writeFile(manifestPath, `${JSON.stringify(manifest, null, 2)}\n`);
  console.log(`updated ${drift.length} dependency/dependencies:`);
  for (const line of drift) console.log(`  ${line}`);
} else {
  console.log("already in sync with the catalog");
}
