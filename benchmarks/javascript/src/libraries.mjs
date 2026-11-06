import { readFile } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parserBytes } from "./parsers.mjs";

// speed-highlight names its grammars after file extensions and renders a name it
// cannot load as plain text, so the name is given here. It has no TSX grammar, and
// TSX goes to TypeScript, as highlight.js's and Sugar High's own aliases send it.
const speedHighlightLanguages = {
  html: "html",
  rust: "rs",
  go: "go",
  markdown: "md",
  tsx: "ts",
  javascript: "js",
};

/**
 * The libraries the comparison sets beside Lumis. Each one imports and builds
 * only itself, so a process that loads one holds none of the others, which is
 * what lets the memory pass measure a library on its own.
 *
 * `highlight` is the library's own call, and the only work the comparison
 * times, so the time on the page is the time to produce the output beside it.
 * It returns nothing for a document the library has no grammar for.
 * `fragment` wraps that output for the page: the theme's stylesheet, and the
 * element the library leaves its caller to write.
 */
const loaders = {
  // The Node addon, built from local language packages as the showcase builds
  // it, so its output is the published Lumis output byte for byte.
  async "lumis-js-node"({ documents, themes }) {
    const [{ createHighlighter, runtimeKind }, { withWasm }, { htmlInline }] = await Promise.all([
      import("@lumis-sh/lumis"),
      import("@lumis-sh/lumis/languages"),
      import("@lumis-sh/lumis/formatters"),
    ]);
    if (runtimeKind() !== "native") throw new Error("the Lumis Node addon did not load");
    const localPackages = JSON.parse(
      await readFile(
        resolve(import.meta.dirname, "../../../target/benchmarks/language-packages/index.json"),
        "utf8",
      ),
    );
    const languages = await Promise.all(
      [...new Set(documents.flatMap((document) => document.load))].map(async (id) => {
        const { default: language } = await import(`@lumis-sh/lumis/langs/${id}`);
        return withWasm(language, await parserBytes(id));
      }),
    );
    const byId = new Map(languages.map((language) => [language.id, language]));
    const lumis = await createHighlighter({
      languages,
      languagePackageResolver: (packageName) => {
        const local = localPackages[packageName];
        if (!local) throw new Error(`missing local language package ${packageName}`);
        return pathToFileURL(local.metadataPath);
      },
    });
    const lumisThemes = new Map(
      await Promise.all(
        themes.map(async (theme) => [
          theme.id,
          (await import(`@lumis-sh/themes/${theme.lumis}`)).default,
        ]),
      ),
    );
    return {
      highlight: (source, document, theme) =>
        lumis.highlight(
          source,
          htmlInline({ language: byId.get(document.language), theme: lumisThemes.get(theme.id) }),
        ),
      fragment: (output) => output,
    };
  },

  async shiki({ documents, themes }) {
    const [{ createHighlighter }, { createOnigurumaEngine }] = await Promise.all([
      import("shiki"),
      import("shiki/engine/oniguruma"),
    ]);
    const shiki = await createHighlighter({
      langs: [...new Set(documents.map((document) => document.language))],
      themes: themes.map((theme) => theme.shiki),
      engine: createOnigurumaEngine(import("shiki/wasm")),
    });
    return {
      highlight: (source, document, theme) =>
        shiki.codeToHtml(source, { lang: document.language, theme: theme.shiki }),
      fragment: (output) => output,
      dispose: () => shiki.dispose(),
    };
  },

  async "highlight-js"({ themes }) {
    const { default: highlightJs } = await import("highlight.js");
    const stylesheets = await highlightJsStylesheets(themes);
    return {
      highlight: (source, document) =>
        highlightJs.highlight(source, { language: document.language }).value,
      fragment: (output, document, theme) =>
        `<style>${stylesheets.get(theme.id)}</style>` +
        `<pre><code class="hljs language-${document.language}">${output}</code></pre>`,
    };
  },

  // Highlights ships Catppuccin ports of its own. Its output is a view into its
  // WebAssembly memory that the next call overwrites, so decoding it is part of
  // producing the HTML and is timed with the call.
  async "pierre-highlights"({ themes }) {
    const [{ codeToHtml, isSupportedLanguage }, bundledThemes] = await Promise.all([
      import("@pierre/highlights"),
      import("@pierre/highlights/themes"),
    ]);
    const decoder = new TextDecoder();
    const highlightsThemes = new Map(
      themes.map((theme) => {
        const highlightsTheme = bundledThemes[theme.pierreHighlights];
        if (!highlightsTheme) {
          throw new Error(`@pierre/highlights has no ${theme.pierreHighlights} theme`);
        }
        return [theme.id, highlightsTheme];
      }),
    );
    return {
      highlight: (source, document, theme) =>
        isSupportedLanguage(document.language)
          ? decoder.decode(
              codeToHtml(source, {
                lang: document.language,
                theme: highlightsThemes.get(theme.id),
              }),
            )
          : undefined,
      fragment: (output) => output,
    };
  },

  // A language TanStack Highlight lacks comes back as plain text under its
  // fallback name. That is no output to compare, so it counts as no grammar, and
  // the document has to have declared it unsupported. The same goes for every
  // library below that has no grammar for the document.
  async "tanstack-highlight"({ themes }) {
    const [{ createHighlighter }, languages, { createThemeCss, themeTokenClasses }] =
      await Promise.all([
        import("@tanstack/highlight/core"),
        import("@tanstack/highlight/languages"),
        import("@tanstack/highlight/theme"),
      ]);
    // Like highlight.js, TanStack Highlight, Prism and starry-night get every
    // language they ship, so a README fence they can read is highlighted rather
    // than left plain.
    const tanStack = createHighlighter({ languages: Object.values(languages) });
    const styles = await stylesFor(themes, (theme, palette) =>
      createThemeCss({
        themes: [{ selector: ":root", theme: tanStackTheme(theme, palette, themeTokenClasses) }],
      }),
    );
    return {
      highlight(source, document) {
        const result = tanStack.highlight(source, { lang: document.language });
        // It answers an alias, `javascript`, under its own name, `js`.
        return result.lang === tanStack.normalizeLanguage(document.language)
          ? result.html
          : undefined;
      },
      fragment: (output, document, theme) => `<style>${styles.get(theme.id)}</style>${output}`,
    };
  },

  async "sugar-high"({ themes }) {
    const [{ highlight }, { SugarHigh }, { lang: sugarHighLanguage }] = await Promise.all([
      import("sugar-high"),
      import("sugar-high/core"),
      import("sugar-high/lang"),
    ]);
    const styles = await stylesFor(themes, (_theme, palette) =>
      sugarHighCss(palette, SugarHigh.TokenTypes),
    );
    return {
      highlight(source, document) {
        const lang = sugarHighLanguage(document.language);
        return lang ? highlight(source, { lang }) : undefined;
      },
      fragment: (output, document, theme) =>
        `<style>${styles.get(theme.id)}</style><pre><code>${output}</code></pre>`,
    };
  },

  async prism({ assetsDir, themes }) {
    const [{ default: Prism }, { default: loadPrismLanguages }] = await Promise.all([
      import("prismjs"),
      import("prismjs/components/index.js"),
    ]);
    loadPrismLanguages();
    const styles = new Map(
      await Promise.all(
        themes.map(async (theme) => [
          theme.id,
          await readFile(resolve(assetsDir, theme.prism), "utf8"),
        ]),
      ),
    );
    return {
      highlight(source, document) {
        const grammar = Prism.languages[document.language];
        return grammar ? Prism.highlight(source, grammar, document.language) : undefined;
      },
      fragment(output, document, theme) {
        const prismClass = `language-${document.language}`;
        return (
          `<style>${styles.get(theme.id)}</style>` +
          `<pre class="${prismClass}"><code class="${prismClass}">${output}</code></pre>`
        );
      },
    };
  },

  async "speed-highlight"({ themes }) {
    const { highlightHTML } = await import("@speed-highlight/core");
    // speed-highlight's default theme colours every token class its grammars emit,
    // so it is the list a theme has to cover.
    const tokens = [
      ...new Set(
        Array.from(
          (await readPackageFile("@speed-highlight/core/themes/default.css")).matchAll(
            /\.shj-syn-([a-z]+)/gu,
          ),
          ([, token]) => token,
        ),
      ),
    ];
    const styles = await stylesFor(themes, (_theme, palette) => speedHighlightCss(palette, tokens));
    return {
      highlight(source, document) {
        const lang = speedHighlightLanguages[document.language];
        return lang ? highlightHTML(source, lang, { block: false }) : undefined;
      },
      fragment: (output, document, theme) =>
        `<style>${styles.get(theme.id)}</style>` +
        `<pre class="shj-lang-${speedHighlightLanguages[document.language]}"><code>${output}</code></pre>`,
    };
  },

  async "starry-night"({ themes }) {
    const [{ all, createStarryNight }, { toHtml }] = await Promise.all([
      import("@wooorm/starry-night"),
      import("hast-util-to-html"),
    ]);
    const starryNight = await createStarryNight(all);
    const stylesheet = await readPackageFile("@wooorm/starry-night/style/core");
    const styles = await stylesFor(themes, (_theme, palette) =>
      starryNightCss(palette, stylesheet),
    );
    return {
      highlight(source, document) {
        const scope = starryNight.flagToScope(document.language);
        return scope ? toHtml(starryNight.highlight(source, scope)) : undefined;
      },
      fragment: (output, document, theme) =>
        `<style>${styles.get(theme.id)}</style><pre><code>${output}</code></pre>`,
    };
  },
};

