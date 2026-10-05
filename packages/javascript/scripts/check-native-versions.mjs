// The release workflow publishes every native package before `@lumis-sh/lumis`,
// and npm refuses to republish a name/version that already exists. pnpm also
// rewrites `workspace:*` to the workspace version when it packs, so a main
// package bumped on its own would ship optionalDependencies pointing at the
// previous native release.
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// Keep the Node 18-compatible path calculation until the shared scripts drop Node 18.
// oxlint-disable-next-line unicorn/prefer-import-meta-properties
const javascriptDir = dirname(dirname(fileURLToPath(import.meta.url)));
const lumisDir = join(javascriptDir, "lumis");
const nativeNpmDir = join(lumisDir, "native", "npm");

// Named rather than counted. A directory that disappears has to fail here, not
// silently shrink the set the release workflow publishes.
const PLATFORM_DIRS = [
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64-gnu",
  "linux-arm64-musl",
  "linux-x64-gnu",
  "linux-x64-musl",
  "win32-arm64-msvc",
  "win32-x64-msvc",
];
const SELECTOR_DIR = "meta";
const EXPECTED_PLATFORM_NAMES = PLATFORM_DIRS.map((target) => `@lumis-sh/lumis-native-${target}`);
const EXPECTED_PLATFORM_NAME_SET = new Set(EXPECTED_PLATFORM_NAMES);

// Directory -> the exact package name that directory must declare. A set would
// let two directories swap names and still pass, and the release workflow
// publishes by directory, so it would publish the selector name from a platform
// directory before the selector step got there.
const EXPECTED_NAME = new Map([
  [SELECTOR_DIR, "@lumis-sh/lumis-native"],
  ...PLATFORM_DIRS.map((target, index) => [target, EXPECTED_PLATFORM_NAMES[index]]),
]);

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8"));
}

const mainPackage = readJson(join(lumisDir, "package.json"));
const expected = mainPackage.version;
const problems = [];

const cargo = readFileSync(join(lumisDir, "native", "Cargo.toml"), "utf8");
const cargoVersion = /^version\s*=\s*"([^"]+)"/mu.exec(cargo)?.[1];
if (cargoVersion !== expected) {
  problems.push(`packages/javascript/lumis/native/Cargo.toml: ${cargoVersion ?? "no version"}`);
}

const present = new Set(readdirSync(nativeNpmDir));
for (const directory of [SELECTOR_DIR, ...PLATFORM_DIRS]) {
  const relative = `packages/javascript/lumis/native/npm/${directory}`;
  if (!present.has(directory)) {
    problems.push(`${relative}: missing`);
    continue;
  }
  present.delete(directory);

  const manifest = readJson(join(nativeNpmDir, directory, "package.json"));
  if (manifest.name !== EXPECTED_NAME.get(directory)) {
    problems.push(
      `${relative}: declares ${manifest.name}, expected ${EXPECTED_NAME.get(directory)}`,
    );
  }
  if (manifest.version !== expected) {
    problems.push(`${relative}: ${manifest.version}`);
  }
}

for (const extra of present) {
  problems.push(
    `packages/javascript/lumis/native/npm/${extra}: not published by javascript-release.yml`,
  );
}

// The main package and the selector both pin the platform packages, and pnpm
// rewrites `workspace:*` to the workspace version at pack time. A missing key
// ships a tarball that cannot install; a key pinned to anything other than
// `workspace:*` falls outside the release's lockstep version.
function checkPlatformDependencies(label, dependencies) {
  for (const name of EXPECTED_PLATFORM_NAMES) {
    const range = dependencies?.[name];
    if (range === undefined) {
      problems.push(`${label}: ${name} is not an optionalDependency`);
    } else if (range !== "workspace:*") {
      problems.push(`${label}: ${name} is "${range}", expected "workspace:*"`);
    }
  }

  for (const name of Object.keys(dependencies ?? {})) {
    if (!EXPECTED_PLATFORM_NAME_SET.has(name)) {
      problems.push(`${label}: unexpected optionalDependency ${name}`);
    }
  }
}

checkPlatformDependencies(
  "packages/javascript/lumis/package.json",
  mainPackage.optionalDependencies,
);
checkPlatformDependencies(
  `packages/javascript/lumis/native/npm/${SELECTOR_DIR}/package.json`,
  readJson(join(nativeNpmDir, SELECTOR_DIR, "package.json")).optionalDependencies,
);

if (problems.length > 0) {
  console.error(
    `@lumis-sh/lumis is ${expected}; the native packages must match and be complete:\n` +
      problems.map((problem) => `  ${problem}`).join("\n") +
      "\n\n`mise run release-prepare npm-lumis <version>` bumps all of them together.",
  );
  process.exit(1);
}

console.log(`${PLATFORM_DIRS.length + 2} native manifests are at ${expected}`);
