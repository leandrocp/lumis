import type {
  BBCodeScopedFormatter,
  BBCodeScopedOptions,
  Formatter,
  HtmlAttrs,
  HtmlInlineOptions,
  HtmlInlineFormatter,
  HtmlLinkedOptions,
  HtmlLinkedFormatter,
  HtmlMultiThemesOptions,
  HtmlMultiThemesFormatter,
  TerminalOptions,
  TerminalFormatter,
} from "./types.js";
import { builtinFormatterKind, markBuiltinFormatter } from "./core/builtin-formatter.js";
import { layerAttrs } from "./core/attr-merge.js";
import { formatBBCode } from "./formatter/bbcode.js";
import { formatHtmlInline } from "./formatter/html-inline.js";
import { formatHtmlLinked } from "./formatter/html-linked.js";
import { formatHtmlMultiThemes } from "./formatter/html-multi-themes.js";

/**
 * Create an inline-styles HTML formatter. Each token gets a `<span>` with
 * `style="color: ..."` pulled from the theme.
 *
 * @example
 * ```ts
 * import { htmlInline } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/lumis/langs/javascript'
 * import dracula from '@lumis-sh/themes/dracula'
 *
 * hl.highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
 * ```
 */
export function htmlInline(options: HtmlInlineOptions = {}): HtmlInlineFormatter {
  const formatter: HtmlInlineFormatter = {
    ...options,
    render(source, events): string {
      return formatHtmlInline(source, events, formatter);
    },
  };
  return markBuiltinFormatter(formatter, "html-inline");
}

/**
 * Create a class-based HTML formatter. Each token gets a `<span class="...">` with
 * semantic scope names. Requires a theme CSS file on the page.
 *
 * @example
 * ```ts
 * import { htmlLinked } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/lumis/langs/javascript'
 * import '@lumis-sh/themes/css/dracula.css'
 *
 * hl.highlight('const x = 1', htmlLinked({ language: javascript }))
 * ```
 */
export function htmlLinked(options: HtmlLinkedOptions = {}): HtmlLinkedFormatter {
  const formatter: HtmlLinkedFormatter = {
    ...options,
    render(source, events): string {
      return formatHtmlLinked(source, events, formatter);
    },
  };
  return markBuiltinFormatter(formatter, "html-linked");
}

/**
 * Create a multi-theme HTML formatter using CSS custom properties.
 * Light/dark switching works via `prefers-color-scheme`.
 *
 * @example
 * ```ts
 * import { htmlMultiThemes } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/lumis/langs/javascript'
 * import githubLight from '@lumis-sh/themes/github_light'
 * import githubDark from '@lumis-sh/themes/github_dark'
 *
 * hl.highlight('const x = 1', htmlMultiThemes({
 *   language: javascript,
 *   themes: { light: githubLight, dark: githubDark },
 *   defaultTheme: 'light-dark()',
 * }))
 * ```
 */
/**
 * The same three rejections as `HtmlMultiThemesBuilder::build` in Rust. Without
 * them an unknown `defaultTheme` renders a `<pre>` with no color, and
 * `light-dark()` with a theme missing renders black-on-white placeholders.
 */
function validateMultiThemes(options: HtmlMultiThemesOptions): void {
  const names = Object.keys(options.themes ?? {});

  if (names.length === 0) {
    throw new Error("htmlMultiThemes requires at least one theme");
  }

  const { defaultTheme } = options;
  if (defaultTheme === undefined) {
    return;
  }

  if (defaultTheme === "light-dark()") {
    const missing = ["light", "dark"].filter((name) => !names.includes(name));
    if (missing.length > 0) {
      throw new Error(
        `htmlMultiThemes defaultTheme "light-dark()" requires themes named "light" and "dark", missing ${missing.join(" and ")}`,
      );
    }
    return;
  }

  if (!names.includes(defaultTheme)) {
    throw new Error(
      `htmlMultiThemes defaultTheme "${defaultTheme}" is not one of the themes given (${names.join(", ")})`,
    );
  }
}

export function htmlMultiThemes(options: HtmlMultiThemesOptions): HtmlMultiThemesFormatter {
  validateMultiThemes(options);

  const formatter: HtmlMultiThemesFormatter = {
    ...options,
    render(source, events): string {
      // Formatter objects are mutable in JavaScript, so construction-time
      // validation alone does not protect the actual render boundary.
      validateMultiThemes(formatter);
      return formatHtmlMultiThemes(source, events, formatter);
    },
  };
  return markBuiltinFormatter(formatter, "html-multi-themes");
}

