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
} from "./types.js";
import { getBuiltinFormatter, markBuiltinFormatter } from "./core/builtin-formatter.js";
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
 * import dracula from '@lumis-sh/themes/dracula'
 * import javascript from '@lumis-sh/wasm-javascript'
 *
 * hl.highlight('const x = 1', htmlInline({ language: javascript, theme: dracula }))
 * ```
 */
export function htmlInline(options: HtmlInlineOptions = {}): HtmlInlineFormatter {
  const formatter: HtmlInlineFormatter = {
    ...options,
    render(source, events, budget): string {
      return formatHtmlInline(source, events, formatter, budget);
    },
  };
  return markBuiltinFormatter({ kind: "html-inline", formatter });
}

/**
 * Create a class-based HTML formatter. Each token gets a `<span class="...">` with
 * semantic scope names. Requires a theme CSS file on the page.
 *
 * @example
 * ```ts
 * import { htmlLinked } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/wasm-javascript'
 * import '@lumis-sh/themes/css/dracula.css'
 *
 * hl.highlight('const x = 1', htmlLinked({ language: javascript }))
 * ```
 */
export function htmlLinked(options: HtmlLinkedOptions = {}): HtmlLinkedFormatter {
  const formatter: HtmlLinkedFormatter = {
    ...options,
    render(source, events, budget): string {
      return formatHtmlLinked(source, events, formatter, budget);
    },
  };
  return markBuiltinFormatter({ kind: "html-linked", formatter });
}

/**
 * Create a multi-theme HTML formatter using CSS custom properties.
 * Light/dark switching works via `prefers-color-scheme`.
 *
 * @example
 * ```ts
 * import { htmlMultiThemes } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/wasm-javascript'
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
    render(source, events, budget): string {
      // Formatter objects are mutable in JavaScript, so construction-time
      // validation alone does not protect the actual render boundary.
      validateMultiThemes(formatter);
      return formatHtmlMultiThemes(source, events, formatter, budget);
    },
  };
  return markBuiltinFormatter({ kind: "html-multi-themes", formatter });
}

/**
 * Create a BBCode scoped formatter using highlight scope names as nested tags.
 * It does not emit standard forum-style BBCode like `[b]`, `[color]`, or `[code]`.
 *
 * @example
 * ```ts
 * import { bbcodeScoped } from '@lumis-sh/lumis/formatters'
 * import javascript from '@lumis-sh/wasm-javascript'
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
  return markBuiltinFormatter({ kind: "bbcode-scoped", formatter });
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
 * add an option before the formatter exists. In JavaScript and TypeScript, the
 * formatter is already built by the time a caller holds one. `render` closes
 * over the object, so spreading it produces something that still renders
 * through the original. This is the derive step those two get for free.
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
 * Rebuilt HTML formatters have the factory's return type; narrower attribute
 * or render types attached to the original cannot be preserved by a rebuild.
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
export function withAttrs<T extends Formatter>(
  formatter: T,
  attrs: FormatterAttrs,
): T | HtmlInlineFormatter | HtmlLinkedFormatter | HtmlMultiThemesFormatter {
  const builtin = getBuiltinFormatter(formatter);
  if (!builtin) return formatter;

  // Rebuild through the matching factory so render closes over the derived
  // options. Its result is a built-in formatter, not an arbitrary subtype T
  // whose render method or attribute values may have narrower types.
  switch (builtin.kind) {
    case "html-inline":
      return htmlInline({
        ...builtin.formatter,
        preAttrs: layerAttrs(builtin.formatter.preAttrs, attrs.preAttrs),
        codeAttrs: layerAttrs(builtin.formatter.codeAttrs, attrs.codeAttrs),
      });
    case "html-linked":
      return htmlLinked({
        ...builtin.formatter,
        preAttrs: layerAttrs(builtin.formatter.preAttrs, attrs.preAttrs),
        codeAttrs: layerAttrs(builtin.formatter.codeAttrs, attrs.codeAttrs),
      });
    case "html-multi-themes":
      return htmlMultiThemes({
        ...builtin.formatter,
        preAttrs: layerAttrs(builtin.formatter.preAttrs, attrs.preAttrs),
        codeAttrs: layerAttrs(builtin.formatter.codeAttrs, attrs.codeAttrs),
      });
    case "bbcode-scoped":
    case "terminal":
      break;
  }
  return formatter;
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
  HtmlStructure,
  HtmlInlineFormatter,
  HtmlInlineOptions,
  HtmlLinkedFormatter,
  HtmlLinkedOptions,
  HtmlMultiThemesFormatter,
  HtmlMultiThemesOptions,
  TerminalFormatter,
  TerminalOptions,
} from "./types.js";
