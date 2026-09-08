import { createHighlighter, highlightEvents, highlightIter } from "../../src/index.browser.ts";
import {
  bbcodeScoped,
  htmlInline,
  htmlLinked,
  htmlMultiThemes,
  terminal,
  type Formatter,
} from "../../src/formatters.ts";
import { lowestCompatibleLanguagePackageVersion } from "../../src/core/languages.ts";
import type { LanguageDefinition, Theme } from "../../src/types.ts";

/**
 * The browser cannot load a parser during the walk that finds an injected
 * language, so it declares the whole corpus up front. Globbed rather than
 * listed, so a new fixture or a new query file is covered without editing this.
 */
const languageModules = import.meta.glob<{ default: LanguageDefinition }>("../../langs/*.ts", {
  eager: true,
});
const themeModules = import.meta.glob<{ default: Theme }>("../../../themes/themes/*.ts", {
  eager: true,
});
const fixtureThemeModules = import.meta.glob<{ default: Theme }>(
  "../../../../../fixtures/conformance-themes/*.json",
  { eager: true },
);
const parserWasms = import.meta.glob<string>("../../../../../fixtures/test-parsers/*.wasm", {
  eager: true,
  query: "?url",
  import: "default",
});
const queryFiles = import.meta.glob<string>("../../../../../queries/processed/*/*.scm", {
  eager: true,
  query: "?raw",
  import: "default",
});
const fixtureFiles = import.meta.glob<string>(
  "../../../../../fixtures/conformance/*/{source.txt,fixture.json}",
  { eager: true, query: "?raw", import: "default" },
);

/** Which parser each language is packaged with; `mdx` ships in markdown's. */
const PARSER_OF: Record<string, string> = { mdx: "markdown" };

function basename(path: string): string {
  return path.slice(path.lastIndexOf("/") + 1);
}

function query(language: string, kind: string): string {
  const entry = Object.entries(queryFiles).find(([path]) =>
    path.endsWith(`/queries/processed/${language}/${kind}.scm`),
  );
  return entry?.[1] ?? "";
}

interface CorpusFixture {
  name: string;
  language: string;
  /** Root plus every injected language, taken from the expected events. */
  languages: string[];
  rainbowBrackets: boolean;
  source: string;
  theme: string;
  htmlMultiThemes?: {
    themes: Record<string, string>;
    /** Absent means CSS-variables-only mode, which is its own rendering branch. */
    defaultTheme?: string;
    highlightLines?: number[];
  };
}

function loadCorpus(): CorpusFixture[] {
  const byFixture = new Map<string, { source?: string; metadata?: string }>();
  for (const [path, contents] of Object.entries(fixtureFiles)) {
    const name = path.split("/").at(-2) ?? "";
    const entry = byFixture.get(name) ?? {};
    if (basename(path) === "source.txt") entry.source = contents;
    else entry.metadata = contents;
    byFixture.set(name, entry);
  }

  return [...byFixture.entries()]
    .map(([name, { source, metadata }]) => {
      if (source === undefined || metadata === undefined) {
        throw new Error(`incomplete conformance fixture: ${name}`);
      }
      const parsed = JSON.parse(metadata) as {
        language: string;
        theme: string;
        rainbowBrackets?: boolean;
        htmlMultiThemes?: CorpusFixture["htmlMultiThemes"];
        events: { type: string; language?: string }[];
      };
      // Injection-only languages such as css are never a fixture's root, and
      // the browser cannot load one mid-walk, so they come from the events.
      const languages = new Set([parsed.language]);
      for (const event of parsed.events) {
        if (event.type === "start" && event.language) languages.add(event.language);
      }
      return {
        name,
        language: parsed.language,
        languages: [...languages],
        rainbowBrackets: parsed.rainbowBrackets ?? false,
        source,
        theme: parsed.theme,
        htmlMultiThemes: parsed.htmlMultiThemes,
      };
    })
    .sort((a, b) => a.name.localeCompare(b.name));
}

interface CustomFormatterResult {
  balancedEvents: boolean;
  eventCount: number;
  eventLanguages: string[];
  eventScopes: string[];
  maxDepth: number;
  reconstructedSource: string;
  resolvedLanguage: string;
  restoredLanguage: Formatter["language"];
  styledTokenCount: number;
  tokenCount: number;
  tokenLanguages: string[];
  tokenScopes: string[];
  unicodeToken?: {
    text: string;
    language: string;
    scope: string;
    startByte: number;
    endByte: number;
  };
}

export interface FixtureOutput {
  bbcodeScoped: string;
  htmlInline: string;
  htmlLinked: string;
  htmlMultiThemes: string;
  terminal: string;
}

