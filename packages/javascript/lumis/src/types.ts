import type { Parser, Language as TSLanguage, Query } from "web-tree-sitter";

/**
 * A highlighted token's byte range and scope.
 *
 * ```ts
 * // { startByte: 0, endByte: 5, scope: "keyword", language: "javascript" }
 * ```
 */
export interface HighlightSpan {
  startByte: number;
  endByte: number;
  scope: string;
  language: string;
}

/**
 * Visual style for a scope from a theme.
 *
 * ```ts
 * const style: HighlightStyle = { fg: '#ff79c6', italic: true }
 * ```
 */
export interface HighlightStyle {
  fg?: string;
  bg?: string;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean | "solid" | "wavy" | "double" | "dotted" | "dashed" | "undercurl";
  strikethrough?: boolean;
}

/**
 * Byte offset range of a highlighted token.
 *
 * ```ts
 * const range: HighlightRange = { start: 0, end: 5 }
 * ```
 */
export interface HighlightRange {
  start: number;
  end: number;
}

/** A zero-based source position with a UTF-8 byte column. */
export interface Position {
  line: number;
  column: number;
}

/** A half-open range expressed as absolute offsets measured in UTF-8 bytes. */
export interface OffsetAnnotationRange extends HighlightRange {
  type: "offset";
}

/** A half-open range expressed as zero-based lines and UTF-8 byte columns. */
export interface PositionAnnotationRange {
  type: "position";
  start: Position;
  end: Position;
}

/** A half-open annotation range expressed as offsets or source positions. */
export type AnnotationRange = OffsetAnnotationRange | PositionAnnotationRange;

/**
 * Metadata about a supported language. Returned by {@link availableLanguages}.
 *
 * ```ts
 * import { availableLanguages } from '@lumis-sh/lumis'
 * const languages = availableLanguages()
 * // [{ id: 'javascript', name: 'JavaScript', aliases: ['js', 'jsx'], extensions: ['*.js', ...] }, ...]
 * ```
 */
export interface LanguageInfo {
  id: string;
  name: string;
  aliases: string[];
  extensions: string[];
  globs: string[];
  emacsModes: string[];
  shebangs: string[];
}

/**
 * Metadata about a built-in theme. Returned by {@link availableThemes}.
 *
 * ```ts
 * import { availableThemes } from '@lumis-sh/lumis'
 * const themes = availableThemes()
 * // [{ name: 'dracula', appearance: 'dark' }, { name: 'github_light', appearance: 'light' }, ...]
 * ```
 */
export interface ThemeInfo {
  name: string;
  appearance: "light" | "dark";
}

/**
 * Theme with color and style mappings for syntax scopes.
 *
 * ```ts
 * import dracula from '@lumis-sh/themes/dracula'
 * // dracula.name        → "dracula"
 * // dracula.appearance  → "dark"
 * // dracula.highlights  → { "keyword": { fg: "#ff79c6" }, ... }
 * ```
 */
export interface Theme {
  name: string;
  appearance: "light" | "dark";
  revision?: string;
  highlights: Record<string, HighlightStyle>;
}

export interface LanguageDefinition {
  id: string;
  aliases: string[];
}

/**
 * Pointer to a WASM parser binary on a CDN.
 *
 * ```ts
 * const ref: WasmRef = {
 *   packageName: '@lumis-sh/wasm-javascript',
 *   name: 'tree-sitter-javascript',
 *   version: '0.26.2',
 *   sha256: '...',
 *   size: 416499,
 * }
 * ```
 */
export interface WasmRef {
  packageName: string;
  name: string;
  version: string;
  /** Lowercase SHA-256 of the exact parser bytes. */
  sha256: string;
  /** Exact parser size in bytes. */
  size: number;
}

export type RuntimeWasmInput = Uint8Array | ArrayBuffer | string | URL | Response;

export type RuntimeWasmBundle = Partial<Record<string, RuntimeWasmInput>>;

/**
 * A language accepted by Lumis.
 *
 * Queries live in the package alongside the parser they were tested against, so
 * this carries a package name rather than query text.
 *
 * ```ts
 * import javascript from '@lumis-sh/lumis/langs/javascript'
 * // javascript.id         → "javascript"
 * // javascript.aliases    → ["js", "jsx"]
 * // javascript.packageName → "@lumis-sh/wasm-javascript"
 * ```
 */
