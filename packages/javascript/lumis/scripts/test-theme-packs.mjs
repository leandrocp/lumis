import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const javascript = fileURLToPath(new URL("../../", import.meta.url));
const project = mkdtempSync(join(tmpdir(), "lumis-theme-packs-"));

function run(command, args, cwd) {
  return execFileSync(command, args, { cwd, encoding: "utf8", timeout: 180_000 });
}

function pack(directory) {
  const tarball = join(project, `${directory}.tgz`);
  run(
    "pnpm",
    ["--config.ignore-scripts=true", "pack", "--out", tarball],
    join(javascript, directory),
  );
  return tarball;
}

function install(consumer) {
  run(
    "pnpm",
    ["install", "--prod", "--ignore-scripts", "--no-optional", "--prefer-offline"],
    consumer,
  );
}

function assertPackAssets(tarball, names) {
  const files = run("tar", ["-tf", tarball], project).trim().split("\n");
  assert.ok(files.includes("package/dist/NOTICE.md"));
  assert.ok(files.some((file) => file.startsWith("package/dist/licenses/")));
  for (const kind of ["css", "json"]) {
    const prefix = `package/dist/${kind}/`;
    const entries = files.filter((file) => file.startsWith(prefix) && file.endsWith(`.${kind}`));
    assert.deepEqual(
      entries
        .map((file) => file.slice(prefix.length, -kind.length - 1))
        .sort((a, b) => a.localeCompare(b)),
      [...names].sort((a, b) => a.localeCompare(b)),
    );
  }
}

try {
  const lumis = pack("lumis");
  const themes = pack("themes");
  const copyleft = pack("themes-copyleft");
  const consumer = join(project, "consumer");
  mkdirSync(consumer);
  const manifest = {
    private: true,
    type: "module",
    dependencies: { "@lumis-sh/lumis": `file:${lumis}` },
  };
  writeFileSync(join(consumer, "package.json"), JSON.stringify(manifest));
  writeFileSync(
    join(consumer, "pnpm-workspace.yaml"),
    `overrides:\n  '@lumis-sh/themes': 'file:${themes}'\n`,
  );
  copyFileSync(
    new URL("../test/theme-pack-consumer.mjs", import.meta.url),
    join(consumer, "test.mjs"),
  );
  const defaultPack = JSON.parse(
    readFileSync(new URL("../../../../themes/packs/default.json", import.meta.url), "utf8"),
  );
  const optionalPack = JSON.parse(
    readFileSync(new URL("../../../../themes/packs/copyleft.json", import.meta.url), "utf8"),
  );
  assertPackAssets(
    themes,
    defaultPack.themes.map((theme) => theme.name),
  );
  assertPackAssets(
    copyleft,
    optionalPack.themes.map((theme) => theme.name),
  );
  writeFileSync(
    join(consumer, "expected.json"),
    JSON.stringify({
      default: defaultPack.themes.map((theme) => theme.name),
      copyleft: optionalPack.themes.map((theme) => theme.name),
    }),
  );
  install(consumer);
  console.log(run(process.execPath, [join(consumer, "test.mjs"), "default"], project));
  console.log(
    run(process.execPath, ["--conditions=browser", join(consumer, "test.mjs"), "browser"], project),
  );
  const installed = JSON.parse(
    readFileSync(join(consumer, "node_modules/@lumis-sh/lumis/package.json"), "utf8"),
  );
  assert.equal(installed.peerDependenciesMeta["@lumis-sh/themes-copyleft"].optional, true);
  assert.equal(installed.optionalDependencies["@lumis-sh/themes-copyleft"], undefined);
  manifest.dependencies["@lumis-sh/themes"] = `file:${themes}`;
  writeFileSync(join(consumer, "package.json"), JSON.stringify(manifest));
  install(consumer);
  console.log(run(process.execPath, [join(consumer, "test.mjs"), "legacy-missing"], project));
  manifest.dependencies["@lumis-sh/themes-copyleft"] = `file:${copyleft}`;
  writeFileSync(join(consumer, "package.json"), JSON.stringify(manifest));
  install(consumer);
  console.log(run(process.execPath, [join(consumer, "test.mjs"), "copyleft"], project));
  console.log(
    run(process.execPath, ["--conditions=browser", join(consumer, "test.mjs"), "browser"], project),
  );
} finally {
  rmSync(project, { recursive: true, force: true });
}
