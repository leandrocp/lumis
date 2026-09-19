import type {
  Decoration,
  HighlightStyle,
  HighlightSpan,
  HighlightEvent,
  HtmlAttrs,
  HtmlElement,
  LineSpec,
  LanguageRef,
  SyntaxHighlightEvent,
  Theme,
} from "../types.js";
import { LineSelection, composeLineDecorations, rainbowBracketScope } from "../decorations.js";
import { HIGHLIGHT_NAMES } from "../highlights.js";
import { sanitizeThemeName } from "../themes.js";
import { mergeAttrs, mergeClasses } from "../core/attr-merge.js";

// Rust exposes this from `lumis::formatters::html`, so the helper modules line up.
export { sanitizeThemeName } from "../themes.js";
export type { HtmlAttrs } from "../types.js";

const _encoder = new TextEncoder();
const _decoder = new TextDecoder();

/** @internal */
export function encodeSource(source: string): Uint8Array {
  return _encoder.encode(source);
}

/** @internal */
export function decodeSourceSlice(
  sourceBytes: Uint8Array,
  startByte: number,
  endByte: number,
): string {
  let start = Math.min(Math.max(startByte, 0), sourceBytes.length);
  while (start < sourceBytes.length && isUtf8Continuation(sourceBytes[start])) start += 1;

  let end = Math.min(Math.max(endByte, 0), sourceBytes.length);
  while (end > 0 && isUtf8Continuation(sourceBytes[end])) end -= 1;

  return _decoder.decode(sourceBytes.subarray(start, Math.max(start, end)));
}

function isUtf8Continuation(byte: number | undefined): boolean {
  return byte !== undefined && (byte & 0xc0) === 0x80;
}

function languageId(language: LanguageRef): string {
  return typeof language === "string" ? language : language.id;
}

function emptySpan(scope: string, language: string): HighlightSpan {
  return {
    startByte: 0,
    endByte: 0,
    scope,
    language,
  };
}

function getSpanStyle(
  theme: Theme | undefined,
  scope: string,
  language: string,
): HighlightStyle | undefined {
  if (scope.length === 0) {
    return undefined;
  }

  return getScopedThemeStyle(theme, scope, language);
}

function getUnderlineDecoration(style: HighlightStyle): string | undefined {
  if (style.underline == null || style.underline === false) {
    return undefined;
  }

  if (style.underline === true || style.underline === "solid") {
    return "underline";
  }

  if (style.underline === "undercurl") {
    return "underline wavy";
  }

  return `underline ${style.underline}`;
}

/**
 * Look up a scope's style in a theme, falling back to parent scopes.
 *
 * ```ts
 * getThemeStyle(dracula, 'string.special.regex')
 * // tries 'string.special.regex', then 'string.special', then 'string'
 * ```
 * @internal
 */
export function getThemeStyle(theme: Theme | undefined, scope: string): HighlightStyle | undefined {
  if (!theme) return RAINBOW_BRACKET_FALLBACKS[scope];

  let current = scope;
  while (current.length > 0) {
    const style = theme.highlights[current];
    if (style) return style;
    const fallback = RAINBOW_BRACKET_FALLBACKS[current];
    if (fallback) return fallback;

    const idx = current.lastIndexOf(".");
    if (idx === -1) {
      break;
    }
    current = current.slice(0, idx);
  }

  return undefined;
}

const RAINBOW_BRACKET_FALLBACKS: Record<string, HighlightStyle> = {
  "punctuation.bracket.rainbow.1": { fg: "#e06c75" },
  "punctuation.bracket.rainbow.2": { fg: "#e5c07b" },
  "punctuation.bracket.rainbow.3": { fg: "#61afef" },
  "punctuation.bracket.rainbow.4": { fg: "#d19a66" },
  "punctuation.bracket.rainbow.5": { fg: "#98c379" },
  "punctuation.bracket.rainbow.6": { fg: "#c678dd" },
};

/**
 * Look up a scope's style, trying a language-specific scope first.
 *
 * ```ts
 * getScopedThemeStyle(dracula, 'string', 'json')
 * // tries 'string.json' first, then falls back to 'string'
 * ```
 * @internal
 */
export function getScopedThemeStyle(
  theme: Theme | undefined,
  scope: string,
  language: LanguageRef | undefined,
): HighlightStyle | undefined {
  if (language === undefined) return getThemeStyle(theme, scope);
  return getThemeStyle(theme, `${scope}.${languageId(language)}`) ?? getThemeStyle(theme, scope);
}

/**
 * Build a CSS `text-decoration` value from a style.
 *
 * ```ts
 * textDecoration({ underline: 'wavy', strikethrough: true })
 * // "underline wavy line-through"
 * ```
 */
export function textDecoration(style: HighlightStyle): string {
  const underline = getUnderlineDecoration(style);

  if (underline && style.strikethrough) {
    return `${underline} line-through`;
  }
  if (underline) return underline;
  if (style.strikethrough) return "line-through";
  return "none";
}

/**
 * Convert a `HighlightStyle` to inline CSS declarations.
 *
 * ```ts
 * styleToCss({ fg: '#ff79c6', bold: true })
 * // "color: #ff79c6; font-weight: bold;"
 * ```
 */
export function styleToCss(
  style: HighlightStyle | undefined,
  options: { italic?: boolean; separator?: string; compact?: boolean } = {},
): string {
  if (!style) return "";

  const separator = options.separator ?? " ";
  const compact = options.compact ?? false;

  return styleDeclarations(style, Boolean(options.italic))
    .map(([property, value]) => (compact ? `${property}:${value};` : `${property}: ${value};`))
    .join(separator);
}

