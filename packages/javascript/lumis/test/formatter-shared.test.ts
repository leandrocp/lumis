import { describe, expect, it } from "vitest";
import type { Theme } from "../src/types.js";
import {
  attrsToString,
  closeCodeTag,
  closePreTag,
  closeTag,
  closingTags,
  escape,
  escapeAttr,
  escapeBraces,
  escapeFragment,
  formatHighlightIterLines,
  getScopedThemeStyle,
  getThemeStyle,
  joinClasses,
  linesFromOffsets,
  lineIsHighlighted,
  openCodeTag,
  openPreTag,
  openSpanTag,
  openTag,
  renderEvents,
  renderLinesFromEvents,
  scopeToClass,
  spanInline,
  spanInlineAttrs,
  spanLinked,
  spanLinkedAttrs,
  spanMultiThemes,
  spanMultiThemesAttrs,
  styleToCss,
  textDecoration,
  wrapLine,
  wrapWithHeader,
} from "../src/formatter/html.js";
import { sanitizeThemeName } from "../src/themes.js";
import { hexToRgb, paint, rgbToAnsi, styleToAnsi, wrapWithAnsi } from "../src/formatter/ansi.js";
import type { Language } from "../src/types.js";

const jsonLang: Language = {
  id: "json",
  aliases: [],
  highlights: "",
  wasm: "json.wasm",
};

const theme = {
  name: "test",
  appearance: "dark",
  highlights: {
    normal: { fg: "#ffffff", bg: "#000000" },
    string: { fg: "#00ff00" },
    "string.json": { fg: "#22ff22", italic: true },
    highlighted: { bg: "#222222" },
    emphasis: { underline: "double", strikethrough: true },
  },
} satisfies Theme;

