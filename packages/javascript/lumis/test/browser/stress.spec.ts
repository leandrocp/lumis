import { mkdir, readFile, rename, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, test, type Page } from "@playwright/test";

const repoDir = fileURLToPath(new URL("../../../../../", import.meta.url));

interface StressCase {
  id: string;
  profile: string;
  language: string;
  generated: { bytes: number };
  generatedPath: string;
  sourceSha256: string;
  origins: unknown[];
}

interface Manifest {
  cases: StressCase[];
  [key: string]: unknown;
}

interface BrowserIteration {
  iteration: number;
  status: "ok";
  wallMs: number;
  outputBytes: number;
  outputSha256: string;
  memory: unknown;
}

interface BrowserCaseResult {
  sourceBytes: number;
  sourceSha256: string;
  iterations: BrowserIteration[];
}

interface BrowserStressApi {
  init(input: { packages: Record<string, string>; wasms: Record<string, string> }): Promise<{
    preloadWallMs: number;
    runtimeKind: string;
  }>;
  render(input: {
    iterations: number;
    language: string;
    source: string;
  }): Promise<BrowserCaseResult>;
}

interface StressWindow extends Window {
  __lumisStressApi: BrowserStressApi;
  __lumisStressReady: boolean;
}

interface StressOptions {
  iterations: number;
  maxCaseMs: number;
  maxOutputAmplification: number;
  characterize: boolean;
}

interface StressReport {
  schemaVersion: number;
  runtime: { id: string; implementation: string };
  status: string;
  generatedAt: string;
  completedAt?: string;
  runningCase?: string;
  options: StressOptions;
  corpus: Manifest;
  preload?: unknown;
  results: Array<Record<string, unknown>>;
  violations: string[];
}

async function writeJson(path: string, value: unknown): Promise<void> {
  await mkdir(dirname(path), { recursive: true });
  const temporary = `${path}.tmp`;
  await writeFile(temporary, `${JSON.stringify(value, null, 2)}\n`);
  await rename(temporary, path);
}

function findViolations(
  results: Array<Record<string, unknown>>,
  maxCaseMs: number,
  maxOutputAmplification: number,
): string[] {
  const violations: string[] = [];
  for (const result of results) {
    const id = String(result.id);
    const iterations = result.iterations as BrowserIteration[];
    for (const iteration of iterations) {
      if (iteration.wallMs > maxCaseMs) {
        violations.push(
          `${id}: iteration ${iteration.iteration} took ${iteration.wallMs} ms ` +
            `(budget ${maxCaseMs} ms)`,
        );
      }
    }
    if (result.deterministic === false) violations.push(`${id}: output was nondeterministic`);
    const amplification = Number(result.outputAmplification);
    if (amplification > maxOutputAmplification) {
      violations.push(
        `${id}: output amplification ${amplification.toFixed(2)}x ` +
          `(budget ${maxOutputAmplification}x)`,
      );
    }
  }
  return violations;
}

function environmentOptions(): {
  manifestPath: string;
  outputPath: string;
  options: StressOptions;
} {
  const manifestPath = process.env.LUMIS_STRESS_MANIFEST;
  const outputPath = process.env.LUMIS_STRESS_OUTPUT;
  if (!manifestPath || !outputPath) throw new Error("stress manifest and output are required");

  return {
    manifestPath,
    outputPath,
    options: {
      iterations: Number(process.env.LUMIS_STRESS_ITERATIONS ?? "1"),
      maxCaseMs: Number(process.env.LUMIS_STRESS_MAX_CASE_MS ?? "30000"),
      maxOutputAmplification: Number(process.env.LUMIS_STRESS_MAX_OUTPUT_AMPLIFICATION ?? "32"),
      characterize: process.env.LUMIS_STRESS_CHARACTERIZE === "1",
    },
  };
}

