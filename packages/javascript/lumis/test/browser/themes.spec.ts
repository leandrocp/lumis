import { readFileSync } from "node:fs";
import { expect, test } from "@playwright/test";
import { readThemes } from "../../../themes/scripts/read-themes.js";

const cases: { expected: string | null; expected_default?: string | null }[] = JSON.parse(
  readFileSync(new URL("../../../../../fixtures/theme-choice-cases.json", import.meta.url), "utf8"),
);

test("the published themes module matches the shared corpus in a browser", async ({ page }) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  expect(cases.length).toBeGreaterThanOrEqual(20);
  await page.goto("/themes.html");
  await expect(page.locator("#output")).toHaveText(
    JSON.stringify(cases.map((fixture) => fixture.expected_default ?? fixture.expected)),
  );
  await expect(page.locator("#output")).toHaveAttribute(
    "data-theme-names",
    JSON.stringify(readThemes().map(({ name }) => name)),
  );
  await expect(page.locator("#output")).toHaveAttribute("data-optional-lookup", "true");
  await expect(page.locator("#output")).toHaveAttribute("data-imported-optional", "modus_vivendi");
  expect(errors).toEqual([]);
});
