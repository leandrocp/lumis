import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { closest, type TerminalColors } from "../src/themes/index.js";
import { coreColors } from "../src/themes/closest.js";
import { THEME_DATA } from "../src/generated/theme-data.js";

interface Case extends TerminalColors {
  name: string;
  expected: string | null;
  expected_default?: string | null;
}

const cases: Case[] = JSON.parse(
  readFileSync(new URL("../../../../fixtures/theme-choice-cases.json", import.meta.url), "utf8"),
);
const excluded: string[] = JSON.parse(
  readFileSync(new URL("../../../../fixtures/theme-choice-excluded.json", import.meta.url), "utf8"),
);

describe("closest", () => {
  it("uses the complete shared corpus", () => {
    expect(cases.length).toBeGreaterThanOrEqual(20);
    expect(THEME_DATA.length).toBeGreaterThan(200);
  });

  it.each(cases)("$name", (testCase) => {
    expect(closest(testCase)?.name ?? null).toBe(testCase.expected_default ?? testCase.expected);
  });

  it("pins the excluded monochrome themes", () => {
    expect(
      THEME_DATA.filter((theme) => coreColors(theme).size <= 3).map((theme) => theme.name),
    ).toEqual(excluded);
  });

  it("returns a full theme without sharing mutable catalog state", () => {
    const first = closest({ background: "#000000" });
    const fixture = cases.find((entry) => entry.name === "black-background");
    if (!fixture || !first) throw new Error("missing expected theme or fixture");
    const expected = fixture.expected_default ?? fixture.expected;
    expect(expected).toBeTypeOf("string");
    expect(first.name).toBe(expected);
    expect(Object.keys(first.highlights).length).toBeGreaterThan(10);
    first.name = "changed";
    first.highlights.normal = { fg: "#000000" };
    expect(closest({ background: "#000000" })?.name).toBe(expected);
    expect(closest({ background: "#000000" })?.highlights.normal?.fg).not.toBe("#000000");
  });

  it.each(["#fff", "#gggggg", "ff+79c", "aéaaa", ""])("rejects invalid color %s", (hex) => {
    expect(() => closest({ background: hex })).toThrow(TypeError);
    expect(() => closest({ foreground: hex })).toThrow(TypeError);
    const ansi: (string | null)[] = Array.from({ length: 16 }, () => null);
    ansi[15] = hex;
    expect(() => closest({ background: "#000000", ansi })).toThrow(TypeError);
  });

  it.each([0, 15, 17])("rejects a palette of %i entries", (length) => {
    expect(() => closest({ ansi: Array.from({ length }, () => null) })).toThrow(TypeError);
  });
});
