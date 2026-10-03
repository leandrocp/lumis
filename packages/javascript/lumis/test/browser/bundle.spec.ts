import { expect, test } from "@playwright/test";

test("the built browser package highlights with a CDN process polyfill", async ({ page }) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  await page.addInitScript(() => {
    // esm.sh's process polyfill reports a Node version in the browser.
    Object.defineProperty(globalThis, "process", {
      value: { versions: { node: "22.0.0" }, env: {}, argv: [] },
      configurable: true,
    });
  });

  await page.goto("/bundle.html");
  await expect(page.locator("#output .l-line")).toHaveText('{"answer": 42}');
  await expect(page.locator("#output span[style]").first()).toBeVisible();
  expect(errors).toEqual([]);
});
