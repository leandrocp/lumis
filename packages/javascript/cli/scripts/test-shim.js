"use strict";

const {
  chmodSync,
  copyFileSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  rmSync,
  writeFileSync,
} = require("node:fs");
const { spawnSync } = require("node:child_process");
const { tmpdir } = require("node:os");
const { join } = require("node:path");
const test = require("node:test");
const assert = require("node:assert/strict");

const { cliTargetFor, hostLibc } = require("../bin/target.js");

const packageDir = join(__dirname, "..");
const npmDir = join(packageDir, "npm");
const shim = join(packageDir, "bin", "lumis");
const version = require("../package.json").version;

const PLATFORMS = ["darwin", "linux", "win32", "freebsd", "openbsd", "sunos", "aix", "android"];
const ARCHS = ["arm64", "x64", "ia32", "arm", "riscv64", "ppc64", "s390x", "loong64"];
const LIBCS = ["gnu", "musl"];
const EXPECTED_TARGETS = [
  "darwin-arm64",
  "darwin-x64",
  "linux-arm64-gnu",
  "linux-arm64-musl",
  "linux-x64-gnu",
  "linux-x64-musl",
  "win32-arm64-msvc",
  "win32-x64-msvc",
];
const addonNpmDir = join(packageDir, "..", "lumis", "native", "npm");

function reachableTargets() {
  const found = new Set();
  for (const platform of PLATFORMS) {
    for (const arch of ARCHS) {
      for (const libc of LIBCS) {
        const target = cliTargetFor(platform, arch, libc);
        if (target) found.add(target);
      }
    }
  }
  return [...found].sort();
}

test("every reachable target has a published package, and the reverse", () => {
  assert.deepEqual(reachableTargets(), EXPECTED_TARGETS);
  assert.deepEqual(readdirSync(npmDir).sort(), EXPECTED_TARGETS);
});

test("rejects an incomplete target corpus", () => {
  const missingWindowsArm64 = EXPECTED_TARGETS.filter((t) => t !== "win32-arm64-msvc");
  assert.throws(() => assert.deepEqual(missingWindowsArm64, EXPECTED_TARGETS));
});

test("each package declares the host it is selected for", () => {
  for (const target of readdirSync(npmDir)) {
    const manifest = require(join(npmDir, target, "package.json"));
    assert.equal(manifest.name, `@lumis-sh/cli-${target}`);
    assert.equal(manifest.version, version, `${target} is not in lockstep with @lumis-sh/cli`);

    const libc = manifest.libc?.[0] === "musl" ? "musl" : "gnu";
    assert.equal(
      cliTargetFor(manifest.os[0], manifest.cpu[0], libc),
      target,
      `${manifest.os[0]}-${manifest.cpu[0]}-${libc}`,
    );
  }
});

test("musl and glibc hosts resolve to different packages", () => {
  assert.equal(cliTargetFor("linux", "x64", "musl"), "linux-x64-musl");
  assert.equal(cliTargetFor("linux", "x64", "gnu"), "linux-x64-gnu");
  assert.equal(cliTargetFor("linux", "arm64", "musl"), "linux-arm64-musl");
  assert.equal(cliTargetFor("linux", "arm64", "gnu"), "linux-arm64-gnu");
});

// The CLI and the addon cover the same hosts and compute the target the same
// way, in two copies neither package can import from the other. A target added
// to the addon alone would leave musl or a new arch without a CLI.
test("covers the same targets as the @lumis-sh/lumis addon", () => {
  const addonTargets = readdirSync(addonNpmDir)
    .filter((entry) => entry !== "meta")
    .sort();
  assert.deepEqual(addonTargets, EXPECTED_TARGETS);
});

test("hostLibc answers gnu off Linux and one of the two flavours on it", () => {
  if (process.platform === "linux") {
    assert.ok(LIBCS.includes(hostLibc()));
  } else {
    assert.equal(hostLibc(), "gnu");
  }
});

test("the shim discovers an installed optional pack without changing user arguments", () => {
  const target = cliTargetFor(process.platform, process.arch, hostLibc());
  if (!target) {
    throw new Error(`this host (${process.platform}-${process.arch}) has no CLI package to test`);
  }

  const isolated = mkdtempSync(join(tmpdir(), "lumis-cli-shim-"));
  const bin = join(isolated, "bin");
  const platformPackage = join(isolated, "node_modules", "@lumis-sh", `cli-${target}`);
  const optionalPackage = join(isolated, "node_modules", "@lumis-sh", "themes-copyleft");
  const binaryName = process.platform === "win32" ? "lumis.exe" : "lumis";
  const binary = join(platformPackage, binaryName);
  mkdirSync(bin);
  mkdirSync(platformPackage, { recursive: true });
  copyFileSync(shim, join(bin, "lumis"));
  copyFileSync(join(packageDir, "bin", "target.js"), join(bin, "target.js"));
  writeFileSync(join(platformPackage, "package.json"), "{}");
  writeFileSync(
    binary,
    "#!/usr/bin/env node\nconsole.log(JSON.stringify(process.argv.slice(2)));\n",
  );
  chmodSync(binary, 0o755);

  try {
    const run = () => {
      const result = spawnSync(
        process.execPath,
        [join(bin, "lumis"), "--theme", "nord", "a b.rs"],
        {
          encoding: "utf8",
          cwd: tmpdir(),
        },
      );
      assert.equal(result.stderr, "");
      assert.equal(result.status, 0);
      return JSON.parse(result.stdout);
    };
    const userArgs = ["--theme", "nord", "a b.rs"];
    assert.deepEqual(run(), userArgs);

    mkdirSync(optionalPackage, { recursive: true });
    const manifest = join(optionalPackage, "lumis-themes.json");
    writeFileSync(manifest, "{}");
    assert.deepEqual(run(), ["--theme-pack", manifest, ...userArgs]);

    rmSync(optionalPackage, { recursive: true });
    assert.deepEqual(run(), userArgs);
  } finally {
    rmSync(isolated, { force: true, recursive: true });
  }
});

// Run a copy of the shim from a directory with no `node_modules` above it, so
// `require.resolve` genuinely fails. Asserting the message inside the workspace
// is impossible: pnpm links every platform package next to the real shim.
test("a missing platform package fails with an actionable message", () => {
  const isolated = mkdtempSync(join(tmpdir(), "lumis-cli-shim-"));
  try {
    const bin = join(isolated, "bin");
    mkdirSync(bin);
    copyFileSync(shim, join(bin, "lumis"));
    copyFileSync(join(packageDir, "bin", "target.js"), join(bin, "target.js"));

    const result = spawnSync(process.execPath, [join(bin, "lumis")], { encoding: "utf8" });
    assert.equal(result.status, 1);
    assert.match(result.stderr, /is not installed/u);
    assert.match(result.stderr, /Reinstall @lumis-sh\/cli/u);
  } finally {
    rmSync(isolated, { force: true, recursive: true });
  }
});
