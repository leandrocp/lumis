import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { test } from "node:test";
import * as esm from "@lumis-sh/lumis/themes";
import * as browser from "../dist/themes.browser.js";

const cjs = createRequire(import.meta.url)("@lumis-sh/lumis/themes");
const cases = JSON.parse(
  readFileSync(new URL("../../../../fixtures/theme-choice-cases.json", import.meta.url), "utf8"),
);

await test("the published browser entry matches the default-pack corpus", () => {
  assert.ok(cases.length >= 20);
  for (const fixture of cases) {
    assert.equal(
      browser.closest(fixture)?.name ?? null,
      fixture.expected_default ?? fixture.expected,
      fixture.name,
    );
  }
});

for (const [format, themes] of Object.entries({ esm, cjs })) {
  await test(`${format} exposes synchronous installed-theme matching`, () => {
    const full = themes.available().length > browser.available().length;
    for (const fixture of cases) {
      const expected = full ? fixture.expected : (fixture.expected_default ?? fixture.expected);
      assert.equal(themes.closest(fixture)?.name ?? null, expected, fixture.name);
    }
    assert.equal(themes.closest({}), undefined);
    assert.ok(themes.available().some((theme) => theme.name === "dracula"));
  });
}