export interface Language extends LanguageDefinition {
  /** Independently released package containing the parser and matching queries. */
  packageName?: string;
  /**
   * WASM parser source:
   * - `WasmRef` fetched from CDN (default for pre-built bundles)
   * - `Uint8Array` or `ArrayBuffer` passed directly (useful with browser bundlers)
   * - `URL` fetched directly (`file://` works in Node.js)
   * - `string` treated as file path (Node.js) or URL (browser)
   */
  wasm?: WasmRef | RuntimeWasmInput;
}

/**
 * A handle to a published language package.
 *
 * Use `wasm` to override where the package's parser bytes come from. The bytes
 * are still checked against the package's size and SHA-256 before loading.
 */
export interface LanguagePackageHandle extends LanguageDefinition {
  packageName: string;
  /** Optional caller-selected source for the package's verified parser bytes. */
  wasm?: WasmRef | RuntimeWasmInput;
}

export interface PlaintextLanguage extends LanguageDefinition {
  id: "plaintext";
}

/**
 * A language definition that contains everything the runtime needs to load it.
 *
 * Each variant declares only the fields it owns, so an object literal cannot
 * pass a forbidden discriminator or load field as explicit `undefined`.
 *
 * Queries are not a variant. A parser and the queries written against it are
 * released together inside a package, so a caller names the package and, at
 * most, where its parser bytes come from. Supplying queries directly is a
 * planned feature rather than a supported one; see `ARCHITECTURE.md`.
 */
export type LoadableLanguage = LanguagePackageHandle | PlaintextLanguage;

/**
 * A lazy language handle from a bundle. Callable to load the full {@link Language}.
 *
 * ```ts
 * import { bundledLanguages } from '@lumis-sh/lumis/bundles/web'
 *
 * bundledLanguages.javascript.id  // "javascript"
 * const language = await bundledLanguages.javascript()  // loads the full Language
 * ```
 */
export interface LazyLanguage {
  (): Promise<Language>;
  id: string;
  aliases: string[];
}

/**
 * A collection of lazy language handles. Import a preset bundle:
 *
 * ```ts
 * import { bundledLanguages } from '@lumis-sh/lumis/bundles/web'
 * import { bundledLanguages } from '@lumis-sh/lumis/bundles/web-extra'
 * import { bundledLanguages } from '@lumis-sh/lumis/bundles/system'
 * import { bundledLanguages } from '@lumis-sh/lumis/bundles/backend'
 * import { bundledLanguages } from '@lumis-sh/lumis/bundles/full'
 * ```
 */
export type LanguageBundle = Record<string, LazyLanguage>;

/**
 * What `createHighlighter({ languages })` accepts.
 *
 * - `Language` — loaded immediately
 * - `Promise<{ default: Language }>` — e.g. `import('@lumis-sh/lumis/langs/css')`
 * - `() => Promise<{ default: Language }>` — lazy, loaded when called
 * - `LanguageBundle` — registered lazily, loaded on first use
 */
export type LanguageInput =
  | Language
  | Promise<{ default: Language }>
  | (() => Promise<{ default: Language }>)
  | LanguageBundle;

/**
 * How formatters and `hl.highlight()` identify a language.
 *
 * - `LanguageDefinition` — a language object or identifier-only `{ id, aliases }`
 * - `LazyLanguage` — a handle from a bundle
 * - `string` — a language ID like `"javascript"`
 */
export type LanguageRef = LanguageDefinition | LazyLanguage | string;

export interface CaptureMetadata {
  highlightScope?: string;
  isInjectionContent: boolean;
  isInjectionLanguage: boolean;
  isInjectionFilename: boolean;
  isLocalScope: boolean;
  isLocalDefinition: boolean;
  isLocalDefinitionValue: boolean;
  isLocalReference: boolean;
}

/** Deltas from `(#offset! @capture start_row start_col end_row end_col)`. */
export interface QueryCaptureOffset {
  startRow: number;
  startColumn: number;
  endRow: number;
  endColumn: number;
}

export interface CompiledHighlightConfig {
  query: Query;
  injectionPatternEnd: number;
  localsPatternEnd: number;
  captureMetadata: Record<string, CaptureMetadata>;
  nonLocalVariablePatterns: boolean[];
  /** Per pattern index, the `#offset!` deltas keyed by capture name. */
  captureOffsets: Array<Record<string, QueryCaptureOffset> | undefined>;
}

export interface CompiledBracketConfig {
  query: Query;
  captureMetadata: Record<string, { isOpen: boolean; isClose: boolean }>;
  rainbowExcludePatterns: boolean[];
}

export interface LoadedLanguage {
  definition: LanguageDefinition;
  parser: Parser;
  language: TSLanguage;
  config: CompiledHighlightConfig;
  brackets?: CompiledBracketConfig;
}

