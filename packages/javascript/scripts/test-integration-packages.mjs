import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const javascriptDir = fileURLToPath(new URL("../", import.meta.url));
const integrations = ["rehype-lumis", "markdown-it-lumis", "react", "vite"];
const lumisName = "@lumis-sh/lumis";
const project = mkdtempSync(join(tmpdir(), "lumis-integration-packages-"));

function readJson(path) {
  return JSON.parse(readFileSync(path, "utf8"));
}

function run(command, args, cwd) {
  return execFileSync(command, args, {
    cwd,
    encoding: "utf8",
    timeout: 180_000,
    stdio: ["ignore", "pipe", "pipe"],
  });
}

function pack(directory) {
  const tarball = join(project, `${directory}.tgz`);
  run(
    "pnpm",
    ["--config.ignore-scripts=true", "pack", "--out", tarball],
    join(javascriptDir, directory),
  );
  const manifest = JSON.parse(run("tar", ["-xOf", tarball, "package/package.json"], project));
  return { tarball, manifest };
}

function dependency(directory, name) {
  return readJson(join(javascriptDir, directory, "node_modules", name, "package.json")).version;
}

try {
  const themes = pack("themes");
  const lumis = pack("lumis");
  const dependencies = { [lumisName]: `file:${lumis.tarball}` };
  for (const directory of integrations) {
    const { tarball, manifest } = pack(directory);
    assert.equal(
      manifest.dependencies?.[lumisName],
      undefined,
      `${manifest.name} bundles a private Lumis dependency`,
    );
    assert.equal(
      manifest.peerDependencies?.[lumisName],
      `^${lumis.manifest.version}`,
      `${manifest.name} must publish its Lumis peer range`,
    );
    assert.notEqual(
      manifest.peerDependenciesMeta?.[lumisName]?.optional,
      true,
      `${manifest.name} must require its Lumis peer`,
    );
    dependencies[manifest.name] = `file:${tarball}`;
  }

  for (const [directory, name] of [
    ["markdown-it-lumis", "markdown-it"],
    ["markdown-it-lumis", "markdown-it-attrs"],
    ["rehype-lumis", "unified"],
    ["rehype-lumis", "hast-util-from-html"],
    ["vite", "hast-util-to-html"],
    ["vite", "vite"],
    ["react", "react"],
    ["react", "react-dom"],
  ]) {
    dependencies[name] = dependency(directory, name);
  }

  const consumer = join(project, "consumer");
  mkdirSync(consumer);
  writeFileSync(
    join(consumer, "package.json"),
    JSON.stringify({
      private: true,
      type: "module",
      // Pin the repository's pnpm so a different global pnpm cannot change peer resolution.
      packageManager: readJson(join(javascriptDir, "../../package.json")).packageManager,
      dependencies,
    }),
  );
  // Lumis's theme data and Vite's rehype dependency must also come from this
  // build, not the registry.
  writeFileSync(
    join(consumer, "pnpm-workspace.yaml"),
    [
      "overrides:",
      `  '${themes.manifest.name}': 'file:${themes.tarball}'`,
      `  '@lumis-sh/rehype-lumis': '${dependencies["@lumis-sh/rehype-lumis"]}'`,
      "",
    ].join("\n"),
  );
  run(
    "pnpm",
    [
      "install",
      "--ignore-scripts",
      "--no-optional",
      "--strict-peer-dependencies",
      "--prefer-offline",
    ],
    consumer,
  );
  copyFileSync(new URL("./integration-consumer.mjs", import.meta.url), join(consumer, "test.mjs"));
  console.log(run(process.execPath, ["test.mjs"], consumer).trim());
} finally {
  rmSync(project, { recursive: true, force: true });
}
