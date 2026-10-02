import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
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

// speed-highlight names its grammars after file extensions and renders a name it
// cannot load as plain text, so the name is given here. It has no TSX grammar, and
// TSX goes to TypeScript, as highlight.js's and Sugar High's own aliases send it.
const speedHighlightLanguages = { html: "html", rust: "rs", go: "go", markdown: "md", tsx: "ts" };

/**
 * The libraries the comparison sets beside Lumis, each built once.
 *
 * `highlight` is the library's own call, and the only work the comparison
 * times, so the time on the page is the time to produce the output beside it.
 * It returns nothing for a document the library has no grammar for.
 * `fragment` wraps that output for the page: the theme's stylesheet, and the
 * element the library leaves its caller to write.
 */
export async function loadLibraries({ assetsDir, documents, themes }) {
  const shiki = await createShikiHighlighter({
    langs: [...new Set(documents.map((document) => document.language))],
    themes: themes.map((theme) => theme.shiki),
    engine: createOnigurumaEngine(import("shiki/wasm")),
  });
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
  const styles = new Map(
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
            speedHighlight: speedHighlightCss(palette, speedHighlightTokens),
            starryNight: starryNightCss(palette, starryNightStylesheet),
          },
        ];
      }),
    ),
  );

  const libraries = [
    {
      id: "shiki",
      highlight: (source, document, theme) =>
        shiki.codeToHtml(source, { lang: document.language, theme: theme.shiki }),
      fragment: (output) => output,
    },
    {
      id: "highlight-js",
      highlight: (source, document) =>
        highlightJs.highlight(source, { language: document.language }).value,
      fragment: (output, document, theme) =>
        `<style>${highlightJsThemes.get(theme.id)}</style>` +
        `<pre><code class="hljs language-${document.language}">${output}</code></pre>`,
    },
    // A language TanStack Highlight lacks comes back as plain text under its
    // fallback name. That is no output to compare, so it counts as no grammar, and
    // the document has to have declared it unsupported. The same goes for every
    // library below that has no grammar for the document.
    {
      id: "tanstack-highlight",
      highlight(source, document) {
        const result = tanStack.highlight(source, { lang: document.language });
        return result.lang === document.language ? result.html : undefined;
      },
      fragment: (output, document, theme) =>
        `<style>${styles.get(theme.id).tanStack}</style>${output}`,
    },
    {
      id: "sugar-high",
      highlight(source, document) {
        const lang = sugarHighLanguage(document.language);
        return lang ? highlightSugarHigh(source, { lang }) : undefined;
      },
      fragment: (output, document, theme) =>
        `<style>${styles.get(theme.id).sugarHigh}</style><pre><code>${output}</code></pre>`,
    },
    {
      id: "prism",
      highlight(source, document) {
        const grammar = Prism.languages[document.language];
        return grammar ? Prism.highlight(source, grammar, document.language) : undefined;
      },
      fragment(output, document, theme) {
        const prismClass = `language-${document.language}`;
        return (
          `<style>${styles.get(theme.id).prism}</style>` +
          `<pre class="${prismClass}"><code class="${prismClass}">${output}</code></pre>`
        );
      },
    },
    {
      id: "speed-highlight",
      highlight(source, document) {
        const lang = speedHighlightLanguages[document.language];
        return lang ? highlightSpeedHighlight(source, lang, { block: false }) : undefined;
      },
      fragment: (output, document, theme) =>
        `<style>${styles.get(theme.id).speedHighlight}</style>` +
        `<pre class="shj-lang-${speedHighlightLanguages[document.language]}"><code>${output}</code></pre>`,
    },
    {
      id: "starry-night",
      highlight(source, document) {
        const scope = starryNight.flagToScope(document.language);
        return scope ? toHtml(starryNight.highlight(source, scope)) : undefined;
      },
      fragment: (output, document, theme) =>
        `<style>${styles.get(theme.id).starryNight}</style><pre><code>${output}</code></pre>`,
    },
  ];

  return {
    libraries,
    dispose() {
      shiki.dispose();
    },
  };
}

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

function speedHighlightCss(palette, speedHighlightTokens) {
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
function starryNightCss(palette, starryNightStylesheet) {
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
