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
      await page.setContent(
        `<style>pre { font: 16px/20px monospace; width: 300px; overflow-x: auto; margin: 0; } code { font: inherit; } ${css}</style>${html}`,
      );
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

        await page.setContent(
          `<style>pre { font: 16px/20px monospace; width: 300px; overflow-x: auto; margin: 0; } code { font: inherit; } ${css}</style>${html}`,
        );
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
