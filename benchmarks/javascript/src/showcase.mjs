import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import highlightJs from "highlight.js";
import { createHighlighter as createShikiHighlighter } from "shiki";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";
import { createHighlighter as createTanStackHighlighter } from "@tanstack/highlight/core";
import * as tanStackLanguages from "@tanstack/highlight/languages";
import { createThemeCss, themeTokenClasses } from "@tanstack/highlight/theme";
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
// Like highlight.js, TanStack Highlight gets every language it ships, so a README
// fence it can read is highlighted rather than left plain.
const tanStack = createTanStackHighlighter({ languages: Object.values(tanStackLanguages) });
const tanStackThemes = new Map(
  themes.map((theme) => [theme.id, tanStackTheme(theme, highlightJsThemes.get(theme.id))]),
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

    // A language TanStack Highlight lacks comes back as plain text under its
    // fallback name. That is no output to compare, so nothing is written, and the
    // document has to have declared it unsupported.
    const tanStackResult = tanStack.highlight(source, { lang: document.language });
    if (tanStackResult.lang !== document.language) continue;
    const tanStackOutput =
      `<style>${createThemeCss({ themes: [{ selector: ":root", theme: tanStackThemes.get(theme.id) }] })}</style>` +
      tanStackResult.html;
    validate(tanStackOutput, source, "TanStack Highlight");
    await writeFile(resolve(fragmentsDir, "tanstack-highlight.html"), tanStackOutput);
  }
}

shiki?.dispose();

// TanStack Highlight ships no Catppuccin. Rather than choose its colours here, the
// theme is read out of the @catppuccin/highlightjs stylesheet the highlight.js
// output uses, each of its token classes taking the colour of the highlight.js
// class that marks the same thing.
function tanStackTheme(theme, stylesheet) {
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
  // The dependency floats like the other comparison libraries, so a token class a
  // later release adds fails here rather than reaching the page uncoloured.
  const unmapped = themeTokenClasses.filter(
    (token) => token !== "token" && !Object.hasOwn(highlightJsClasses, token),
  );
  if (unmapped.length > 0) {
    throw new Error(
      `no highlight.js class is mapped to TanStack Highlight's ${unmapped.join(", ")}`,
    );
  }
  const declaration = (selector, property) => {
    const rule = stylesheet.match(
      new RegExp(`(?:^|\\})${selector.replaceAll(".", "\\.")}\\{([^}]*)\\}`),
    )?.[1];
    const value = rule?.match(new RegExp(`(?:^|;)${property}:(#[0-9a-f]{6})`, "i"))?.[1];
    if (!value) throw new Error(`${theme.highlightJs} has no ${property} for ${selector}`);
    return value;
  };
  const foreground = declaration("code.hljs", "color");

  return {
    name: `catppuccin-${theme.id}`,
    type: theme.appearance,
    background: declaration("code.hljs", "background"),
    foreground,
    tokens: {
      token: foreground,
      ...Object.fromEntries(
        Object.entries(highlightJsClasses).map(([token, className]) => [
          token,
          declaration(`code .hljs-${className}`, "color"),
        ]),
      ),
    },
  };
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
