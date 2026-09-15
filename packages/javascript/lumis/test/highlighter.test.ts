import { readFileSync } from "node:fs";
import { describe, it, expect, beforeAll } from "vitest";
import { createHighlighter, highlight } from "../src/index.js";
import {
  bbcodeScoped,
  htmlInline,
  htmlLinked,
  htmlMultiThemes,
  terminal,
} from "../src/formatters.js";
import * as formatterApi from "../src/formatters.js";
import type { Highlighter, Language, LanguageBundle, LanguageInput, Theme } from "../src/index.js";
import json from "../langs/json.ts";
import htmlLanguage from "../langs/html.ts";
import plaintext from "../langs/plaintext.ts";
import javascript from "../langs/javascript.ts";
import python from "../langs/python.ts";
import { bundledLanguages } from "../bundles/web.ts";
import { configureLocalWasmResolver } from "./wasm.js";

import tokyonightMoon from "../../themes/dist/json/tokyonight_moon.json";
import dracula from "../../themes/dist/json/dracula.json";

interface GuessCase {
  name: string;
  input: string | null;
  source: string;
  expected: string;
}

const guessCases = JSON.parse(
  readFileSync(new URL("../../../../fixtures/language-guess-cases.json", import.meta.url), "utf8"),
) as GuessCase[];

const theme: Theme = tokyonightMoon;
const draculaTheme: Theme = dracula;

let hl: Highlighter;

beforeAll(async () => {
  configureLocalWasmResolver([
    "diff",
    "json",
    "javascript",
    "bash",
    "dockerfile",
    "elixir",
    "html",
    "python",
    "ruby",
    "rust",
    "xml",
  ]);

  hl = await createHighlighter({
    languages: [json],
  });
}, 120_000);

