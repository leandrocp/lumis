import { describe, expect, it } from "vitest";
import { expandBundles } from "../src/core/language-names.js";

describe("expandBundles", () => {
  it("expands a bundle into its members", () => {
    expect(expandBundles(["bundle-web"])).toEqual([
      "css",
      "html",
      "javascript",
      "json",
      "tsx",
      "typescript",
    ]);
  });

  it("accepts underscores, so Elixir's :bundle_web_extra reaches the same entry", () => {
    expect(expandBundles(["bundle_web_extra"])).toEqual(expandBundles(["bundle-web-extra"]));
  });

  it("leaves plain language names alone", () => {
    expect(expandBundles(["rust", "elixir"])).toEqual(["rust", "elixir"]);
  });

  it("deduplicates across a bundle and an explicit name", () => {
    const expanded = expandBundles(["bundle-web", "css"]);

    expect(expanded.filter((id) => id === "css")).toHaveLength(1);
  });

  it("rejects a name that looks like a bundle but is not one", () => {
    expect(() => expandBundles(["bundle-nope"])).toThrow(/Unknown bundle "bundle-nope"/);
  });
});
