/**
 * JavaScript's half of the cross-runtime formatter helper check.
 *
 * `fixtures/formatter-helpers.json` lists the helper capabilities every runtime
 * must offer a custom formatter. JavaScript can read its own exports, so unlike
 * the Rust half this reflects rather than calls:
 *
 * - `exports every helper in the manifest` fails on a capability JavaScript
 *   lacks.
 * - `exports nothing the manifest does not account for` fails on an export in
 *   neither the manifest's helper set, `runtime_only`, nor `deprecated`. This is
 *   the one that catches drift: JavaScript grew 20 helpers Rust never got before
 *   anything checked (#1381).
 * - `matches the shared helper output contract` feeds every helper the
 *   manifest's inputs and compares its string result with Rust's.
 * - `marks every deprecation the manifest claims` reads the source for the
 *   `@deprecated` tag, which does not survive to run time.
 *
 * Helpers are camelCase here and snake_case in the manifest; `toCamel` bridges
 * that, and `spelling.javascript` overrides it where the two do not line up.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import * as ansi from "../src/formatter/ansi.js";
import * as html from "../src/formatter/html.js";
import type { HighlightEvent, HighlightStyle, LineSpec, Theme } from "../src/types.js";

interface ManifestHelper {
  name: string;
  expected: string;
  spelling?: Record<string, string>;
}

interface Contract {
  themes: Record<string, Theme>;
  html: {
    escapeText: string;
    bracedText: string;
    text: string;
    scope: string;
    linkedScope: string;
    language: string;
    theme: string;
    themes: Record<string, string>;
    themeName: string;
    preClass: string;
    line: { number: number; content: string; class: string; style: string };
    lines: LineSpec[];
    selectedLine: number;
    highlightClass: string;
    defaultHighlightClass: string;
    source: string;
    events: HighlightEvent[];
    lineEndingCases: Array<{ source: string; expected: string[] }>;
  };
  style: HighlightStyle;
  ansi: {
    hex: string;
    rgb: [number, number, number];
    background: boolean;
    text: string;
  };
}

interface Manifest {
  contract: Contract;
  modules: Record<string, { helpers: ManifestHelper[] }>;
  runtime_only: Record<string, Record<string, Record<string, string>>>;
  deprecated: Record<string, Record<string, { runtimes?: string[] }>>;
  waived: Record<string, unknown>;
}

const manifest: Manifest = JSON.parse(
  readFileSync(new URL("../../../../fixtures/formatter-helpers.json", import.meta.url), "utf8"),
);

const modules: Record<string, Record<string, unknown>> = { html, ansi };

it("preserves the shared line-ending contract", () => {
  for (const testCase of manifest.contract.html.lineEndingCases) {
    expect(
      html.renderLinesFromEvents(
        testCase.source,
        [{ type: "source", start: 0, end: new TextEncoder().encode(testCase.source).length }],
        () => "",
      ),
      JSON.stringify(testCase.source),
    ).toEqual(testCase.expected);
  }
});

// A helper defined in one file and re-exported from another is one helper, so
// every file behind a module is read for the `@deprecated` tag.
const sources: Record<string, string[]> = {
  html: ["../src/formatter/html.ts"],
  ansi: ["../src/formatter/ansi.ts", "../src/formatter/ansi-core.ts"],
};

function toCamel(name: string): string {
  return name.replaceAll(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());
}

function jsName(helper: ManifestHelper): string {
  return helper.spelling?.javascript ?? toCamel(helper.name);
}

function exportedNames(module: string): string[] {
  return Object.keys(modules[module] ?? {}).sort();
}

function fixtureTheme(name: string): Theme {
  const theme = manifest.contract.themes[name];
  if (!theme) throw new Error(`\`${name}\` is not a fixture theme`);
  return theme;
}

function contractOutputs(): Record<string, Record<string, string>> {
  const input = manifest.contract;
  const htmlInput = input.html;
  const theme = fixtureTheme(htmlInput.theme);
  const themes = Object.fromEntries(
    Object.entries(htmlInput.themes).map(([name, themeName]) => [name, fixtureTheme(themeName)]),
  );
  const [red, green, blue] = input.ansi.rgb;
  const parsedRgb = ansi.hexToRgb(input.ansi.hex);

  return {
    html: {
      escape: html.escape(htmlInput.escapeText),
      escape_attr: html.escapeAttr(htmlInput.escapeText),
      escape_braces: html.escapeBraces(htmlInput.bracedText),
      scope_to_class: html.scopeToClass(htmlInput.linkedScope),
      style_to_css: html.styleToCss(input.style, { italic: true }),
      text_decoration: html.textDecoration(input.style),
      sanitize_theme_name: html.sanitizeThemeName(htmlInput.themeName),
      open_span: html.openSpanTag({ class: html.scopeToClass(htmlInput.scope) }),
      span_inline_attrs: html.openSpanTag(
        html.spanInlineAttrs({
          language: htmlInput.language,
          scope: htmlInput.scope,
          theme,
        }),
      ),
      span_inline: html.spanInline(htmlInput.text, {
        language: htmlInput.language,
        scope: htmlInput.scope,
        theme,
      }),
      span_linked_attrs: html.spanLinkedAttrs(htmlInput.linkedScope),
      span_linked: html.spanLinked(htmlInput.text, htmlInput.linkedScope),
      span_multi_themes_attrs: html.openSpanTag(
        html.spanMultiThemesAttrs({
          language: htmlInput.language,
          scope: htmlInput.scope,
          themes,
        }),
      ),
      span_multi_themes: html.spanMultiThemes(htmlInput.text, {
        language: htmlInput.language,
        scope: htmlInput.scope,
        themes,
      }),
      open_pre_tag: html.openPreTag({ preClass: htmlInput.preClass, theme }),
      open_multi_themes_pre_tag: html.openMultiThemesPreTag({
        preClass: htmlInput.preClass,
        themes,
      }),
      open_code_tag: html.openCodeTag(htmlInput.language),
      close_pre_tag: html.closePreTag(),
      close_code_tag: html.closeCodeTag(),
      closing_tags: html.closingTags(),
      wrap_line: html.wrapLine(htmlInput.line.number, htmlInput.line.content, {
        className: htmlInput.line.class,
        style: htmlInput.line.style,
      }),
      line_is_highlighted: String(html.lineIsHighlighted(htmlInput.lines, htmlInput.selectedLine)),
      highlight_line_class:
        html.getHighlightLineClass(
          htmlInput.lines,
          htmlInput.selectedLine,
          htmlInput.highlightClass,
          htmlInput.defaultHighlightClass,
        ) ?? "",
      render_lines_from_events: JSON.stringify(
        html.renderLinesFromEvents(htmlInput.source, htmlInput.events, (scope) =>
          html.spanLinkedAttrs(scope),
        ),
      ),
    },
    ansi: {
      hex_to_rgb: parsedRgb?.join(",") ?? "",
      rgb_to_ansi: ansi.rgbToAnsi(red, green, blue, input.ansi.background),
      style_to_ansi: ansi.styleToAnsi(input.style),
      paint: ansi.paint(input.ansi.text, input.style),
      reset: ansi.ANSI_RESET,
    },
  };
}

/** Names carrying a `@deprecated` JSDoc tag on their `export` in `module`'s sources. */
function deprecatedInSource(module: string): Set<string> {
  const marked = new Set<string>();
  const pattern =
    /\/\*\*(?:(?!\*\/)[\s\S])*?@deprecated[\s\S]*?\*\/\s*export\s+(?:async\s+)?function\s+(\w+)/g;

  for (const source of sources[module] ?? []) {
    const text = readFileSync(new URL(source, import.meta.url), "utf8");
    for (const match of text.matchAll(pattern)) {
      marked.add(match[1]);
    }
  }

  return marked;
}

