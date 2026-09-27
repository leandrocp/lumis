import { expect, test } from "@playwright/test";
import { HtmlValidate } from "html-validate";
import { buildCss } from "../../../themes/src/css.ts";
import { htmlInline, htmlLinked, htmlMultiThemes } from "../../src/formatters.ts";
import { formatHtmlInline } from "../../src/formatter/html-inline.ts";
import { formatHtmlLinked } from "../../src/formatter/html-linked.ts";
import { formatHtmlMultiThemes } from "../../src/formatter/html-multi-themes.ts";
import type { HighlightEvent, HighlightLinesInline, Theme } from "../../src/types.ts";

const theme = {
  name: "line-layout",
  appearance: "dark",
  highlights: {
    normal: { fg: "#eeeeee", bg: "#111111" },
    string: { fg: "#aabbcc" },
    highlighted: { bg: "#334455" },
    line_number: { fg: "#777777" },
  },
} satisfies Theme;

const layoutCss =
  "pre { font: 16px/20px monospace; width: 300px; overflow-x: auto; margin: 0; } code { font: inherit; }";

const cases = {
  basic: "a = 1\nb = 2\nc = 3",
  empty: "a = 1\n\nc = 3",
  trailing: "a = 1\nb = 2\n",
  trailingEmpty: "a = 1\n\n",
  onlyNewlines: "\n\n",
  crlf: "a = 1\r\nb = 2\r\nc = 3\r\n",
  long: `a = 1\nb = "${"long ".repeat(60)}"\nc = 3`,
  multilineString: 'a = """\nfirst\n\nlast\n"""',
  emptySource: "",
};

const validator = new HtmlValidate({ rules: { "element-permitted-content": "error" } });
const adjacentHighlights: Record<string, HighlightLinesInline> = {
  default: { lines: [1, 2] },
  noColor: { lines: [1, 2], style: null },
  custom: { lines: [1, 2], style: "background-color: red;", class: "selected" },
};

function render(source: string, lineNumbers: boolean, highlightLines?: HighlightLinesInline) {
  const events: HighlightEvent[] = [
    { type: "start", scope: "string", language: "plaintext" },
    { type: "source", start: 0, end: new TextEncoder().encode(source).length },
    { type: "end" },
  ];
  const options = {
    language: "plaintext",
    lineNumbers,
    highlightLines: highlightLines ?? { lines: [2], class: "l-highlighted" },
  };
  return {
    linked: { html: formatHtmlLinked(source, events, htmlLinked(options)), css: buildCss(theme) },
    inline: { html: formatHtmlInline(source, events, htmlInline({ ...options, theme })), css: "" },
    multi: {
      html: formatHtmlMultiThemes(
        source,
        events,
        htmlMultiThemes({ ...options, themes: { dark: theme }, defaultTheme: "dark" }),
      ),
      css: "",
    },
  };
}

for (const [name, highlightLines] of Object.entries(adjacentHighlights)) {
  for (const [formatter, { html, css }] of Object.entries(
    render(`short\n\n${"long ".repeat(60)}`, true, highlightLines),
  )) {
    test(`${formatter}: adjacent highlighted rows, ${name}`, async ({ page }) => {
      await page.setContent(`<style>${layoutCss} ${css}</style>${html}`);
      const result = await page.locator("code").evaluate((code) => {
        const rows = [...code.querySelectorAll<HTMLElement>(":scope > .l-line")];
        const pre = code.parentElement!;
        pre.scrollLeft = pre.scrollWidth;
        return {
          widths: rows.slice(0, 2).map((row) => row.getBoundingClientRect().width),
          heights: rows.slice(0, 2).map((row) => row.getBoundingClientRect().height),
          backgrounds: rows.slice(0, 2).map((row) => getComputedStyle(row).backgroundColor),
          classes: rows.slice(0, 2).map((row) => row.className),
          height: code.getBoundingClientRect().height,
          width: code.getBoundingClientRect().width,
          scrollWidth: pre.scrollWidth,
          scrolled: pre.scrollLeft,
        };
      });
      expect(result.scrolled).toBeGreaterThan(0);
      expect(result.height).toBeCloseTo(60, 0);
      for (const width of result.widths) {
        expect(width).toBeCloseTo(result.width, 0);
        expect(width).toBeGreaterThanOrEqual(result.scrollWidth - 1);
      }
      for (const height of result.heights) expect(height).toBeGreaterThanOrEqual(20);
      if (name === "noColor" && formatter !== "linked") {
        expect(result.backgrounds).toEqual(["rgba(0, 0, 0, 0)", "rgba(0, 0, 0, 0)"]);
      }
      if (name === "custom") {
        // Linked output intentionally leaves colors to the caller's stylesheet.
        if (formatter !== "linked") {
          expect(result.backgrounds).toEqual(["rgb(255, 0, 0)", "rgb(255, 0, 0)"]);
        }
        expect(result.classes).toEqual(["l-line selected", "l-line selected"]);
      }
    });
  }
}

