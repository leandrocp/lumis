import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import highlightJs from "highlight.js";
import { createHighlighter as createShikiHighlighter } from "shiki";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";
import { createHighlighter as createTanStackHighlighter } from "@tanstack/highlight/core";
import * as tanStackLanguages from "@tanstack/highlight/languages";
import { createThemeCss, themeTokenClasses } from "@tanstack/highlight/theme";
import { highlight as highlightSugarHigh } from "sugar-high";
import { SugarHigh } from "sugar-high/core";
import { lang as sugarHighLanguage } from "sugar-high/lang";
import Prism from "prismjs";
import loadPrismLanguages from "prismjs/components/index.js";
import { highlightHTML as highlightSpeedHighlight } from "@speed-highlight/core";
import { all as starryNightGrammars, createStarryNight } from "@wooorm/starry-night";
import { toHtml } from "hast-util-to-html";
import { createHighlighter, runtimeKind, withWasm } from "@lumis-sh/lumis";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import bash from "@lumis-sh/lumis/langs/bash";
import comment from "@lumis-sh/lumis/langs/comment";
import css from "@lumis-sh/lumis/langs/css";
import elixir from "@lumis-sh/lumis/langs/elixir";
import go from "@lumis-sh/lumis/langs/go";
import heex from "@lumis-sh/lumis/langs/heex";
import html from "@lumis-sh/lumis/langs/html";
import javascript from "@lumis-sh/lumis/langs/javascript";
import json from "@lumis-sh/lumis/langs/json";
import markdown from "@lumis-sh/lumis/langs/markdown";
import markdownInline from "@lumis-sh/lumis/langs/markdown_inline";
import java from "@lumis-sh/lumis/langs/java";
import rust from "@lumis-sh/lumis/langs/rust";
import tsx from "@lumis-sh/lumis/langs/tsx";
import bashWasm from "@lumis-sh/wasm-bash";
import commentWasm from "@lumis-sh/wasm-comment";
import cssWasm from "@lumis-sh/wasm-css";
import elixirWasm from "@lumis-sh/wasm-elixir";
import goWasm from "@lumis-sh/wasm-go";
import heexWasm from "@lumis-sh/wasm-heex";
import htmlWasm from "@lumis-sh/wasm-html";
import javascriptWasm from "@lumis-sh/wasm-javascript";
import jsonWasm from "@lumis-sh/wasm-json";
import markdownWasm from "@lumis-sh/wasm-markdown";
import markdownInlineWasm from "@lumis-sh/wasm-markdown_inline";
import javaWasm from "@lumis-sh/wasm-java";
import rustWasm from "@lumis-sh/wasm-rust";
import tsxWasm from "@lumis-sh/wasm-tsx";

const benchmarksDir = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const generatedDir = resolve(benchmarksDir, "showcase/generated");
const assetsDir = resolve(generatedDir, "assets");
const documents = JSON.parse(await readFile(resolve(assetsDir, "documents.json"), "utf8"));
const themes = JSON.parse(await readFile(resolve(assetsDir, "themes.json"), "utf8"));
const localPackages = JSON.parse(
  await readFile(
    resolve(benchmarksDir, "../target/benchmarks/language-packages/index.json"),
    "utf8",
  ),
);
const languagePackageResolver = (packageName) => {
  const local = localPackages[packageName];
  if (!local) throw new Error(`missing local language package ${packageName}`);
  return pathToFileURL(local.metadataPath);
};
const languages = [
  withWasm(html, htmlWasm),
  withWasm(comment, commentWasm),
  withWasm(css, cssWasm),
  withWasm(json, jsonWasm),
  withWasm(javascript, javascriptWasm),
  withWasm(rust, rustWasm),
  withWasm(elixir, elixirWasm),
  withWasm(heex, heexWasm),
  withWasm(go, goWasm),
  withWasm(markdown, markdownWasm),
  withWasm(markdownInline, markdownInlineWasm),
  withWasm(bash, bashWasm),
  withWasm(java, javaWasm),
  withWasm(tsx, tsxWasm),
];
const byId = new Map(languages.map((language) => [language.id, language]));

const lumis = await createHighlighter({ languages, languagePackageResolver });
const lumisId = runtimeKind() === "native" ? "lumis-js-node" : "lumis-js-wasm";
const shiki =
  process.env.BENCH_SHOWCASE_LUMIS_ONLY === "1"
    ? undefined
    : await createShikiHighlighter({
        langs: [...new Set(documents.map((document) => document.language))],
        themes: themes.map((theme) => theme.shiki),
        engine: createOnigurumaEngine(import("shiki/wasm")),
      });
