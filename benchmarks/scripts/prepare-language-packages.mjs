#!/usr/bin/env node

import { createHash } from "node:crypto";
import { copyFile, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";

const benchmarksDir = resolve(import.meta.dirname, "..");
const repoDir = resolve(benchmarksDir, "..");
const outputDir = resolve(repoDir, "target/benchmarks/language-packages");
const benchmarkRequire = createRequire(resolve(benchmarksDir, "javascript/package.json"));
const benchmarkPackage = JSON.parse(
  await readFile(resolve(benchmarksDir, "javascript/package.json"), "utf8"),
);
const packageNames = Object.keys(benchmarkPackage.dependencies)
  .filter((name) => name.startsWith("@lumis-sh/wasm-"))
  .sort();

// Also laid out the way LanguageStore reads a source directory, so the CLI,
// Elixir and Node benchmarks all resolve from these bytes without a network.
const storeDir = resolve(outputDir, "parsers");

await rm(outputDir, { recursive: true, force: true });
await mkdir(outputDir, { recursive: true });
await mkdir(storeDir, { recursive: true });

const index = {};
for (const packageName of packageNames) {
  const language = packageName.slice("@lumis-sh/wasm-".length);
  const wasmName = `tree-sitter-${language}`;
  const wasmPath = benchmarkRequire.resolve(`${packageName}/${wasmName}.wasm`);
  const wasm = await readFile(wasmPath);
  const packageJson = JSON.parse(
    await readFile(resolve(dirname(wasmPath), "package.json"), "utf8"),
  );
  const grammarName = wasmGrammarName(wasm);
  const queries = await readQueries(language);
  const sha256 = createHash("sha256").update(wasm).digest("hex");
  const definitionHash = createHash("sha256")
    .update(wasmName)
    .update(sha256)
    .update(Object.values(queries).join("\0"))
    .digest("hex");
  const directory = resolve(outputDir, `wasm-${language}`);
  const metadataPath = resolve(directory, "lumis.json");
  const localWasmPath = resolve(directory, `${wasmName}.wasm`);

  await mkdir(directory, { recursive: true });
  await copyFile(wasmPath, localWasmPath);
  await copyFile(wasmPath, resolve(storeDir, `${wasmName}-${packageJson.version}-${sha256}.wasm`));
  const document = `${JSON.stringify(
    {
      packageName,
      version: packageJson.version,
      definitionHash,
      parser: {
        name: wasmName,
        grammarName,
        sha256,
        size: wasm.byteLength,
      },
      languages: {
        [language]: {
          aliases: [],
          ...queries,
        },
      },
    },
    null,
    2,
  )}\n`;
  await writeFile(metadataPath, document);
  await writeFile(resolve(storeDir, `${language}.lumis.json`), document);

  index[packageName] = {
    language,
    metadataPath,
    wasmPath: localWasmPath,
  };
}

await writeFile(resolve(outputDir, "index.json"), `${JSON.stringify(index, null, 2)}\n`);
console.log(outputDir);

async function readQueries(language) {
  const query = async (name) => {
    try {
      return await readFile(resolve(repoDir, "queries/processed", language, `${name}.scm`), "utf8");
    } catch (error) {
      if (error.code === "ENOENT") return "";
      throw error;
    }
  };
  const brackets =
    (await query("brackets")) ||
    (await readFile(resolve(repoDir, "queries/processed/default/brackets.scm"), "utf8"));
  return {
    highlights: await query("highlights"),
    injections: await query("injections"),
    locals: await query("locals"),
    brackets,
  };
}

function wasmGrammarName(bytes) {
  const names = WebAssembly.Module.exports(new WebAssembly.Module(bytes))
    .filter(({ kind, name }) => kind === "function" && name.startsWith("tree_sitter_"))
    .map(({ name }) => name.slice("tree_sitter_".length))
    .filter((name) => !name.startsWith("external_scanner_"));
  if (names.length !== 1) {
    throw new Error(`expected one Tree-sitter grammar export, found: ${names.join(", ")}`);
  }
  return names[0];
}
