#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { copyFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { basename, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repoDir = resolve(benchmarksDir, "..");
const cacheRoot = resolve(repoDir, "target/benchmarks/cli");
const commandsDir = resolve(cacheRoot, "commands");
const dataDir = resolve(cacheRoot, "data");
const xdgCacheDir = resolve(cacheRoot, "xdg-cache");
const lumis = resolve(
  repoDir,
  "target/benchmarks/rust-target/release",
  process.platform === "win32" ? "lumis.exe" : "lumis",
);
const bat = findBat();
const scenarioShell = findScenarioShell();

await rm(cacheRoot, { recursive: true, force: true });
await mkdir(commandsDir, { recursive: true });
await mkdir(resolve(dataDir, "parsers"), { recursive: true });
await mkdir(xdgCacheDir, { recursive: true });

const manifest = JSON.parse(
  await readFile(resolve(repoDir, "target/benchmarks/fixtures/scenarios.json"), "utf8"),
);
const localPackages = JSON.parse(
  await readFile(resolve(repoDir, "target/benchmarks/language-packages/index.json"), "utf8"),
);
const languages = [...new Set(Object.values(localPackages).map(({ language }) => language))];

for (const language of languages) {
  const packageName = `@lumis-sh/wasm-${language}`;
  const local = localPackages[packageName];
  if (!local) throw new Error(`missing local language package ${packageName}`);
  const metadata = JSON.parse(await readFile(local.metadataPath, "utf8"));
  const parser = metadata.parser;
  await copyFile(local.metadataPath, resolve(dataDir, "parsers", `${language}.lumis.json`));
  await copyFile(
    local.wasmPath,
    resolve(dataDir, "parsers", `${parser.name}-${metadata.version}-${parser.sha256}.wasm`),
  );
}

const benchmarkEnv = {
  ...process.env,
  XDG_CACHE_HOME: xdgCacheDir,
  LUMIS_DATA_DIR: dataDir,
  LUMIS_CONFIG: resolve(cacheRoot, "missing-config.toml"),
  BAT_OPTS: "",
  BAT_PAGER: "cat",
  PAGER: "cat",
  CLICOLOR_FORCE: "1",
};
delete benchmarkEnv.NO_COLOR;

const implementations = [
  {
    id: "lumis-cli",
    command: lumis,
    args: (file) => [
      "--data-dir",
      dataDir,
      "highlight",
      "--language",
      file.language,
      "--formatter",
      "terminal",
      // The default time limit is about five megabytes of source and the large
      // scenario is exactly that, so whether it is reached comes down to how
      // fast the machine is. A render that reaches it returns the file as plain
      // text and exits 0, which would time the bail-out and trip the check
      // below on a slow runner. bat has no such bound, so unbounded is also
      // what the two implementations have in common.
      "--budget-time-limit",
      "0",
      "--theme",
      "github_dark",
      resolve(repoDir, file.path),
    ],
  },
  {
    id: "bat",
    command: bat,
    args: (file) => [
      "--no-config",
      "--paging=never",
      "--style=plain",
      "--color=always",
      `--language=${file.syntax}`,
      "--theme=Monokai Extended",
      resolve(repoDir, file.path),
    ],
  },
];
const metadata = [];

for (const scenario of manifest.scenarios) {
  const definitions = [];

  for (const implementation of implementations) {
    let outputBytes = 0;
    for (const file of scenario.files) {
      const result = spawnSync(implementation.command, implementation.args(file), {
        cwd: repoDir,
        env: benchmarkEnv,
        encoding: "buffer",
        maxBuffer: 128 * 1024 * 1024,
      });
      if (result.status !== 0) {
        throw new Error(
          `${implementation.id} failed to prepare ${file.path}: ${result.stderr.toString()}`,
        );
      }
      if (result.stdout.length <= file.bytes) {
        throw new Error(`${implementation.id} did not highlight ${file.path}`);
      }
      outputBytes += result.stdout.length;
    }

    const commands = scenario.files.map((file) =>
      shellCommand([implementation.command, ...implementation.args(file)]),
    );
    const scenarioCommand = commands.join(" && ");

    const commandResult = spawnSync(scenarioShell, ["-c", scenarioCommand], {
      cwd: repoDir,
      env: benchmarkEnv,
      encoding: "buffer",
      maxBuffer: 128 * 1024 * 1024,
    });
    if (commandResult.status !== 0 || commandResult.stdout.length !== outputBytes) {
      throw new Error(`${implementation.id}/${scenario.id} generated an invalid scenario command`);
    }

    const name = implementation.id === "lumis-cli" ? "LUMIS_CLI_COMMAND" : "BAT_COMMAND";
    definitions.push(`${name}=${shellQuote(scenarioCommand)}`);

    metadata.push({
      implementation: implementation.id,
      scenario: scenario.id,
      inputBytes: scenario.inputBytes,
      outputBytes,
      fileCount: scenario.fileCount,
      languageCount: scenario.languageCount,
    });
  }

  await writeFile(resolve(commandsDir, `${scenario.id}.sh`), `${definitions.join("\n")}\n`);
}

await writeFile(
  resolve(cacheRoot, "metadata.json"),
  `${JSON.stringify({ schemaVersion: 1, runner: "hyperfine", results: metadata }, null, 2)}\n`,
);

console.log(JSON.stringify({ commandsDir, dataDir, languages }));

function shellCommand(parts) {
  return parts.map((part) => shellQuote(part)).join(" ");
}

function shellQuote(value) {
  return `'${String(value).replaceAll("'", `'\\''`)}'`;
}

function findBat() {
  for (const candidate of ["bat", "batcat"]) {
    const result = spawnSync(candidate, ["--version"], { stdio: "ignore" });
    if (!result.error && result.status === 0) return candidate;
  }
  throw new Error("bat is required");
}

function findScenarioShell() {
  const shell = process.env.SHELL;
  if (shell && new Set(["bash", "dash", "ksh", "sh", "zsh"]).has(basename(shell))) return shell;
  return "/bin/sh";
}
