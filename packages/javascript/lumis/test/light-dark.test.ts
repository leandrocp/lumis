/**
 * `light-dark()` is a color function. Anything else it wraps is invalid CSS the
 * browser throws away, so these parse the emitted style attribute back into
 * declarations and check each one against the values its property accepts.
 *
 * The same cases live in `crates/lumis-core/src/formatter/html.rs`; the bytes
 * the two produce are pinned by `fixtures/conformance`.
 */
import { describe, expect, it } from "vitest";
import type { Theme } from "../src/types.js";
import { spanMultiThemesAttrs } from "../src/formatter/html.js";

/**
 * Two themes that agree on one scope's non-color properties and disagree on the
 * others, so both halves of the `light-dark()` rule are covered.
 */
const themes: Record<string, Theme> = {
  light: {
    name: "light",
    appearance: "light",
    highlights: {
      normal: { fg: "#111111", bg: "#ffffff" },
      keyword: { fg: "#d73a49", bold: true },
      comment: { fg: "#6a737d", italic: true },
      string: { fg: "#032f62", underline: true },
    },
  },
  dark: {
    name: "dark",
    appearance: "dark",
    highlights: {
      normal: { fg: "#eeeeee", bg: "#000000" },
      keyword: { fg: "#ff7b72", bold: true },
      comment: { fg: "#8b949e" },
      string: { fg: "#a5d6ff", strikethrough: true },
    },
  },
};

function lightDarkStyle(scope: string, italic: boolean): string {
  return spanMultiThemesAttrs({ scope, themes, defaultTheme: "light-dark()", italic }).style ?? "";
}

/** Split a style attribute back into the declarations a browser would parse out of it. */
function cssDeclarations(style: string): Array<[string, string]> {
  return style
    .split(";")
    .map((declaration) => declaration.trim())
    .filter((declaration) => declaration.length > 0)
    .map((declaration) => {
      const colon = declaration.indexOf(":");
      expect(colon, `\`${declaration}\` is not a declaration`).toBeGreaterThan(0);
      return [declaration.slice(0, colon).trim(), declaration.slice(colon + 1).trim()];
    });
}

function isColor(value: string): boolean {
  return /^#[0-9a-f]+$/i.test(value);
}

const DECORATION_KEYWORDS = new Set([
  "underline",
  "line-through",
  "wavy",
  "double",
  "dotted",
  "dashed",
]);

/**
 * What each property the formatter emits accepts. `light-dark()` is a color
 * function, so a declaration using it is valid only where a color is, and only
 * over two colors.
 */
const VALID_VALUE: Record<string, (value: string) => boolean> = {
  color: isLightDarkColor,
  "background-color": isLightDarkColor,
  "font-weight": (value) => value === "normal" || value === "bold",
  "font-style": (value) => value === "normal" || value === "italic",
  "text-decoration": (value) =>
    value.split(" ").every((keyword) => DECORATION_KEYWORDS.has(keyword)),
};

function isLightDarkColor(value: string): boolean {
  const colors = /^light-dark\((.*)\)$/.exec(value)?.[1];
  if (colors === undefined) return isColor(value);
  return colors.split(", ").every((color) => isColor(color));
}

function isValidDeclaration(property: string, value: string): boolean {
  if (property.startsWith("--")) return true;
  return VALID_VALUE[property]?.(value) ?? false;
}

describe("light-dark() spans", () => {
  it("emits only declarations a browser keeps", () => {
    for (const scope of ["normal", "keyword", "comment", "string"]) {
      for (const [property, value] of cssDeclarations(lightDarkStyle(scope, true))) {
        expect(
          isValidDeclaration(property, value),
          `browsers discard \`${property}: ${value};\``,
        ).toBe(true);
      }
    }
  });

  it("writes a shared non-color property as a plain declaration", () => {
    expect(lightDarkStyle("keyword", true)).toBe(
      "color: light-dark(#d73a49, #ff7b72); font-weight: bold; " +
        "--lumis-dark-font-weight:bold; --lumis-light-font-weight:bold;",
    );
  });

  it("drops a non-color property both themes leave at its initial value", () => {
    expect(lightDarkStyle("normal", true)).toBe(
      "color: light-dark(#111111, #eeeeee); background-color: light-dark(#ffffff, #000000);",
    );
  });

  it("switches a disputed non-color property through variables", () => {
    expect(lightDarkStyle("comment", true)).toBe(
      "color: light-dark(#6a737d, #8b949e); font-style: italic; " +
        "--lumis-dark-font-style:normal; --lumis-light-font-style:italic;",
    );
    expect(lightDarkStyle("string", true)).toBe(
      "color: light-dark(#032f62, #a5d6ff); text-decoration: underline; " +
        "--lumis-dark-text-decoration:line-through; --lumis-light-text-decoration:underline;",
    );
  });

  it("leaves font-style alone when italics are off", () => {
    expect(lightDarkStyle("comment", false)).toBe("color: light-dark(#6a737d, #8b949e);");
  });
});