describe("createHighlighter", () => {
  it("loads languages passed in init", () => {
    expect(hl.languages).toContain("json");
  });

  it("loads multiple languages passed in init", async () => {
    const multi = await createHighlighter({
      languages: [json, javascript],
    });

    expect(multi.languages).toContain("json");
    expect(multi.languages).toContain("javascript");
  });

  it("resolves an identifier-only initial language by id", async () => {
    const identifierOnly = await createHighlighter({
      languages: [{ id: "JSON", aliases: [] }],
    });

    expect(identifierOnly.languages).toContain("json");
    expect(identifierOnly.highlight('{"a": 1}', htmlLinked({ language: "JSON" }))).toContain(
      'class="l-number"',
    );
  });

  it.each([
    { id: "json", aliases: [], packageName: "" },
    { id: "json", aliases: [], highlights: "(string) @string" },
    {
      id: "json",
      aliases: [],
      packageName: "@lumis-sh/wasm-json",
      highlights: "(string) @string",
    },
    {
      id: "json",
      aliases: [],
      packageName: undefined,
      highlights: "(string) @string",
      wasm: new Uint8Array(),
    },
  ])("rejects an incomplete initial language definition", async (language) => {
    await expect(createHighlighter({ languages: [language] })).rejects.toThrow(
      'Language "json" has an incomplete or conflicting load definition.',
    );
  });

  it.each([
    { aliases: [] },
    { id: "", aliases: [] },
    { id: "json" },
    { id: "json", aliases: "json" },
    { id: "json", aliases: [1] },
  ])("rejects a malformed initial language definition", async (language) => {
    await expect(
      createHighlighter({ languages: [language as unknown as Language] }),
    ).rejects.toThrow('Language definition has an invalid or missing "id" or "aliases".');
  });

  it.each(["injections", "locals", "brackets"] as const)(
    "rejects a non-string $field query at the shared boundary",
    async (field) => {
      const language = {
        id: `invalid-${field}`,
        aliases: [],
        highlights: "(string) @string",
        wasm: new Uint8Array(),
        [field]: {},
      } as unknown as Language;

      await expect(createHighlighter({ languages: [language] })).rejects.toThrow(
        `Language "invalid-${field}" has an incomplete or conflicting load definition.`,
      );
    },
  );

  it.each([
    Promise.resolve({ default: { id: "json" } }),
    async () => ({ default: { id: "json" } }),
  ])("validates the language returned by an import input", async (input) => {
    await expect(
      createHighlighter({ languages: [input as unknown as LanguageInput] }),
    ).rejects.toThrow('Language definition has an invalid or missing "id" or "aliases".');
  });

  it("validates every lazy handle in a language bundle", async () => {
    const valid = Object.assign(async () => json, { id: "json", aliases: [] });
    const invalid = Object.assign(async () => python, {
      id: "python",
      aliases: "python",
    });
    const bundle = { json: valid, python: invalid } as unknown as LanguageBundle;

    await expect(createHighlighter({ languages: [bundle] })).rejects.toThrow(
      'Language definition has an invalid or missing "id" or "aliases".',
    );
  });

  it("validates a language returned by a lazy bundle handle", async () => {
    const invalid = Object.assign(async () => ({ id: "json" }) as unknown as Language, {
      id: "json",
      aliases: [],
    });
    const highlighter = await createHighlighter({
      languages: [{ json: invalid }],
    });

    await expect(highlighter.loadLanguage("json")).rejects.toThrow(
      'Language definition has an invalid or missing "id" or "aliases".',
    );
  });

  it("validates a lazy bundle result resolved by an identifier-only input", async () => {
    const invalid = Object.assign(async () => ({ id: "json" }) as unknown as Language, {
      id: "json",
      aliases: [],
    });

    await expect(
      createHighlighter({
        languages: [{ json: invalid }, { id: "json", aliases: [] }],
      }),
    ).rejects.toThrow('Language definition has an invalid or missing "id" or "aliases".');
  });

  it("validates a language returned by a lazy async-helper reference", async () => {
    const invalid = Object.assign(async () => ({ id: "json" }) as unknown as Language, {
      id: "json",
      aliases: [],
    });

    await expect(highlight('{"a": 1}', htmlLinked({ language: invalid }))).rejects.toThrow(
      'Language definition has an invalid or missing "id" or "aliases".',
    );
  });

  it("rejects an incomplete language definition loaded later", async () => {
    const highlighter = await createHighlighter();

    await expect(
      highlighter.loadLanguage({ id: "python", aliases: [], packageName: "" }),
    ).rejects.toThrow('Language "python" has an incomplete or conflicting load definition.');
  });

  it("rejects a malformed language definition loaded later", async () => {
    const highlighter = await createHighlighter();

    await expect(highlighter.loadLanguage({ id: "python" } as unknown as Language)).rejects.toThrow(
      'Language definition has an invalid or missing "id" or "aliases".',
    );
  });

  it("rejects an incomplete language definition used by the async helper", async () => {
    const language: Language = { id: "json", aliases: [], packageName: "" };

    await expect(highlight('{"a": 1}', htmlLinked({ language }))).rejects.toThrow(
      'Language "json" has an incomplete or conflicting load definition.',
    );
  });

  it("rejects a malformed language definition used by the async helper", async () => {
    const language = { id: "json" } as unknown as Language;

    await expect(highlight('{"a": 1}', htmlLinked({ language }))).rejects.toThrow(
      'Language definition has an invalid or missing "id" or "aliases".',
    );
  });

  it("rejects an incomplete language definition used by a sync highlighter", async () => {
    const highlighter = await createHighlighter({ languages: [json] });
    const language: Language = { id: "json", aliases: [], highlights: "" };

    expect(() => highlighter.highlight('{"a": 1}', htmlLinked({ language }))).toThrow(
      'Language "json" has an incomplete or conflicting load definition.',
    );
  });

  it("rejects a malformed language definition used by a sync highlighter", async () => {
    const highlighter = await createHighlighter({ languages: [json] });
    const language = { id: "json" } as unknown as Language;

    expect(() => highlighter.highlight('{"a": 1}', htmlLinked({ language }))).toThrow(
      'Language definition has an invalid or missing "id" or "aliases".',
    );
  });

  it("always loads plaintext for fallback highlighting", () => {
    expect(hl.languages).toContain("plaintext");
  });

  it("loads languages dynamically via loadLanguage", async () => {
    await hl.loadLanguage(json);
    expect(hl.languages).toContain("json");
  });

  it("loads a lazy bundle through an ASCII-case-insensitive alias", async () => {
    const lazy = await createHighlighter({ languages: [bundledLanguages] });

    await lazy.loadLanguage("JS");

    expect(lazy.languages).toContain("javascript");
    expect(
      lazy.highlight("const value = 1", htmlInline({ language: "JS", theme: draculaTheme })),
    ).toContain('class="language-javascript"');
  });

  it("keeps loaded languages isolated per instance", async () => {
    const first = await createHighlighter({ languages: [json] });
    const second = await createHighlighter();

    expect(first.languages).toContain("json");
    expect(second.languages).toContain("plaintext");
    expect(second.languages).not.toContain("json");
  });

  it("restores specialized injections when a target language is loaded later", async () => {
    const source = 'element.innerHTML = "<strong>hello</strong>"';
    const dynamic = await createHighlighter({ languages: [javascript] });
    await dynamic.loadLanguage(htmlLanguage);
    const eager = await createHighlighter({ languages: [javascript, htmlLanguage] });

    expect(dynamic.highlight(source, htmlLinked({ language: javascript }))).toBe(
      eager.highlight(source, htmlLinked({ language: javascript })),
    );
  });
});

