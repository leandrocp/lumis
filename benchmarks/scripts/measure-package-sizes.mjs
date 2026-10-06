#!/usr/bin/env node

import { execFile as execFileCallback } from "node:child_process";
import { gzipSync } from "node:zlib";
import { createRequire } from "node:module";
import { mkdir, mkdtemp, readFile, realpath, rm, stat, writeFile } from "node:fs/promises";
import { cpus, platform, release, tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { promisify } from "node:util";
import { implementationById } from "./implementations.mjs";

const execFile = promisify(execFileCallback);
const benchmarksDir = resolve(import.meta.dirname, "..");
const repoDir = resolve(benchmarksDir, "..");
const runDir = resolve(
  process.env.BENCH_RUN_DIR ?? resolve(repoDir, "target/benchmarks/runs/current"),
);
const releaseDir = resolve(
  process.env.CARGO_TARGET_DIR ?? resolve(repoDir, "target/benchmarks/rust-target"),
  "release",
);
const requireFromScript = createRequire(import.meta.url);
const npmTarLibrary = process.env.BENCH_NPM_TAR
  ? requireFromScript(process.env.BENCH_NPM_TAR)
  : undefined;
const pnpmExecutable = process.env.BENCH_PNPM ?? "pnpm";
const requestedGroup = process.env.BENCH_PACKAGE_SIZE_GROUP;
const validGroups = new Set(["javascript", "rust", "cli", "elixir"]);
const groups = requestedGroup ? new Set([requestedGroup]) : validGroups;
if ([...groups].some((group) => !validGroups.has(group))) {
  throw new Error(`unknown package-size group: ${requestedGroup}`);
}
const temporaryDir = await mkdtemp(join(tmpdir(), "lumis-package-sizes-"));

try {
  const entries = [];
  if (groups.has("javascript")) {
    // The platform addon is an optional dependency, so a Node install pulls it
    // and a browser bundle does not. Measuring one closure for both would
    // attribute the addon's bytes to a runtime that never downloads it.
    const lumisNodePackages = await packageClosure(
      resolve(repoDir, "packages/javascript/lumis"),
      temporaryDir,
    );
    const lumisWasmPackages = await packageClosure(
      resolve(repoDir, "packages/javascript/lumis"),
      temporaryDir,
      { includeOptional: false },
    );
    const shikiPackages = await packageClosure(
      resolve(repoDir, "benchmarks/javascript/node_modules/shiki"),
      temporaryDir,
    );
    const highlightJsPackages = await packageClosure(
      resolve(repoDir, "benchmarks/javascript/node_modules/highlight.js"),
      temporaryDir,
    );
    const parserPackages = new Map();
    for (const language of [
      "c",
      "css",
      "go",
      "html",
      "java",
      "javascript",
      "json",
      "python",
      "ruby",
      "rust",
    ]) {
      parserPackages.set(
        language,
        await packageClosure(
          resolve(repoDir, `benchmarks/javascript/node_modules/@lumis-sh/wasm-${language}`),
          temporaryDir,
        ),
      );
    }
    entries.push(
      packageEntry(
        "lumis-js-node",
        "runtime",
        "npm production closure including the platform addon; parsers load on demand",
        lumisNodePackages,
      ),
      packageEntry(
        "lumis-js-node",
        "10 languages",
        "npm production closure including the platform addon, plus benchmark parsers",
        combinePackages(lumisNodePackages, ...parserPackages.values()),
      ),
      packageEntry(
        "lumis-js-wasm",
        "runtime",
        "npm production closure without the platform addon; parsers load on demand",
        lumisWasmPackages,
      ),
      packageEntry(
        "lumis-js-wasm",
        "1 language",
        "npm production closure plus Rust parser",
        combinePackages(lumisWasmPackages, parserPackages.get("rust")),
      ),
      packageEntry(
        "lumis-js-wasm",
        "10 languages",
        "npm production closure plus benchmark parsers",
        combinePackages(lumisWasmPackages, ...parserPackages.values()),
      ),
      packageEntry("shiki", undefined, "npm production closure", shikiPackages),
      packageEntry("highlight-js", undefined, "npm production closure", highlightJsPackages),
    );
  }

  if (groups.has("rust")) {
    entries.push(
      await binaryEntry(
        "lumis-rust",
        "stripped 10-language benchmark executable",
        resolve(releaseDir, "lumis-size"),
      ),
      await binaryEntry(
        "syntect",
        "stripped default-syntax benchmark executable",
        resolve(releaseDir, "syntect-size"),
      ),
    );
  }

  if (groups.has("cli")) {
    entries.push(
      await binaryEntry(
        "lumis-cli",
        "stripped release executable",
        resolve(releaseDir, executableName("lumis")),
      ),
      await binaryEntry("bat", "release executable", requiredEnvironment("BAT_BINARY")),
    );
  }

  if (groups.has("elixir")) {
    entries.push(
      await binaryEntry(
        "lumis-elixir",
        "stripped release NIF shared library",
        await nifPath(releaseDir),
      ),
    );
  }

  const report = {
    schemaVersion: 2,
    group: requestedGroup ?? "all",
    system: {
      platform: platform(),
      release: release(),
      architecture: process.arch,
      cpu: cpus()[0]?.model ?? "unknown",
    },
    boundaries: {
      npm: "sum of npm pack archives and unpacked bytes for the unique production dependency closure",
      native:
        "raw local release artifact and the same bytes compressed with deterministic gzip level 9",
    },
    entries,
  };
  const output = requestedGroup
    ? resolve(runDir, "package-sizes", `${requestedGroup}.json`)
    : resolve(runDir, "package-sizes.json");
  await mkdir(dirname(output), { recursive: true });
  await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
  console.log(output);
} finally {
  await rm(temporaryDir, { recursive: true, force: true });
}

async function packageClosure(packageDir, packRoot, { includeOptional = true } = {}) {
  const packages = new Map();
  const queue = [await realpath(packageDir)];
  while (queue.length > 0) {
    const directory = queue.shift();
    const manifest = JSON.parse(await readFile(resolve(directory, "package.json"), "utf8"));
    const key = `${manifest.name}@${manifest.version}`;
    if (packages.has(key)) continue;

    const packed = await packPackage(directory, key, packRoot);
    packages.set(key, {
      key,
      name: manifest.name,
      version: manifest.version,
      packedBytes: packed.size,
      unpackedBytes: packed.unpackedSize,
    });

    for (const dependency of Object.keys({
      ...manifest.dependencies,
      ...(includeOptional ? manifest.optionalDependencies : {}),
    })) {
      const resolved = await resolveDependencyDirectory(directory, dependency, manifest, key);
      if (resolved) queue.push(resolved);
    }
  }
  return packages;
}

// An optional dependency that will not resolve is skipped; a production one that
// will not resolve is an error naming the package that wanted it.
async function resolveDependencyDirectory(directory, dependency, manifest, key) {
  try {
    return await resolvePackageDirectory(directory, dependency);
  } catch (error) {
    if (manifest.optionalDependencies?.[dependency]) return;
    throw new Error(`${key} is missing production dependency ${dependency}`, { cause: error });
  }
}

async function resolvePackageDirectory(directory, name) {
  for (const candidate of [
    resolve(directory, "node_modules", name),
    resolve(dirname(directory), name),
  ]) {
    const resolved = await packageDirectoryNamed(candidate, name);
    if (resolved) return resolved;
  }

  // Fall back to Node's package resolution.
  const require = createRequire(resolve(directory, "package.json"));
  try {
    return dirname(await realpath(require.resolve(`${name}/package.json`)));
  } catch (error) {
    if (error.code !== "ERR_PACKAGE_PATH_NOT_EXPORTED") throw error;
  }

  return walkUpToPackageRoot(dirname(await realpath(require.resolve(name))), name);
}

// The candidate directory, if it exists and its manifest claims `name`.
async function packageDirectoryNamed(candidate, name) {
  try {
    const resolved = await realpath(candidate);
    const manifest = JSON.parse(await readFile(resolve(resolved, "package.json"), "utf8"));
    if (manifest.name === name) return resolved;
  } catch {
    // Not a package directory, or not the one asked for.
  }
}

// A package whose exports hide its manifest resolves to a file inside it; walk
// up from there until a manifest claims `name`.
async function walkUpToPackageRoot(start, name) {
  let candidate = start;
  while (true) {
    const resolved = await packageDirectoryNamed(candidate, name);
    if (resolved) return resolved;

    const parent = dirname(candidate);
    if (parent === candidate) throw new Error(`could not resolve package root for ${name}`);
    candidate = parent;
  }
}

async function packPackage(directory, key, packRoot) {
  if (npmTarLibrary) {
    return packPackageWithPnpm(directory, key, packRoot);
  }

  const destination = resolve(
    packRoot,
    key.replaceAll("/", "_").replaceAll("@", "_").replaceAll(":", "_"),
  );
  await mkdir(destination, { recursive: true });
  const args = ["pack", "--json", "--ignore-scripts", "--pack-destination", destination, directory];
  const stdout = await npmPack(args, packRoot, key);
  const result = JSON.parse(stdout).at(-1);
  if (!result || !Number.isSafeInteger(result.size) || !Number.isSafeInteger(result.unpackedSize)) {
    throw new Error(`npm pack returned invalid sizes for ${key}`);
  }
  return result;
}

// npm intermittently fails with "Exit handler never called"; one retry is
// enough, and anything else is the real failure.
async function npmPack(args, packRoot, key) {
  for (let attempt = 1; attempt <= 2; attempt += 1) {
    try {
      const { stdout } = await execFile("npm", args, {
        env: {
          ...process.env,
          npm_config_cache: resolve(packRoot, "npm-cache"),
          npm_config_ignore_scripts: "true",
        },
        maxBuffer: 10 * 1024 * 1024,
      });
      return stdout;
    } catch (error) {
      if (attempt === 2 || !error.stderr?.includes("Exit handler never called")) throw error;
      console.warn(`npm pack hit npm's exit-handler failure for ${key}; retrying once`);
    }
  }

  throw new Error(`npm pack did not produce output for ${key}`);
}

async function packPackageWithPnpm(directory, key, packRoot) {
  const pnpmDestination = resolve(packRoot, "pnpm");
  await mkdir(pnpmDestination, { recursive: true });
  const { stdout } = await execFile(
    pnpmExecutable,
    [
      "-C",
      directory,
      "--config.ignore-scripts=true",
      "pack",
      "--json",
      "--skip-manifest-obfuscation",
      "--pack-destination",
      pnpmDestination,
    ],
    { maxBuffer: 10 * 1024 * 1024 },
  );
  const packed = JSON.parse(stdout);
  if (!packed.filename) {
    throw new Error(`pnpm pack returned no archive for ${key}`);
  }

  // pnpm's archive has the same files as npm but different tar metadata.
  // Repack its ordered file list with npm's settings to retain exact npm sizes.
  const files = [];
  const fileListParser = npmTarLibrary.list({
    onReadEntry(entry) {
      files.push(entry.path.replace(/^package\//u, ""));
    },
  });
  fileListParser.end(await readFile(packed.filename));
  const manifest = JSON.parse(await readFile(resolve(directory, "package.json"), "utf8"));
  const bins = new Set(
    typeof manifest.bin === "string" ? [manifest.bin] : Object.values(manifest.bin ?? {}),
  );
  const tarball = await npmTarLibrary
    .create(
      {
        cwd: directory,
        prefix: "package/",
        portable: true,
        sync: true,
        gzip: { level: 9 },
        mtime: new Date("1985-10-26T08:15:00.000Z"),
        filter(path, details) {
          if (bins.has(path.replace(/^[^\\/]*[\\/]/u, ""))) details.mode |= 0o111;
          return true;
        },
      },
      files,
    )
    .concat();
  let unpackedSize = 0;
  const sizeParser = npmTarLibrary.list({
    onReadEntry(entry) {
      unpackedSize += entry.size;
    },
  });
  sizeParser.end(tarball);
  return { size: tarball.length, unpackedSize };
}

function combinePackages(...collections) {
  const combined = new Map();
  for (const collection of collections) {
    for (const [key, value] of collection) combined.set(key, value);
  }
  return combined;
}

function packageEntry(implementation, variant, artifact, packages) {
  implementationById(implementation);
  return {
    implementation,
    ...(variant && { variant }),
    artifact,
    format: "npm",
    rawBytes: sum(packages, "unpackedBytes"),
    compressedBytes: sum(packages, "packedBytes"),
    packageCount: packages.size,
  };
}

async function binaryEntry(implementation, artifact, path) {
  const label = implementationById(implementation).label;
  const bytes = await readFile(path);
  const details = await stat(path);
  if (!details.isFile() || bytes.length === 0) {
    throw new Error(`${label} artifact is missing or empty: ${path}`);
  }
  return {
    implementation,
    artifact,
    format: "native",
    rawBytes: bytes.length,
    compressedBytes: gzipSync(bytes, { level: 9, mtime: 0 }).length,
  };
}

async function nifPath(directory) {
  for (const filename of ["liblumis_nif.dylib", "liblumis_nif.so", "lumis_nif.dll"]) {
    const path = resolve(directory, filename);
    try {
      if ((await stat(path)).isFile()) return path;
    } catch {
      // Try the next platform filename.
    }
  }
  throw new Error(`could not find the Lumis NIF in ${directory}`);
}

function executableName(name) {
  return process.platform === "win32" ? `${name}.exe` : name;
}

function requiredEnvironment(name) {
  const value = process.env[name];
  if (!value) throw new Error(`${name} must point to the comparison artifact`);
  return value;
}

function sum(packages, field) {
  return [...packages.values()].reduce((total, entry) => total + entry[field], 0);
}