function styleDeclarations(style: HighlightStyle, italic: boolean): Array<[string, string]> {
  const declarations: Array<[string, string]> = [];

  if (style.fg) declarations.push(["color", style.fg]);
  if (style.bg) declarations.push(["background-color", style.bg]);
  if (style.bold) declarations.push(["font-weight", "bold"]);
  if (italic && style.italic) declarations.push(["font-style", "italic"]);

  const decoration = textDecoration(style);
  if (decoration !== "none") declarations.push(["text-decoration", decoration]);

  return declarations;
}

/**
 * Escape HTML special characters.
 *
 * ```ts
 * escape('<div class="a">')
 * // "&lt;div class=&quot;a&quot;&gt;"
 * ```
 */
export function escape(text: string): string {
  let result = "";
  for (let i = 0; i < text.length; i += 1) {
    const ch = text[i]!;
    switch (ch) {
      case "&":
        result += "&amp;";
        break;
      case "<":
        result += "&lt;";
        break;
      case ">":
        result += "&gt;";
        break;
      case '"':
        result += "&quot;";
        break;
      case "'":
        result += "&#39;";
        break;
      default:
        result += ch;
    }
  }
  return result;
}

/**
 * Escape a string for use inside an HTML attribute value.
 *
 * ```ts
 * escapeAttr('font-size: 14px; color: "red"')
 * // "font-size: 14px; color: &quot;red&quot;"
 * ```
 */
export function escapeAttr(value: string): string {
  let result = "";
  for (let i = 0; i < value.length; i += 1) {
    const ch = value[i]!;
    switch (ch) {
      case "&":
        result += "&amp;";
        break;
      case "<":
        result += "&lt;";
        break;
      case ">":
        result += "&gt;";
        break;
      case '"':
        result += "&quot;";
        break;
      case "'":
        result += "&#39;";
        break;
      default:
        result += ch;
    }
  }
  return result;
}

function classList(...classes: Array<string | undefined | false | null>): string | undefined {
  const value = classes.filter(
    (className): className is string => !!className && className.length > 0,
  );
  return value.length > 0 ? value.join(" ") : undefined;
}

/**
 * Whether a name is one HTML can carry on an attribute.
 *
 * A name with a space, a quote, `=`, `/` or `>` in it cannot be escaped into
 * safety: a space alone splits it into two attributes, and the second one can
 * be an event handler.
 *
 * ```ts
 * isValidAttrName('data-copy')          // true
 * isValidAttrName('x onclick=alert(1)') // false
 * ```
 */