describe("hl.highlight", () => {
  it("produces inline-styled HTML with theme", () => {
    const html = hl.highlight('{"a": 1}', htmlInline({ language: json, theme }));
    expect(html).toContain('<pre class="lumis"');
    expect(html).toContain('class="language-json"');
    expect(html).toContain('data-line="1"');
    expect(html).toContain('style="color:');
  });

  it("accepts Language as language", () => {
    const html = hl.highlight('{"a": 1}', htmlInline({ language: json, theme }));
    expect(html).toContain("<span");
    expect(html).toContain('class="language-json"');
  });

  it("accepts language aliases as strings", async () => {
    const jsHighlighter = await createHighlighter({
      languages: [javascript],
    });

    const html = jsHighlighter.highlight(
      "const x = 1",
      htmlInline({ language: "js", theme: draculaTheme }),
    );

    expect(html).toContain('class="language-javascript"');
    expect(html).toContain("<span");
  });

  it("highlights Python class scopes when captures omit match metadata", async () => {
    const pythonHighlighter = await createHighlighter({ languages: [python] });
    const source = `from dataclasses import dataclass

@dataclass(frozen=True)
class User:
    name: str
    active: bool = True
`;

    const html = pythonHighlighter.highlight(source, htmlInline({ language: python, theme }));

    expect(html).toContain('class="language-python"');
    expect(html).toContain("User");
    expect(html).toContain("<span");
  });

  it("adds custom class to pre element", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlInline({ language: json, theme, preClass: "code-block" }),
    );
    expect(html).toContain('class="lumis code-block"');
  });

  it("accepts italic option", () => {
    const html = hl.highlight('{"a": 1}', htmlInline({ language: json, theme, italic: true }));
    expect(html).toContain('class="language-json"');
    expect(html).toContain("style=");
  });

  it("adds data-highlight attributes when enabled", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlInline({ language: json, theme, includeHighlights: true }),
    );
    expect(html).toContain("data-highlight=");
  });

  it("wraps output with header element", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlInline({
        language: json,
        theme,
        header: {
          openTag: '<div class="wrapper">',
          closeTag: "</div>",
        },
      }),
    );
    expect(html).toMatch(/^<div class="wrapper">/);
    expect(html).toMatch(/<\/div>$/);
  });

  it("produces CSS-class HTML for htmlLinked", () => {
    const html = hl.highlight('{"a": 1}', htmlLinked({ language: json }));
    expect(html).toContain('<pre class="lumis">');
    expect(html).toContain('class="language-json"');
    expect(html).toMatch(/class="l-(string|number|punctuation)/);
  });

  it("produces nested bbcodeScoped tags and escapes raw brackets", async () => {
    const jsHighlighter = await createHighlighter({
      languages: [json, javascript],
    });

    const output = jsHighlighter.highlight(
      'const x = "[url=x]"\n',
      bbcodeScoped({ language: javascript }),
    );
    expect(output).toContain("[keyword-javascript]const[/keyword-javascript]");
    expect(output).toContain('[string-javascript]"&#91;url=x&#93;"[/string-javascript]');
  });

  it("wraps htmlLinked output with header", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlLinked({
        language: json,
        header: {
          openTag: '<div class="linked-wrapper">',
          closeTag: "</div>",
        },
      }),
    );
    expect(html).toMatch(/^<div class="linked-wrapper">/);
    expect(html).toMatch(/<\/div>$/);
  });

  it("throws for unloaded language", () => {
    expect(() =>
      hl.highlight("code", htmlInline({ language: { id: "python", aliases: [] }, theme })),
    ).toThrow(/not loaded/);
  });

  it("uses plaintext when language is omitted", () => {
    const html = hl.highlight("plain text", htmlInline({ theme }));
    expect(html).toContain('class="language-plaintext"');
  });
});