export interface BrowserTestResult {
  customFormatter: CustomFormatterResult;
  /** Every conformance fixture, rendered through all five formatters. */
  fixtures: Record<string, FixtureOutput>;
  formatters: FixtureOutput;
  languages: string[];
  requestedWasms: string[];
}

declare global {
  interface Window {
    __lumisBrowserError?: string;
    __lumisBrowserResult?: BrowserTestResult;
  }
}

function languageId(language: Formatter["language"]): string {
  if (typeof language === "string") return language;
  return language?.id ?? "";
}

function getTheme(id: string): Theme {
  const module = [...Object.entries(themeModules), ...Object.entries(fixtureThemeModules)].find(
    ([path]) => [`.ts`, `.json`].some((extension) => basename(path) === `${id}${extension}`),
  );
  if (!module) throw new Error(`no theme module for ${id}`);
  return module[1].default;
}

// One package per parser, carrying every language that parser serves, because
// `mdx` and `markdown` share one and a package holding only the last would
// leave the other unresolvable.
function groupLanguagesByParser(needed: string[]): Map<string, string[]> {
  const languagesByParser = new Map<string, string[]>();

  for (const language of needed) {
    const parser = PARSER_OF[language] ?? language;
    languagesByParser.set(parser, [...(languagesByParser.get(parser) ?? []), language]);
  }

  return languagesByParser;
}

function languageDefinition(language: string): LanguageDefinition {
  const module = Object.entries(languageModules).find(
    ([path]) => basename(path) === `${language}.ts`,
  );
  if (!module) throw new Error(`no language module for ${language}`);
  return module[1].default;
}

function parserWasmUrl(parser: string, wasmName: string): string {
  const wasmEntry = Object.entries(parserWasms).find(
    ([path]) => basename(path) === `${wasmName}.wasm`,
  );
  if (!wasmEntry) throw new Error(`no committed parser fixture for ${parser}`);
  return wasmEntry[1];
}

