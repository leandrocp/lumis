#!/usr/bin/env node

import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";

const benchmarksDir = resolve(import.meta.dirname, "..");
const repoDir = resolve(benchmarksDir, "..");
const manifestPath = resolve(benchmarksDir, "fixtures/manifest.json");
const outputDir = resolve(repoDir, "target/benchmarks/fixtures");
const resolvedPath = resolve(outputDir, "scenarios.json");

function run(command, args) {
  const result = spawnSync(command, args, { cwd: repoDir, encoding: "utf8" });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(" ")} failed\n${result.stdout}${result.stderr}`);
  }
}

run(process.execPath, [resolve(benchmarksDir, "scripts/generate-fixtures.mjs")]);

const manifest = JSON.parse(await readFile(manifestPath, "utf8"));
if (manifest.schemaVersion !== 1 || manifest.scenarios?.length !== 4) {
  throw new Error("benchmark manifest must define exactly four scenarios");
}

const compiledRust = new Set();
const scenarios = [];
for (const scenario of manifest.scenarios) {
  const files = [];
  for (const fixture of scenario.files) {
    const path = resolve(repoDir, fixture.path);
    const source = await readFile(path);
    const text = source.toString("utf8");
    if (source.length === 0) throw new Error(`${fixture.path} is empty`);
    if (text.includes("\r\n")) throw new Error(`${fixture.path} must use LF line endings`);

    const sha256 = createHash("sha256").update(source).digest("hex");
    files.push({ ...fixture, bytes: source.length, sha256 });

    if (fixture.language === "rust" && !compiledRust.has(path)) {
      compiledRust.add(path);
      const output = resolve(outputDir, "validation", `${sha256}.rmeta`);
      await mkdir(dirname(output), { recursive: true });
      run("rustc", [
        "--edition=2021",
        "--crate-type=lib",
        "--emit=metadata",
        "-Awarnings",
        path,
        "-o",
        output,
      ]);
    }
  }

  if (scenario.id === "large-one-language" && files[0].bytes < 5 * 1024 * 1024) {
    throw new Error("large-one-language must contain at least 5 MiB");
  }
  if (scenario.id.startsWith("ten-files") && files.length !== 10) {
    throw new Error(`${scenario.id} must contain exactly ten files`);
  }
  if (
    scenario.id.startsWith("ten-files") &&
    new Set(files.map((fixture) => fixture.sha256)).size !== 10
  ) {
    throw new Error(`${scenario.id} must contain ten different files`);
  }
  if (
    scenario.id === "ten-files-ten-languages" &&
    new Set(files.map((fixture) => fixture.language)).size !== 10
  ) {
    throw new Error("ten-files-ten-languages must contain ten different languages");
  }

  scenarios.push({
    id: scenario.id,
    description: scenario.description,
    fileCount: files.length,
    languageCount: new Set(files.map((fixture) => fixture.language)).size,
    inputBytes: files.reduce((total, fixture) => total + fixture.bytes, 0),
    files,
  });
}

await mkdir(outputDir, { recursive: true });
await writeFile(resolvedPath, `${JSON.stringify({ schemaVersion: 1, scenarios }, null, 2)}\n`);
console.log(resolvedPath);
