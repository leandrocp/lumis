/** Syntax highlighting with Tree-sitter and Neovim themes. */

import { createHighlighterModule } from "./core/highlighter.js";
import { createRuntime, getDefaultRuntime } from "./runtime/node.js";
import * as languages from "./languages.js";
import { available as listThemes } from "./themes/installed.js";
import { sanitizeThemeName as sanitizeName } from "./themes.js";

export { runtimeKind } from "./runtime/node.js";
export { highlightIter, highlightEvents } from "./core/highlighter.js";

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
 * import dracula from '@lumis-sh/themes/dracula'
 * import javascript from '@lumis-sh/wasm-javascript'
 *
 * const hl = await createHighlighter({ languages: [javascript] })
 * const html = hl.highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
 * ```
 *
 * @example With a bundle
 * ```ts
 * import { createHighlighter } from '@lumis-sh/lumis'
 * import { htmlInline } from '@lumis-sh/lumis/formatters'
 * import dracula from '@lumis-sh/themes/dracula'
 * import web from '@lumis-sh/wasm-bundle-web'
 *
 * // Register all web languages. None are loaded yet.
 * const hl = await createHighlighter({ languages: [web] })
 *
 * // Load a language, then highlight synchronously.
 * await hl.loadLanguage(web.javascript)
 * const html = hl.highlight('const x = 1', htmlInline({ language: web.javascript, theme: dracula }))
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
 * import dracula from '@lumis-sh/themes/dracula'
 * import javascript from '@lumis-sh/wasm-javascript'
 *
 * const html = await highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
 * ```
 */
export function highlight(...args: Parameters<typeof highlighter.highlight>) {
  return highlighter.highlight(...args);
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
export type { LanguagePackageResolver, WasmResolver } from "./core/languages.js";

/** @deprecated Import { guess } from "@lumis-sh/lumis/languages" instead. */
export const guessLanguage = languages.guess;

/** @deprecated Import { get } from "@lumis-sh/lumis/languages" instead. */
export const getLanguage = languages.get;

/** @deprecated Import { available } from "@lumis-sh/lumis/languages" instead. */
export const availableLanguages = languages.available;

/** @deprecated Import { load } from "@lumis-sh/lumis/languages" instead. */
export const loadLanguages = languages.load;

/** @deprecated Import { loaded } from "@lumis-sh/lumis/languages" instead. */
export const loadedLanguages = languages.loaded;

/** @deprecated Import { configureWasmResolver } from "@lumis-sh/lumis/languages" instead. */
export const configureWasmResolver = languages.configureWasmResolver;

/** @deprecated Import { configurePackageResolver } from "@lumis-sh/lumis/languages" instead. */
export const configureLanguagePackageResolver = languages.configurePackageResolver;

/** @deprecated Import { withWasm } from "@lumis-sh/lumis/languages" instead. */
export const withWasm = languages.withWasm;

/** @deprecated Import { withWasmBundle } from "@lumis-sh/lumis/languages" instead. */
export const withWasmBundle = languages.withWasmBundle;

/** @deprecated Import { available } from "@lumis-sh/lumis/themes" instead. */
export const availableThemes = listThemes;

/** @deprecated Import { sanitizeThemeName } from "@lumis-sh/lumis/formatters/html" instead. */
export const sanitizeThemeName = sanitizeName;