const lumisThemes = new Map(
  await Promise.all(
    themes.map(async (theme) => [
      theme.id,
      (await import(`@lumis-sh/themes/${theme.lumis}`)).default,
    ]),
  ),
);
const highlightJsThemes = new Map(
  await Promise.all(
    themes.map(async (theme) => [
      theme.id,
      await readFile(fileURLToPath(import.meta.resolve(theme.highlightJs)), "utf8"),
    ]),
  ),
);
// Like highlight.js, TanStack Highlight, Prism and starry-night get every language
// they ship, so a README fence they can read is highlighted rather than left plain.
const tanStack = createTanStackHighlighter({ languages: Object.values(tanStackLanguages) });
loadPrismLanguages();
const starryNight = await createStarryNight(starryNightGrammars);
const starryNightStylesheet = await readFile(
  fileURLToPath(import.meta.resolve("@wooorm/starry-night/style/core")),
  "utf8",
);
// speed-highlight's default theme colours every token class its grammars emit, so
// it is the list a theme has to cover.
const speedHighlightTokens = [
  ...new Set(
    Array.from(
      (
        await readFile(
          fileURLToPath(import.meta.resolve("@speed-highlight/core/themes/default.css")),
          "utf8",
        )
      ).matchAll(/\.shj-syn-([a-z]+)/g),
      ([, token]) => token,
    ),
  ),
];
// speed-highlight names its grammars after file extensions and renders a name it
// cannot load as plain text, so the name is given here. It has no TSX grammar, and
// TSX goes to TypeScript, as highlight.js's and Sugar High's own aliases send it.
const speedHighlightLanguages = { html: "html", rust: "rs", go: "go", markdown: "md", tsx: "ts" };
const libraryThemes = new Map(
  await Promise.all(
    themes.map(async (theme) => {
      const palette = highlightJsPalette(theme, highlightJsThemes.get(theme.id));
      return [
        theme.id,
        {
          tanStack: createThemeCss({
            themes: [{ selector: ":root", theme: tanStackTheme(theme, palette) }],
          }),
          sugarHigh: sugarHighCss(palette),
          prism: await readFile(resolve(assetsDir, theme.prism), "utf8"),
          speedHighlight: speedHighlightCss(palette),
          starryNight: starryNightCss(palette),
        },
      ];
    }),
  ),
);

for (const document of documents) {
  const source = await readFile(resolve(assetsDir, document.file), "utf8");
  const language = byId.get(document.language);
  if (!language)
    throw new Error(`showcase document names an unloaded language: ${document.language}`);

  for (const theme of themes) {
    const fragmentsDir = resolve(generatedDir, "fragments", document.id, theme.id);
    await mkdir(fragmentsDir, { recursive: true });

    const lumisOutput = lumis.highlight(
      source,
      htmlInline({ language, theme: lumisThemes.get(theme.id) }),
    );
    validate(lumisOutput, source, lumisId);
    await writeFile(resolve(fragmentsDir, `${lumisId}.html`), lumisOutput);

    // The runtime is chosen once per process, so the showcase runs this script twice
    // to render both. Shiki and highlight.js do not vary with it, and rebuilding the
    // Oniguruma engine for a second identical result is the slowest step here.
    if (!shiki) continue;

    const shikiOutput = shiki.codeToHtml(source, { lang: document.language, theme: theme.shiki });
    validate(shikiOutput, source, "Shiki");
    await writeFile(resolve(fragmentsDir, "shiki.html"), shikiOutput);

    const highlightJsOutput =
      `<style>${highlightJsThemes.get(theme.id)}</style>` +
      `<pre><code class="hljs language-${document.language}">` +
      `${highlightJs.highlight(source, { language: document.language }).value}</code></pre>`;
    validate(highlightJsOutput, source, "highlight.js");
    await writeFile(resolve(fragmentsDir, "highlight-js.html"), highlightJsOutput);

    const styles = libraryThemes.get(theme.id);

    // A language TanStack Highlight lacks comes back as plain text under its
    // fallback name. That is no output to compare, so nothing is written, and the
    // document has to have declared it unsupported. The same goes for every
    // library below that has no grammar for the document.
    const tanStackResult = tanStack.highlight(source, { lang: document.language });
    if (tanStackResult.lang === document.language) {
      const tanStackOutput = `<style>${styles.tanStack}</style>${tanStackResult.html}`;
      validate(tanStackOutput, source, "TanStack Highlight");
      await writeFile(resolve(fragmentsDir, "tanstack-highlight.html"), tanStackOutput);
    }

    const sugarHighLang = sugarHighLanguage(document.language);
    if (sugarHighLang) {
      const sugarHighOutput =
        `<style>${styles.sugarHigh}</style>` +
        `<pre><code>${highlightSugarHigh(source, { lang: sugarHighLang })}</code></pre>`;
      validate(sugarHighOutput, source, "Sugar High");
      await writeFile(resolve(fragmentsDir, "sugar-high.html"), sugarHighOutput);
    }

    const prismGrammar = Prism.languages[document.language];
    if (prismGrammar) {
      const prismClass = `language-${document.language}`;
      const prismOutput =
        `<style>${styles.prism}</style>` +
        `<pre class="${prismClass}"><code class="${prismClass}">` +
        `${Prism.highlight(source, prismGrammar, document.language)}</code></pre>`;
      validate(prismOutput, source, "Prism");
      await writeFile(resolve(fragmentsDir, "prism.html"), prismOutput);
    }

    const speedHighlightLang = speedHighlightLanguages[document.language];
    if (speedHighlightLang) {
      const speedHighlightOutput =
        `<style>${styles.speedHighlight}</style>` +
        `<pre class="shj-lang-${speedHighlightLang}"><code>` +
        `${await highlightSpeedHighlight(source, speedHighlightLang, { block: false })}</code></pre>`;
      validate(speedHighlightOutput, source, "speed-highlight");
      await writeFile(resolve(fragmentsDir, "speed-highlight.html"), speedHighlightOutput);
    }

    const starryNightScope = starryNight.flagToScope(document.language);
    if (starryNightScope) {
      const starryNightOutput =
        `<style>${styles.starryNight}</style>` +
        `<pre><code>${toHtml(starryNight.highlight(source, starryNightScope))}</code></pre>`;
      validate(starryNightOutput, source, "starry-night");
      await writeFile(resolve(fragmentsDir, "starry-night.html"), starryNightOutput);
    }
  }
}