describe("formatter shared helpers", () => {
  it("falls back from specific scopes to parent scopes", () => {
    expect(getThemeStyle(theme, "string.special.regex")).toEqual({ fg: "#00ff00" });
  });

  it("prefers language-specific styles when present", () => {
    expect(getScopedThemeStyle(theme, "string", "json")).toEqual({
      fg: "#22ff22",
      italic: true,
    });
  });

  it("formats CSS declarations with optional italic handling", () => {
    expect(styleToCss({ fg: "#fff", italic: true }, { italic: false })).toBe("color: #fff;");
    expect(styleToCss({ fg: "#fff", italic: true }, { italic: true })).toBe(
      "color: #fff; font-style: italic;",
    );
  });

  it("renders text decoration combinations", () => {
    expect(textDecoration({ underline: "double", strikethrough: true })).toBe(
      "underline double line-through",
    );
    expect(textDecoration({})).toBe("none");
  });

  it("escapes HTML entities without touching braces", () => {
    expect(escape(`<tag attr='x'>&{"y"}`)).toBe("&lt;tag attr=&#39;x&#39;&gt;&amp;{&quot;y&quot;}");
  });

  it("escapes HTML attributes without touching braces", () => {
    expect(escapeAttr(`"quoted" & <tag> {'x'}`)).toBe(
      "&quot;quoted&quot; &amp; &lt;tag&gt; {&#39;x&#39;}",
    );
  });

  it("renders HTML attrs and skips falsey non-string values", () => {
    expect(
      attrsToString({
        class: "lumis code",
        tabindex: 0,
        hidden: false,
        inert: true,
        style: undefined,
      }),
    ).toBe('class="lumis code" tabindex="0" inert');
  });

  it("joins classes and drops empty values", () => {
    expect(joinClasses("lumis", undefined, false, "custom")).toBe("lumis custom");
    expect(joinClasses(undefined, false, null)).toBeUndefined();
  });

  it("opens and closes tags with escaped attrs", () => {
    expect(openTag("span", { title: `"quoted"` })).toBe('<span title="&quot;quoted&quot;">');
    expect(closeTag("span")).toBe("</span>");
    expect(closePreTag()).toBe("</pre>");
    expect(closeCodeTag()).toBe("</code>");
    expect(closingTags()).toBe("</code></pre>");
    expect(openSpanTag({ class: "token" })).toBe('<span class="token">');
    expect(openSpanTag()).toBe("<span>");
    expect(openSpanTag({ class: undefined })).toBe("<span>");
    expect(openPreTag()).toBe('<pre class="lumis">');
    expect(openPreTag({ preClass: "custom" })).toBe('<pre class="lumis custom">');
    expect(openPreTag({ theme })).toBe(
      '<pre class="lumis" style="color: #ffffff; background-color: #000000;">',
    );
    expect(openPreTag({ preClass: "custom", theme })).toBe(
      '<pre class="lumis custom" style="color: #ffffff; background-color: #000000;">',
    );
    expect(openCodeTag({ id: "json", aliases: [] })).toBe(
      '<code class="language-json" translate="no" tabindex="0">',
    );
  });

  it("wraps highlighted output with an optional header", () => {
    expect(
      wrapWithHeader("<pre></pre>", {
        openTag: '<section class="wrapper">',
        closeTag: "</section>",
      }),
    ).toBe('<section class="wrapper"><pre></pre></section>');
    expect(wrapWithHeader("<pre></pre>")).toBe("<pre></pre>");
  });

  it("turns scopes into linked formatter classes", () => {
    expect(scopeToClass("keyword.operator")).toBe("l-keyword-operator");
    expect(scopeToClass("unknown.scope.name")).toBe("l-text");
  });

  it("matches single lines and inclusive ranges", () => {
    expect(lineIsHighlighted([1, [3, 5]], 1)).toBe(true);
    expect(lineIsHighlighted([1, [3, 5]], 4)).toBe(true);
    expect(lineIsHighlighted([1, [3, 5]], 6)).toBe(false);
  });

  it("wraps lines with optional class and style", () => {
    expect(wrapLine(2, "code", { className: "highlighted", style: "color: red;" })).toBe(
      '<div class="l-line highlighted" style="color: red;" data-line="2">code\n</div>',
    );
  });

  it("renders highlight events into line fragments", () => {
    const { lines, language } = formatHighlightIterLines(
      "a\nb",
      [
        { type: "start", scope: "string", language: "json" },
        { type: "source", start: 0, end: 1 },
        { type: "end" },
        { type: "source", start: 1, end: 2 },
        { type: "start", scope: "number", language: "json" },
        { type: "source", start: 2, end: 3 },
        { type: "end" },
      ],
      jsonLang,
      undefined,
      {
        openSpan: (span) => `<span data-scope="${span.scope}">`,
      },
    );

    expect(language).toBe("json");
    expect(lines).toEqual([
      '<span data-scope="string">a</span>',
      '<span data-scope="number">b</span>',
    ]);
  });

  it("splits multiline source events into separate rendered lines", () => {
    const { lines } = formatHighlightIterLines(
      "ab\ncd",
      [
        { type: "start", scope: "string", language: "json" },
        { type: "source", start: 0, end: 5 },
        { type: "end" },
      ],
      jsonLang,
      undefined,
      {
        openSpan: (span) => `<span data-scope="${span.scope}">`,
      },
    );

    expect(lines).toEqual([
      '<span data-scope="string">ab</span>',
      '<span data-scope="string">cd</span>',
    ]);
  });

  it("keeps a deliberately omitted custom span balanced", () => {
    const { lines } = formatHighlightIterLines(
      "a\nb",
      [
        { type: "start", scope: "string", language: "json" },
        { type: "source", start: 0, end: 3 },
        { type: "end" },
      ],
      jsonLang,
      undefined,
      { openSpan: () => "" },
    );

    expect(lines).toEqual(["a", "b"]);
  });

  it("escapes braces only through the framework helper", () => {
    expect(escapeBraces("fn() {}")).toBe("fn() &lbrace;&rbrace;");
    expect(escapeBraces("<div>{x}</div>")).toBe("<div>&lbrace;x&rbrace;</div>");
    expect(escapeFragment("<div>{x}</div>")).toBe("&lt;div&gt;{x}&lt;/div&gt;");
  });

  it("renders HTML lines from events with span attrs callback", () => {
    const lines = renderLinesFromEvents(
      "a\nb",
      [
        { type: "start", scope: "string", language: "json" },
        { type: "source", start: 0, end: 3 },
        { type: "end" },
      ],
      (scope) => `class="${scope}"`,
    );

    expect(lines).toEqual(['<span class="string">a</span>', '<span class="string">b</span>']);
  });

  it("shrinks source ranges that split a UTF-8 character", () => {
    expect(renderLinesFromEvents("éx", [{ type: "source", start: 0, end: 1 }], () => "")).toEqual([
      "",
    ]);
    expect(renderLinesFromEvents("éx", [{ type: "source", start: 1, end: 3 }], () => "")).toEqual([
      "x",
    ]);
  });

  it("opens a bare span for a scope whose attrs come out empty", () => {
    const lines = renderLinesFromEvents(
      "ab",
      [
        { type: "start", scope: "string", language: "json" },
        { type: "source", start: 0, end: 1 },
        { type: "start", scope: "unstyled", language: "json" },
        { type: "source", start: 1, end: 2 },
        { type: "end" },
        { type: "end" },
      ],
      (scope) => (scope === "unstyled" ? "" : `class="${scope}"`),
    );

    expect(lines).toEqual(['<span class="string">a<span>b</span></span>']);
  });

  it("reopens a bare span across a newline", () => {
    const lines = renderLinesFromEvents(
      "a\nb",
      [
        { type: "start", scope: "unstyled", language: "json" },
        { type: "source", start: 0, end: 3 },
        { type: "end" },
      ],
      () => "",
    );

    expect(lines).toEqual(["<span>a</span>", "<span>b</span>"]);
  });

  it("opens a bare span in renderEvents when the callback writes nothing", () => {
    const [html] = renderEvents(
      "ab",
      [
        { type: "start", scope: "unstyled", language: "json" },
        { type: "source", start: 0, end: 2 },
        { type: "end" },
      ],
      () => {},
    );

    expect(new TextDecoder().decode(html)).toBe("<span>ab</span>");
  });

  it("renders event HTML and slices it back into lines", () => {
    const [html, offsets] = renderEvents(
      "a\n<b>",
      [
        { type: "start", scope: "string", language: "json" },
        { type: "source", start: 0, end: 5 },
        { type: "end" },
      ],
      (scope, _language, out) => {
        out.push(`class="${scope}"`);
      },
    );

    expect(new TextDecoder().decode(html)).toBe('<span class="string">a\n&lt;b&gt;</span>');
    expect(linesFromOffsets(html, offsets)).toEqual([
      '<span class="string">a\n',
      "&lt;b&gt;</span>",
    ]);
  });

  it("sanitizes theme names for CSS variable use", () => {
    expect(sanitizeThemeName("dracula")).toBe("dracula");
    expect(sanitizeThemeName("one-dark")).toBe("one-dark");
    expect(sanitizeThemeName("my theme!")).toBe("my-theme-");
    expect(sanitizeThemeName("theme_v2")).toBe("theme_v2");
    expect(sanitizeThemeName("Rosé Pine (Dawn)")).toBe("Rosé-Pine--Dawn-");
  });

  it("generates span inline attrs with theme styling", () => {
    const attrs = spanInlineAttrs({
      scope: "string",
      language: "json",
      theme,
      includeHighlights: true,
    });
    expect(attrs["data-highlight"]).toBe("string");
    expect(attrs.style).toContain("color: #22ff22");
  });

  it("generates span inline without highlights when disabled", () => {
    const attrs = spanInlineAttrs({ scope: "string", language: "json", theme });
    expect(attrs["data-highlight"]).toBeUndefined();
    expect(attrs.style).toContain("color: #22ff22");
  });

  it("wraps text in inline styled span", () => {
    const html = spanInline("hello", { language: "json", scope: "string", theme });
    expect(html).toContain("<span");
    expect(html).toContain("hello");
    expect(html).toContain("#22ff22");
  });

  it("opens a bare span around escaped text when no style matches", () => {
    const html = spanInline("<b>", { language: "json", scope: "string" });
    expect(html).toBe("<span>&lt;b&gt;</span>");
  });

  it("generates linked span attrs", () => {
    expect(spanLinkedAttrs("keyword.operator")).toBe('class="l-keyword-operator"');
  });

  it("wraps text in linked span", () => {
    expect(spanLinked("if", "keyword.conditional")).toBe(
      '<span class="l-keyword-conditional">if</span>',
    );
  });

  it("generates multi-themes span attrs with default theme", () => {
    const themes = { dracula: theme };
    const attrs = spanMultiThemesAttrs({
      scope: "string",
      language: "json",
      themes,
      defaultTheme: "dracula",
    });
    expect(attrs.style).toContain("#22ff22");
  });

  it("wraps text in multi-themes span", () => {
    const themes = { dracula: theme };
    const html = spanMultiThemes("x", {
      scope: "string",
      language: "json",
      themes,
      defaultTheme: "dracula",
    });
    expect(html).toContain("<span");
    expect(html).toContain("x");
  });

  it("opens a bare multi-themes span with empty themes", () => {
    const html = spanMultiThemes("<b>", { scope: "string", language: "json", themes: {} });
    expect(html).toBe("<span>&lt;b&gt;</span>");
  });
});

