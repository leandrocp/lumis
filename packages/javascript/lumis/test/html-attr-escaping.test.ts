/**
 * The TypeScript half of the HTML attribute escaping parity check.
 *
 * `fixtures/html-attr-escaping.json` holds one expected tag per case.
 * `crates/lumis-core/tests/html_attr_escaping.rs` asserts Rust produces it;
 * this asserts the port does too. Rust is the reference, so a difference here
 * is a bug in this port.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import { openPreTag, spanInline, spanMultiThemes, wrapLine } from "../src/formatter/html.js";
import type { Theme } from "../src/types.js";

type Helper = "preTag" | "spanInline" | "spanMultiThemes" | "line";

interface Case {
  name: string;
  helper: Helper;
  expected: string;
  text?: string;
  scope?: string;
  theme?: string;
  includeHighlights?: boolean;
  themes?: Record<string, string>;
  defaultTheme?: string;
  preClass?: string;
  lineClass?: string;
  lineStyle?: string;
}

const manifest: { themes: Record<string, Theme>; cases: Case[] } = JSON.parse(
  readFileSync(new URL("../../../../fixtures/html-attr-escaping.json", import.meta.url), "utf8"),
);

function theme(key: string): Theme {
  const found = manifest.themes[key];
  if (!found) throw new Error(`\`${key}\` is not a fixture theme`);
  return found;
}

function themesOf(testCase: Case): Record<string, Theme> {
  return Object.fromEntries(
    Object.entries(testCase.themes ?? {}).map(([name, key]) => [name, theme(key)]),
  );
}

function themeOf(testCase: Case): Theme | undefined {
  return testCase.theme ? theme(testCase.theme) : undefined;
}

const RENDERERS: Record<Helper, (testCase: Case) => string> = {
  preTag: (testCase) => openPreTag({ preClass: testCase.preClass, theme: themeOf(testCase) }),

  spanInline: (testCase) =>
    spanInline(testCase.text ?? "", {
      language: "plaintext",
      scope: testCase.scope ?? "",
      theme: themeOf(testCase),
      italic: false,
      includeHighlights: testCase.includeHighlights ?? false,
    }),

  spanMultiThemes: (testCase) =>
    spanMultiThemes(testCase.text ?? "", {
      language: "plaintext",
      scope: testCase.scope ?? "",
      themes: themesOf(testCase),
      defaultTheme: testCase.defaultTheme,
      cssVariablePrefix: "--lumis",
      italic: false,
      includeHighlights: testCase.includeHighlights ?? false,
    }),

  // The tag only: Rust takes the trailing newline in `content` and this port
  // appends it, so the two agree up to the `>`.
  line: (testCase) => wrapLine(1, "", { className: testCase.lineClass, style: testCase.lineStyle }),
};

function render(testCase: Case): string {
  return RENDERERS[testCase.helper](testCase);
}

describe("html attribute escaping parity", () => {
  it("covers both vectors that reach an attribute", () => {
    const names = manifest.cases.map((testCase) => testCase.name);

    for (const required of [
      "pre/caller-class-closes-the-attribute",
      "pre/theme-colour-closes-the-attribute",
      "span/theme-colour-closes-the-attribute",
      "multi-themes/theme-colour-closes-the-attribute",
      "line/caller-class-closes-the-attribute",
      "line/caller-style-closes-the-attribute",
    ]) {
      expect(names, `the corpus lost its \`${required}\` case`).toContain(required);
    }
  });

  it("produces the same tags as Rust", () => {
    for (const testCase of manifest.cases) {
      const rendered = render(testCase);

      if (testCase.helper === "line") {
        expect(rendered.startsWith(testCase.expected), `${testCase.name}: got ${rendered}`).toBe(
          true,
        );
      } else {
        expect(rendered, `${testCase.name}: diverged from Rust`).toBe(testCase.expected);
      }
    }
  });
});
