#!/usr/bin/env node

import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { benchmarkImplementations, implementationById } from "./implementations.mjs";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoDir = resolve(benchmarksDir, "..");
const runDir = resolve(
  process.env.BENCH_RUN_DIR ?? resolve(repoDir, "target/benchmarks/runs/current"),
);
const manifest = await readJson(resolve(repoDir, "target/benchmarks/fixtures/scenarios.json"));
const rustMetadata = await readJson(resolve(runDir, "rust-metadata.json"));
const cliMetadata = await readJson(resolve(runDir, "cli/metadata.json"));
const packageSizes = await readJson(resolve(runDir, "package-sizes.json"));
const results = [];
for (const scenario of manifest.scenarios) {
  const scenarioResults = [];
  for (const { id, runner } of benchmarkImplementations) {
    const metadata = await implementationMetadata(id, runner, scenario.id);
    validateMetadata(metadata, scenario, id);
    const measurement = await measurementFor(id, runner, scenario.id);
    if (
      !Number.isFinite(measurement.totalNs) ||
      measurement.totalNs <= 0 ||
      measurement.samples < 3
    ) {
      throw new Error(`${id}/${scenario.id} has an invalid Total or fewer than three samples`);
    }
    scenarioResults.push({
      implementation: id,
      runner,
      totalNs: measurement.totalNs,
      setupNs: metadata.setupNanoseconds,
      outputBytes: metadata.outputBytes,
    });
  }

  results.push({
    id: scenario.id,
    description: scenario.description,
    inputBytes: scenario.inputBytes,
    fileCount: scenario.fileCount,
    languageCount: scenario.languageCount,
    results: scenarioResults,
  });
}

const mergedReport = {
  schemaVersion: 1,
  metric: "total",
  timingBoundary:
    "every highlight in the scenario, harness fixture reads excluded; building the highlighter is reported apart as setupNs, except for the CLI rows, where each sample is a whole command and carries process start and file I/O with it",
  results,
};
await mkdir(runDir, { recursive: true });
await writeFile(resolve(runDir, "results.json"), `${JSON.stringify(mergedReport, null, 2)}\n`);
const markdown = renderMarkdown(mergedReport, packageSizes);
await writeFile(resolve(runDir, "results.md"), markdown);
await writeFile(resolve(benchmarksDir, "README.md"), markdown);
console.log(resolve(runDir, "results.json"));

async function implementationMetadata(implementation, runner, scenario) {
  if (runner === "criterion") {
    return rustMetadata.implementations
      .find((entry) => entry.implementation === implementation)
      .scenarios.find((entry) => entry.scenario === scenario);
  }
  if (runner === "hyperfine") {
    return cliMetadata.results.find(
      (entry) => entry.implementation === implementation && entry.scenario === scenario,
    );
  }
  if (runner === "benchee") {
    return readJson(resolve(runDir, `elixir/${scenario}-metadata.json`));
  }
  return readJson(resolve(runDir, `javascript/${implementation}/${scenario}.json`));
}

async function measurementFor(implementation, runner, scenario) {
  if (runner === "criterion") {
    const directory = resolve(runDir, `criterion/${scenario}/${implementation}/total/new`);
    const [estimates, sample] = await Promise.all([
      readJson(resolve(directory, "estimates.json")),
      readJson(resolve(directory, "sample.json")),
    ]);
    return { totalNs: estimates.median.point_estimate, samples: sample.times.length };
  }
  if (runner === "hyperfine") {
    const report = await readJson(resolve(runDir, `cli/${scenario}.json`));
    const result = report.results.find(({ command }) => command === implementation);
    if (!result) throw new Error(`Hyperfine is missing ${implementation}/${scenario}`);
    return { totalNs: result.median * 1e9, samples: result.times.length };
  }
  if (runner === "benchee") {
    const report = await readJson(resolve(runDir, `elixir/${scenario}.json`));
    const result = report.find(({ job_name: job }) => job === "lumis-elixir/total");
    if (!result) throw new Error(`Benchee is missing lumis-elixir/${scenario}`);
    return {
      totalNs: result.run_time_data.statistics.median,
      samples: result.run_time_data.statistics.sample_size,
    };
  }
  const report = await readJson(resolve(runDir, `javascript/${implementation}/${scenario}.json`));
  return { totalNs: report.total.p50, samples: report.total.samples.length };
}

function validateMetadata(metadata, scenario, implementation) {
  if (
    !metadata ||
    metadata.scenario !== scenario.id ||
    metadata.inputBytes !== scenario.inputBytes ||
    metadata.fileCount !== scenario.fileCount ||
    metadata.languageCount !== scenario.languageCount ||
    metadata.outputBytes <= metadata.inputBytes
  ) {
    throw new Error(`${implementation} returned incompatible metadata for ${scenario.id}`);
  }
}

