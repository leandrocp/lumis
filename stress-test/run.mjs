import { spawn } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { integerArgument, numberArgument, pathArgument, stringArgument } from "./report.mjs";

const repoDir = fileURLToPath(new URL("../", import.meta.url));
const runtimes = ["rust", "cli", "javascript-native", "javascript-wasm", "browser", "elixir"];

const FLAGS = new Map([
  ["--characterize", (options) => (options.characterize = true)],
  ["--timeout-storm", (options) => (options.timeoutStorm = true)],
]);

const VALUES = new Map([
  [
    "--caller-timeout-ms",
    (options, raw) => (options.callerTimeoutMs = integerArgument("--caller-timeout-ms", raw, 1)),
  ],
  ["--case", (options, raw) => options.cases.push(stringArgument("--case", raw))],
  [
    "--iterations",
    (options, raw) => (options.iterations = integerArgument("--iterations", raw, 1)),
  ],
  [
    "--max-case-ms",
    (options, raw) => (options.maxCaseMs = numberArgument("--max-case-ms", raw, 0)),
  ],
  [
    "--max-output-amplification",
    (options, raw) =>
      (options.maxOutputAmplification = numberArgument("--max-output-amplification", raw, 0)),
  ],
  [
    "--max-probe-ms",
    (options, raw) => (options.maxProbeMs = numberArgument("--max-probe-ms", raw, 0)),
  ],
  ["--output", (options, raw) => (options.output = pathArgument("--output", raw))],
  ["--profile", (options, raw) => (options.profile = stringArgument("--profile", raw))],
  ["--runtime", (options, raw) => (options.runtime = stringArgument("--runtime", raw))],
  ["--scale", (options, raw) => (options.scale = numberArgument("--scale", raw, Number.MIN_VALUE))],
  [
    "--storm-callers",
    (options, raw) => (options.stormCallers = integerArgument("--storm-callers", raw, 1)),
  ],
]);

function parseArguments(argv) {
  const options = {
    callerTimeoutMs: 1_000,
    cases: [],
    characterize: false,
    iterations: 1,
    maxCaseMs: 30_000,
    maxOutputAmplification: 32,
    maxProbeMs: 5_000,
    output: resolve(repoDir, "target/stress-test"),
    profile: "all",
    runtime: "all",
    scale: 1,
    stormCallers: undefined,
    timeoutStorm: false,
  };

  for (let index = 0; index < argv.length; index += 1) {
    const argument = argv[index];
    const flag = FLAGS.get(argument);
    if (flag) {
      flag(options);
      continue;
    }
    const withValue = VALUES.get(argument);
    if (!withValue) throw new Error(`unknown argument: ${argument}`);
    withValue(options, argv[++index]);
  }

  if (options.runtime !== "all" && !runtimes.includes(options.runtime)) {
    throw new Error(`unknown runtime: ${options.runtime}`);
  }
  if (options.scale > 1) throw new Error("--scale must be greater than zero and at most one");
  return options;
}

function run(command, args, settings = {}) {
  return new Promise((resolveRun, rejectRun) => {
    const child = spawn(command, args, {
      cwd: settings.cwd ?? repoDir,
      env: { ...process.env, ...settings.env },
      stdio: "inherit",
    });
    child.on("error", rejectRun);
    child.on("close", (code, signal) => resolveRun({ code: code ?? 1, signal }));
  });
}

function commonRunnerArguments(options, manifest, output) {
  const args = [
    "--manifest",
    manifest,
    "--output",
    output,
    "--iterations",
    String(options.iterations),
    "--max-case-ms",
    String(options.maxCaseMs),
    "--max-output-amplification",
    String(options.maxOutputAmplification),
  ];
  if (options.characterize) args.push("--characterize");
  return args;
}

async function generate(options) {
  const corpusDir = resolve(options.output, "corpus");
  const args = [
    "stress-test/generate.mjs",
    "--output",
    corpusDir,
    "--profile",
    options.profile,
    "--scale",
    String(options.scale),
  ];
  for (const testCase of options.cases) args.push("--case", testCase);
  const result = await run(process.execPath, args);
  if (result.code !== 0) throw new Error("stress corpus generation failed");
  return resolve(corpusDir, "manifest.json");
}

