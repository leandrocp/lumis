import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { test } from "node:test";
import * as esm from "@lumis-sh/lumis/themes";

const cjs = createRequire(import.meta.url)("@lumis-sh/lumis/themes");
const cases = JSON.parse(
  readFileSync(new URL("../../../../fixtures/theme-choice-cases.json", import.meta.url), "utf8"),
);

for (const [format, themes] of Object.entries({ esm, cjs })) {
  await test(`${format} maps styles through the published themes entry point`, () => {
    const original = themes.get("github_light");
    const mapped = themes.mapStyles(original, (_scope, style) => ({ ...style, italic: false }));
    assert.ok(Object.values(original.highlights).some((style) => style.italic));
    assert.ok(Object.values(mapped.highlights).every((style) => style.italic === false));
    assert.equal(mapped.name, original.name);
  });

  await test(`${format} exposes closest through the published themes entry point`, () => {
    assert.ok(cases.length >= 20);
    for (const fixture of cases) {
      assert.equal(themes.closest(fixture)?.name ?? null, fixture.expected, fixture.name);
    }
  });
}