describe("plaintext", () => {
  let plaintextHl: Highlighter;

  beforeAll(async () => {
    plaintextHl = await createHighlighter({
      languages: [json, plaintext],
    });
  });

  it("renders when language is omitted (html_inline)", () => {
    const html = plaintextHl.highlight("just text\nwith <html> & stuff", htmlInline({ theme }));
    expect(html).toContain('class="language-plaintext"');
    expect(html).toContain("&lt;html&gt;");
    expect(html).toContain("&amp;");
    expect(html).toContain('data-line="1"');
    expect(html).toContain('data-line="2"');
    expect(html).not.toContain("<span");
  });

  it("renders when language is omitted (html_linked)", () => {
    const html = plaintextHl.highlight("plain text", htmlLinked());
    expect(html).toContain('class="language-plaintext"');
    expect(html).toContain("plain text");
  });

  it("renders with language: plaintext", () => {
    const html = plaintextHl.highlight("hello world", htmlInline({ language: plaintext, theme }));
    expect(html).toContain('class="language-plaintext"');
    expect(html).not.toContain("<span");
  });

  it("renders with plaintext Language", () => {
    const html = plaintextHl.highlight("hello world", htmlInline({ language: plaintext, theme }));
    expect(html).toContain('class="language-plaintext"');
    expect(html).not.toContain("<span");
  });

  it("works with stateless highlight()", async () => {
    const html = await highlight("hello", htmlInline({ language: plaintext, theme }));
    expect(html).toContain('class="language-plaintext"');
  });

  it("adds custom class to pre element", () => {
    const html = plaintextHl.highlight(
      "text",
      htmlInline({ language: plaintext, theme, preClass: "my-class" }),
    );
    expect(html).toContain('class="lumis my-class"');
  });

  it("wraps with header element", () => {
    const html = plaintextHl.highlight(
      "text",
      htmlInline({
        language: plaintext,
        theme,
        header: { openTag: "<div>", closeTag: "</div>" },
      }),
    );
    expect(html).toMatch(/^<div>/);
    expect(html).toMatch(/<\/div>$/);
  });

  it("renders when createHighlighter() is created without explicit languages", async () => {
    const defaultHl = await createHighlighter();
    const html = defaultHl.highlight("plain text", htmlInline({ theme }));
    expect(html).toContain('class="language-plaintext"');
  });
});

describe("highlight() async", () => {
  it("auto-loads Language", async () => {
    const html = await highlight('{"a": 1}', htmlInline({ language: json, theme }));
    expect(html).toContain("<span");
    expect(html).toContain('class="language-json"');
  });

  it("renders plaintext when language is omitted", async () => {
    const html = await highlight("hello", htmlInline({ theme }));
    expect(html).toContain('class="language-plaintext"');
  });

  it.each(guessCases)("guesses language internally: $name", async ({ input, source, expected }) => {
    const html = await highlight(
      source,
      htmlInline({ ...(input == null ? {} : { language: input }), theme: draculaTheme }),
    );
    expect(html).toContain(`class="language-${expected}"`);
  });
});

