import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { do_not_optimize, measure } from "mitata";
import { implementations } from "../../scripts/implementations.mjs";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const repoDir = resolve(benchmarksDir, "..");
const implementation = process.env.BENCH_IMPLEMENTATION;
const scenarioId = process.env.BENCH_SCENARIO;
const requestedOutput = process.env.BENCH_OUTPUT;
const minimumSamples = Number.parseInt(process.env.BENCH_SAMPLES ?? "10", 10);
const measurementSeconds = Number.parseFloat(process.env.BENCH_TIME_SECONDS ?? "1");

if (!requestedOutput) throw new Error("BENCH_OUTPUT is required");
if (!scenarioId) throw new Error("BENCH_SCENARIO is required");
if (!Number.isSafeInteger(minimumSamples) || minimumSamples < 2) {
  throw new Error("BENCH_SAMPLES must be at least two");
}
if (!Number.isFinite(measurementSeconds) || measurementSeconds <= 0) {
  throw new Error("BENCH_TIME_SECONDS must be positive");
}
if (!implementations.some(({ id, runner }) => id === implementation && runner === "mitata")) {
  throw new Error(`unknown JavaScript benchmark implementation: ${implementation}`);
}

// No time limit, so a scenario measures highlighting rather than how close the
// machine came to the default bound. The default is about five megabytes of
// source and the large scenario is exactly that, so on a slow enough machine the
// render would run out, return the file as plain text, and report a throughput
// the highlighter never reached. Shiki and highlight.js have no equivalent
// bound, which is the other reason the comparison runs without one.
const UNBOUNDED = { budget: { timeLimit: 0 } };

const resolvedManifest = JSON.parse(
  await readFile(resolve(repoDir, "target/benchmarks/fixtures/scenarios.json"), "utf8"),
);
const scenarioSpec = resolvedManifest.scenarios.find(({ id }) => id === scenarioId);
if (!scenarioSpec) throw new Error(`unknown benchmark scenario: ${scenarioId}`);
const scenario = {
  ...scenarioSpec,
  files: await Promise.all(
    // oxlint-disable-next-line no-map-spread -- one-shot setup; mutating the spec is not worth the bytes.
    scenarioSpec.files.map(async (file) => ({
      ...file,
      source: await readFile(resolve(repoDir, file.path), "utf8"),
    })),
  ),
};

if (
  scenario.files.reduce((total, file) => total + Buffer.byteLength(file.source), 0) !==
  scenario.inputBytes
) {
  throw new Error(`${scenario.id} input bytes changed after fixture verification`);
}

await prepareRuntime();
const adapter =
  implementation === "shiki"
    ? await loadShiki()
    : implementation === "highlight-js"
      ? await loadHighlightJs()
      : await loadLumis();
const validationRuntime = await adapter.initialize();
const outputBytes = adapter.render(validationRuntime, true);
adapter.dispose(validationRuntime);
if (outputBytes <= scenario.inputBytes) {
  throw new Error(`${implementation} did not expand ${scenario.id}`);
}

let result;
const cleanup = () => {
  result = undefined;
  globalThis.gc?.();
};
// Setup is measured once and reported separately, never inside the loop. Lumis
// keeps its parser catalog for the life of the process while web-tree-sitter
// rebuilds one per highlighter, so timing setup per sample would compare how
// each runtime caches rather than how fast either highlights.
const setupStart = performance.now();
const measuredRuntime = await adapter.initialize();
const setupNanoseconds = (performance.now() - setupStart) * 1e6;

const stats = await measure(
  async () => {
    result = { runtime: measuredRuntime, outputBytes: adapter.render(measuredRuntime) };
    do_not_optimize(result.outputBytes);
  },
  {
    min_samples: minimumSamples,
    max_samples: 1_000_000,
    min_cpu_time: measurementSeconds * 1e9,
    warmup_samples: 1,
    batch_samples: 1,
    gc: cleanup,
    inner_gc: true,
  },
);
cleanup();
adapter.dispose(measuredRuntime);

const { debug: _debug, ...serializableStats } = stats;
const report = {
  schemaVersion: 1,
  runner: "mitata",
  implementation,
  scenario: scenario.id,
  inputBytes: scenario.inputBytes,
  outputBytes,
  fileCount: scenario.fileCount,
  languageCount: scenario.languageCount,
  total: serializableStats,
  setupNanoseconds,
};
const outputPath = isAbsolute(requestedOutput)
  ? requestedOutput
  : resolve(repoDir, requestedOutput);
await mkdir(dirname(outputPath), { recursive: true });
await writeFile(outputPath, `${JSON.stringify(report, null, 2)}\n`);
console.log(outputPath);