describe("ansi helpers", () => {
  it("converts hex to rgb", () => {
    expect(hexToRgb("#ff5555")).toEqual([255, 85, 85]);
    expect(hexToRgb("ff5555")).toEqual([255, 85, 85]);
    expect(hexToRgb("#fff")).toBeUndefined();
    expect(hexToRgb("invalid")).toBeUndefined();
  });

  it("generates foreground and background ANSI codes", () => {
    expect(rgbToAnsi(255, 85, 85, false)).toBe("\u001B[38;2;255;85;85m");
    expect(rgbToAnsi(40, 42, 54, true)).toBe("\u001B[48;2;40;42;54m");
  });

  it("converts style to ANSI codes", () => {
    expect(styleToAnsi({ bold: true })).toBe("\u001B[1m");
    expect(styleToAnsi({ fg: "#8be9fd" })).toContain("\u001B[38;2;");
    expect(styleToAnsi()).toBe("");
  });

  it("renders text with ANSI codes", () => {
    expect(paint("text")).toBe("text");
    const result = paint("fn", { fg: "#8be9fd" });
    expect(result).toContain("\u001B[0m");
    expect(result).toContain("fn");
  });

  it("keeps wrapWithAnsi aligned with paint", () => {
    expect(wrapWithAnsi("fn", { fg: "#8be9fd" })).toBe(paint("fn", { fg: "#8be9fd" }));
  });
});
