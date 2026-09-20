import { createHash } from "node:crypto";
import { mkdir, readFile, rename, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";

export function parseRunnerArguments(argv) {
  const options = {
    characterize: false,
    iterations: 1,
    manifest: resolve("target/stress-test/corpus/manifest.json"),
    maxCaseMs: 30_000,
    maxOutputAmplification: 32,
    output: undefined,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    if (argument === "--characterize") options.characterize = true;
    else if (argument === "--iterations") options.iterations = Number(argv[++index]);
    else if (argument === "--manifest") options.manifest = resolve(argv[++index]);
    else if (argument === "--max-case-ms") options.maxCaseMs = Number(argv[++index]);
    else if (argument === "--max-output-amplification") {
      options.maxOutputAmplification = Number(argv[++index]);
    } else if (argument === "--output") options.output = resolve(argv[++index]);
    else throw new Error(`unknown runner argument: ${argument}`);
  }

  if (!Number.isInteger(options.iterations) || options.iterations < 1) {
    throw new Error("--iterations must be at least one");
  }
  if (!options.output) throw new Error("--output is required");
  return options;
}

export async function loadManifest(path) {
  return JSON.parse(await readFile(path, "utf8"));
}

export function createReport(runtime, manifest, options) {
  return {
    schemaVersion: 1,
    runtime,
    status: "running",
    generatedAt: new Date().toISOString(),
    completedAt: undefined,
    runningCase: undefined,
    options: {
      iterations: options.iterations,
      maxCaseMs: options.maxCaseMs,
      maxOutputAmplification: options.maxOutputAmplification,
      characterize: options.characterize,
    },
    corpus: manifest,
    results: [],
    violations: [],
  };
}

export async function writeReport(path, report) {
  await mkdir(dirname(path), { recursive: true });
  const temporary = `${path}.tmp`;
  await writeFile(temporary, `${JSON.stringify(report, null, 2)}\n`);
  await rename(temporary, path);
}

export async function checkpoint(options, report, runningCase) {
  report.generatedAt = new Date().toISOString();
  report.runningCase = runningCase;
  await writeReport(options.output, report);
}

export async function finish(options, report) {
  report.violations = violations(report.results, options);
  report.status = report.violations.length === 0 ? "ok" : "failed";
  report.completedAt = new Date().toISOString();
  await checkpoint(options, report, undefined);
  for (const violation of report.violations) process.stderr.write(`VIOLATION: ${violation}\n`);
  if (report.violations.length > 0 && !options.characterize) process.exitCode = 2;
}

function violations(results, options) {
  const found = [];
  for (const result of results) {
    if (result.status !== "ok") found.push(`${result.id}: render failed`);
    if (result.deterministic === false) found.push(`${result.id}: output was nondeterministic`);
    for (const iteration of result.iterations ?? []) {
      if (iteration.wallMs > options.maxCaseMs) {
        found.push(
          `${result.id}: iteration ${iteration.iteration} took ${iteration.wallMs} ms ` +
            `(budget ${options.maxCaseMs} ms)`,
        );
      }
    }
    if (result.outputAmplification > options.maxOutputAmplification) {
      found.push(
        `${result.id}: output amplification ${result.outputAmplification.toFixed(2)}x ` +
          `(budget ${options.maxOutputAmplification}x)`,
      );
    }
  }
  return found;
}

export function sha256(bytes) {
  return createHash("sha256").update(bytes).digest("hex");
}

export async function gitRevision() {
  const { execFile } = await import("node:child_process");
  return new Promise((resolveRevision) => {
    execFile("git", ["rev-parse", "HEAD"], (error, stdout) => {
      resolveRevision(error ? undefined : stdout.trim());
    });
  });
}
