import { expect, test } from "@playwright/test";
import type { LargeParserOutcome } from "./large-parser.js";

test("loads a parser over 8 MB on the main thread", async ({ page }) => {
  await page.goto("/large-parser.html");
  await page.waitForFunction(() => window.lumisLargeParser !== undefined);
  const outcome: LargeParserOutcome | undefined = await page.evaluate(
    () => window.lumisLargeParser,
  );

  expect(outcome?.error).toBeUndefined();
  expect(outcome?.bytes).toBeGreaterThan(8 * 1024 * 1024);
  expect(outcome?.html).toContain('class="l-number"');
});
