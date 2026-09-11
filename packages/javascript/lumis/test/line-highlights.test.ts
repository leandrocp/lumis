import { describe, expect, it } from "vitest";
import { selectedLineFlags } from "../src/formatter/line-highlights.js";

describe("selectedLineFlags", () => {
  it("clamps, sorts, and merges ranges before marking output lines", () => {
    const selected = selectedLineFlags([[3, 8], 1, [2, 4], [9, 7]], 5);

    expect(Array.from(selected ?? [])).toEqual([1, 1, 1, 1, 1]);
  });

  it("handles many sparse specs without rescanning them per output line", () => {
    const lines = Array.from({ length: 10_000 }, (_, index) => index * 2 + 1);
    const selected = selectedLineFlags(lines, 20_000);

    expect(selected?.reduce((count, value) => count + value, 0)).toBe(10_000);
    expect(selected?.[0]).toBe(1);
    expect(selected?.[1]).toBe(0);
    expect(selected?.[19_998]).toBe(1);
  });
});
