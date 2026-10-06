import { execFileSync } from "node:child_process";
import { mkdtempSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { afterAll, afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { createHighlighter, runtimeKind, withWasmBundle } from "../src/index.js";
import { htmlLinked } from "../src/formatters.js";
import type { Language, LanguageBundle } from "../src/types.js";
import { installLocalPackages } from "./wasm.js";

const root = mkdtempSync(join(tmpdir(), "lumis-package-roots-"));
const cwd = process.cwd();
const project = join(root, "project");
const bundles = join(root, "bundles");

function installIsolatedPackage(language: string): string {
  const manifests = installLocalPackages(join(root, "store", language), [language]);
  return dirname(manifests[`@lumis-sh/wasm-${language}`]);
}

function linkPackage(parent: string, language: string, directory: string): void {
  const scope = join(parent, "node_modules", "@lumis-sh");
  mkdirSync(scope, { recursive: true });
  symlinkSync(directory, join(scope, `wasm-${language}`), "junction");
}

beforeAll(() => {
  installLocalPackages(project, ["json", "html"]);
  execFileSync(
    process.execPath,
    ["--import", "tsx", "scripts/build-wasm-bundles.ts", "--out", bundles],
    { cwd: resolve(cwd, "..") },
  );
  const full = join(bundles, "wasm-bundle-full");
  for (const language of ["lua", "comment", "css"]) {
    linkPackage(full, language, installIsolatedPackage(language));
  }
  process.chdir(project);
});

afterAll(() => {
  process.chdir(cwd);
  rmSync(root, { recursive: true, force: true });
});

afterEach(() => vi.restoreAllMocks());

describe("parser package search roots", () => {
  it("finds a later bundle's dependencies after the initial installed set was cached", async () => {
    const first = await createHighlighter();
    await first.loadLanguage("json");
    await first.loadLanguage("html");
    const elsewhere = join(root, "elsewhere");
    mkdirSync(elsewhere);
    process.chdir(elsewhere);
    const generated = await import(pathToFileURL(join(bundles, "wasm-bundle-full/index.js")).href);
    const bundle = generated.default as LanguageBundle;
    const highlighter = await createHighlighter({ languages: [withWasmBundle(bundle, {})] });

    expect(highlighter.languages).toEqual(["plaintext"]);
    await highlighter.loadLanguage("lua");
    if (runtimeKind() === "wasm") await highlighter.loadLanguage("comment");
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    const languages = new Set<string>();
    highlighter.highlightIter("-- TODO: fix\nlocal x = 1", "lua", undefined, (_token, language) => {
      languages.add(language);
    });
    expect([...languages].sort()).toEqual(["comment", "lua"]);
    expect(warn).not.toHaveBeenCalled();
    warn.mockRestore();

    // A later registration must also reach an existing highlighter's synchronous walk.
    if (runtimeKind() === "wasm") await first.loadLanguage("css");
    const injected = new Set<string>();
    first.highlightIter(
      "<style>a { color: red }</style>",
      "html",
      undefined,
      (_token, language) => {
        injected.add(language);
      },
    );
    expect([...injected].sort()).toEqual(["css", "html"]);

    // A string on another highlighter has no lazy loader to supply the parser.
    await first.loadLanguage("css");
    expect(first.highlight("a { color: red }", htmlLinked({ language: "css" }))).toContain(
      'class="l-',
    );
    const later = await createHighlighter();
    await later.loadLanguage("json");
    expect(later.highlight("42", htmlLinked({ language: "json" }))).toContain('class="l-number"');
  });

  it("discovers an imported language and the dependency installed beside it", async () => {
    const markdown = installIsolatedPackage("markdown");
    linkPackage(markdown, "markdown_inline", installIsolatedPackage("markdown_inline"));
    const manifestPath = join(markdown, "package.json");
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
    manifest.dependencies = { "@lumis-sh/wasm-markdown_inline": "^0.26" };
    writeFileSync(manifestPath, JSON.stringify(manifest));
    const imported = await import(pathToFileURL(join(markdown, "index.js")).href);
    const highlighter = await createHighlighter({ languages: [imported.default as Language] });

    if (runtimeKind() === "wasm") await highlighter.loadLanguage("markdown_inline");
    expect(highlighter.highlight("**bold**", htmlLinked({ language: "markdown" }))).toContain(
      'class="l-markup-strong"',
    );
  });

  it("reports the working directory when an unimported package cannot be resolved", async () => {
    const highlighter = await createHighlighter();
    await expect(highlighter.loadLanguage("haskell")).rejects.toThrow(
      `working directory ${process.cwd()}`,
    );
  });
});
