import { describe, it, expect } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { available as availableLanguages, get as getLanguage } from "../src/languages.js";
import { available as availableThemes } from "../src/themes/index.js";

describe("getLanguage", () => {
  it("finds a language by id", () => {
    expect(getLanguage("javascript")?.name).toBe("JavaScript");
  });

  it("finds a language by alias", () => {
    expect(getLanguage("js")?.id).toBe("javascript");
    expect(getLanguage("JS")?.id).toBe("javascript");
    expect(getLanguage("  js  ")?.id).toBe("javascript");
  });

  it("returns undefined for an unknown name", () => {
    expect(getLanguage("not-a-language")).toBeUndefined();
  });

  it("returns the same record availableLanguages yields", () => {
    const fromList = availableLanguages().find((language) => language.id === "rust");
    expect(getLanguage("rust")).toEqual(fromList);
  });

  it("returns a fresh record the caller can mutate", () => {
    const first = getLanguage("rust")!;
    first.aliases.push("mutated");
    expect(getLanguage("rust")!.aliases).not.toContain("mutated");
  });

  it("resolves every alias in the catalog", () => {
    for (const language of availableLanguages()) {
      for (const alias of language.aliases) {
        expect(getLanguage(alias)?.id, `alias ${alias}`).toBe(language.id);
      }
    }
  });
});

describe("availableLanguages", () => {
  it("returns a non-empty array", () => {
    const languages = availableLanguages();
    expect(languages.length).toBeGreaterThan(0);
  });

  it("includes known languages", () => {
    const languages = availableLanguages();
    const ids = languages.map((language) => language.id);
    expect(ids).toContain("javascript");
    expect(ids).toContain("rust");
    expect(ids).toContain("python");
    expect(ids).toContain("json");
    expect(ids).toContain("plaintext");
  });

  it("is sorted by language ID", () => {
    const ids = availableLanguages().map((language) => language.id);

    expect(ids).toEqual([...ids].sort());
  });

  it("has correct shape", () => {
    const js = availableLanguages().find((l) => l.id === "javascript")!;
    expect(js.name).toBe("JavaScript");
    expect(js.aliases).toContain("js");
    expect(js.extensions).toContain("*.js");
    expect(js.globs).toContain("*.js");
  });

  it("plaintext has correct metadata", () => {
    const pt = availableLanguages().find((l) => l.id === "plaintext")!;
    expect(pt.name).toBe("Plain Text");
    expect(pt.aliases).toEqual(["text", "txt", "plain"]);
    expect(pt.extensions).toEqual([]);
    expect(pt.globs).toEqual([]);
    expect(pt.emacsModes).toEqual(["fundamental", "text"]);
    expect(pt.shebangs).toEqual([]);
  });

  it("returns fresh records and nested arrays", () => {
    const first = availableLanguages();
    const plaintext = first.find((language) => language.id === "plaintext")!;
    plaintext.name = "mutated";
    plaintext.aliases.push("mutated");
    first.length = 0;

    const next = availableLanguages();
    const nextPlaintext = next.find((language) => language.id === "plaintext")!;
    expect(nextPlaintext.name).toBe("Plain Text");
    expect(nextPlaintext.aliases).not.toContain("mutated");
    expect(next.length).toBeGreaterThan(0);
  });

  it("includes filename globs beyond extensions", () => {
    const bash = availableLanguages().find((l) => l.id === "bash")!;
    expect(bash.globs).toContain(".bashrc");

    const dockerfile = availableLanguages().find((l) => l.id === "dockerfile")!;
    expect(dockerfile.globs).toContain("Dockerfile");
  });
});

describe("availableThemes", () => {
  it("returns a non-empty array", () => {
    const themes = availableThemes();
    expect(themes.length).toBeGreaterThan(0);
  });

  it("includes known themes", () => {
    const themes = availableThemes();
    const names = themes.map((t) => t.name);
    expect(names).toContain("dracula");
    expect(names).toContain("github_light");
    expect(names).toContain("catppuccin_mocha");
  });

  it("has correct appearance values", () => {
    const themes = availableThemes();
    const dracula = themes.find((t) => t.name === "dracula")!;
    expect(dracula.appearance).toBe("dark");

    const githubLight = themes.find((t) => t.name === "github_light")!;
    expect(githubLight.appearance).toBe("light");
  });

  it("all themes have valid appearance", () => {
    for (const theme of availableThemes()) {
      expect(["light", "dark"]).toContain(theme.appearance);
    }
  });

  it("matches the default theme pack", () => {
    const pack: { themes: { name: string }[] } = JSON.parse(
      readFileSync(resolve(import.meta.dirname, "../../../../themes/packs/default.json"), "utf8"),
    );
    const expected = pack.themes.map((theme) => theme.name).sort();

    expect(expected.length).toBeGreaterThan(200);
    expect(
      availableThemes()
        .map((theme) => theme.name)
        .sort(),
    ).toEqual(expected);
  });

  it("returns fresh theme records", () => {
    const first = availableThemes();
    first[0].name = "mutated";
    first.length = 0;

    const next = availableThemes();
    expect(next[0].name).not.toBe("mutated");
    expect(next.length).toBeGreaterThan(0);
  });
});