export function isValidAttrName(name: string): boolean {
  // eslint-disable-next-line no-control-regex
  return name.length > 0 && !/[\s"'>/=\u0000-\u001F\u007F-\u009F]/.test(name);
}

function renderAttrs(attrs: HtmlAttrs): string {
  const parts: string[] = [];

  for (const [name, value] of Object.entries(attrs)) {
    if (value == null || value === false) continue;
    if (!isValidAttrName(name)) {
      throw new TypeError(`invalid HTML attribute name: ${JSON.stringify(name)}`);
    }
    if (value === true) {
      parts.push(name);
      continue;
    }

    parts.push(`${name}="${escapeAttr(String(value))}"`);
  }

  return parts.join(" ");
}

function tag(name: string, attrs: HtmlAttrs = {}): string {
  const renderedAttrs = renderAttrs(attrs);
  return renderedAttrs.length > 0 ? `<${name} ${renderedAttrs}>` : `<${name}>`;
}

/**
 * Join CSS class names, filtering out falsy values.
 *
 * ```ts
 * joinClasses('l-line', undefined, 'l-highlighted')  // "l-line l-highlighted"
 * joinClasses(undefined, false)                   // undefined
 * ```
 *
 * @deprecated Generic plumbing that Rust and Elixir do inline; it is not
 * Lumis's to own. Removed in the next major.
 */
export function joinClasses(
  ...classes: Array<string | undefined | false | null>
): string | undefined {
  return classList(...classes);
}

/**
 * Render an `HtmlAttrs` map to an HTML attribute string.
 *
 * ```ts
 * attrsToString({ class: 'foo', style: 'color: red', hidden: true })
 * // 'class="foo" style="color: red" hidden'
 * ```
 *
 * @deprecated Use {@link openSpanTag}, which renders the attributes it is
 * given. Removed in the next major.
 */
export function attrsToString(attrs: HtmlAttrs): string {
  return renderAttrs(attrs);
}

/**
 * Build an opening HTML tag from attributes, escaping every value.
 *
 * This is what {@link preAttrs}, {@link multiThemesPreAttrs} and
 * {@link codeAttrs} are built for: merge their result with your own attributes,
 * then render the whole thing here rather than assembling the string and
 * remembering to escape it.
 *
 * ```ts
 * openTag('span', { class: 'keyword', style: 'color: red' })
 * // '<span class="keyword" style="color: red">'
 * openTag('pre', { class: 'lumis', hidden: true })
 * // '<pre class="lumis" hidden>'
 * ```
 *
 * Throws for a name HTML cannot carry, per {@link isValidAttrName}.
 */
export function openTag(name: string, attrs: HtmlAttrs = {}): string {
  return tag(name, attrs);
}

/**
 * Build a closing HTML tag.
 *
 * ```ts
 * closeTag('span')  // "</span>"
 * ```
 *
 * @deprecated Use {@link closePreTag}, {@link closeCodeTag} or
 * {@link closingTags}. Removed in the next major.
 */
export function closeTag(name: string): string {
  return `</${name}>`;
}

/**
 * Open a `<span>` carrying the given attributes.
 * ```ts
 * openSpanTag({ class: 'l-keyword' })  // '<span class="l-keyword">'
 * openSpanTag({})                      // '<span>'
 * ```
 */
export function openSpanTag(attrs: HtmlAttrs = {}): string {
  return openSpan(renderAttrs(attrs));
}

function openSpan(attrs: string): string {
  return attrs.length > 0 ? `<span ${attrs}>` : "<span>";
}

/**
 * Options for {@link openPreTag}.
 *
 * ```ts
 * openPreTag({ preClass: 'my-code', theme: dracula })
 * ```
 */
export interface OpenPreTagOptions {
  preClass?: string;
  theme?: Theme;
  /** Additional attributes to merge after Lumis's generated values. */
  attrs?: HtmlAttrs;
}

/** Build the attributes used by inline and linked `<pre>` tags. */
export function preAttrs(options: OpenPreTagOptions = {}): HtmlAttrs {
  const className = options.preClass ? `lumis ${options.preClass}` : "lumis";
  const style = styleToCss(getThemeStyle(options.theme, "normal"));

  return mergeAttrs(
    {
      class: className,
      style: style.length > 0 ? style : undefined,
    },
    options.attrs,
  );
}

/**
 * Open a `<pre>` tag with the `lumis` class and optional theme background.
 *
 * ```ts
 * openPreTag({ theme: dracula })
 * // '<pre class="lumis" style="color: #f8f8f2; background-color: #282a36;">'
 * ```
 */
export function openPreTag(options: OpenPreTagOptions = {}): string {
  return tag("pre", preAttrs(options));
}

/**
 * Options for {@link openMultiThemesPreTag}.
 */
export interface OpenMultiThemesPreTagOptions {
  preClass?: string;
  themes: Record<string, Theme>;
  defaultTheme?: string;
  /** Defaults to `"--lumis"`. */
  cssVariablePrefix?: string;
  /** Additional attributes to merge after Lumis's generated values. */
  attrs?: HtmlAttrs;
}

/** Build the attributes used by a multi-theme `<pre>` tag. */
export function multiThemesPreAttrs(options: OpenMultiThemesPreTagOptions): HtmlAttrs {
  const classes =
    classList("lumis", "lumis-themes", options.preClass, ...sortedThemeNames(options.themes)) ??
    "lumis lumis-themes";

  return mergeAttrs(
    // A `preClass` naming one of the themes would otherwise appear twice.
    { class: mergeClasses(classes, ""), style: multiThemesPreStyle(options) },
    options.attrs,
  );
}

/**
 * Open a `<pre>` tag carrying every theme's name as a class and its `normal`
 * colours as CSS custom properties.
 *
 * The `<pre>` counterpart of {@link spanMultiThemesAttrs}: `defaultTheme` is
 * written inline and every other theme becomes a variable, except for
 * `"light-dark()"`, which writes both inline and contributes no variables.
 *
 * ```ts
 * openMultiThemesPreTag({ themes: { light: githubLight, dark: githubDark }, defaultTheme: 'light' })
 * // '<pre class="lumis lumis-themes dark light" style="color:#1f2328; ... --lumis-dark:#e6edf3; ...">'
 * ```
 */
export function openMultiThemesPreTag(options: OpenMultiThemesPreTagOptions): string {
  return tag("pre", multiThemesPreAttrs(options));
}

/** Build the attributes used by every HTML formatter's `<code>` tag. */
export function codeAttrs(language: LanguageRef | undefined, attrs: HtmlAttrs = {}): HtmlAttrs {
  const id = language ? languageId(language) : "plaintext";
  return mergeAttrs(
    {
      class: `language-${id}`,
      translate: "no",
      tabindex: 0,
    },
    attrs,
  );
}

/**
 * Open a `<code>` tag with the language class.
 *
 * ```ts
 * openCodeTag(javascript)  // '<code class="language-javascript" translate="no" tabindex="0">'
 * ```
 */
export function openCodeTag(language: LanguageRef | undefined, attrs: HtmlAttrs = {}): string {
  return tag("code", codeAttrs(language, attrs));
}

/**
 * Close a `<pre>` tag.
 *
 * ```ts
 * closePreTag()  // "</pre>"
 * ```
 */
export function closePreTag(): string {
  return "</pre>";
}

/**
 * Close a `<code>` tag.
 *
 * ```ts
 * closeCodeTag()  // "</code>"
 * ```
 */
export function closeCodeTag(): string {
  return "</code>";
}

/**
 * Close both `</code>` and `</pre>` tags.
 *
 * ```ts
 * closingTags()  // "</code></pre>"
 * ```
 */
export function closingTags(): string {
  return `${closeCodeTag()}${closePreTag()}`;
}

/**
 * Wrap content with an optional header element.
 *
 * ```ts
 * wrapWithHeader('<pre>...</pre>', { openTag: '<div>', closeTag: '</div>' })
 * // "<div><pre>...</pre></div>"
 * ```
 * @internal
 */
export function wrapWithHeader(content: string, header?: HtmlElement): string {
  if (!header) return content;
  return `${header.openTag}${content}${header.closeTag}`;
}

/**
 * Escape curly braces to HTML entities.
 *
 * ```ts
 * escapeBraces('{foo}')  // "&lbrace;foo&rbrace;"
 * ```
 */
export function escapeBraces(text: string): string {
  return text.replaceAll("{", "&lbrace;").replaceAll("}", "&rbrace;");
}

/**
 * @deprecated A pure alias of {@link escape}. Removed in the next major.
 */
export function escapeFragment(text: string): string {
  return escape(text);
}

/**
 * Convert a dot-separated scope to a CSS class name.
 *
 * ```ts
 * scopeToClass('string.special.regex')  // "l-string-special-regex"
 * ```
 */
export function scopeToClass(scope: string): string {
  return HIGHLIGHT_NAMES.includes(scope) ? `l-${scope.replaceAll(".", "-")}` : "l-text";
}

/**
 * Options for {@link spanInline} and {@link spanInlineAttrs}.
 *
 * ```ts
 * spanInline('const', { language: 'javascript', scope: 'keyword', theme: dracula })
 * ```
 */
export interface SpanInlineOptions {
  language: LanguageRef;
  scope: string;
  theme?: Theme;
  italic?: boolean;
  includeHighlights?: boolean;
}

/**
 * Build HTML attributes for an inline-styled `<span>`.
 *
 * ```ts
 * spanInlineAttrs({ language: 'javascript', scope: 'keyword', theme: dracula })
 * // { style: "color: #ff79c6;" }
 * ```
 */
export function spanInlineAttrs(options: SpanInlineOptions): HtmlAttrs {
  const attrs: HtmlAttrs = {};

  if (options.includeHighlights) {
    attrs["data-highlight"] = options.scope;
  }

  const css = styleToCss(getScopedThemeStyle(options.theme, options.scope, options.language), {
    italic: options.italic,
  });
  if (css) {
    attrs.style = css;
  }

  return attrs;
}

/**
 * Render an inline-styled `<span>` for a token.
 * ```ts
 * spanInline('const', { language: 'javascript', scope: 'keyword', theme: dracula })
 * // '<span style="color: #ff79c6;">const</span>'
 *
 * spanInline('const', { language: 'javascript', scope: 'keyword', theme: undefined })
 * // '<span>const</span>'
 * ```
 */
export function spanInline(text: string, options: SpanInlineOptions): string {
  return `${openSpanTag(spanInlineAttrs(options))}${escape(text)}</span>`;
}

/**
 * Build HTML attributes for a class-based `<span>`.
 *
 * ```ts
 * spanLinkedAttrs('keyword')  // 'class="l-keyword"'
 * ```
 */
export function spanLinkedAttrs(scope: string): string {
  return `class="${scopeToClass(scope)}"`;
}

/**
 * Render a class-based `<span>` for a token.
 *
 * ```ts
 * spanLinked('const', 'keyword')  // '<span class="l-keyword">const</span>'
 * ```
 */
export function spanLinked(text: string, scope: string): string {
  const escaped = escape(text);
  const cls = scopeToClass(scope);
  return `<span class="${cls}">${escaped}</span>`;
}

/**
 * Options for {@link spanMultiThemes} and {@link spanMultiThemesAttrs}.
 *
 * ```ts
 * spanMultiThemes('const', {
 *   language: 'javascript',
 *   scope: 'keyword',
 *   themes: { light: githubLight, dark: githubDark },
 *   defaultTheme: 'light-dark()',
 * })
 * ```
 */
export interface SpanMultiThemesOptions {
  /** Omit for scopes, such as line-number gutters, that are not language-specific. */
  language?: LanguageRef;
  scope: string;
  themes: Record<string, Theme | undefined>;
  defaultTheme?: string;
  /** Defaults to `"--lumis"`. */
  cssVariablePrefix?: string;
  italic?: boolean;
  includeHighlights?: boolean;
}

/**
 * Build HTML attributes for a multi-theme `<span>` with CSS custom properties.
 *
 * ```ts
 * spanMultiThemesAttrs({ language: 'js', scope: 'keyword', themes: { light: l, dark: d } })
 * // { style: "--lumis-light:#000; --lumis-dark:#fff; ..." }
 * ```
 */
export function spanMultiThemesAttrs(options: SpanMultiThemesOptions): HtmlAttrs {
  const { scope, language, themes, italic, includeHighlights } = options;
  const defaultTheme = options.defaultTheme;
  const cssVariablePrefix = options.cssVariablePrefix ?? "--lumis";

  if (Object.keys(themes).length === 0) {
    return {};
  }

  const attrs: HtmlAttrs = {};
  if (includeHighlights) {
    attrs["data-highlight"] = scope;
  }

  const inlineStyles: string[] = [];
  const cssVars: string[] = [];

  if (defaultTheme === "light-dark()") {
    const lightStyle = getScopedThemeStyle(themes.light, scope, language);
    const darkStyle = getScopedThemeStyle(themes.dark, scope, language);

    if (lightStyle && darkStyle) {
      appendLightDarkStyles(inlineStyles, lightStyle, darkStyle, italic);
    }
  } else if (defaultTheme) {
    applyDefaultMultiTheme(inlineStyles, cssVars, options);
    pushThemeCssVarsForAll(cssVars, cssVariablePrefix, themes, scope, language, defaultTheme);
  } else {
    pushThemeCssVarsForAll(cssVars, cssVariablePrefix, themes, scope, language);
  }

  const styleParts = [...inlineStyles, ...cssVars].filter(Boolean);
  if (styleParts.length > 0) {
    attrs.style = styleParts.join(" ");
  }

  return attrs;
}

function appendDefaultThemeCssVars(
  cssVars: string[],
  prefix: string,
  themeName: string,
  style: HighlightStyle,
): void {
  const sanitized = sanitizeThemeName(themeName);
  cssVars.push(`${prefix}-${sanitized}-font-style:${style.italic ? "italic" : "normal"};`);
  cssVars.push(`${prefix}-${sanitized}-font-weight:${style.bold ? "bold" : "normal"};`);
  cssVars.push(`${prefix}-${sanitized}-text-decoration:${textDecoration(style)};`);
}

function appendLightDarkStyles(
  inlineStyles: string[],
  lightStyle: HighlightStyle,
  darkStyle: HighlightStyle,
  italic?: boolean,
): void {
  if (lightStyle.fg && darkStyle.fg) {
    inlineStyles.push(`color: light-dark(${lightStyle.fg}, ${darkStyle.fg});`);
  }
  if (lightStyle.bg && darkStyle.bg) {
    inlineStyles.push(`background-color: light-dark(${lightStyle.bg}, ${darkStyle.bg});`);
  }

  inlineStyles.push(lightDarkWeight(lightStyle, darkStyle));

  if (italic) {
    inlineStyles.push(lightDarkStyle(lightStyle, darkStyle));
  }

  const lightDecoration = textDecoration(lightStyle) ?? "none";
  const darkDecoration = textDecoration(darkStyle);
  inlineStyles.push(`text-decoration: light-dark(${lightDecoration}, ${darkDecoration});`);
}

function lightDarkWeight(lightStyle: HighlightStyle, darkStyle: HighlightStyle): string {
  const light = lightStyle.bold ? "bold" : "normal";
  const dark = darkStyle.bold ? "bold" : "normal";
  return `font-weight: light-dark(${light}, ${dark});`;
}

function lightDarkStyle(lightStyle: HighlightStyle, darkStyle: HighlightStyle): string {
  const light = lightStyle.italic ? "italic" : "normal";
  const dark = darkStyle.italic ? "italic" : "normal";
  return `font-style: light-dark(${light}, ${dark});`;
}

function applyDefaultMultiTheme(
  inlineStyles: string[],
  cssVars: string[],
  options: SpanMultiThemesOptions,
): void {
  const { defaultTheme, themes, scope, language, italic, cssVariablePrefix = "--lumis" } = options;
  if (!defaultTheme || defaultTheme === "light-dark()") {
    return;
  }

  const defaultStyle = getScopedThemeStyle(themes[defaultTheme], scope, language);
  if (!defaultStyle) {
    return;
  }

  const css = styleToCss(defaultStyle, { italic, compact: true });
  if (css) {
    inlineStyles.push(css);
  }

  appendDefaultThemeCssVars(cssVars, cssVariablePrefix, defaultTheme, defaultStyle);
}

/**
 * Render a multi-theme `<span>` with CSS custom properties.
 *
 * ```ts
 * spanMultiThemes('const', {
 *   language: 'javascript',
 *   scope: 'keyword',
 *   themes: { light: githubLight, dark: githubDark },
 *   defaultTheme: 'light-dark()',
 * })
 * ```
 */
export function spanMultiThemes(text: string, options: SpanMultiThemesOptions): string {
  const escaped = escape(text);
  return `${openSpanTag(spanMultiThemesAttrs(options))}${escaped}</span>`;
}

function pushThemeCssVars(
  cssVars: string[],
  prefix: string,
  themeName: string,
  scope: string,
  language: LanguageRef | undefined,
  theme: Theme | undefined,
): void {
  const style = getScopedThemeStyle(theme, scope, language);
  if (!style) return;

  const sanitized = sanitizeThemeName(themeName);
  if (style.fg) cssVars.push(`${prefix}-${sanitized}:${style.fg};`);
  if (style.bg) cssVars.push(`${prefix}-${sanitized}-bg:${style.bg};`);
  cssVars.push(`${prefix}-${sanitized}-font-style:${style.italic ? "italic" : "normal"};`);
  cssVars.push(`${prefix}-${sanitized}-font-weight:${style.bold ? "bold" : "normal"};`);
  cssVars.push(`${prefix}-${sanitized}-text-decoration:${textDecoration(style)};`);
}

/**
 * Theme names in a stable order.
 *
 * Rust holds themes in a `HashMap` and sorts before emitting, so this has to
 * sort rather than follow insertion order for the two to agree byte for byte.
 *
 * `Array.prototype.sort` orders by UTF-16 code unit, which puts astral
 * characters before U+E000..U+FFFF; Rust orders `&str` by UTF-8 byte, which
 * puts them after. Comparing the encoded bytes is what makes the two agree.
 *
 * Exported for the multi-themes formatter next door, not for callers; Rust
 * keeps its counterpart private.
 * @internal
 */
export function sortedThemeNames(themes: Record<string, unknown>): string[] {
  const encoder = new TextEncoder();

  return Object.keys(themes).sort((left, right) => {
    const a = encoder.encode(left);
    const b = encoder.encode(right);

    for (let i = 0; i < Math.min(a.length, b.length); i += 1) {
      if (a[i] !== b[i]) return a[i]! - b[i]!;
    }

    return a.length - b.length;
  });
}

function pushThemeCssVarsForAll(
  cssVars: string[],
  prefix: string,
  themes: Record<string, Theme | undefined>,
  scope: string,
  language: LanguageRef | undefined,
  excludeTheme?: string,
): void {
  for (const themeName of sortedThemeNames(themes)) {
    if (themeName === excludeTheme) {
      continue;
    }

    pushThemeCssVars(cssVars, prefix, themeName, scope, language, themes[themeName]);
  }
}

/**
 * @deprecated Internal multi-themes plumbing, exported by accident; Rust keeps
 * its counterpart private. Use {@link spanMultiThemesAttrs}. Removed in the
 * next major.
 */
export function appendThemeCssVars(
  cssVars: string[],
  prefix: string,
  themes: Record<string, Theme | undefined>,
  scope: string,
  language: LanguageRef,
  excludeTheme?: string,
): void {
  pushThemeCssVarsForAll(cssVars, prefix, themes, scope, language, excludeTheme);
}

function pushNormalThemeVars(
  styles: string[],
  prefix: string,
  themes: Record<string, Theme>,
  excludeTheme?: string,
): void {
  for (const themeName of sortedThemeNames(themes)) {
    if (themeName === excludeTheme) {
      continue;
    }

    const sanitized = sanitizeThemeName(themeName);
    const style = getThemeStyle(themes[themeName], "normal");
    if (style?.fg) styles.push(`${prefix}-${sanitized}:${style.fg};`);
    if (style?.bg) styles.push(`${prefix}-${sanitized}-bg:${style.bg};`);
  }
}

/**
 * @deprecated Internal multi-themes plumbing, exported by accident; Rust keeps
 * its counterpart private. Use {@link openMultiThemesPreTag}. Removed in the
 * next major.
 */
export function buildNormalThemeVars(
  styles: string[],
  prefix: string,
  themes: Record<string, Theme>,
  excludeTheme?: string,
): void {
  pushNormalThemeVars(styles, prefix, themes, excludeTheme);
}

// The `<pre>` colours for `light-dark()`, falling back to black on white and
// white on black where a theme leaves them unset.
function lightDarkPreStyles(themes: Record<string, Theme>): string[] {
  const lightNormal = getThemeStyle(themes.light, "normal");
  const darkNormal = getThemeStyle(themes.dark, "normal");
  const lightFg = lightNormal?.fg ?? "#000000";
  const lightBg = lightNormal?.bg ?? "#ffffff";
  const darkFg = darkNormal?.fg ?? "#ffffff";
  const darkBg = darkNormal?.bg ?? "#000000";

  return [
    `color: light-dark(${lightFg}, ${darkFg});`,
    `background-color: light-dark(${lightBg}, ${darkBg});`,
  ];
}

function multiThemesPreStyle(options: {
  themes: Record<string, Theme>;
  defaultTheme?: string;
  cssVariablePrefix?: string;
}): string | undefined {
  const prefix = options.cssVariablePrefix ?? "--lumis";
  const styles: string[] = [];

  if (options.defaultTheme === "light-dark()") {
    styles.push(...lightDarkPreStyles(options.themes));
  } else if (options.defaultTheme) {
    const defaultStyle = getThemeStyle(options.themes[options.defaultTheme], "normal");
    if (defaultStyle?.fg) styles.push(`color:${defaultStyle.fg};`);
    if (defaultStyle?.bg) styles.push(`background-color:${defaultStyle.bg};`);
    pushNormalThemeVars(styles, prefix, options.themes, options.defaultTheme);
  } else {
    pushNormalThemeVars(styles, prefix, options.themes);
  }

  return styles.length > 0 ? styles.join(" ") : undefined;
}

/**
 * @deprecated Internal multi-themes plumbing, exported by accident; Rust keeps
 * its counterpart private. Use {@link openMultiThemesPreTag}, which builds the
 * whole tag. Removed in the next major.
 */
export function buildPreThemeStyle(options: {
  themes: Record<string, Theme>;
  defaultTheme?: string;
  cssVariablePrefix?: string;
}): string | undefined {
  return multiThemesPreStyle(options);
}

/**
 * @deprecated Composes {@link openPreTag}, {@link openCodeTag},
 * {@link wrapLine} and {@link closingTags} in the one order the built-in
 * formatters use, which is not a custom formatter's shape. Call them directly.
 * Each `lines` entry must already contain its source terminator, if any.
 * Removed in the next major.
 */
export function renderHtmlBlock(options: {
  lines: string[];
  language: LanguageRef | undefined;
  pre: string;
  lineOptions: (lineNumber: number) => { className?: string; style?: string };
  header?: HtmlElement;
}): string {
  const code = openCodeTag(options.language);
  const body = options.lines
    .map((line, idx) => wrapLine(idx + 1, line, options.lineOptions(idx + 1)))
    .join("");

  return wrapWithHeader(`${options.pre}${code}${body}${closingTags()}`, options.header);
}

/**
 * Wrap a line of highlighted HTML in a `<div>` with line metadata.
 * `content` is inserted verbatim, including any trailing newline.
 *
 * ```ts
 * wrapLine(1, '<span>const</span>\n', { className: 'l-highlighted' })
 * // '<div class="l-line l-highlighted" data-line="1"><span>const</span>\n</div>'
 * ```
 */
export function wrapLine(
  lineNumber: number,
  content: string,
  options: { className?: string; style?: string } = {},
): string {
  return `${tag("div", {
    class: classList("l-line", options.className),
    style: options.style,
    "data-line": lineNumber,
  })}${content}</div>`;
}

/**
 * The class the gutter element carries, for a stylesheet to hang a column off.
 *
 * The number is written out rather than left to `content: attr(data-line)`
 * because a formatter that cannot reach a stylesheet — `terminal` — has to show
 * the same thing, and because generated content is not in the document a reader
 * can inspect. It is `aria-hidden`, so a screen reader is not read a number
 * before every line.
 *
 * Private for the same reason `l-line` is: it is a class the built-in formatters
 * write, not a helper a custom one is built from.
 */
const LINE_NUMBER_CLASS = "l-line-number";
const HIGHLIGHTED_LINE_NUMBER_CLASS = "l-line-number-highlighted";

function lineNumberGutter(lineNumber: number, highlighted: boolean, attrs: HtmlAttrs): string {
  const open = openSpanTag({
    class: classList(LINE_NUMBER_CLASS, highlighted && HIGHLIGHTED_LINE_NUMBER_CLASS),
    ...attrs,
    "aria-hidden": "true",
  });
  return `${open}${lineNumber}</span>`;
}

/**
 * Check if a line number is in a list of highlighted lines.
 *
 * ```ts
 * lineIsHighlighted([1, [3, 5]], 4)  // true
 * lineIsHighlighted([1, [3, 5]], 2)  // false
 * ```
 */
export function lineIsHighlighted(lines: LineSpec[] | undefined, lineNumber: number): boolean {
  if (!lines) return false;

  return lines.some((line) =>
    typeof line === "number" ? line === lineNumber : lineNumber >= line[0] && lineNumber <= line[1],
  );
}

/**
 * Get the CSS class for a highlighted line, or `undefined` if not highlighted.
 *
 * ```ts
 * getHighlightLineClass([1, [3, 5]], 4, 'active')  // "active"
 * getHighlightLineClass([1, [3, 5]], 2, 'active')  // undefined
 * ```
 */
export function getHighlightLineClass(
  lines: LineSpec[] | undefined,
  lineNumber: number,
  className: string | undefined,
  defaultClass?: string,
): string | undefined {
  if (!lineIsHighlighted(lines, lineNumber)) {
    return undefined;
  }

  return className ?? defaultClass;
}

/**
 * Append a text fragment to the last line, splitting on newlines.
 *
 * ```ts
 * const lines = ['hello']
 * appendFragment(lines, ' world\nnew line')
 * // lines → ['hello world', 'new line']
 * ```
 * @internal
 */
export function appendFragment(lines: string[], fragment: string): void {
  if (!fragment.includes("\n")) {
    lines[lines.length - 1] += fragment;
    return;
  }

  const parts = fragment.split("\n");

  for (let i = 0; i < parts.length; i += 1) {
    lines[lines.length - 1] += parts[i] ?? "";
    if (i < parts.length - 1) {
      lines.push("");
    }
  }
}

/** What a formatter does with each step of a line-decorated stream. */
interface LineRenderOptions {
  formatText?: (text: string) => string;
  openSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  closeSpan?: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
}

interface LineRenderState {
  /** The current line's rendered content, without its terminator. */
  line: string;
  /** The exact source terminator held until every syntax span has closed. */
  ending: string;
  decoration: Extract<Decoration, { type: "line" }>;
  /** Lumis-owned layers, used to distinguish line ends from rainbow ends. */
  decorations: Decoration[];
  /** The document's language: the innermost scope's, once one has been open. */
  language: string;
  /** The close tag of each open scope, empty when the formatter omitted it. */
  openScopes: Array<{ close: string; language: string }>;
  /**
   * A line boundary reopens every span still open, so resolving a scope's tags
   * once is the difference between paying per scope and paying per scope per
   * line.
   */
  tags: Map<string, { open: string; close: string }>;
}

interface LineRenderContext {
  sourceBytes: Uint8Array;
  theme: Theme | undefined;
  formatText: (text: string) => string;
  openSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  closeSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  onLine: (content: string, decoration: Extract<Decoration, { type: "line" }>) => void;
}

function openSpanEvent(
  state: LineRenderState,
  context: LineRenderContext,
  event: { scope: string; language: string },
): void {
  const key = `${event.language} ${event.scope}`;
  let tags = state.tags.get(key);

  if (tags === undefined) {
    const span = emptySpan(event.scope, event.language);
    const style = getSpanStyle(context.theme, event.scope, event.language);
    const open = context.openSpan(span, style);
    // An `openSpan` that returns nothing means the formatter is deliberately
    // omitting the span, so nothing closes it either.
    tags = { open, close: open.length > 0 ? context.closeSpan(span, style) : "" };
    state.tags.set(key, tags);
  }

  state.line += tags.open;
  state.openScopes.push({ close: tags.close, language: event.language });
}

function sourceEvent(
  state: LineRenderState,
  context: LineRenderContext,
  event: { start: number; end: number },
): void {
  if (!state.language || state.language === "plaintext") {
    state.language = state.openScopes.at(-1)?.language ?? state.language;
  }

  const text = decodeSourceSlice(context.sourceBytes, event.start, event.end);
  const ending = text.endsWith("\r\n") ? "\r\n" : text.endsWith("\n") ? "\n" : "";
  state.ending = ending;
  state.line += context.formatText(ending ? text.slice(0, -ending.length) : text);
}

function startLineDecoration(
  state: LineRenderState,
  context: LineRenderContext,
  decoration: Decoration,
): void {
  state.decorations.push(decoration);
  if (decoration.type === "line") {
    state.decoration = decoration;
    state.line = "";
    state.ending = "";
    return;
  }
  openSpanEvent(state, context, {
    scope: rainbowBracketScope(decoration.depth),
    language: state.language,
  });
}

function endLineDecoration(state: LineRenderState, context: LineRenderContext): void {
  const decoration = state.decorations.pop();
  if (decoration?.type === "line") {
    context.onLine(`${state.line}${state.ending}`, state.decoration);
  } else if (decoration?.type === "rainbowBracket") {
    state.line += state.openScopes.pop()?.close ?? "";
  }
}

function applyLineEvent(
  state: LineRenderState,
  context: LineRenderContext,
  event: HighlightEvent,
): void {
  switch (event.type) {
    case "decorationStart":
      startLineDecoration(state, context, event.decoration);
      break;
    case "decorationEnd":
      endLineDecoration(state, context);
      break;
    case "start":
      openSpanEvent(state, context, event);
      break;
    case "end":
      state.line += state.openScopes.pop()?.close ?? "";
      break;
    case "source":
      sourceEvent(state, context, event);
      break;
    // Caller annotations carry data a built-in formatter has never seen.
    default:
      break;
  }
}

/**
 * Walk a line-decorated stream, handing each line's rendered content to `onLine`.
 *
 * Every span still open at a line boundary was closed and reopened by
 * {@link composeLineDecorations}, so this only has to render what it is given.
 *
 * Returns the document's language.
 */
function renderDecoratedLines(
  sourceBytes: Uint8Array,
  composed: readonly HighlightEvent[],
  theme: Theme | undefined,
  language: string,
  options: LineRenderOptions,
  onLine: (content: string, decoration: Extract<Decoration, { type: "line" }>) => void,
): string {
  const context: LineRenderContext = {
    sourceBytes,
    theme,
    formatText: options.formatText ?? escape,
    openSpan: options.openSpan,
    closeSpan: options.closeSpan ?? (() => "</span>"),
    onLine,
  };
  const state: LineRenderState = {
    line: "",
    ending: "",
    decoration: { type: "line", number: 1, highlighted: false },
    decorations: [],
    language,
    openScopes: [],
    tags: new Map(),
  };

  for (const event of composed) {
    applyLineEvent(state, context, event);
  }

  return state.language;
}

/** @internal */
export function formatHighlightIterLines(
  source: string,
  events: readonly HighlightEvent[],
  languageRef: LanguageRef | undefined,
  theme: Theme | undefined,
  options: LineRenderOptions,
): { lines: string[]; language: string } {
  const sourceBytes = encodeSource(source);
  const lines: string[] = [];
  const inferredLanguage = events.find((event) => event.type === "start");
  const language = renderDecoratedLines(
    sourceBytes,
    composeLineDecorations(sourceBytes, events, new LineSelection(undefined)),
    theme,
    languageRef ? languageId(languageRef) : (inferredLanguage?.language ?? "plaintext"),
    options,
    (content) => lines.push(content),
  );

  return { lines, language };
}

/**
 * Render a line-decorated stream as the `<div class="l-line">` blocks every
 * built-in HTML formatter emits.
 *
 * The three of them differ only in the attributes a span and a highlighted line
 * carry, so those are the two inputs; the walk itself is shared. A highlighted
 * line's attributes are resolved once rather than once per line.
 * The line renderer supplies the source terminator after closing syntax spans;
 * `wrapLine` places that content verbatim before `</div>`.
 *
 * @internal
 */
export function formatHtmlLines(
  source: string,
  events: readonly HighlightEvent[],
  formatter: {
    language: LanguageRef | undefined;
    theme: Theme | undefined;
    lines: readonly LineSpec[] | undefined;
    lineNumbers: boolean | undefined;
    lineNumberAttrs: { regular: HtmlAttrs; highlighted: HtmlAttrs };
    highlightedAttrs: { className?: string; style?: string };
    openSpan: (span: HighlightSpan, style: HighlightStyle | undefined) => string;
  },
): string {
  const sourceBytes = encodeSource(source);
  const numbered = formatter.lineNumbers === true;
  const composed = composeLineDecorations(sourceBytes, events, new LineSelection(formatter.lines));
  const parts: string[] = [];

  renderDecoratedLines(
    sourceBytes,
    composed,
    formatter.theme,
    formatter.language ? languageId(formatter.language) : "plaintext",
    { openSpan: formatter.openSpan },
    (content, decoration) => {
      parts.push(
        wrapLine(
          decoration.number,
          numbered
            ? `${lineNumberGutter(
                decoration.number,
                decoration.highlighted,
                decoration.highlighted
                  ? formatter.lineNumberAttrs.highlighted
                  : formatter.lineNumberAttrs.regular,
              )}${content}`
            : content,
          decoration.highlighted ? formatter.highlightedAttrs : {},
        ),
      );
    },
  );

  return parts.join("");
}

/**
 * Render highlight events into escaped HTML lines, reopening active spans across newlines.
 * Each line carries its exact source terminator after any closing span tags;
 * an unterminated final line has none, so the results can go straight to
 * {@link wrapLine}.
 *
 * ```ts
 * renderLinesFromEvents('a\nb', events, (scope) => `class="${scope}"`)
 * // ['<span class="...">a</span>\n', '<span class="...">b</span>']
 * ```
 */
export function renderLinesFromEvents(
  source: string,
  events: readonly HighlightEvent[],
  spanAttrs: (scope: string, language: string) => string,
): string[] {
  return formatHighlightIterLines(source, events, undefined, undefined, {
    openSpan: (span) => openSpan(spanAttrs(span.scope, span.language)),
  }).lines;
}

/**
 * Render highlight events into a single HTML buffer plus line offsets.
 *
 * The returned offsets can be passed to {@link linesFromOffsets}.
 */
const ESCAPED_CHARS: Record<string, string> = {
  "&": "&amp;",
  "<": "&lt;",
  ">": "&gt;",
  '"': "&quot;",
  "'": "&#39;",
};

function escapeChar(char: string): string {
  return ESCAPED_CHARS[char] ?? char;
}

/**
 * @deprecated Records line offsets without closing and reopening the spans that
 * cross a line boundary, so slicing the buffer at them yields lines whose tags
 * do not nest. Use {@link renderLinesFromEvents}. Removed in the next major.
 */
export function renderEvents(
  source: string,
  events: SyntaxHighlightEvent[],
  attributeCallback: (scope: string, language: string, html: string[]) => void,
): [Uint8Array, number[]] {
  const sourceBytes = encodeSource(source);
  const html: string[] = [];
  const lineOffsets = [0];
  let renderedLength = 0;

  const push = (fragment: string): void => {
    html.push(fragment);
    renderedLength += fragment.length;
  };

  for (const event of events) {
    if (event.type === "start") {
      const attrs: string[] = [];
      attributeCallback(event.scope, event.language, attrs);
      push(openSpan(attrs.join("")));
      continue;
    }

    if (event.type === "end") {
      push("</span>");
      continue;
    }

    const text = decodeSourceSlice(sourceBytes, event.start, event.end);
    for (let i = 0; i < text.length; i += 1) {
      const char = text[i]!;
      push(escapeChar(char));
      if (char === "\n") lineOffsets.push(renderedLength);
    }
  }

  return [encodeSource(html.join("")), lineOffsets];
}

/**
 * Slice rendered HTML back into lines using offsets from {@link renderEvents}.
 *
 * @deprecated Only useful for slicing what {@link renderEvents} returns. Use
 * {@link renderLinesFromEvents}. Removed in the next major.
 */
export function linesFromOffsets(html: Uint8Array, lineOffsets: number[]): string[] {
  const rendered = _decoder.decode(html);
  const lines: string[] = [];

  for (let i = 0; i < lineOffsets.length; i += 1) {
    const start = lineOffsets[i] ?? 0;
    const end = lineOffsets[i + 1] ?? rendered.length;
    lines.push(rendered.slice(start, end));
  }

  return lines;
}
