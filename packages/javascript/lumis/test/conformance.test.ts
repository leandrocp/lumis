import { beforeAll, describe, expect, it } from "vitest";

import dracula from "../../themes/dist/json/dracula.json";
import dark from "../../../../fixtures/conformance-themes/dark.json";
import light from "../../../../fixtures/conformance-themes/light.json";
import mid from "../../../../fixtures/conformance-themes/mid.json";
import css from "../langs/css.ts";
import diff from "../langs/diff.ts";
import elixir from "../langs/elixir.ts";
import html from "../langs/html.ts";
import javascript from "../langs/javascript.ts";
import json from "../langs/json.ts";
import lua from "../langs/lua.ts";
import markdown from "../langs/markdown.ts";
import markdownInline from "../langs/markdown_inline.ts";
import mdx from "../langs/mdx.ts";
import python from "../langs/python.ts";
import rust from "../langs/rust.ts";
import { createHighlighter } from "../src/index.js";
import {
  bbcodeScoped,
  htmlInline,
  htmlLinked,
  htmlMultiThemes,
  terminal,
} from "../src/formatters.js";
import type { Highlighter } from "../src/core/highlighter.js";
import type { Language, Theme } from "../src/types.js";

import { loadConformanceFixtures } from "./conformance.js";
import { configureLocalWasmResolver } from "./wasm.js";

const conformanceFixtures = loadConformanceFixtures();
const themes: Record<string, Theme> = {
  dark,
  dracula,
  light,
  mid,
};

const langBundles: Record<string, Language> = {
  json,
  diff,
  elixir,
  html,
  javascript,
  css,
  lua,
  markdown,
  markdownInline,
  mdx,
  python,
  rust,
};

function getLanguage(id: string): Language {
  const language = langBundles[id];
  if (!language) throw new Error(`No bundle for language "${id}" in test setup`);
  return language;
}

function getTheme(id: string): Theme {
  const theme = themes[id];
  if (!theme) throw new Error(`No theme for "${id}" in test setup`);
  return theme;
}

let highlighter: Highlighter;

beforeAll(async () => {
  configureLocalWasmResolver([
    "diff",
    "elixir",
    "json",
    "html",
    "javascript",
    "css",
    "lua",
    "markdown",
    "markdown_inline",
    "python",
    "rust",
  ]);
  // No `wasm` overrides. Parser bytes and the package that describes them have
  // to come from the same place, or the integrity check rejects the pair: CI
  // stages freshly built parsers while `ensureLocalWasm` returns the committed
  // fixture, which is a different build of the same grammar.
  highlighter = await createHighlighter({
    languages: [
      json,
      diff,
      elixir,
      html,
      javascript,
      css,
      lua,
      markdown,
      markdownInline,
      mdx,
      python,
      rust,
    ],
  });
}, 120_000);

describe.each(conformanceFixtures.map((f) => [f.name, f]))("%s", (_name, fixture) => {
  it("html-inline", () => {
    const output = highlighter.highlight(
      fixture.source,
      htmlInline({
        language: getLanguage(fixture.language),
        theme: getTheme(fixture.theme),
      }),
      { rainbowBrackets: fixture.rainbowBrackets },
    );
    expect(output).toBe(fixture.htmlInline);
  });

  it("html-linked", () => {
    const output = highlighter.highlight(
      fixture.source,
      htmlLinked({
        language: getLanguage(fixture.language),
      }),
      { rainbowBrackets: fixture.rainbowBrackets },
    );
    expect(output).toBe(fixture.htmlLinked);
  });

  it("html-multi-themes", () => {
    const config = fixture.htmlMultiThemesOptions;
    // Reversed on purpose. Rust holds themes in a `HashMap` and sorts before
    // emitting, so output must not depend on the order the caller inserted
    // them. Feeding them in already-sorted order would not prove that.
    const formatterThemes = config
      ? Object.fromEntries(
          Object.entries(config.themes)
            .toReversed()
            .map(([name, theme]) => [name, getTheme(theme)]),
        )
      : { main: getTheme(fixture.theme) };
    const output = highlighter.highlight(
      fixture.source,
      htmlMultiThemes({
        language: getLanguage(fixture.language),
        themes: formatterThemes,
        defaultTheme: config ? config.defaultTheme : "main",
        highlightLines: config?.highlightLines?.length
          ? { lines: config.highlightLines, style: "theme" }
          : undefined,
      }),
      { rainbowBrackets: fixture.rainbowBrackets },
    );
    expect(output).toBe(fixture.htmlMultiThemes);
  });

  it("bbcodeScoped", () => {
    const output = highlighter.highlight(
      fixture.source,
      bbcodeScoped({
        language: getLanguage(fixture.language),
      }),
      { rainbowBrackets: fixture.rainbowBrackets },
    );
    expect(output).toBe(fixture.bbcode);
  });

  it("terminal", () => {
    const output = highlighter.highlight(
      fixture.source,
      terminal({
        language: getLanguage(fixture.language),
        theme: getTheme(fixture.theme),
      }),
      { rainbowBrackets: fixture.rainbowBrackets },
    );
    expect(output).toBe(fixture.terminal);
  });
});