/** The id of every library `loadLibraries` can load. */
export const libraryIds = Object.keys(loaders);

/**
 * Builds the libraries named by `ids`, every one of them by default.
 */
export async function loadLibraries({ assetsDir, documents, themes, ids = libraryIds }) {
  const libraries = [];
  for (const id of ids) {
    const load = loaders[id];
    if (!load) throw new Error(`no comparison library is named ${id}`);
    libraries.push({ id, ...(await load({ assetsDir, documents, themes })) });
  }
  return {
    libraries,
    dispose() {
      for (const library of libraries) library.dispose?.();
    },
  };
}

async function readPackageFile(specifier) {
  return readFile(fileURLToPath(import.meta.resolve(specifier)), "utf8");
}

async function highlightJsStylesheets(themes) {
  return new Map(
    await Promise.all(
      themes.map(async (theme) => [theme.id, await readPackageFile(theme.highlightJs)]),
    ),
  );
}

async function stylesFor(themes, style) {
  const stylesheets = await highlightJsStylesheets(themes);
  return new Map(
    themes.map((theme) => [
      theme.id,
      style(theme, highlightJsPalette(theme, stylesheets.get(theme.id))),
    ]),
  );
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
      new RegExp(`(?:^|\\})${selector.replaceAll(".", "\\.")}\\{([^}]*)\\}`, "u"),
    )?.[1];
    const value = rule?.match(new RegExp(`(?:^|;)${property}:([^;]+)`, "iu"))?.[1];
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
    // Dependabot moves these to each new release like the other comparison libraries,
    // so a token class a later release adds fails here rather than reaching the page
    // uncoloured.
    assertMapped(library, tokens, highlightJsClasses) {
      const unmapped = tokens.filter((token) => !Object.hasOwn(highlightJsClasses, token));
      if (unmapped.length > 0) {
        throw new Error(`no highlight.js class is mapped to ${library}'s ${unmapped.join(", ")}`);
      }
    },
  };
}

function tanStackTheme(theme, palette, themeTokenClasses) {
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
function sugarHighCss(palette, tokenTypes) {
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
  palette.assertMapped("Sugar High", tokenTypes, highlightJsClasses);
  const properties = tokenTypes.map(
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
        starryNightStylesheet.matchAll(/var\(--color-prettylights-syntax-([a-z-]+)\)/gu),
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