for (const [name, source] of Object.entries(cases)) {
  for (const numbered of [false, true]) {
    for (const [formatter, { html, css }] of Object.entries(render(source, numbered))) {
      test(`${formatter}: ${name}, line numbers ${numbered}`, async ({ page, browserName }) => {
        const report = await validator.validateString(html);
        expect(report.results.flatMap((result) => result.messages)).toEqual([]);

        await page.setContent(`<style>${layoutCss} ${css}</style>${html}`);
        const lines = source.replaceAll("\r\n", "\n").split("\n");
        if (source.endsWith("\n")) lines.pop();
        const expected = lines.join("\n");
        const textWithGutters = lines
          .map((line, i) => `${numbered ? i + 1 : ""}${line}`)
          .join("\n");

        const actual = await page.locator("code").evaluate((code) => {
          const lineElements = [...code.querySelectorAll<HTMLElement>(":scope > .l-line")];
          const range = document.createRange();
          range.selectNodeContents(code);
          const selection = window.getSelection()!;
          selection.removeAllRanges();
          selection.addRange(range);
          const copy = selection.toString();
          const pre = code.parentElement!;
          pre.scrollLeft = pre.scrollWidth;
          const highlighted = code.querySelector<HTMLElement>(".l-highlighted");
          return {
            tags: lineElements.map((line) => line.tagName),
            innerText: (code as HTMLElement).innerText,
            textContent: code.textContent,
            copy,
            lineHeights: lineElements.map((line) => line.getBoundingClientRect().height),
            height: code.getBoundingClientRect().height,
            highlightedWidth: highlighted?.getBoundingClientRect().width,
            codeWidth: code.getBoundingClientRect().width,
            scrollWidth: pre.scrollWidth,
            scrolled: pre.scrollLeft,
            separators: [...code.childNodes]
              .filter((node) => node.nodeType === Node.TEXT_NODE)
              .map((node) => node.textContent),
            lastNodeIsLine: code.lastChild === lineElements.at(-1),
          };
        });

        expect(actual.tags).toEqual(lines.map(() => "SPAN"));
        expect(actual.separators).toEqual(lines.slice(1).map(() => "\n"));
        expect(actual.lastNodeIsLine).toBe(true);
        expect(actual.innerText).toBe(textWithGutters);
        expect(actual.textContent).toBe(textWithGutters);
        // Chromium ends a selection at the last selectable text when the final
        // rows contain only unselectable gutters. Source-backed copy avoids this.
        const copied =
          browserName === "chromium" && numbered ? expected.replace(/\n+$/, "") : expected;
        expect(actual.copy).toBe(copied);
        if (source !== "") expect(actual.height).toBeCloseTo(lines.length * 20, 0);
        if (name === "empty") expect(actual.lineHeights[1]).toBeGreaterThanOrEqual(20);
        if (name === "long") {
          expect(actual.scrolled).toBeGreaterThan(0);
          expect(actual.highlightedWidth).toBeCloseTo(actual.codeWidth, 0);
          expect(actual.highlightedWidth).toBeGreaterThanOrEqual(actual.scrollWidth - 1);
        }
      });
    }
  }
}