async function runRust(options, manifest) {
  const args = [
    "run",
    "--release",
    "--manifest-path",
    "crates/dev/Cargo.toml",
    "--features",
    "lumis-stress-languages",
    "--",
    "stress",
    ...commonRunnerArguments(
      options,
      manifest,
      resolve(options.output, `rust-${options.profile}.json`),
    ),
  ];
  return run("cargo", args);
}

async function runCli(options, manifest) {
  const targetDir = resolve(repoDir, "target/stress-test/cargo");
  const built = await run("cargo", [
    "build",
    "--release",
    "-p",
    "lumis-cli",
    "--target-dir",
    targetDir,
  ]);
  if (built.code !== 0) return built;
  return run(process.execPath, [
    "stress-test/run-command.mjs",
    "--runtime",
    "cli",
    "--binary",
    resolve(targetDir, "release/lumis"),
    "--",
    ...commonRunnerArguments(
      options,
      manifest,
      resolve(options.output, `cli-${options.profile}.json`),
    ),
  ]);
}

async function buildJavaScript(native) {
  if (native) {
    const addon = await run("pnpm", ["--filter", "@lumis-sh/lumis", "run", "build:native"]);
    if (addon.code !== 0) return addon;
  }
  return run("pnpm", ["--filter", "@lumis-sh/lumis", "run", "build"]);
}

async function runJavaScript(options, manifest, native) {
  const built = await buildJavaScript(native);
  if (built.code !== 0) return built;
  const runtime = native ? "javascript-native" : "javascript-wasm";
  return run(
    process.execPath,
    [
      "packages/javascript/lumis/stress_test/corpus.mjs",
      ...commonRunnerArguments(
        options,
        manifest,
        resolve(options.output, `${runtime}-${options.profile}.json`),
      ),
    ],
    { env: { LUMIS_TEST_RUNTIME: native ? "native" : "wasm" } },
  );
}

async function runElixir(options, manifest) {
  const args = [
    "run",
    "stress_test/corpus.exs",
    "--",
    ...commonRunnerArguments(
      options,
      manifest,
      resolve(options.output, `elixir-${options.profile}.json`),
    ),
    "--max-probe-ms",
    String(options.maxProbeMs),
    "--caller-timeout-ms",
    String(options.callerTimeoutMs),
  ];
  if (options.timeoutStorm) args.push("--timeout-storm");
  if (options.stormCallers !== undefined)
    args.push("--storm-callers", String(options.stormCallers));
  return run("mix", args, {
    cwd: resolve(repoDir, "packages/elixir/lumis"),
    env: { LUMIS_BUILD: "1", MIX_ENV: "prod" },
  });
}

async function runBrowser(options, manifest) {
  const generated = await run("pnpm", ["--filter", "@lumis-sh/lumis", "run", "build:generate"]);
  if (generated.code !== 0) return generated;

  const environment = {
    LUMIS_STRESS_MANIFEST: manifest,
    LUMIS_STRESS_OUTPUT: resolve(options.output, `browser-${options.profile}.json`),
    LUMIS_STRESS_ITERATIONS: String(options.iterations),
    LUMIS_STRESS_MAX_CASE_MS: String(options.maxCaseMs),
    LUMIS_STRESS_MAX_OUTPUT_AMPLIFICATION: String(options.maxOutputAmplification),
    LUMIS_STRESS_CHARACTERIZE: options.characterize ? "1" : "0",
  };
  return run(
    "pnpm",
    [
      "--filter",
      "@lumis-sh/lumis",
      "exec",
      "playwright",
      "test",
      "--config",
      "playwright.stress.config.ts",
    ],
    { env: environment },
  );
}

async function runRuntime(runtime, options, manifest) {
  if (runtime === "rust") return runRust(options, manifest);
  if (runtime === "cli") return runCli(options, manifest);
  if (runtime === "javascript-native") return runJavaScript(options, manifest, true);
  if (runtime === "javascript-wasm") return runJavaScript(options, manifest, false);
  if (runtime === "browser") return runBrowser(options, manifest);
  return runElixir(options, manifest);
}

const options = parseArguments(process.argv.slice(2));
const manifest = await generate(options);
const selected = options.runtime === "all" ? runtimes : [options.runtime];
let failed = false;
for (const runtime of selected) {
  process.stdout.write(`\n=== ${runtime} ===\n`);
  const result = await runRuntime(runtime, options, manifest);
  if (result.code !== 0) failed = true;
}
if (failed) process.exitCode = 2;