shiki?.dispose();

// TanStack Highlight, Sugar High, speed-highlight and starry-night ship no
// Catppuccin. Rather than choose their colours here, each theme is read out of the
// @catppuccin/highlightjs stylesheet the highlight.js output uses: a token class
// takes the colour of the highlight.js class that marks the same thing, and one
// that highlight.js has no class for (`null` below) keeps the text colour, as it
// would in highlight.js's output.
function highlightJsPalette(theme, stylesheet) {
  const declaration = (selector, property) => {
    const rule = stylesheet.match(
      new RegExp(`(?:^|\\})${selector.replaceAll(".", "\\.")}\\{([^}]*)\\}`),
    )?.[1];
    const value = rule?.match(new RegExp(`(?:^|;)${property}:([^;]+)`, "i"))?.[1];
    if (!value) throw new Error(`${theme.highlightJs} has no ${property} for ${selector}`);
    return value;
  };
  const foreground = declaration("code.hljs", "color");

  return {
    background: declaration("code.hljs", "background"),
    foreground,
    colour: (className) =>
      className === null ? foreground : declaration(`code .hljs-${className}`, "color"),
    // Only highlight.js's diff classes paint a background.
    fill: (className) =>
      className === null ? "transparent" : declaration(`code .hljs-${className}`, "background"),
    // These dependencies float like the other comparison libraries, so a token class
    // a later release adds fails here rather than reaching the page uncoloured.
    assertMapped(library, tokens, highlightJsClasses) {
      const unmapped = tokens.filter((token) => !Object.hasOwn(highlightJsClasses, token));
      if (unmapped.length > 0) {
        throw new Error(`no highlight.js class is mapped to ${library}'s ${unmapped.join(", ")}`);
      }
    },
  };
}

function tanStackTheme(theme, palette) {
  const highlightJsClasses = {
    attr: "attr",
    "code-inline": "code",
    command: "built_in",
    comment: "comment",
    deleted: "deletion",
    function: "title.function_",
    heading: "section",
    inserted: "addition",
    keyword: "keyword",
    link: "link",
    literal: "literal",
    meta: "meta",
    number: "number",
    operator: "operator",
    property: "property",
    // highlight.js splits a selector into tag, id and class; TanStack Highlight
    // colours it whole, so it takes the class colour, the part stylesheets use most.
    selector: "selector-class",
    string: "string",
    // `tag` is the tag name; highlight.js's `hljs-tag` is the whole tag, brackets
    // and attributes included, and names the tag `hljs-name`.
    tag: "name",
    type: "type",
    variable: "variable",
  };
  palette.assertMapped(
    "TanStack Highlight",
    themeTokenClasses.filter((token) => token !== "token"),
    highlightJsClasses,
  );

  return {
    name: `catppuccin-${theme.id}`,
    type: theme.appearance,
    background: palette.background,
    foreground: palette.foreground,
    tokens: {
      token: palette.foreground,
      ...Object.fromEntries(
        Object.entries(highlightJsClasses).map(([token, className]) => [
          token,
          palette.colour(className),
        ]),
      ),
    },
  };
}