/**
 * Create a BBCode scoped formatter using highlight scope names as nested tags.
 * It does not emit standard forum-style BBCode like `[b]`, `[color]`, or `[code]`.
 *
 * @example
 * ```ts
 * import { bbcodeScoped } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/lumis/langs/javascript'
 *
 * const output = hl.highlight('const x = 1', bbcodeScoped({ language: javascript }))
 * console.log(output)
 * ```
 */
export function bbcodeScoped(options: BBCodeScopedOptions = {}): BBCodeScopedFormatter {
  const formatter: BBCodeScopedFormatter = {
    ...options,
    render(source, events): string {
      return formatBBCode(source, events, formatter);
    },
  };
  return markBuiltinFormatter(formatter, "bbcode-scoped");
}

/** Attributes {@link withAttrs} layers onto a formatter. */
export interface FormatterAttrs {
  /** Merged into the wrapping `<pre>` tag, over any `preAttrs` already set. */
  preAttrs?: HtmlAttrs;
  /** Merged into the nested `<code>` tag, over any `codeAttrs` already set. */
  codeAttrs?: HtmlAttrs;
}

/**
 * Derive a formatter that carries extra `<pre>` and `<code>` attributes.
 *
 * Rust spells this as a builder and Elixir as a keyword list, both of which can
 * add an option before the formatter exists. A JavaScript formatter is already
 * built by the time a caller holds one — `render` closes over the object, so
 * spreading it produces something that still renders through the original — and
 * this is the derive step those two get for free.
 *
 * It is what a renderer integration wants: `rehype-lumis` and
 * `markdown-it-lumis` take a formatter from the user and have per-block
 * attributes to add, without a say in how the formatter was constructed.
 *
 * The attributes merge the same way the formatters' own do: `class` unions,
 * `style` appends, everything else replaces, and `false` removes.
 *
 * Returns the formatter untouched when it is not a built-in HTML one, since
 * there is no attribute contract to honour on a custom `render`.
 *
 * @example
 * ```ts
 * import { htmlInline, withAttrs } from '@lumis-sh/lumis/formatters'
 *
 * const base = htmlInline({ language: javascript, theme: dracula })
 * const block = withAttrs(base, { preAttrs: { id: 'example', class: 'card' } })
 * // base is unchanged
 * ```
 */
export function withAttrs<T extends Formatter>(formatter: T, attrs: FormatterAttrs): T {
  const current = formatter as Formatter & FormatterAttrs;
  const preAttrs = layerAttrs(current.preAttrs, attrs.preAttrs);
  const codeAttrs = layerAttrs(current.codeAttrs, attrs.codeAttrs);

  // `render` closes over the formatter it was built with, so the derived one
  // has to come back through the factory rather than out of a spread. Each
  // factory writes its own `render` after spreading what it is given, so the
  // stale one carried in here is replaced rather than inherited.
  switch (builtinFormatterKind(formatter)) {
    case "html-inline":
      return htmlInline({
        ...(formatter as unknown as HtmlInlineOptions),
        preAttrs,
        codeAttrs,
      }) as unknown as T;
    case "html-linked":
      return htmlLinked({
        ...(formatter as unknown as HtmlLinkedOptions),
        preAttrs,
        codeAttrs,
      }) as unknown as T;
    case "html-multi-themes":
      return htmlMultiThemes({
        ...(formatter as unknown as HtmlMultiThemesOptions),
        preAttrs,
        codeAttrs,
      }) as unknown as T;
    default:
      return formatter;
  }
}

export type {
  BBCodeScopedFormatter,
  BBCodeScopedOptions,
  Annotation,
  Formatter,
  HighlightOptions,
  HighlightCallback,
  HighlightEvent,
  HighlightIterFn,
  HighlightRange,
  HighlightSpan,
  HighlightStyle,
  HtmlAttrs,
  HtmlInlineFormatter,
  HtmlInlineOptions,
  HtmlLinkedFormatter,
  HtmlLinkedOptions,
  HtmlMultiThemesFormatter,
  HtmlMultiThemesOptions,
  TerminalFormatter,
  TerminalOptions,
} from "./types.js";