function renderMarkdown(report, sizes) {
  const scenarios = {
    "small-one-language": "1 small file for 1 language",
    "large-one-language": "1 big file for 1 language",
    "ten-files-one-language": "10 different files for 1 language",
    "ten-files-ten-languages": "10 different files for 10 languages",
  };
  validatePackageSizes(sizes);
  const lines = [
    "# Benchmarks",
    "",
    "Run all benchmarks and regenerate this file with `mise run -C benchmarks run`.",
    "",
    "The timing rows use local workspace source for every Lumis runtime.",
    "",
    "**Highlight** is the per-call cost with a prepared highlighter. **Setup** is",
    "what building that highlighter costs once the process is warm. They are",
    "separated because the runtimes cache very differently, and timing them",
    "together compares caching rather than highlighting.",
    "",
    "Every Lumis row except Rust loads the same WebAssembly parsers; Rust compiles",
    "its own in and is the floor rather than something anyone installs. What differs",
    "between the two JavaScript rows is which engine runs those parsers and where",
    "the highlight pass happens: **(Node, Wasmtime)** runs them under Wasmtime in a",
    "native addon and walks and formats in Rust, the same code the CLI and Elixir",
    "run; **(web-tree-sitter)** runs them under V8 and walks and formats in",
    "JavaScript. That second one is what browsers use, and what Node falls back to",
    "where no addon is built.",
    "",
    "See the [focused before/after report](elixir-runtime.md) for Elixir cold",
    "loading, concurrency, and memory.",
    "",
    "## Visual output comparison",
    "",
    "Generate the gallery with `mise run -C benchmarks showcase`, then open",
    "[`index.html`](index.html) directly in a browser.",
    "",
    "This comparison is separate from timing. Every implementation highlights the",
    "same pinned 1,397-line Three.js HTML file, including injected CSS, JSON, and",
    "JavaScript, in Catppuccin Latte and Frappé. The gallery covers Lumis Rust,",
    "JavaScript Wasm, Elixir, and CLI alongside Shiki, highlight.js, syntect, and",
    "bat. Its preparation step verifies SHA-256 hashes for the fixture and for",
    "syntect's Catppuccin tmThemes before rendering.",
    "",
    "It also counts the tokens each implementation coloured in each document, so",
    "the gallery can say how finely a file was resolved next to the output itself.",
    "",
    "## Publishing to the website",
    "",
    "The deploy workflow builds only `website/` and `docs/`, so it cannot run any",
    "of this. Both sets of numbers are committed, and these refresh them:",
    "",
    "```sh",
    "mise run -C benchmarks showcase-publish",
    "mise run -C benchmarks comparison-timings",
    "mise run -C benchmarks comparison-timings-publish",
    "```",
    "",
    "`showcase-publish` writes the gallery and its token counts, and CI fails when",
    "the committed bytes stop matching. `comparison-timings` times every library on",
    "every document the gallery renders, and `comparison-timings-publish` writes",
    "those times beside the outputs. No check can verify them because they are",
    "measurements; run them from a quiet machine, or the site will quote whatever",
    "that machine was also doing. The timings below stay in this file.",
    "",
    "## Package size",
    "",
    `Measured on ${sizes.system.cpu} (${sizes.system.architecture}, ${sizes.system.platform}).`,
    "npm rows sum the packed and unpacked sizes of each unique production package.",
    "Native rows compare the raw release artifact with deterministic gzip level 9.",
    "Compare rows within the same artifact class; npm packages, executables, and a NIF",
    "are not interchangeable distribution formats.",
    "",
    "| Tool | Measured artifact | Raw / unpacked | Download / gzip -9 |",
    "| --- | --- | ---: | ---: |",
  ];
  for (const entry of sizes.entries) {
    lines.push(
      `| ${packageSizeLabel(entry)} | ${entry.artifact} | ${formatBytes(entry.rawBytes)} | ${formatBytes(entry.compressedBytes)} |`,
    );
  }
  lines.push("");
  for (const scenario of report.results) {
    lines.push(
      `## ${scenarios[scenario.id]}`,
      "",
      scenario.description,
      "",
      "| Tool | Highlight | Setup |",
      "| --- | ---: | ---: |",
    );
    for (const result of scenario.results.toSorted((left, right) => left.totalNs - right.totalNs)) {
      const setup = Number.isFinite(result.setupNs) ? formatDuration(result.setupNs) : "—";
      lines.push(
        `| ${implementationById(result.implementation).label} | ${formatDuration(result.totalNs)} | ${setup} |`,
      );
    }
    lines.push("");
  }
  return `${lines.join("\n").trimEnd()}\n`;
}

function validatePackageSizes(report) {
  if (
    report?.schemaVersion !== 2 ||
    !report.system?.cpu ||
    !Array.isArray(report.entries) ||
    report.entries.length === 0 ||
    report.entries.some(
      (entry) =>
        !entry.implementation ||
        !Number.isSafeInteger(entry.rawBytes) ||
        entry.rawBytes <= 0 ||
        !Number.isSafeInteger(entry.compressedBytes) ||
        entry.compressedBytes <= 0,
    )
  ) {
    throw new Error("package size report is missing or invalid");
  }
  for (const entry of report.entries) implementationById(entry.implementation);
}

function packageSizeLabel(entry) {
  const label = implementationById(entry.implementation).label;
  return entry.variant ? `${label} (${entry.variant})` : label;
}

function formatDuration(ns) {
  if (ns >= 1e9) return `${(ns / 1e9).toFixed(3)} s`;
  if (ns >= 1e6) return `${(ns / 1e6).toFixed(3)} ms`;
  if (ns >= 1e3) return `${(ns / 1e3).toFixed(3)} µs`;
  return `${ns.toFixed(0)} ns`;
}

function formatBytes(bytes) {
  const units = ["B", "KiB", "MiB", "GiB"];
  let value = bytes;
  let unit = units.shift();
  while (value >= 1024 && units.length > 0) {
    value /= 1024;
    unit = units.shift();
  }
  return `${value.toFixed(unit === "B" ? 0 : 2)} ${unit}`;
}

async function readJson(path) {
  try {
    return JSON.parse(await readFile(path, "utf8"));
  } catch (error) {
    throw new Error(`could not read benchmark report ${path}`, { cause: error });
  }
}
