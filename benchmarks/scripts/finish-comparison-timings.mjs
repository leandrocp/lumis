#!/usr/bin/env node

import { createHash } from "node:crypto";
import { readFile, writeFile } from "node:fs/promises";
import { cpus, platform, release } from "node:os";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { implementationById, lumisReference } from "./implementations.mjs";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoDir = resolve(benchmarksDir, "..");
const runDir = resolve(
  process.env.BENCH_RUN_DIR ?? resolve(repoDir, "target/benchmarks/runs/current"),
  "comparison",
);

const manifest = await readJson(resolve(repoDir, "website/public/comparison-data/manifest.json"));
const javascript = await readJson(resolve(runDir, "javascript.json"));
// Both runs time the first flavour.
const theme = manifest.themes[0].id;

const documents = [];
for (const document of manifest.documents) {
  const results = [];
  for (const { id, label } of manifest.implementations) {
    // The published Lumis column is its reference runtime's output.
    const implementation = id === "lumis" ? lumisReference : id;
    const totalNs = await timeFor(implementation, document.id);
    // Every output on the page gets its own time and nothing else does, so a pair
    // with no output has no time and a pair with output cannot go untimed.
    if (document.unsupported.includes(id)) {
      if (totalNs !== undefined)
        throw new Error(`${label} timed ${document.id}, which it does not support`);
      continue;
    }
    if (!(totalNs > 0)) throw new Error(`${label} has no timing for ${document.id}`);

    // The time is published beside the output, so it has to be the time to produce
    // that output. One that differs from the published output means the comparison
    // is stale, and its time would describe a page no one can see. The hash goes
    // out with the time, so the page can drop a time once the output it measured
    // has been replaced.
    const fragment = await readFile(
      resolve(runDir, "fragments", document.id, `${implementation}.html`),
    );
    const sha256 = createHash("sha256").update(fragment).digest("hex");
    if (sha256 !== document.outputSha256[id]?.[theme]) {
      throw new Error(
        `${label} does not render ${document.id} as the published comparison shows it; ` +
          "run `mise run -C benchmarks showcase-publish` first",
      );
    }
    results.push({ id, totalNs, sha256 });
  }
  documents.push({ id: document.id, results });
}

const report = {
  schemaVersion: 1,
  metric: "median",
  timingBoundary:
    "one highlight of the whole document through the call that rendered the published output, " +
    `with Lumis as ${implementationById(lumisReference).label}; ` +
    "building the highlighter and loading its languages are excluded",
  theme,
  // These numbers are only comparable to each other, so the page says which
  // machine measured them.
  system: {
    platform: platform(),
    release: release(),
    architecture: process.arch,
    cpu: cpus()[0]?.model,
  },
  documents,
};
const output = resolve(runDir, "timings.json");
await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
console.log(
  `Timed ${documents.reduce((total, document) => total + document.results.length, 0)} ` +
    `outputs of ${documents.length} documents on ${report.system.cpu}: ${output}`,
);

async function timeFor(implementation, document) {
  if (implementationById(implementation).runner === "criterion") {
    const estimates = await readJson(
      resolve(runDir, "criterion", document, implementation, "new/estimates.json"),
      true,
    );
    return estimates?.median.point_estimate;
  }
  return javascript.results.find(
    (result) => result.document === document && result.implementation === implementation,
  )?.totalNs;
}

async function readJson(path, optional = false) {
  try {
    return JSON.parse(await readFile(path, "utf8"));
  } catch (error) {
    if (optional && error.code === "ENOENT") return;
    throw error;
  }
}
