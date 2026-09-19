import { readdirSync, readFileSync } from "node:fs";
import { expect, test, type BrowserContext, type Page } from "@playwright/test";
import type { BrowserTestResult, FixtureOutput } from "./smoke.js";

const conformanceDir = new URL("../../../../../fixtures/conformance/", import.meta.url);
const CUSTOM_FORMATTER_FIXTURE = "javascript-html-template-nested-script-css";

function readFixture(fixture: string, name: string): string {
  return readFileSync(new URL(`${fixture}/${name}`, conformanceDir), "utf8");
}

function expectedOutput(fixture: string): FixtureOutput {
  return {
    bbcodeScoped: readFixture(fixture, "bbcode.txt"),
    htmlInline: readFixture(fixture, "html-inline.html"),
    htmlLinked: readFixture(fixture, "html-linked.html"),
    htmlMultiThemes: readFixture(fixture, "html-multi-themes.html"),
    terminal: readFixture(fixture, "terminal.txt"),
  };
}

/** The whole corpus, so the browser is held to what every other runtime is. */
const fixtureNames = readdirSync(conformanceDir, { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => entry.name)
  .sort();

const fixtureSource = readFixture(CUSTOM_FORMATTER_FIXTURE, "source.txt");
const customSource = `const nested = foo(bar([1, 2], { a: "3" }));\nconst view = ${fixtureSource.trim()};\n`;

test.describe("browser runtime", () => {
  let context: BrowserContext;
  let page: Page;
  let result: BrowserTestResult;
  let reloadedResult: BrowserTestResult;
  const externalRequests: string[] = [];
  const pageErrors: string[] = [];

  test.beforeAll(async ({ browser }) => {
    context = await browser.newContext();
    page = await context.newPage();
    page.on("request", (request) => {
      if (!request.url().startsWith("http://127.0.0.1:4173/")) {
        externalRequests.push(request.url());
      }
    });
    page.on("pageerror", (error) => pageErrors.push(String(error)));

    await page.goto("/");
    await page.waitForFunction(
      () => window.__lumisBrowserResult !== undefined || window.__lumisBrowserError !== undefined,
    );

    const outcome = await page.evaluate(() => ({
      error: window.__lumisBrowserError,
      result: window.__lumisBrowserResult,
    }));

    expect(outcome.error).toBeUndefined();
    result = outcome.result as BrowserTestResult;

    await page.reload();
    await page.waitForFunction(
      () => window.__lumisBrowserResult !== undefined || window.__lumisBrowserError !== undefined,
    );
    const reloadedOutcome = await page.evaluate(() => ({
      error: window.__lumisBrowserError,
      result: window.__lumisBrowserResult,
    }));
    expect(reloadedOutcome.error).toBeUndefined();
    reloadedResult = reloadedOutcome.result as BrowserTestResult;
  });

  test.afterAll(async () => {
    await context.close();
  });

  test("loads every parser locally without browser errors", () => {
    expect(pageErrors).toEqual([]);
    expect(externalRequests).toEqual([]);
    expect(result.languages).toEqual(
      expect.arrayContaining(["plaintext", "javascript", "html", "css"]),
    );
    expect(result.requestedWasms).toEqual(
      expect.arrayContaining(["tree-sitter-javascript", "tree-sitter-html", "tree-sitter-css"]),
    );
  });

  test("reuses verified parser bytes after a page reload", () => {
    expect(reloadedResult.requestedWasms).toEqual([]);
  });

  test("renders every conformance fixture exactly as the other runtimes do", () => {
    // A discovery bug that found nothing would otherwise pass silently.
    expect(fixtureNames.length).toBeGreaterThan(20);
    expect(Object.keys(result.fixtures).sort()).toEqual(fixtureNames);

    for (const fixture of fixtureNames) {
      expect(result.fixtures[fixture], fixture).toEqual(expectedOutput(fixture));
    }
  });

  test("supports a stateful custom formatter", () => {
    const custom = result.customFormatter;

    expect(custom.resolvedLanguage).toBe("javascript");
    expect(custom.restoredLanguage).toBe("js");
    expect(custom.balancedEvents).toBe(true);
    expect(custom.reconstructedSource).toBe(customSource);
    expect(custom.eventLanguages).toEqual(["css", "html", "javascript"]);
    expect(custom.tokenLanguages).toEqual(["css", "html", "javascript"]);
    expect(custom.maxDepth).toBeGreaterThan(1);
    expect(custom.eventCount).toBeGreaterThan(custom.tokenCount);
    expect(custom.tokenCount).toBeGreaterThan(20);
    expect(custom.styledTokenCount).toBeGreaterThan(20);
    expect(custom.rainbowDepths).toContain(0);
    expect(custom.tokenScopes).toContain("punctuation.bracket");
    expect(custom.unicodeToken).toMatchObject({
      text: '"😀"',
      language: "css",
      scope: "string",
    });
    expect(custom.unicodeToken!.endByte - custom.unicodeToken!.startByte).toBeGreaterThan(
      custom.unicodeToken!.text.length,
    );
  });
});