function createReport(manifest: Manifest, options: StressOptions): StressReport {
  return {
    schemaVersion: 1,
    runtime: { id: "browser", implementation: "web-tree-sitter-chromium" },
    status: "running",
    generatedAt: new Date().toISOString(),
    options,
    corpus: manifest,
    results: [],
    violations: [],
  };
}

async function loadRuntimeAssets(
  manifest: Manifest,
): Promise<{ packages: Record<string, string>; wasms: Record<string, string> }> {
  const packages: Record<string, string> = {};
  const wasms: Record<string, string> = {};
  const languageIds = new Set(manifest.cases.map((testCase) => testCase.language));
  for (const languageId of languageIds) {
    const metadataPath = resolve(repoDir, `tmp/wasm/local/parsers/${languageId}.lumis.json`);
    const metadataSource = await readFile(metadataPath, "utf8");
    const metadata = JSON.parse(metadataSource) as {
      packageName: string;
      version: string;
      parser: { name: string; sha256: string };
    };
    const filename = `${metadata.parser.name}-${metadata.version}-${metadata.parser.sha256}.wasm`;
    const parserPath = resolve(repoDir, "tmp/wasm/local/parsers", filename);
    packages[metadata.packageName] = metadataSource;
    wasms[languageId] = (await readFile(parserPath)).toString("base64");
  }
  return { packages, wasms };
}

async function renderCase(
  page: Page,
  testCase: StressCase,
  iterations: number,
): Promise<Record<string, unknown>> {
  const source = await readFile(resolve(repoDir, testCase.generatedPath), "utf8");
  const measured = await page.evaluate(
    (input) => (window as unknown as StressWindow).__lumisStressApi.render(input),
    { iterations, language: testCase.language, source },
  );
  expect(measured.sourceBytes, testCase.id).toBe(testCase.generated.bytes);
  expect(measured.sourceSha256, testCase.id).toBe(testCase.sourceSha256);
  const outputBytes = Math.max(...measured.iterations.map((iteration) => iteration.outputBytes));
  return {
    id: testCase.id,
    profile: testCase.profile,
    language: testCase.language,
    status: "ok",
    generated: testCase.generated,
    sourceSha256: testCase.sourceSha256,
    origins: testCase.origins,
    deterministic: new Set(measured.iterations.map(({ outputSha256 }) => outputSha256)).size <= 1,
    outputBytes,
    outputAmplification: outputBytes / Math.max(measured.sourceBytes, 1),
    iterations: measured.iterations,
  };
}

test("runs the generated stress corpus in a browser", async ({ page }) => {
  test.setTimeout(45 * 60 * 1_000);
  const { manifestPath, outputPath, options } = environmentOptions();
  const manifest = JSON.parse(await readFile(manifestPath, "utf8")) as Manifest;
  const report = createReport(manifest, options);
  await writeJson(outputPath, report);

  await page.goto("/stress.html");
  await page.waitForFunction(() => (window as unknown as StressWindow).__lumisStressReady);

  const assets = await loadRuntimeAssets(manifest);
  report.preload = await page.evaluate(
    (input) => (window as unknown as StressWindow).__lumisStressApi.init(input),
    assets,
  );
  expect((report.preload as { runtimeKind: string }).runtimeKind).toBe("wasm");
  await writeJson(outputPath, report);

  for (const testCase of manifest.cases) {
    report.runningCase = testCase.id;
    report.generatedAt = new Date().toISOString();
    await writeJson(outputPath, report);
    report.results.push(await renderCase(page, testCase, options.iterations));
    report.runningCase = undefined;
    report.generatedAt = new Date().toISOString();
    await writeJson(outputPath, report);
  }

  report.violations = findViolations(
    report.results,
    options.maxCaseMs,
    options.maxOutputAmplification,
  );
  report.status = report.violations.length === 0 ? "ok" : "failed";
  report.completedAt = new Date().toISOString();
  await writeJson(outputPath, report);
  if (!options.characterize) expect(report.violations).toEqual([]);
});
