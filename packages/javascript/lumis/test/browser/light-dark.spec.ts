/**
 * What a browser does with the `light-dark()` spans, rather than what they look
 * like as a string: invalid declarations are dropped on parse, so a wrong one
 * shows up here as a computed style that never changes.
 */
import { expect, test, type Page } from "@playwright/test";

interface Computed {
  /** Empty where the browser refused the inline declaration. */
  inlineFontWeight: string;
  color: string;
  fontStyle: string;
  fontWeight: string;
  textDecorationLine: string;
}

async function computed(page: Page, scope: string): Promise<Computed> {
  return page.locator(`[data-testid="${scope}"]`).evaluate((element) => {
    const style = getComputedStyle(element);
    return {
      inlineFontWeight: (element as HTMLElement).style.fontWeight,
      color: style.color,
      fontStyle: style.fontStyle,
      fontWeight: style.fontWeight,
      textDecorationLine: style.textDecorationLine,
    };
  });
}

test.describe("light-dark() spans", () => {
  test("keep the emphasis both themes ask for in either color scheme", async ({ page }) => {
    await page.goto("/light-dark.html");

    await page.emulateMedia({ colorScheme: "light" });
    let keyword = await computed(page, "keyword");
    expect(keyword.inlineFontWeight).toBe("bold");
    expect(keyword.fontWeight).toBe("700");
    expect(keyword.color).toBe("rgb(215, 58, 73)");

    await page.emulateMedia({ colorScheme: "dark" });
    keyword = await computed(page, "keyword");
    expect(keyword.fontWeight).toBe("700");
    expect(keyword.color).toBe("rgb(255, 123, 114)");
  });

  test("switch a property the two themes disagree on through the variables", async ({ page }) => {
    await page.goto("/light-dark.html");

    await page.emulateMedia({ colorScheme: "light" });
    expect((await computed(page, "comment")).fontStyle).toBe("italic");
    expect((await computed(page, "string")).textDecorationLine).toBe("underline");

    await page.emulateMedia({ colorScheme: "dark" });
    expect((await computed(page, "comment")).fontStyle).toBe("normal");
    expect((await computed(page, "string")).textDecorationLine).toBe("line-through");
  });

  test("leave a scope neither theme emphasises unstyled", async ({ page }) => {
    await page.goto("/light-dark.html");

    for (const colorScheme of ["light", "dark"] as const) {
      await page.emulateMedia({ colorScheme });
      const variable = await computed(page, "variable");

      expect(variable.fontWeight).toBe("400");
      expect(variable.fontStyle).toBe("normal");
      expect(variable.textDecorationLine).toBe("none");
    }
  });
});
