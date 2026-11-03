import { toInline, type Formatter } from "@lumis-sh/lumis/formatters";
import { openTag, type HtmlAttrs } from "@lumis-sh/lumis/formatters/html";
import { isValidElement, type ReactNode } from "react";
import { fromHtml } from "hast-util-from-html";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";

export interface LumisBaseOptions {
  children: string;
  formatter: Formatter;
}

export interface InlineCodeOptions {
  /**
   * Attributes for the `<code>` element, merged with the `lumis` class and the
   * theme's colors.
   */
  attrs?: HtmlAttrs;
}

export interface LumisRenderOptions extends LumisBaseOptions, InlineCodeOptions {
  /**
   * Render a `<code>` element holding the highlighted spans, for code inside
   * prose, instead of a `<pre>` block.
   */
  inline?: boolean;
}

interface HtmlHighlighter {
  highlight(source: string, formatter: Formatter): string;
}

export function highlightHtml(highlighter: HtmlHighlighter, options: LumisRenderOptions): string {
  if (options.inline !== true) {
    return highlighter.highlight(options.children, options.formatter);
  }

  const inline = toInline(options.formatter, options.attrs);
  if (!inline) {
    throw new TypeError("Inline code needs a built-in HTML formatter");
  }

  return `${openTag("code", inline.attrs)}${highlighter.highlight(options.children, inline.formatter)}</code>`;
}

export function getLanguageId(language: Formatter["language"]): string | undefined {
  if (typeof language === "string") {
    return language;
  }

  if (typeof language === "object" && language !== null && "id" in language) {
    return language.id;
  }

  return undefined;
}

export function toReactNode(html: string): ReactNode {
  const tree = fromHtml(html, { fragment: true });
  // The upstream HAST helper's global JSX.Element declaration does not establish React's element contract.
  // Validate its output before exposing it as a ReactNode.
  const node: unknown = toJsxRuntime(tree, {
    Fragment,
    jsx,
    jsxs,
  });

  if (!isValidElement(node)) {
    throw new TypeError("Expected highlighted HTML to produce a React element");
  }

  return node;
}
