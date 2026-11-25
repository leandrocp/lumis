/** Syntax highlighting with Tree-sitter and Neovim themes. */

import { createHighlighterModule } from "./core/highlighter.js";
import { createRuntime, getDefaultRuntime } from "./runtime/browser.js";
import * as languages from "./languages.browser.js";
import { availableThemes as listThemes, sanitizeThemeName as sanitizeName } from "./themes.js";

export { runtimeKind } from "./runtime/browser.js";
export { highlightIter, highlightEvents } from "./core/highlighter.js";
export { parseCodeFenceMeta } from "./code-fence.js";
export type { CodeFenceMeta, MetaAttribute } from "./code-fence.js";

const highlighter = createHighlighterModule({
  createRuntime,
  getDefaultRuntime,
});

/** {@inheritDoc index.createHighlighter} */
export function createHighlighter(...args: Parameters<typeof highlighter.createHighlighter>) {
  return highlighter.createHighlighter(...args);
}
/** {@inheritDoc index.highlight} */
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

/** @deprecated Import { guess } from "@lumis-sh/lumis/client/languages" instead. */
export const guessLanguage = languages.guess;

/** @deprecated Import { get } from "@lumis-sh/lumis/client/languages" instead. */
export const getLanguage = languages.get;

/** @deprecated Import { available } from "@lumis-sh/lumis/client/languages" instead. */
export const availableLanguages = languages.available;

/** @deprecated Import { load } from "@lumis-sh/lumis/client/languages" instead. */
export const loadLanguages = languages.load;

/** @deprecated Import { loaded } from "@lumis-sh/lumis/client/languages" instead. */
export const loadedLanguages = languages.loaded;

/** @deprecated Import { configureWasmResolver } from "@lumis-sh/lumis/client/languages" instead. */
export const configureWasmResolver = languages.configureWasmResolver;

/** @deprecated Import { configurePackageResolver } from "@lumis-sh/lumis/client/languages" instead. */
export const configureLanguagePackageResolver = languages.configurePackageResolver;

/** @deprecated Import { withWasm } from "@lumis-sh/lumis/client/languages" instead. */
export const withWasm = languages.withWasm;

/** @deprecated Import { withWasmBundle } from "@lumis-sh/lumis/client/languages" instead. */
export const withWasmBundle = languages.withWasmBundle;

/** @deprecated Import { available } from "@lumis-sh/lumis/themes" instead. */
export const availableThemes = listThemes;

/** @deprecated Import { sanitizeThemeName } from "@lumis-sh/lumis/formatters/html" instead. */
export const sanitizeThemeName = sanitizeName;
