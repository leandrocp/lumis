#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFile, writeFile } from "node:fs/promises";
import { cpus, platform, release, totalmem } from "node:os";
import { resolve } from "node:path";
import {
  comparisonTableDocuments,
  implementationById,
  lumisReference,
} from "./implementations.mjs";
import { countTokens } from "./tokens.mjs";

const benchmarksDir = resolve(import.meta.dirname, "..");
const repoDir = resolve(benchmarksDir, "..");
const runDir = resolve(
  process.env.BENCH_RUN_DIR ?? resolve(repoDir, "target/benchmarks/runs/current"),
  "comparison",
);

const manifest = await readJson(resolve(repoDir, "website/public/comparison-data/manifest.json"));
const javascript = await readJson(resolve(runDir, "javascript.json"));
const memory = await readJson(resolve(runDir, "memory.json"));
// Both runs time the first flavour.
const theme = manifest.themes[0].id;
if (memory.theme !== theme) {
  throw new Error(`the memory pass rendered ${memory.theme}, not ${theme}; rerun it`);
}

const assetsDir = resolve(benchmarksDir, "showcase/generated/assets");
const timedOnly = await readJson(resolve(assetsDir, "timed-documents.json"));
// The published manifest leaves out each document's file, which the showcase keeps.
const assetDocuments = [...(await readJson(resolve(assetsDir, "documents.json"))), ...timedOnly];
// The Lumis column is its reference runtime. The Node addon is reported beside
// it, as what a JavaScript caller of Lumis pays.
const lumisRows = [
  { row: "lumis", implementation: lumisReference },
  { row: "lumis-js-node", implementation: "lumis-js-node" },
];

const documents = [];
for (const { id: documentId, size } of comparisonTableDocuments) {
  const document = [...manifest.documents, ...timedOnly].find((entry) => entry.id === documentId);
  if (!document) throw new Error(`no comparison document is named ${documentId}`);
  const published = manifest.documents.includes(document);
  const { file } = assetDocuments.find((entry) => entry.id === document.id);
  const source = await readFile(resolve(assetsDir, file));
  const results = [];
  for (const { id, label } of manifest.implementations) {
    for (const { row, implementation } of id === "lumis"
      ? lumisRows
      : [{ row: id, implementation: id }]) {
      const timing = await timingFor(implementation, document.id);
      const measured = memory.results.find(
        (result) => result.document === document.id && result.implementation === implementation,
      );
      // Every output on the page gets its own time and nothing else does, so a pair
      // with no output has no time and a pair with output cannot go untimed.
      if (document.unsupported.includes(id)) {
        if (timing !== undefined || measured !== undefined) {
          throw new Error(`${label} measured ${document.id}, which it does not support`);
        }
        continue;
      }
      if (!(timing?.totalNs > 0)) throw new Error(`${label} has no timing for ${document.id}`);
      // The memory pass checked each of its outputs against the timed one, so a
      // figure here belongs to the same output as the time.
      if (!(measured?.peakRssBytes > 0)) {
        throw new Error(`${label} has no peak RSS for ${document.id}`);
      }

      // The time is published beside the output, so it has to be the time to
      // produce that output. A gallery output that differs from the published one
      // means the comparison is stale. A timed-only document has no published
      // output, so every Lumis runtime has to agree with the reference instead.
      const fragment = await readFile(
        resolve(runDir, "fragments", document.id, `${implementation}.html`),
      );
      const sha256 = createHash("sha256").update(fragment).digest("hex");
      const expected = published
        ? document.outputSha256[id]?.[theme]
        : await fileSha256(
            resolve(
              runDir,
              "fragments",
              document.id,
              `${id === "lumis" ? lumisReference : id}.html`,
            ),
          );
      if (sha256 !== expected) {
        throw new Error(
          `${implementationById(implementation).label} does not render ${document.id} as ` +
            `${published ? "the published comparison shows it" : "Lumis Rust does"}; ` +
            "run `mise run -C benchmarks showcase-publish` first",
        );
      }
      results.push({
        id: row,
        totalNs: timing.totalNs,
        samples: timing.samples,
        setupNs: measured.setupNs,
        peakRssBytes: measured.peakRssBytes,
        // Counted as the gallery counts it, so a timed-only document has one too.
        tokens: countTokens(fragment.toString("utf8"), label),
        sha256,
      });
    }
  }
  documents.push({
    id: document.id,
    size,
    label: document.label,
    languageLabel: document.languageLabel,
    source: document.source,
    bytes: source.byteLength,
    lines: source.toString("utf8").split("\n").length - (source.at(-1) === 0x0a ? 1 : 0),
    results,
  });
}

