#!/usr/bin/env node

import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";

const benchmarksDir = resolve(import.meta.dirname, "..");
const repoDir = resolve(benchmarksDir, "..");
const runDir = resolve(
  process.env.BENCH_RUN_DIR ?? resolve(repoDir, "target/benchmarks/runs/current"),
);
const groups = ["javascript", "rust", "cli", "elixir"];
const reports = await Promise.all(
  groups.map((group) => readJson(resolve(runDir, "package-sizes", `${group}.json`))),
);
const reference = reports[0];

for (const [index, report] of reports.entries()) {
  const group = groups[index];
  if (
    report.schemaVersion !== 2 ||
    report.group !== group ||
    report.system?.platform !== reference.system?.platform ||
    report.system?.architecture !== reference.system?.architecture ||
    JSON.stringify(report.boundaries) !== JSON.stringify(reference.boundaries) ||
    !Array.isArray(report.entries) ||
    report.entries.length === 0
  ) {
    throw new Error(`package-size fragment is invalid or incompatible: ${group}`);
  }
}

const report = {
  schemaVersion: 2,
  system: reference.system,
  boundaries: reference.boundaries,
  entries: reports.flatMap(({ entries }) => entries),
};
const output = resolve(runDir, "package-sizes.json");
await mkdir(runDir, { recursive: true });
await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
console.log(output);

async function readJson(path) {
  try {
    return JSON.parse(await readFile(path, "utf8"));
  } catch (error) {
    throw new Error(`could not read package-size fragment ${path}`, { cause: error });
  }
}
