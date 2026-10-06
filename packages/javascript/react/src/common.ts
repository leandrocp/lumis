import type { Formatter } from "@lumis-sh/lumis/formatters";
import { isValidElement, type ReactNode } from "react";
import { fromHtml } from "hast-util-from-html";
import { toJsxRuntime } from "hast-util-to-jsx-runtime";
import { Fragment } from "react";
import { jsx, jsxs } from "react/jsx-runtime";

export interface LumisBaseOptions {
  children: string;
  formatter: Formatter;
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
