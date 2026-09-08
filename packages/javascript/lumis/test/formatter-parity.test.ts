/**
 * Formatter options JavaScript used to be missing or to interpret differently
 * from Rust. Each case here corresponds to an entry in `API_DRIFT.md`.
 *
 * The output these produce is pinned against the other runtimes by
 * `fixtures/conformance`. What this file pins is that the option exists and
 * that its edge values mean what they mean in Rust.
 */
import { describe, expect, it } from "vitest";
import type { HighlightEvent, TerminalFormatter, Theme } from "../src/types.js";
import { formatTerminal } from "../src/formatter/terminal.js";
import { formatHtmlInline } from "../src/formatter/html-inline.js";
import { htmlMultiThemes } from "../src/formatter.js";

const theme: Theme = {
  name: "test",
  appearance: "dark",
  highlights: {
    normal: { fg: "#c0c0c0", bg: "#101010" },
    highlighted: { bg: "#303030" },
    keyword: { fg: "#ff0000" },
  },
};

function sourceEvents(source: string): HighlightEvent[] {
  return [{ type: "source", startByte: 0, endByte: Buffer.byteLength(source) }];
}

function terminalFormatter(options: Partial<TerminalFormatter>): TerminalFormatter {
  return { format: () => "", ...options };
}

describe("D3 terminal background and width", () => {
  it("inherits the terminal background when background is omitted", () => {
    const source = "ab\n";
    const output = formatTerminal(source, sourceEvents(source), terminalFormatter({ theme }));

    expect(output).toBe("ab\n");
  });

  it("fills unstyled text with the theme background when background is 'theme'", () => {
    const source = "ab";
    const output = formatTerminal(
      source,
      sourceEvents(source),
      terminalFormatter({ theme, background: "theme" }),
    );

    expect(output).toContain("48;2;16;16;16");
    expect(output).toContain("ab");
  });

  it("uses an explicit color as the fallback background", () => {
    const source = "ab";
    const output = formatTerminal(
      source,
      sourceEvents(source),
      terminalFormatter({ theme, background: "#010203" }),
    );

    expect(output).toContain("48;2;1;2;3");
  });

  it("pads each line out to width", () => {
    const source = "ab\ncd\n";
    const output = formatTerminal(
      source,
      sourceEvents(source),
      terminalFormatter({ theme, background: "#010203", width: 6 }),
    );

    // Four padding spaces after each two-character line.
    expect(output.split("\n")[0]).toContain("    ");
    expect(output.split("\n")[1]).toContain("    ");
  });

  it("counts a tab as four columns when padding, like Rust", () => {
    const source = "\ta";
    const padded = formatTerminal(
      source,
      sourceEvents(source),
      terminalFormatter({ theme, background: "#010203", width: 10 }),
    );
    const spaces = padded.match(/ {2,}/g) ?? [];

    expect(spaces.at(-1)).toHaveLength(10 - 5);
  });

  it("ignores width when no background is set, since padding would be invisible", () => {
    const source = "ab";
    const output = formatTerminal(
      source,
      sourceEvents(source),
      terminalFormatter({ theme, width: 40 }),
    );

    expect(output).toBe("ab");
  });
});

describe("D4 highlightLines style", () => {
  const source = "kw";
  const events = sourceEvents(source);

  function lineAttrs(style: string | null | undefined): string {
    return formatHtmlInline(source, events, {
      format: () => "",
      theme,
      highlightLines: { lines: [1], style, class: "active" },
    });
  }

  it("uses the theme's highlighted style when style is omitted", () => {
    expect(lineAttrs()).toContain("background-color: #303030");
  });

  it("uses the theme's highlighted style for the explicit 'theme' sentinel", () => {
    expect(lineAttrs("theme")).toContain("background-color: #303030");
  });

  it("uses raw CSS for any other string", () => {
    expect(lineAttrs("outline: 1px solid red;")).toContain("outline: 1px solid red;");
  });

  it("emits no inline style at all for null, leaving the class to do it", () => {
    const html = lineAttrs(null);

    expect(html).toContain('class="l-line active"');
    expect(html).not.toContain("background-color: #303030");
  });
});

describe("D5 htmlMultiThemes validation", () => {
  it("rejects an empty themes map", () => {
    expect(() => htmlMultiThemes({ themes: {} })).toThrow(/at least one theme/);
  });

  it("rejects a defaultTheme that is not one of the themes", () => {
    expect(() => htmlMultiThemes({ themes: { light: theme }, defaultTheme: "dark" })).toThrow(
      /not one of the themes/,
    );
  });

  it("rejects light-dark() without both light and dark themes", () => {
    expect(() =>
      htmlMultiThemes({ themes: { light: theme }, defaultTheme: "light-dark()" }),
    ).toThrow(/missing dark/);
  });

  it("accepts light-dark() when both are present", () => {
    expect(() =>
      htmlMultiThemes({ themes: { light: theme, dark: theme }, defaultTheme: "light-dark()" }),
    ).not.toThrow();
  });

  it("accepts a themes map with no defaultTheme, the CSS-variables-only mode", () => {
    expect(() => htmlMultiThemes({ themes: { light: theme } })).not.toThrow();
  });

  it("revalidates mutable options when rendering", () => {
    const formatter = htmlMultiThemes({ themes: { light: theme }, defaultTheme: "light" });
    formatter.themes = {};

    expect(() => formatter.render("const x = 1", [])).toThrow(/at least one theme/);
  });
});