test.describe("theme layout regressions", () => {
  const wrappingCases = {
    words: "long ".repeat(60).trimEnd(),
    token: "a".repeat(300),
  };

  for (const [name, longLine] of Object.entries(wrappingCases)) {
    const source = `short\n\n${longLine}`;
    for (const numbered of [false, true]) {
      for (const [formatter, { html, css }] of Object.entries(
        render(source, numbered, { lines: [1, 2, 3] }),
      )) {
        test(`${formatter}: pre-wrap ${name}, line numbers ${numbered}`, async ({ page }) => {
          await page.setContent(
            `<style>${layoutCss} pre { white-space: pre-wrap; } ${css}</style>${html}`,
          );
          const result = await page.locator("pre").evaluate((pre) => {
            const code = pre.querySelector("code")!;
            return {
              width: pre.clientWidth,
              scrollWidth: pre.scrollWidth,
              codeWidth: code.getBoundingClientRect().width,
              height: code.getBoundingClientRect().height,
              innerText: code.innerText,
              textContent: code.textContent,
              rows: [...code.querySelectorAll<HTMLElement>(":scope > .l-line")].map((row) => {
                const rect = row.getBoundingClientRect();
                return {
                  height: rect.height,
                  width: rect.width,
                  top: rect.top,
                  bottom: rect.bottom,
                };
              }),
            };
          });
          const expectedText = source
            .split("\n")
            .map((line, index) => `${numbered ? index + 1 : ""}${line}`)
            .join("\n");

          expect(result.rows).toHaveLength(3);
          expect(result.innerText).toBe(expectedText);
          expect(result.textContent).toBe(expectedText);
          expect(result.scrollWidth).toBeLessThanOrEqual(result.width + 1);
          expect(result.codeWidth).toBeCloseTo(result.width, 0);
          expect(result.rows[0].height).toBeCloseTo(20, 0);
          expect(result.rows[1].height).toBeCloseTo(20, 0);
          expect(result.rows[2].height).toBeGreaterThan(20);
          for (const row of result.rows) expect(row.width).toBeCloseTo(result.width, 0);
          expect(result.rows[1].top).toBeCloseTo(result.rows[0].bottom, 0);
          expect(result.rows[2].top).toBeCloseTo(result.rows[1].bottom, 0);
          expect(result.height).toBeCloseTo(
            result.rows.reduce((total, row) => total + row.height, 0),
            0,
          );
        });
      }
    }
  }

  test("legacy div lines retain their block layout", async ({ page }) => {
    // Deliberately keep the invalid legacy markup; new CSS must not reflow it.
    const html =
      '<pre class="lumis"><code><div class="l-line">one\n</div>' +
      '<div class="l-line l-highlighted">two\n</div>' +
      '<div class="l-line">three</div></code></pre>';
    await page.setContent(`<style>${layoutCss} ${buildCss(theme)}</style>${html}`);
    const result = await page.locator("pre").evaluate((pre) => ({
      height: pre.getBoundingClientRect().height,
      text: pre.textContent,
      rows: [...pre.querySelectorAll("code > .l-line")].map((row) => ({
        tag: row.tagName,
        display: getComputedStyle(row).display,
        height: row.getBoundingClientRect().height,
      })),
    }));

    expect(result.text).toBe("one\ntwo\nthree");
    expect(result.height).toBeCloseTo(60, 0);
    expect(result.rows).toEqual([
      { tag: "DIV", display: "block", height: 20 },
      { tag: "DIV", display: "block", height: 20 },
      { tag: "DIV", display: "block", height: 20 },
    ]);
  });

  test("line spans in inline code do not become block rows", async ({ page }) => {
    await page.setContent(
      `<style>${layoutCss} p { font: 16px/20px monospace; width: 300px; } ${buildCss(theme)}</style><p>before <code><span class="l-line">inline</span></code> after</p>`,
    );
    await expect(page.locator("code")).toHaveCSS("display", "inline");
    await expect(page.locator(".l-line")).toHaveCSS("display", "inline");
    const result = await page.locator("p").evaluate((paragraph) => ({
      height: paragraph.getBoundingClientRect().height,
      text: paragraph.textContent,
    }));
    expect(result.height).toBeCloseTo(20, 0);
    expect(result.text).toBe("before inline after");
  });

  for (const position of ["before", "after"]) {
    test(`unlayered CSS overrides layout ${position} the theme`, async ({ page }) => {
      const { html, css } = render(cases.basic, false).linked;
      const override = ".l-line { min-height: 30px; }";
      const styles = position === "before" ? `${override} ${css}` : `${css} ${override}`;
      await page.setContent(`<style>${layoutCss} ${styles}</style>${html}`);
      const rows = page.locator("code > .l-line");
      await expect(rows).toHaveCount(3);
      for (const row of await rows.all()) await expect(row).toHaveCSS("min-height", "30px");
      const height = await page
        .locator("code")
        .evaluate((code) => code.getBoundingClientRect().height);
      expect(height).toBeCloseTo(90, 0);
    });
  }

  for (const [order, minHeight] of [
    ["lumis, utilities", 32],
    ["utilities, lumis", 20],
  ] as const) {
    test(`utility layer follows the declared order: ${order}`, async ({ page }) => {
      const { html, css } = render(cases.basic, false, {
        lines: [2],
        class: "min-h-8",
      }).linked;
      await page.setContent(
        `<style>@layer ${order}; ${layoutCss} @layer utilities { .min-h-8 { min-height: 32px; } } ${css}</style>${html}`,
      );
      await expect(page.locator("code > .min-h-8")).toHaveCSS("min-height", `${minHeight}px`);
      const height = await page
        .locator("code")
        .evaluate((code) => code.getBoundingClientRect().height);
      expect(height).toBeCloseTo(40 + minHeight, 0);
    });
  }
});