export const PLAINTEXT_LANG_ID = "plaintext";

/**
 * Wraps the `<pre>` block with custom HTML.
 *
 * ```ts
 * htmlInline({ header: { openTag: '<div class="code">', closeTag: '</div>' } })
 * ```
 */
export interface HtmlElement {
  openTag: string;
  closeTag: string;
}

/**
 * A single line number (1-based) or `[start, end]` inclusive range.
 *
 * ```ts
 * const lines: LineSpec[] = [1, [3, 5], 8]  // lines 1, 3-5, and 8
 * ```
 */
export type LineSpec = number | [number, number];

/**
 * Line highlighting for inline and multi-themes formatters.
 *
 * ```ts
 * htmlInline({ highlightLines: { lines: [1, [3, 5]], style: 'theme' } })
 * ```
 */
export interface HighlightLinesInline {
  lines: LineSpec[];
  /**
   * `"theme"` uses the theme's highlight background. Any other string is raw
   * CSS. `null` emits no inline style at all, for highlighting by class alone.
   *
   * Omitting it is the same as `"theme"`.
   */
  style?: string | null;
  class?: string;
}

/**
 * Line highlighting for the linked formatter.
 *
 * ```ts
 * htmlLinked({ highlightLines: { lines: [1, [3, 5]], class: 'active' } })
 * ```
 */
export interface HighlightLinesLinked {
  lines: LineSpec[];
  /** Defaults to `"l-highlighted"`. */
  class?: string;
}

/**
 * A caller-provided semantic range with typed data.
 *
 * This annotation marks only `price` in a one-line source:
 *
 * ```ts
 * const source = "let total = price;"
 * const annotation: Annotation<string> = {
 *   range: { type: "offset", start: 12, end: 17 },
 *   data: "search-match",
 * }
 *
 * // Offsets are UTF-8 bytes, which String.prototype.slice does not count.
 * const bytes = new TextEncoder().encode(source)
 * new TextDecoder().decode(bytes.subarray(annotation.range.start, annotation.range.end)) // "price"
 * ```
 *
 * When passed in highlighting options, a custom formatter receives
 * `annotationStart` before `price` and `annotationEnd` after it.
 */
export interface Annotation<T = unknown> {
  /** A tagged offset or position range into the formatted source. */
  range: AnnotationRange;
  /** Caller-owned data interpreted by custom formatters. */
  data: T;
}

/** An annotation materialized to the offset range consumed by formatters. */
export interface ResolvedAnnotation<T = unknown> {
  range: HighlightRange;
  data: T;
}

/** Options for one highlighting operation. */
export interface HighlightOptions<T = unknown> {
  /** Caller-provided semantic ranges composed into the formatter event stream. */
  annotations?: readonly Annotation<T>[];
  /** Render nested brackets with rainbow bracket scopes. */
  rainbowBrackets?: boolean;
}

/**
 * A nested syntax highlight event from tree-sitter.
 *
 * Events form a nested structure: a `start` event opens a scope,
 * `source` events provide text ranges, and `end` closes the scope.
 * Parent scopes stay open across child scopes (e.g. a `string` scope
 * wraps injected `tag` scopes inside template literals).
 */
export type SyntaxHighlightEvent =
  | { type: "start"; scope: string; language: string }
  | { type: "source"; startByte: number; endByte: number }
  | { type: "end" };

/** A unified syntax and caller-provided annotation event. */
export type HighlightEvent<T = unknown> =
  | SyntaxHighlightEvent
  | { type: "annotationStart"; annotation: ResolvedAnnotation<T> }
  | { type: "annotationEnd" };

/**
 * Signature of the `highlightIter` free function and the `hl.highlightIter`
 * method on a {@link Highlighter} instance.
 *
 * ```ts
 * highlightIter(source, javascript, dracula, (text, language, range, scope, style) => {
 *   console.log(`${scope}: ${text}`)
 * })
 * ```
 *
 * Pass {@link HighlightOptions} last to opt into the same scopes the built-in
 * formatters produce:
 *
 * ```ts
 * highlightIter(source, javascript, dracula, onToken, {rainbowBrackets: true})
 * ```
 */
export type HighlightIterFn = (
  source: string,
  language: LanguageRef | undefined,
  theme: Theme | undefined,
  onToken: HighlightCallback,
  options?: HighlightOptions,
) => void;