describe("htmlMultiThemes", () => {
  it("renders with two themes and CSS variables", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
        defaultTheme: "light",
      }),
    );
    expect(html).toContain("lumis-themes");
    expect(html).toContain('class="language-json"');
    expect(html).toContain("--lumis");
  });

  it("inlines the default theme colors", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
        defaultTheme: "dark",
      }),
    );
    expect(html).toContain("lumis-themes");
    expect(html).toContain("style=");
  });

  it("uses custom CSS variable prefix", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
        defaultTheme: "light",
        cssVariablePrefix: "--my-prefix",
      }),
    );
    expect(html).toContain("--my-prefix");
  });

  it("adds custom class to pre element", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
        preClass: "custom-class",
      }),
    );
    expect(html).toContain("custom-class");
  });

  it("accepts italic option", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
        defaultTheme: "light",
        italic: true,
      }),
    );
    expect(html).toContain("lumis-themes");
    expect(html).toContain("style=");
  });

  it("adds data-highlight attributes when enabled", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
        includeHighlights: true,
      }),
    );
    expect(html).toContain("data-highlight=");
  });

  it("highlights lines with the default theme's own colour", () => {
    const dark = {
      ...draculaTheme,
      highlights: {
        ...draculaTheme.highlights,
        highlighted: { ...draculaTheme.highlights.highlighted, fg: "#eeeeee" },
      },
    };
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark },
        defaultTheme: "dark",
        highlightLines: { lines: [1], style: "theme" },
      }),
    );
    expect(html).toContain('style="color: #eeeeee; background-color: #44475a;"');
  });

  function renderLightDarkHighlightedLine(
    lightBackground: string | undefined,
    darkBackground: string | undefined,
  ): string {
    const light = {
      ...theme,
      highlights: {
        ...theme.highlights,
        highlighted: {
          ...theme.highlights.highlighted,
          fg: "#111111",
          bg: lightBackground,
        },
      },
    };
    const dark = {
      ...draculaTheme,
      highlights: {
        ...draculaTheme.highlights,
        highlighted: {
          ...draculaTheme.highlights.highlighted,
          fg: "#eeeeee",
          bg: darkBackground,
        },
      },
    };

    return hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light, dark },
        defaultTheme: "light-dark()",
        highlightLines: { lines: [1], style: "theme" },
      }),
    );
  }

  it("highlights lines in both colour schemes", () => {
    const html = renderLightDarkHighlightedLine("#2f334d", "#44475a");

    expect(html).toContain('style="background-color: light-dark(#2f334d, #44475a);"');
  });

  it.each([
    ["light", undefined, "#44475a"],
    ["dark", "#2f334d", undefined],
  ])("does not style lines without the %s highlighted background", (_theme, lightBg, darkBg) => {
    const html = renderLightDarkHighlightedLine(lightBg, darkBg);

    expect(html).toContain('<div class="l-line" data-line="1">');
  });

  it("wraps output with header element", () => {
    const html = hl.highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
        header: {
          openTag: '<div class="multi-wrapper">',
          closeTag: "</div>",
        },
      }),
    );
    expect(html).toMatch(/^<div class="multi-wrapper">/);
    expect(html).toMatch(/<\/div>$/);
  });

  it("works with stateless highlight()", async () => {
    const html = await highlight(
      '{"a": 1}',
      htmlMultiThemes({
        language: json,
        themes: { light: theme, dark: draculaTheme },
      }),
    );
    expect(html).toContain("lumis-themes");
    expect(html).toContain('class="language-json"');
  });
});