const report = {
  schemaVersion: 3,
  measuredAt: new Date().toISOString(),
  metric: "median",
  timingBoundary:
    "one highlight of the whole document through the call that rendered the published output, " +
    `with Lumis as ${implementationById(lumisReference).label}; ` +
    "building the highlighter and loading its languages are excluded",
  setupBoundary:
    "a fresh process building one library's highlighter, loading the languages the document " +
    "needs and highlighting it once, timed inside the process; the Lumis data directory " +
    "already holds its compiled parsers, as it does after a first run",
  memory: {
    metric: "median",
    runs: memory.runs,
    boundary:
      "peak resident set size of a fresh process that builds one library's highlighter, " +
      "loads its languages and highlights the document once; each process holds only that library",
    definition:
      "resident set size counts the runtime (Node.js and V8, or the Rust executable), compiled code, " +
      "WebAssembly memory and allocator overhead, not only the live heap",
    baselines: memory.baselines,
  },
  theme,
  // These numbers are only comparable to each other, so the page says which
  // machine measured them.
  system: {
    platform: platform(),
    release: release(),
    os: operatingSystem(),
    architecture: process.arch,
    cpu: cpus()[0]?.model,
    cores: cpus().length,
    memoryBytes: totalmem(),
  },
  runtimes: {
    node: javascript.run.node,
    rust: execFileSync("rustc", ["--version"], { encoding: "utf8" }).trim(),
  },
  implementations: manifest.implementations.flatMap(({ id, label, version }) =>
    id === "lumis"
      ? lumisRows.map(({ row, implementation }) => ({
          id: row,
          label: implementationById(implementation).label,
          version,
        }))
      : [{ id, label, version }],
  ),
  documents,
};
const output = resolve(runDir, "timings.json");
await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
console.log(
  `Timed ${documents.reduce((total, document) => total + document.results.length, 0)} ` +
    `outputs of ${documents.length} documents on ${report.system.cpu}: ${output}`,
);

async function timingFor(implementation, document) {
  if (implementationById(implementation).runner === "criterion") {
    const directory = resolve(runDir, "criterion", document, implementation, "new");
    const estimates = await readJson(resolve(directory, "estimates.json"), true);
    if (!estimates) return;
    const sample = await readJson(resolve(directory, "sample.json"));
    return { totalNs: estimates.median.point_estimate, samples: sample.iters.length };
  }
  const result = javascript.results.find(
    (entry) => entry.document === document && entry.implementation === implementation,
  );
  return result && { totalNs: result.totalNs, samples: result.samples };
}

// `release()` is the kernel version, which on macOS is Darwin's rather than the
// one a reader knows the machine by.
function operatingSystem() {
  if (platform() === "darwin") {
    return `${product("-productName")} ${product("-productVersion")}`;
  }
  if (platform() === "linux") {
    return execFileSync("sh", ["-c", '. /etc/os-release && printf %s "$PRETTY_NAME"'], {
      encoding: "utf8",
    });
  }
  return `${platform()} ${release()}`;
}

function product(key) {
  return execFileSync("sw_vers", [key], { encoding: "utf8" }).trim();
}

async function fileSha256(path) {
  return createHash("sha256")
    .update(await readFile(path))
    .digest("hex");
}

async function readJson(path, optional = false) {
  try {
    return JSON.parse(await readFile(path, "utf8"));
  } catch (error) {
    if (optional && error.code === "ENOENT") return;
    throw error;
  }
}
