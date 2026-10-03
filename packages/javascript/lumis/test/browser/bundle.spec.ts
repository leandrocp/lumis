import { readFileSync } from "node:fs";
import { isBuiltin } from "node:module";
import { expect, test } from "@playwright/test";
import ts from "typescript";

test("emitted browser modules do not import Node builtins", () => {
  const pending = [new URL("../../dist/index.browser.js", import.meta.url)];
  const visited = new Set<string>();

  for (const file of pending) {
    if (visited.has(file.href)) continue;
    visited.add(file.href);

    const { importedFiles } = ts.preProcessFile(readFileSync(file, "utf8"), true, true);
    for (const { fileName: specifier } of importedFiles) {
      expect(
        specifier.startsWith("node:") || isBuiltin(specifier),
        `${file.pathname} imports ${specifier}`,
      ).toBe(false);
      if (specifier.startsWith(".")) pending.push(new URL(specifier, file));
    }
  }

  expect([...visited].some((file) => file.includes("/web-tree-sitter-"))).toBe(true);
});

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