describe("terminal", () => {
  it("produces ANSI escape codes with theme", () => {
    const output = hl.highlight('{"a": 1}', terminal({ language: json, theme }));
    expect(output).toContain("\u001B[");
  });

  it("outputs plain text without theme", () => {
    const output = hl.highlight('{"a": 1}', terminal({ language: json }));
    expect(output).toContain('{"a": 1}');
    expect(output).not.toContain("\u001B[");
  });

  it("works with stateless highlight()", async () => {
    const output = await highlight('{"a": 1}', terminal({ language: json, theme }));
    expect(output).toContain("\u001B[");
  });

  // The native addon formats in Rust and the Wasm runtime formats in
  // JavaScript. Every terminal option has to survive whichever boundary it
  // crosses; `background` and `width` did not, and nothing noticed.
  it("honours background and width on whichever runtime formats", () => {
    const output = hl.highlight(
      '{"a": 1}',
      terminal({ language: json, theme, background: "theme", width: 40 }),
    );

    // tokyonight_moon's `normal` background, #222436.
    expect(output).toContain("48;2;34;36;54");
    expect(output).toContain("      ");
  });
});

describe("highlightLines", () => {
  it("htmlInline: highlighted lines get style attribute", () => {
    const html = hl.highlight(
      '{"a": 1}\n{"b": 2}\n{"c": 3}',
      htmlInline({
        language: json,
        theme,
        highlightLines: { lines: [[2, 2]] },
      }),
    );
    expect(html).toMatch(/style="[^"]*"[^>]*data-line="2"|data-line="2"[^>]*style="/);
  });

  it("htmlInline: highlighted lines with custom class", () => {
    const html = hl.highlight(
      '{"a": 1}\n{"b": 2}\n{"c": 3}',
      htmlInline({
        language: json,
        theme,
        highlightLines: { lines: [[1, 1]], class: "my-highlight" },
      }),
    );
    expect(html).toContain("my-highlight");
  });

  it("htmlInline: highlighted lines with custom style string", () => {
    const html = hl.highlight(
      '{"a": 1}\n{"b": 2}',
      htmlInline({
        language: json,
        theme,
        highlightLines: {
          lines: [[1, 2]],
          style: "background-color: yellow",
        },
      }),
    );
    expect(html).toContain("background-color: yellow");
  });

  it("htmlLinked: highlighted lines get default class", () => {
    const html = hl.highlight(
      '{"a": 1}\n{"b": 2}\n{"c": 3}',
      htmlLinked({
        language: json,
        highlightLines: { lines: [[2, 2]] },
      }),
    );
    expect(html).toContain("highlighted");
  });

  it("htmlLinked: highlighted lines with custom class name", () => {
    const html = hl.highlight(
      '{"a": 1}\n{"b": 2}\n{"c": 3}',
      htmlLinked({
        language: json,
        highlightLines: { lines: [[1, 3]], class: "hl-line" },
      }),
    );
    expect(html).toContain("hl-line");
  });

  it("terminal: a highlighted line takes the theme's highlighted background", () => {
    const output = hl.highlight(
      '{"a": 1}\n{"b": 2}',
      terminal({ language: json, theme, highlightLines: { lines: [[1, 1]] } }),
    );

    // tokyonight_moon's `highlighted` background, #2f334d.
    expect(output).toContain("48;2;47;51;77");
  });

  it("terminal: an explicit background wins over the theme", () => {
    const output = hl.highlight(
      '{"a": 1}\n{"b": 2}',
      terminal({
        language: json,
        theme,
        highlightLines: { lines: [[2, 2]], background: "#ff0000" },
      }),
    );

    expect(output).toContain("48;2;255;0;0");
  });

  it("bbcodeScoped: a highlighted line is wrapped in [highlighted]", () => {
    const output = hl.highlight(
      '{"a": 1}\n{"b": 2}',
      bbcodeScoped({ language: json, highlightLines: { lines: [[1, 1]] } }),
    );

    expect(output).toContain("[highlighted]");
    expect(output).toContain("[/highlighted]");
  });

  it("bbcodeScoped: no lines means no line structure at all", () => {
    const plain = hl.highlight('{"a": 1}\n{"b": 2}', bbcodeScoped({ language: json }));

    expect(plain).not.toContain("[highlighted]");
  });
});

describe("package exports", () => {
  it("does not expose defineFormatter on the formatter entrypoint", () => {
    expect("defineFormatter" in formatterApi).toBe(false);
  });
});