// Sugar High colours each token through a `--sh-<type>` custom property.
function sugarHighCss(palette) {
  const highlightJsClasses = {
    // Whitespace, which it wraps in spans of its own.
    break: null,
    space: null,
    // Capitalised names, numbers and `null`: everything it reads as a value
    // rather than a variable.
    class: "title.class_",
    comment: "comment",
    // JSX and HTML tag names, as TanStack Highlight's `tag` above.
    entity: "name",
    identifier: null,
    // JSX text.
    jsxliterals: null,
    keyword: "keyword",
    property: "property",
    // Operators and punctuation alike.
    sign: "punctuation",
    string: "string",
  };
  palette.assertMapped("Sugar High", SugarHigh.TokenTypes, highlightJsClasses);
  const properties = SugarHigh.TokenTypes.map(
    (token) => `--sh-${token}:${palette.colour(highlightJsClasses[token])}`,
  );

  return `pre{background:${palette.background};color:${palette.foreground};${properties.join(";")}}`;
}

function speedHighlightCss(palette) {
  const highlightJsClasses = {
    bool: "literal",
    // Capitalised names, and HTML attribute names.
    class: "title.class_",
    cmnt: "comment",
    deleted: "deletion",
    err: null,
    func: "title.function_",
    insert: "addition",
    kwd: "keyword",
    num: "number",
    oper: "operator",
    section: "section",
    str: "string",
    type: "type",
    // Variables, and HTML tag names.
    var: "variable",
  };
  palette.assertMapped("speed-highlight", speedHighlightTokens, highlightJsClasses);
  const rules = speedHighlightTokens.map(
    (token) => `.shj-syn-${token}{color:${palette.colour(highlightJsClasses[token])}}`,
  );

  return (
    `[class*=shj-lang-]{background:${palette.background};color:${palette.foreground}}` +
    rules.join("")
  );
}

// starry-night's stylesheet colours its classes through GitHub's
// `--color-prettylights-syntax-*` custom properties, so the theme is a value for
// each of them, read from the stylesheet itself.
function starryNightCss(palette) {
  const highlightJsClasses = {
    "brackethighlighter-angle": "punctuation",
    "brackethighlighter-unmatched": null,
    "carriage-return-bg": null,
    "carriage-return-text": null,
    comment: "comment",
    constant: "variable.constant_",
    "constant-other-reference-link": "link",
    entity: "title",
    "entity-tag": "name",
    "invalid-illegal-bg": null,
    "invalid-illegal-text": null,
    keyword: "keyword",
    "markup-bold": "strong",
    "markup-changed-bg": null,
    "markup-changed-text": null,
    "markup-deleted-bg": "deletion",
    "markup-deleted-text": "deletion",
    "markup-heading": "section",
    "markup-ignored-bg": null,
    "markup-ignored-text": null,
    "markup-inserted-bg": "addition",
    "markup-inserted-text": "addition",
    "markup-italic": "emphasis",
    "markup-list": "bullet",
    "meta-diff-range": "meta",
    // GitHub gives it the text colour too.
    "storage-modifier-import": null,
    string: "string",
    "string-regexp": "regexp",
    "sublimelinter-gutter-mark": null,
    variable: "variable",
  };
  const variables = [
    ...new Set(
      Array.from(
        starryNightStylesheet.matchAll(/var\(--color-prettylights-syntax-([a-z-]+)\)/g),
        ([, variable]) => variable,
      ),
    ),
  ];
  palette.assertMapped("starry-night", variables, highlightJsClasses);
  const properties = variables.map((variable) => {
    const className = highlightJsClasses[variable];
    const value = variable.endsWith("-bg") ? palette.fill(className) : palette.colour(className);
    return `--color-prettylights-syntax-${variable}:${value}`;
  });

  return (
    `:root{${properties.join(";")}}` +
    `pre{background:${palette.background};color:${palette.foreground}}` +
    starryNightStylesheet
  );
}

function validate(output, source, implementation) {
  if (
    Buffer.byteLength(output) <= Buffer.byteLength(source) ||
    !output.includes("<pre") ||
    !output.includes("<span")
  ) {
    throw new Error(`${implementation} did not produce highlighted HTML`);
  }
}
