// `@lumis-sh/cli` pins its platform packages as optionalDependencies and pnpm
// rewrites `workspace:*` to the workspace version at pack time, so a main
// package bumped on its own would ship dependencies pointing at the previous
// release. npm refuses to republish an existing version, so the release
// workflow would fail before reaching the main package.
//
// The version the CLI downloaded used to be derived from its own npm version,
// which drifted from the crate twice and left `@lumis-sh/cli` 0.5.0 and 0.6.0
// answering every install with a 404. npm resolves the binary now, so the
// lockstep that matters is this one.
import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const javascriptDir = dirname(dirname(fileURLToPath(import.meta.url)));
const repoDir = dirname(dirname(javascriptDir));
const cliDir = join(javascriptDir, "cli");
const npmDir = join(cliDir, "npm");

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
const EXPECTED_NAMES = PLATFORM_DIRS.map((target) => `@lumis-sh/cli-${target}`);
const EXPECTED_NAME_SET = new Set(EXPECTED_NAMES);

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8"));
}

const mainPackage = readJson(join(cliDir, "package.json"));
const expected = mainPackage.version;
const problems = [];
const cargoManifest = readFileSync(join(repoDir, "crates/lumis-cli/Cargo.toml"), "utf8");
const cargoVersion = cargoManifest.match(/^version = "([^"]+)"$/m)?.[1];

if (mainPackage.binaryVersion !== cargoVersion) {
  problems.push(
    `packages/javascript/cli/package.json: binaryVersion is ${mainPackage.binaryVersion}, expected ${cargoVersion}`,
  );
}

const present = new Set(readdirSync(npmDir));
for (const directory of PLATFORM_DIRS) {
  const relative = `packages/javascript/cli/npm/${directory}`;
  if (!present.has(directory)) {
    problems.push(`${relative}: missing`);
    continue;
  }
  present.delete(directory);

  const manifest = readJson(join(npmDir, directory, "package.json"));
  if (manifest.name !== `@lumis-sh/cli-${directory}`) {
    problems.push(`${relative}: declares ${manifest.name}, expected @lumis-sh/cli-${directory}`);
  }
  if (manifest.version !== expected) {
    problems.push(`${relative}: ${manifest.version}`);
  }
}

for (const extra of present) {
  problems.push(`packages/javascript/cli/npm/${extra}: not published by javascript-release.yml`);
}

const dependencies = mainPackage.optionalDependencies ?? {};
for (const name of EXPECTED_NAMES) {
  const range = dependencies[name];
  if (range === undefined) {
    problems.push(`packages/javascript/cli/package.json: ${name} is not an optionalDependency`);
  } else if (range !== "workspace:*") {
    problems.push(
      `packages/javascript/cli/package.json: ${name} is "${range}", expected "workspace:*"`,
    );
  }
}

for (const name of Object.keys(dependencies)) {
  if (!EXPECTED_NAME_SET.has(name)) {
    problems.push(`packages/javascript/cli/package.json: unexpected optionalDependency ${name}`);
  }
}

// A postinstall here would mean the binary is being fetched again rather than
// resolved by npm, which is the failure this layout exists to remove.
for (const hook of ["preinstall", "install", "postinstall"]) {
  if (mainPackage.scripts?.[hook]) {
    problems.push(`packages/javascript/cli/package.json: ${hook} script reintroduces a download`);
  }
}

if (problems.length > 0) {
  console.error(
    `@lumis-sh/cli is ${expected}; its platform packages must match and be complete:\n` +
      problems.map((problem) => `  ${problem}`).join("\n") +
      "\n\n`mise run release-prepare npm-cli <version>` bumps all of them together.",
  );
  process.exit(1);
}

console.log(
  `${PLATFORM_DIRS.length + 1} CLI manifests are at ${expected}; binaryVersion is ${cargoVersion}`,
);
