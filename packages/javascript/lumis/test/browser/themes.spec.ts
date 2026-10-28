import { readFileSync } from "node:fs";
import { expect, test } from "@playwright/test";

const cases: { expected: string | null }[] = JSON.parse(
  readFileSync(new URL("../../../../../fixtures/theme-choice-cases.json", import.meta.url), "utf8"),
);

test("the published themes module matches the shared corpus in a browser", async ({ page }) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  expect(cases.length).toBeGreaterThanOrEqual(20);
  await page.goto("/themes.html");
  await expect(page.locator("#output")).toHaveText(
    JSON.stringify(cases.map((fixture) => fixture.expected)),
  );
  expect(errors).toEqual([]);
});

test("mapped themes preserve emphasis while removing code-fence italics in both HTML forms", async ({
  page,
}) => {
  await page.goto("/themes.html");
  for (const [name, raw, emphasis] of [
    ["original", "italic", "italic"],
    ["selective", "normal", "italic"],
    ["upright", "normal", "normal"],
  ]) {
    await expect(page.locator(`#${name} .linked .l-markup-raw-block`)).toHaveCSS("font-style", raw);
    await expect(page.locator(`#${name} .linked .l-markup-italic`)).toHaveCSS(
      "font-style",
      emphasis,
    );
    await expect(page.locator(`#${name} .inline [data-highlight="markup.raw.block"]`)).toHaveCSS(
      "font-style",
      raw,
    );
    await expect(page.locator(`#${name} .inline [data-highlight="markup.italic"]`)).toHaveCSS(
      "font-style",
      emphasis,
    );
  }
});