async function prepareRuntime() {
  const runtimeDir = resolve(repoDir, "target/benchmarks/javascript-runtime");
  await mkdir(runtimeDir, { recursive: true });
  process.env.LUMIS_DATA_DIR = resolve(runtimeDir, "wasm-cache");

  // Which runtime `@lumis-sh/lumis` picks on Node. The addon is the default, so
  // the Wasm row has to ask for the other one; the addon resolves parsers in
  // Rust rather than through the JavaScript resolver the Wasm row configures,
  // and reads them from the tree `prepare:languages` writes.
  if (implementation === "lumis-js-wasm") {
    process.env.LUMIS_TEST_RUNTIME = "wasm";
  } else if (implementation === "lumis-js-node") {
    delete process.env.LUMIS_TEST_RUNTIME;
    process.env.LUMIS_DATA_DIR = resolve(repoDir, "target/benchmarks/language-packages");
  }

  process.chdir(runtimeDir);
}

async function loadLumis() {
  const [lumis, { htmlInline }, { default: theme }] = await Promise.all([
    import("@lumis-sh/lumis"),
    import("@lumis-sh/lumis/formatters"),
    import("@lumis-sh/themes/github_dark"),
  ]);
  const { createHighlighter, withWasm, runtimeKind } = lumis;

  // Node picks the addon and falls back to Wasm silently, so a row that cannot
  // confirm which one it measured would quietly report the other one's numbers.
  const expected = implementation === "lumis-js-node" ? "native" : "wasm";
  const actual = runtimeKind();
  if (actual !== expected) {
    throw new Error(`${implementation} needs the ${expected} runtime, got ${actual}`);
  }
  const uniqueIds = [...new Set(["comment", ...scenario.files.map(({ language }) => language)])];
  const localPackages = JSON.parse(
    await readFile(resolve(repoDir, "target/benchmarks/language-packages/index.json"), "utf8"),
  );
  const languagePackageResolver = (packageName) => {
    const local = localPackages[packageName];
    if (!local) throw new Error(`missing local language package ${packageName}`);
    return pathToFileURL(local.metadataPath);
  };
  const languages = Object.fromEntries(
    await Promise.all(
      uniqueIds.map(async (id) => {
        const [{ default: language }, { default: wasm }] = await Promise.all([
          import(`@lumis-sh/lumis/langs/${id}`),
          import(`@lumis-sh/wasm-${id}`),
        ]);
        return [id, withWasm(language, wasm)];
      }),
    ),
  );

  return {
    async initialize() {
      const highlighter = await createHighlighter({
        languages: Object.values(languages),
        languagePackageResolver,
      });
      const formatters = Object.fromEntries(
        Object.entries(languages).map(([id, language]) => [id, htmlInline({ language, theme })]),
      );
      return { formatters, highlighter };
    },
    render({ formatters, highlighter }, validate = false) {
      let renderedBytes = 0;
      for (const file of scenario.files) {
        const output = highlighter.highlight(file.source, formatters[file.language], UNBOUNDED);
        if (validate) assertHtml(output, file.source, implementation);
        renderedBytes += Buffer.byteLength(output);
      }
      return renderedBytes;
    },
    dispose() {},
  };
}

async function loadShiki() {
  const [{ createHighlighter }, { createOnigurumaEngine }] = await Promise.all([
    import("shiki"),
    import("shiki/engine/oniguruma"),
  ]);
  const languages = [...new Set(scenario.files.map(({ language }) => language))];

  return {
    async initialize() {
      return createHighlighter({
        langs: languages,
        themes: ["github-dark"],
        engine: createOnigurumaEngine(import("shiki/wasm")),
      });
    },
    render(highlighter, validate = false) {
      let renderedBytes = 0;
      for (const file of scenario.files) {
        const output = highlighter.codeToHtml(file.source, {
          lang: file.language,
          theme: "github-dark",
        });
        if (validate) assertHtml(output, file.source, implementation);
        renderedBytes += Buffer.byteLength(output);
      }
      return renderedBytes;
    },
    dispose(highlighter) {
      highlighter.dispose();
    },
  };
}

async function loadHighlightJs() {
  const { default: highlightJs } = await import("highlight.js/lib/core");
  const languageNames = [...new Set(scenario.files.map(({ language }) => language))];
  const languageModules = Object.fromEntries(
    await Promise.all(
      languageNames.map(async (language) => {
        const moduleName = language === "html" ? "xml" : language;
        const { default: definition } = await import(`highlight.js/lib/languages/${moduleName}`);
        return [language, definition];
      }),
    ),
  );

  return {
    async initialize() {
      const highlighter = highlightJs.newInstance();
      for (const [language, definition] of Object.entries(languageModules)) {
        highlighter.registerLanguage(language, definition);
      }
      return highlighter;
    },
    render(highlighter, validate = false) {
      let renderedBytes = 0;
      for (const file of scenario.files) {
        const highlighted = highlighter.highlight(file.source, { language: file.language }).value;
        const output = `<pre><code class="hljs language-${file.language}">${highlighted}</code></pre>`;
        if (validate) assertHtml(output, file.source, implementation);
        renderedBytes += Buffer.byteLength(output);
      }
      return renderedBytes;
    },
    dispose() {},
  };
}

function assertHtml(output, source, name) {
  if (
    Buffer.byteLength(output) <= Buffer.byteLength(source) ||
    !output.includes("<pre") ||
    !output.includes("<span")
  ) {
    throw new Error(`${name} did not produce highlighted HTML`);
  }
}
