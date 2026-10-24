import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import * as esm from "@lumis-sh/lumis/themes";

const require = createRequire(import.meta.url);
const mode = process.argv[2];
const optional = mode === "copyleft";
const formats = mode === "browser" ? { esm } : { esm, cjs: require("@lumis-sh/lumis/themes") };
const expected = JSON.parse(readFileSync(new URL("./expected.json", import.meta.url), "utf8"));
const names = [...expected.default, ...(optional ? expected.copyleft : [])].sort((a, b) =>
  a.localeCompare(b),
);
const lumisRequire = createRequire(require.resolve("@lumis-sh/lumis/themes"));
const defaultManifest = JSON.parse(
  readFileSync(lumisRequire.resolve("@lumis-sh/themes/lumis-themes.json"), "utf8"),
);
assert.deepEqual(
  defaultManifest.themes.map((theme) => theme.name).sort((a, b) => a.localeCompare(b)),
  [...expected.default].sort((a, b) => a.localeCompare(b)),
);

for (const [format, themes] of Object.entries(formats)) {
  assert.ok(names.length > 200);
  assert.deepEqual(
    themes
      .available()
      .map((theme) => theme.name)
      .sort((a, b) => a.localeCompare(b)),
    names,
    `${mode} ${format} catalog`,
  );
  assert.equal(themes.get("dracula").name, "dracula");
  assert.equal(themes.get("modus_vivendi")?.name, optional ? "modus_vivendi" : undefined);
  assert.equal(themes.get("oceanic_next")?.name, optional ? "oceanic_next" : undefined);
  for (const name of ["darkplus", "no_clown_fiesta", "poimandres", "yorumi"]) {
    assert.equal(themes.get(name), undefined);
    assert.ok(!themes.available().some((entry) => entry.name === name));
  }
  const first = themes.get("dracula");
  first.highlights.normal.fg = "#123456";
  assert.notEqual(themes.get("dracula").highlights.normal.fg, "#123456");
  assert.equal(themes.closest({ background: "#000000" })?.name === "modus_vivendi", optional);
  assert.equal(themes.closest({ background: "#666666" }), undefined);
}

if (optional) {
  const legacy = await import("@lumis-sh/themes/modus_vivendi");
  const direct = await import("@lumis-sh/themes-copyleft/modus_vivendi");
  assert.deepEqual(legacy.default, direct.default);
  assert.deepEqual(
    require("@lumis-sh/themes/modus_vivendi"),
    require("@lumis-sh/themes-copyleft/modus_vivendi"),
  );
  const { buildCss } = await import("@lumis-sh/themes-copyleft");
  assert.match(buildCss(direct.default), /\.lumis/u);
} else if (mode === "default") {
  assert.throws(() => require.resolve("@lumis-sh/themes-copyleft/lumis-themes.json"));
} else if (mode === "legacy-missing") {
  await assert.rejects(import("@lumis-sh/themes/modus_vivendi"), /themes-copyleft/u);
  assert.throws(() => require("@lumis-sh/themes/modus_vivendi"), /themes-copyleft/u);
}

console.log(`${mode}: production consumer checks passed (${Object.keys(formats).join(", ")})`);