function sectionNames(section: Record<string, Record<string, unknown>>, module: string): string[] {
  return Object.keys(section.javascript?.[module] ?? {}).filter((name) => !name.startsWith("$"));
}

function deprecatedNames(module: string): string[] {
  return (
    Object.entries(manifest.deprecated[module] ?? {})
      .filter(([name]) => !name.startsWith("$"))
      .filter(([, entry]) => entry.runtimes?.includes("javascript"))
      // A key is canonical snake_case where every runtime has the helper and the
      // JavaScript-only spelling where only JavaScript does; `toCamel` leaves the
      // latter alone.
      .map(([name]) => toCamel(name))
  );
}

describe("formatter helper manifest", () => {
  it("covers the modules JavaScript publishes", () => {
    expect(Object.keys(manifest.modules).sort()).toEqual(Object.keys(modules).sort());
  });

  for (const [module, entry] of Object.entries(manifest.modules)) {
    it(`${module} exports every helper in the manifest`, () => {
      const missing = entry.helpers
        .map(jsName)
        .filter((name) => !Object.hasOwn(modules[module] ?? {}, name));

      expect(missing).toEqual([]);
    });

    it(`${module} matches the shared helper output contract`, () => {
      const outputs = contractOutputs()[module] ?? {};
      const expected = Object.fromEntries(
        entry.helpers.map((helper) => [helper.name, helper.expected]),
      );

      expect(Object.keys(outputs).sort(), `${module}: contract adapters drifted`).toEqual(
        Object.keys(expected).sort(),
      );

      for (const [name, value] of Object.entries(expected)) {
        expect(outputs[name], `${module}.${name}`).toBe(value);
      }
    });

    it(`${module} exports nothing the manifest does not account for`, () => {
      const accounted = new Set([
        ...entry.helpers.map(jsName),
        ...sectionNames(manifest.runtime_only, module),
        ...deprecatedNames(module),
      ]);

      const unaccounted = exportedNames(module).filter((name) => !accounted.has(name));

      expect(
        unaccounted,
        "add them to fixtures/formatter-helpers.json and to the other runtimes, or classify them",
      ).toEqual([]);
    });

    it(`${module} marks every deprecation the manifest claims`, () => {
      const marked = deprecatedInSource(module);
      const unmarked = deprecatedNames(module).filter((name) => !marked.has(name));

      expect(unmarked).toEqual([]);
    });

    it(`${module} still exports every runtime_only helper`, () => {
      const gone = sectionNames(manifest.runtime_only, module).filter(
        (name) => !Object.hasOwn(modules[module] ?? {}, name),
      );

      expect(gone, "drop the entry").toEqual([]);
    });
  }

  it("has no waiver left standing", () => {
    const waivers = Object.keys(manifest.waived).filter((key) => !key.startsWith("$"));

    expect(waivers).toEqual([]);
  });
});
