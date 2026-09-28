/** Syntax highlighting with Tree-sitter and Neovim themes. */

import { createHighlighterModule } from "./core/highlighter.js";
import { createLoadLanguages } from "./core/load-languages.js";
import { bundleWithPackages, languageWithPackage } from "./bundle-helpers.js";
import {
  availableLanguages,
  configureLanguagePackageResolver,
  configureWasmResolver,
  createRuntime,
  getDefaultRuntime,
  loadedLanguages,
  loadLanguage as runtimeLoadLanguage,
} from "./runtime/node.js";

export { runtimeKind } from "./runtime/node.js";

export { highlightIter, highlightEvents } from "./core/highlighter.js";
export { guessLanguage } from "./guess-language.js";

/**
 * Load languages into the runtime `highlight()` uses, by name.
 *
 * This is the JavaScript / TypeScript spelling of `Lumis.Languages.load/1`:
 * it caches verified parser bytes and, on the native addon, their compiled
 * Wasmtime module. It keeps the languages in the default runtime so no later
 * call reloads them.
 *
 * Accepts catalog names, aliases, and `bundle-<name>` tokens.
 *
 * Highlighting loads on demand regardless, so this is an optimization. At
 * startup, prefer not blocking on it:
 *
 * ```ts
 * import { loadLanguages } from '@lumis-sh/lumis'
 *
 * await startServer()
 *
 * loadLanguages(['javascript', 'html', 'css']).catch((error) => {
 *   logger.warn({ error }, 'Lumis warm-up failed; languages load on demand')
 * })
 * ```
 *
 * Every name is attempted. If any fail, it rejects with an `AggregateError`
 * naming each one, after the rest have loaded.
 */
export const loadLanguages = createLoadLanguages(runtimeLoadLanguage);

const highlighter = createHighlighterModule({
  createRuntime,
  getDefaultRuntime,
});

/**
 * Create a reusable highlighter with languages loaded during setup.
 *
 * `createHighlighter` is async; the returned `hl.highlight()` is synchronous.
 *
 * The `languages` array accepts `Language` objects, `LanguageBundle` collections, and dynamic imports.
 *
 * @example Cherry-pick languages
 * ```ts
 * import { createHighlighter } from '@lumis-sh/lumis'
 * import { htmlInline } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/lumis/langs/javascript'
 * import dracula from '@lumis-sh/themes/dracula'
 *
 * const hl = await createHighlighter({ languages: [javascript] })
 * const html = hl.highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
 * ```
 *
 * @example With a bundle
 * ```ts
 * import { createHighlighter } from '@lumis-sh/lumis'
 * import { htmlInline } from '@lumis-sh/lumis/formatters'
 * import { bundledLanguages } from '@lumis-sh/lumis/bundles/web'
 * import dracula from '@lumis-sh/themes/dracula'
 *
 * // Register all web languages. None are loaded yet.
 * const hl = await createHighlighter({ languages: [bundledLanguages] })
 *
 * // Load a language, then highlight synchronously.
 * await hl.loadLanguage(bundledLanguages.javascript)
 * const html = hl.highlight('const x = 1', htmlInline({ language: bundledLanguages.javascript, theme: dracula }))
 * ```
 */
export function createHighlighter(...args: Parameters<typeof highlighter.createHighlighter>) {
  return highlighter.createHighlighter(...args);
}

/**
 * Highlight code in a single async call.
 *
 * Initializes the parser, loads the language, and returns formatted output.
 * Uses a shared runtime so loaded languages persist across calls.
 *
 * For repeated highlighting, prefer {@link createHighlighter} which separates
 * async setup from synchronous rendering.
 *
 * @example
 * ```ts
 * import { highlight } from '@lumis-sh/lumis'
 * import { htmlInline } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/lumis/langs/javascript'
 * import dracula from '@lumis-sh/themes/dracula'
 *
 * const html = await highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
 * ```
 */
export function highlight(...args: Parameters<typeof highlighter.highlight>) {
  return highlighter.highlight(...args);
}

/**
 * Return a copy of a language that loads from an imported parser package.
 *
 * A browser loads only the packages passed in this way, so a bundler ships
 * them and nothing is fetched. Pass the whole package, so its manifest comes
 * along with the parser:
 *
 * ```ts
 * import * as elixirPackage from '@lumis-sh/wasm-elixir'
 * const elixirLanguage = withWasm(elixir, elixirPackage)
 * ```
 *
 * Parser bytes on their own also work in Node, which reads the manifest from
 * the installed package.
 */
export function withWasm<T extends import("./types.js").Language>(
  language: T,
  source: import("./types.js").LanguagePackageExports | import("./types.js").RuntimeWasmInput,
): Omit<T, "wasm" | "manifest"> & {
  wasm: import("./types.js").RuntimeWasmInput;
  manifest?: object;
} {
  return languageWithPackage(language, source);
}

/**
 * Apply imported parser packages to every matching language in a bundle.
 *
 * Pass `bundledPackages` from a `@lumis-sh/wasm-bundle-*` package.
 */
export function withWasmBundle(
  bundle: import("./types.js").LanguageBundle,
  sources:
    | import("./types.js").RuntimeLanguagePackageBundle
    | import("./types.js").RuntimeWasmBundle,
): import("./types.js").LanguageBundle {
  return bundleWithPackages(bundle, sources);
}

export type { CreateHighlighterOptions, Highlighter } from "./core/highlighter.js";
export type { LoadLanguages } from "./core/load-languages.js";
export type {
  Annotation,
  AnnotationRange,
  Budget,
  Decoration,
  Formatter,
  HighlightEvent,
  HighlightIterFn,
  HighlightOptions,
  HtmlElement,
  HighlightLinesInline,
  HighlightLinesLinked,
  HighlightLinesTerminal,
  HighlightLinesBBCode,
  LineSpec,
  HighlightRange,
  OffsetAnnotationRange,
  Position,
  PositionAnnotationRange,
  HighlightStyle,
  HtmlAttrs,
  HtmlStructure,
  Language,
  LanguageDefinition,
  LanguagePackageHandle,
  PlaintextLanguage,
  LoadableLanguage,
  LanguageBundle,
  LanguageInput,
  LanguageRef,
  LumisHighlightEvent,
  LazyLanguage,
  Theme,
  WasmRef,
  RuntimeWasmInput,
  RuntimeWasmBundle,
  LanguagePackageExports,
  RuntimeLanguagePackageBundle,
  SyntaxHighlightEvent,
  LanguageInfo,
  ThemeInfo,
} from "./types.js";
export {
  availableLanguages,
  configureLanguagePackageResolver,
  configureWasmResolver,
  loadedLanguages,
};
export { getLanguage } from "./catalog-metadata.js";
export type { LanguagePackageResolver, WasmResolver } from "./core/languages.js";
export { availableThemes, sanitizeThemeName } from "./themes.js";
