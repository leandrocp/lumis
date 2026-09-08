/**
 * JavaScript's half of the cross-runtime formatter option check.
 *
 * `fixtures/formatter-options.json` lists the options every covered runtime must
 * accept. This has two halves, like the Rust one:
 *
 * - The literals below name every option. TypeScript rejects unknown
 *   properties in an object literal, so an option the JavaScript types lack
 *   fails `pnpm lint` (oxlint runs type-aware) rather than this test.
 * - `manifest matches the options exercised here` reads the manifest and
 *   requires it to name exactly these, so an option added to JavaScript
 *   without a manifest entry fails too.
 *
 * Options are camelCase here and snake_case in the manifest; `toCamel` bridges
 * that rather than duplicating both spellings.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type {
  BBCodeScopedOptions,
  HtmlInlineOptions,
  HtmlLinkedOptions,
  HtmlMultiThemesOptions,
  TerminalOptions,
} from "../src/types.js";
import type { Theme } from "../src/types.js";

interface ManifestOption {
  name: string;
}

interface Manifest {
  formatters: Record<string, { options: ManifestOption[] }>;
  waived: Record<string, unknown>;
}

const manifest: Manifest = JSON.parse(
  readFileSync(new URL("../../../../fixtures/formatter-options.json", import.meta.url), "utf8"),
);

const theme: Theme = { name: "t", appearance: "dark", highlights: {} };

const htmlInline: Required<HtmlInlineOptions> = {
  language: "rust",
  theme,
  preClass: "code",
  italic: true,
  includeHighlights: true,
  highlightLines: { lines: [1], style: "theme", class: "active" },
  header: { openTag: "<figure>", closeTag: "</figure>" },
};

const htmlLinked: Required<HtmlLinkedOptions> = {
  language: "rust",
  preClass: "code",
  highlightLines: { lines: [1], class: "active" },
  header: { openTag: "<figure>", closeTag: "</figure>" },
};

const htmlMultiThemes: Required<HtmlMultiThemesOptions> = {
  language: "rust",
  themes: { light: theme },
  defaultTheme: "light",
  cssVariablePrefix: "--lumis",
  preClass: "code",
  italic: true,
  includeHighlights: true,
  highlightLines: { lines: [1], style: "theme", class: "active" },
  header: { openTag: "<figure>", closeTag: "</figure>" },
};

const terminal: Required<TerminalOptions> = {
  language: "rust",
  theme,
  background: "theme",
  width: 120,
};

const bbcodeScoped: Required<BBCodeScopedOptions> = {
  language: "rust",
};

const exercised: Record<string, string[]> = {
  html_inline: Object.keys(htmlInline),
  html_linked: Object.keys(htmlLinked),
  html_multi_themes: Object.keys(htmlMultiThemes),
  terminal: Object.keys(terminal),
  bbcode_scoped: Object.keys(bbcodeScoped),
};

function toCamel(name: string): string {
  return name.replaceAll(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());
}

describe("formatter option manifest", () => {
  it("covers all five formatters", () => {
    expect(Object.keys(exercised).sort()).toEqual(Object.keys(manifest.formatters).sort());
  });

  for (const [formatter, entry] of Object.entries(manifest.formatters)) {
    it(`${formatter} accepts every option in the manifest`, () => {
      const expected = entry.options.map((option) => toCamel(option.name)).sort();
      const actual = [...(exercised[formatter] ?? [])].sort();

      expect(actual).toEqual(expected);
    });
  }

  it("has no waiver left standing", () => {
    const waivers = Object.keys(manifest.waived).filter((key) => !key.startsWith("$"));

    expect(waivers).toEqual([]);
  });
});