/**
 * A formatter renders highlighted source code into an output string.
 *
 * Built-in formatters are created with `htmlInline()`, `htmlLinked()`, etc.
 * Custom formatters implement the same interface and render the already
 * highlighted, properly nested event stream.
 *
 * While `render()` is running, `this.language` is set to the resolved language
 * after detection, so the formatter can render language-dependent output
 * (e.g. `<code class="language-...">`) without re-running detection.
 *
 * ```ts
 * import { type Formatter } from '@lumis-sh/lumis'
 *
 * const formatter: Formatter = {
 *   language: javascript,
 *   render(source, events) {
 *     // Byte offsets, so decode the slice rather than using String.slice,
 *     // which counts UTF-16 code units.
 *     const bytes = new TextEncoder().encode(source)
 *     const decoder = new TextDecoder()
 *
 *     return events
 *       .filter(event => event.type === 'source')
 *       .map(event => decoder.decode(bytes.subarray(event.startByte, event.endByte)))
 *       .join('')
 *   },
 * }
 * ```
 *
 * Lumis adds event kinds as it grows, so a formatter should render the ones it
 * knows and ignore the rest rather than assuming the union is closed.
 */
export interface Formatter<T = unknown> {
  language?: LanguageRef;
  render(source: string, events: readonly HighlightEvent<T>[]): string;
}

/**
 * Called for each highlighted token in `highlightIter`.
 *
 * ```ts
 * hl.highlightIter(source, javascript, dracula, (text, language, range, scope, style) => {
 *   console.log(`${scope}: ${text}`)
 * })
 * ```
 */
export type HighlightCallback = (
  text: string,
  language: string,
  range: HighlightRange,
  scope: string,
  style: HighlightStyle | undefined,
) => void;

/**
 * Options for {@link htmlInline}.
 *
 * ```ts
 * htmlInline({ language: javascript, theme: dracula, preClass: 'my-code', italic: true })
 * ```
 */
export interface HtmlInlineOptions {
  language?: LanguageRef;
  theme?: Theme;
  preClass?: string;
  /** Use italic styles from the theme. */
  italic?: boolean;
  /** Add `data-highlight` attributes with scope names. */
  includeHighlights?: boolean;
  highlightLines?: HighlightLinesInline;
  header?: HtmlElement;
}

export interface HtmlInlineFormatter extends Formatter, HtmlInlineOptions {}

/**
 * Options for {@link htmlLinked}.
 *
 * ```ts
 * htmlLinked({ language: javascript, preClass: 'my-code' })
 * ```
 */
export interface HtmlLinkedOptions {
  language?: LanguageRef;
  preClass?: string;
  highlightLines?: HighlightLinesLinked;
  header?: HtmlElement;
}

export interface HtmlLinkedFormatter extends Formatter, HtmlLinkedOptions {}

/**
 * Options for {@link htmlMultiThemes}.
 *
 * ```ts
 * htmlMultiThemes({
 *   language: javascript,
 *   themes: { light: githubLight, dark: githubDark },
 *   defaultTheme: 'light-dark()',
 * })
 * ```
 */
export interface HtmlMultiThemesOptions {
  language?: LanguageRef;
  themes: Record<string, Theme>;
  /**
   * Theme whose colors are inlined as defaults.
   * Pass `"light-dark()"` to use the CSS `light-dark()` function instead.
   */
  defaultTheme?: string;
  /** Prefix for CSS custom properties. Defaults to `"--lumis"`. */
  cssVariablePrefix?: string;
  preClass?: string;
  italic?: boolean;
  includeHighlights?: boolean;
  highlightLines?: HighlightLinesInline;
  header?: HtmlElement;
}

export interface HtmlMultiThemesFormatter extends Formatter, HtmlMultiThemesOptions {}

/**
 * Options for {@link bbcodeScoped}.
 *
 * ```ts
 * bbcodeScoped({ language: javascript })
 * ```
 */
export interface BBCodeScopedOptions {
  language?: LanguageRef;
}

export interface BBCodeScopedFormatter extends Formatter, BBCodeScopedOptions {}

/**
 * Options for {@link terminal}.
 *
 * ```ts
 * terminal({ language: javascript, theme: dracula })
 * terminal({ language: javascript, theme: dracula, background: 'theme', width: 120 })
 * ```
 */
export interface TerminalOptions {
  language?: LanguageRef;
  theme?: Theme;
  /**
   * Fallback background for text the theme gives no background.
   *
   * Omit it to inherit the terminal's own background. `"theme"` reuses the
   * theme's `normal` background. Any other string is used as the color.
   */
  background?: string;
  /**
   * Pad each rendered line out to this width. Only takes effect alongside
   * {@link TerminalOptions.background}, since padding is only visible as
   * background fill.
   */
  width?: number;
}

export interface TerminalFormatter extends Formatter, TerminalOptions {}
