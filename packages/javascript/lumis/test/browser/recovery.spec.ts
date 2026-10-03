import { expect, test } from "@playwright/test";
import { recoveryLanguages } from "../parser-recovery-fixture.js";
import type { RecoveryInput } from "./recovery.js";

for (const entry of ["source", "bundle"]) {
  test(`recovers automatically after real Wasm memory corruption (${entry})`, async ({ page }) => {
    test.setTimeout(60_000);
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    const inputs: RecoveryInput[] = recoveryLanguages().map((language) => {
      if (!(language.wasm instanceof Uint8Array)) throw new Error("Expected fixture bytes");
      return {
        id: language.id,
        aliases: language.aliases,
        packageName: language.packageName,
        languagePackage: language.languagePackage,
        wasm: Array.from(language.wasm),
      };
    });
    await page.goto(entry === "bundle" ? "/recovery.html?bundle" : "/recovery.html");
    await page.waitForFunction(() => typeof window.runParserRecovery === "function");
    const result = await page.evaluate((fixtures) => window.runParserRecovery(fixtures), inputs);
    expect(result.largeParserBytes).toBeGreaterThan(8 * 1024 * 1024);
    expect(result.failures).toHaveLength(4);
    expect(result.recovered).toBe(20);
    expect(result.baseline).toContain('class="l-string');
    expect(result.markdown).toContain('class="l-number');
    expect(errors).toEqual([]);
  });
}
