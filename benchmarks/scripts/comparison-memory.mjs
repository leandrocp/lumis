#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import {
  comparisonTableDocuments,
  implementationById,
  lumisReference,
} from "./implementations.mjs";

// The peak resident set size of a fresh process that builds one library's
// highlighter for one of the table's documents and highlights it once, the
// median of `runs` processes. Each process holds only that library and that
// document's languages, and every one of them has to render the published
// output byte for byte, as the timing pass does, so the figure is the memory of
// the output beside it.
const runs = 5;

const benchmarksDir = resolve(import.meta.dirname, "..");
const repoDir = resolve(benchmarksDir, "..");
const runDir = resolve(
  process.env.BENCH_RUN_DIR ?? resolve(repoDir, "target/benchmarks/runs/current"),
  "comparison",
);
const rustBinary = resolve(
  process.env.CARGO_TARGET_DIR ?? resolve(repoDir, "target/benchmarks/rust-target"),
  "release",
  process.platform === "win32" ? "comparison-memory.exe" : "comparison-memory",
);
const javascriptEntry = resolve(benchmarksDir, "javascript/src/comparison-memory.mjs");
const fragmentsDir = resolve(runDir, "memory");

const manifest = JSON.parse(
  await readFile(resolve(repoDir, "website/public/comparison-data/manifest.json"), "utf8"),
);
// The timing pass renders the first flavour, so this one does too.
const theme = manifest.themes[0].id;

const timedOnly = JSON.parse(
  await readFile(resolve(benchmarksDir, "showcase/generated/assets/timed-documents.json"), "utf8"),
);
// The Node addon is timed beside the Lumis column, so a reader can see what a
// JavaScript caller of Lumis pays, not only a Rust one.
const lumisRows = [lumisReference, "lumis-js-node"];

const results = [];
for (const { id: documentId } of comparisonTableDocuments) {
  const document = [...manifest.documents, ...timedOnly].find((entry) => entry.id === documentId);
  if (!document) throw new Error(`no comparison document is named ${documentId}`);
  await mkdir(resolve(fragmentsDir, document.id), { recursive: true });
  const published = manifest.documents.includes(document);
  for (const { id } of manifest.implementations) {
    if (document.unsupported.includes(id)) continue;
    for (const implementation of id === "lumis" ? lumisRows : [id]) {
      // A gallery output has to be the published one. A timed-only document has
      // none, so it has to be what the timing pass rendered, and every Lumis
      // runtime has to render what the reference does.
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
      const label = implementationById(implementation).label;
      const samples = [];
      for (let run = 0; run < runs; run += 1) {
        const fragment = resolve(fragmentsDir, document.id, `${implementation}.html`);
        samples.push(measure(implementation, document.id, fragment));
        if ((await fileSha256(fragment)) !== expected) {
          throw new Error(
            `${label} does not render ${document.id} as the ${published ? "published comparison" : "timing pass"} ` +
              "shows it; run `mise run -C benchmarks showcase-publish` first",
          );
        }
      }
      const peakRssBytes = median(samples.map((sample) => sample.peakRssBytes));
      const setupNs = median(samples.map((sample) => sample.setupNs));
      results.push({ document: document.id, implementation, peakRssBytes, setupNs, samples });
      console.log(
        `${document.id} · ${label}: ${(peakRssBytes / 2 ** 20).toFixed(1)} MiB, ` +
          `${(setupNs / 1e6).toFixed(1)} ms setup`,
      );
    }
  }
}

// What each runtime costs before it loads a library, measured the same way, so a
// reader can tell the library from the process it runs in.
const baselines = {
  node: baseline("baseline-node"),
  rust: baseline("baseline-rust"),
};

await writeFile(
  resolve(runDir, "memory.json"),
  `${JSON.stringify({ runs, theme, baselines, results }, null, 2)}\n`,
);
console.log(resolve(runDir, "memory.json"));

function measure(implementation, document, fragment) {
  const [command, args] = commandFor(implementation);
  const stdout = execFileSync(command, [...args, document, ...(fragment ? [fragment] : [])], {
    cwd: repoDir,
    encoding: "utf8",
  });
  const { peakRssBytes, setupNs } = JSON.parse(stdout);
  if (!Number.isSafeInteger(peakRssBytes) || peakRssBytes <= 0) {
    throw new Error(`${implementation} reported no peak RSS for ${document}`);
  }
  if (!Number.isSafeInteger(setupNs) || setupNs < 0) {
    throw new Error(`${implementation} reported no setup time for ${document}`);
  }
  return { peakRssBytes, setupNs };
}

function baseline(implementation) {
  return median(Array.from({ length: runs }, () => measure(implementation, "webgpu").peakRssBytes));
}

async function fileSha256(path) {
  return createHash("sha256")
    .update(await readFile(path))
    .digest("hex");
}

function commandFor(implementation) {
  if (implementation === "baseline-node") return [process.execPath, [javascriptEntry, "baseline"]];
  if (implementation === "baseline-rust") return [rustBinary, ["baseline"]];
  return implementationById(implementation).runner === "criterion"
    ? [rustBinary, [implementation]]
    : [process.execPath, [javascriptEntry, implementation]];
}

function median(values) {
  const sorted = values.toSorted((a, b) => a - b);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[middle] : (sorted[middle - 1] + sorted[middle]) / 2;
}