async function run(): Promise<void> {
  const corpus = loadCorpus();
  const needed = [...new Set(corpus.flatMap((fixture) => fixture.languages))].sort();
  const requestedWasms: string[] = [];
  const wasmUrls: Record<string, string> = {};
  const languagePackages = new Map<string, string>();
  const definitions: LanguageDefinition[] = [];

  const languagesByParser = groupLanguagesByParser(needed);

  for (const language of needed) {
    definitions.push(languageDefinition(language));
  }

  for (const [parser, languages] of languagesByParser) {
    const wasmName = `tree-sitter-${parser}`;
    const wasmUrl = parserWasmUrl(parser, wasmName);

    wasmUrls[wasmName] = wasmUrl;
    languagePackages.set(
      `@lumis-sh/wasm-${parser}`,
      await languagePackageDataUrl(parser, languages, wasmUrl),
    );
  }

  const highlighter = await createHighlighter({
    languages: definitions,
    languagePackageResolver: (packageName) => {
      const url = languagePackages.get(packageName);
      if (!url) throw new Error(`Unexpected language package request: ${packageName}`);
      return url;
    },
    wasmResolver: (_language, wasm) => {
      requestedWasms.push(wasm.name);
      const url = wasmUrls[wasm.name];
      if (!url) throw new Error(`Unexpected parser request: ${wasm.name}`);
      return url;
    },
  });

  for (const definition of definitions) {
    await highlighter.loadLanguage(definition);
  }

  const render = (fixture: CorpusFixture): FixtureOutput => {
    const { source, language, rainbowBrackets } = fixture;
    const theme = getTheme(fixture.theme);
    const config = fixture.htmlMultiThemes;
    // Reversed on purpose. Rust holds themes in a `HashMap` and sorts before
    // emitting, so output must not depend on the order the caller inserted
    // them. Feeding them in already-sorted order would not prove that.
    const formatterThemes = config
      ? Object.fromEntries(
          Object.entries(config.themes)
            .toReversed()
            .map(([name, themeId]) => [name, getTheme(themeId)]),
        )
      : { main: theme };

    const options = { rainbowBrackets };

    return {
      htmlInline: highlighter.highlight(source, htmlInline({ language, theme }), options),
      htmlLinked: highlighter.highlight(source, htmlLinked({ language }), options),
      htmlMultiThemes: highlighter.highlight(
        source,
        htmlMultiThemes({
          language,
          themes: formatterThemes,
          defaultTheme: config ? config.defaultTheme : "main",
          highlightLines: config?.highlightLines?.length
            ? { lines: config.highlightLines, style: "theme" }
            : undefined,
        }),
        options,
      ),
      bbcodeScoped: highlighter.highlight(source, bbcodeScoped({ language }), options),
      terminal: highlighter.highlight(source, terminal({ language, theme }), options),
    };
  };

  const fixtures: Record<string, FixtureOutput> = {};
  for (const fixture of corpus) {
    fixtures[fixture.name] = render(fixture);
  }

  const fixtureSource = corpus.find(
    (fixture) => fixture.name === "javascript-html-template-nested-script-css",
  )?.source;
  if (!fixtureSource) throw new Error("the custom-formatter fixture is missing");
  const formatters = fixtures["javascript-html-template-nested-script-css"];

  const customFormatter: Formatter = {
    language: "js",
    render(source: string): string {
      const events = highlightEvents(source, this.language, { rainbowBrackets: true });
      const sourceBytes = new TextEncoder().encode(source);
      const decoder = new TextDecoder();
      const eventLanguages = new Set<string>();
      const eventScopes = new Set<string>();
      let depth = 0;
      let maxDepth = 0;
      let balancedEvents = true;
      let reconstructedSource = "";

      for (const event of events) {
        if (event.type === "start") {
          eventLanguages.add(event.language);
          eventScopes.add(event.scope);
          depth += 1;
          maxDepth = Math.max(maxDepth, depth);
        } else if (event.type === "end") {
          depth -= 1;
          balancedEvents &&= depth >= 0;
        } else {
          reconstructedSource += decoder.decode(
            sourceBytes.subarray(event.startByte, event.endByte),
          );
        }
      }
      balancedEvents &&= depth === 0;

      const tokenLanguages = new Set<string>();
      const tokenScopes = new Set<string>();
      let styledTokenCount = 0;
      let tokenCount = 0;
      let unicodeToken: CustomFormatterResult["unicodeToken"];

      highlightIter(
        source,
        this.language,
        getTheme("dracula"),
        (text, language, range, scope, style) => {
          tokenCount += 1;
          tokenLanguages.add(language);
          if (scope) tokenScopes.add(scope);
          if (style) styledTokenCount += 1;
          if (text.includes("😀")) {
            unicodeToken = {
              text,
              language,
              scope,
              startByte: range.start,
              endByte: range.end,
            };
          }
        },
      );

      return JSON.stringify({
        balancedEvents,
        eventCount: events.length,
        eventLanguages: [...eventLanguages].sort(),
        eventScopes: [...eventScopes].sort(),
        maxDepth,
        reconstructedSource,
        resolvedLanguage: languageId(this.language),
        styledTokenCount,
        tokenCount,
        tokenLanguages: [...tokenLanguages].sort(),
        tokenScopes: [...tokenScopes].sort(),
        unicodeToken,
      } satisfies Omit<CustomFormatterResult, "restoredLanguage">);
    },
  };

  const customSource = `const nested = foo(bar([1, 2], { a: "3" }));\nconst view = ${fixtureSource.trim()};\n`;
  const customFormatterResult = JSON.parse(
    highlighter.highlight(customSource, customFormatter),
  ) as Omit<CustomFormatterResult, "restoredLanguage">;

  window.__lumisBrowserResult = {
    customFormatter: {
      ...customFormatterResult,
      restoredLanguage: customFormatter.language,
    },
    fixtures,
    formatters,
    languages: highlighter.languages,
    requestedWasms,
  };
}

async function languagePackageDataUrl(
  parser: string,
  languages: string[],
  wasmUrl: string,
): Promise<string> {
  const wasm = new Uint8Array(await (await fetch(wasmUrl)).arrayBuffer());
  const digest = await crypto.subtle.digest("SHA-256", wasm);
  const sha256 = Array.from(new Uint8Array(digest), (byte) =>
    byte.toString(16).padStart(2, "0"),
  ).join("");
  const metadata = JSON.stringify({
    packageName: `@lumis-sh/wasm-${parser}`,
    version: lowestCompatibleLanguagePackageVersion(),
    definitionHash: sha256,
    parser: {
      name: `tree-sitter-${parser}`,
      grammarName: parser,
      sha256,
      size: wasm.byteLength,
    },
    languages: Object.fromEntries(
      languages.map((language) => [
        language,
        {
          aliases: [],
          highlights: query(language, "highlights"),
          injections: query(language, "injections"),
          locals: query(language, "locals"),
          brackets: query(language, "brackets") || query("default", "brackets"),
        },
      ]),
    ),
  });
  const bytes = new TextEncoder().encode(metadata);
  let binary = "";
  for (const byte of bytes) binary += String.fromCodePoint(byte);
  return `data:application/json;base64,${btoa(binary)}`;
}

try {
  await run();
} catch (error: unknown) {
  window.__lumisBrowserError = error instanceof Error ? error.stack : String(error);
}
