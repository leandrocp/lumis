import type { Root, Element, Properties, RootContent } from "hast";
import type { Highlighter, LanguageInput } from "@lumis-sh/lumis";
import type { Formatter } from "@lumis-sh/lumis/formatters";
import type { Plugin } from "unified";
import { createHighlighter } from "@lumis-sh/lumis";
import { fromHtml } from "hast-util-from-html";
import { toString } from "hast-util-to-string";
import { visit } from "unist-util-visit";

const LANGUAGE_PREFIX = "language-";

export interface RehypeLumisOptions {
  formatter: (language: string | undefined) => Formatter;
  languages?: LanguageInput[];
}

interface ParsedCodeBlock {
  code: string;
  codeProperties: Properties;
  language?: string;
  preProperties: Properties;
}

function getPropertyString(value: unknown): string | undefined {
  return typeof value === "string" && value.length > 0 ? value : undefined;
}

function getClassNames(node: Element): string[] {
  const className = node.properties.className;
  if (!Array.isArray(className)) {
    return [];
  }

  return className.filter((value): value is string => typeof value === "string");
}

function getLanguageFromClassNames(node: Element): string | undefined {
  return getClassNames(node)
    .find((className) => className.startsWith(LANGUAGE_PREFIX))
    ?.slice(LANGUAGE_PREFIX.length);
}

function getLanguageFromProperties(node: Element): string | undefined {
  return (
    getPropertyString(node.properties.dataLanguage) ??
    getPropertyString(node.properties["data-language"]) ??
    getPropertyString(node.properties.language)
  );
}

function parseCodeBlock(node: Element): ParsedCodeBlock | undefined {
  const head = node.children[0];
  if (!head || head.type !== "element" || head.tagName !== "code") {
    return undefined;
  }

  const language =
    getLanguageFromClassNames(head) ??
    getLanguageFromClassNames(node) ??
    getLanguageFromProperties(node);

  return {
    code: toString(head),
    codeProperties: head.properties,
    language,
    preProperties: node.properties,
  };
}

function parseFragment(html: string): RootContent[] {
  return fromHtml(html, { fragment: true }).children;
}

function getPropertyClassNames(value: Properties["className"]): string[] {
  const values = Array.isArray(value) ? value : [value];
  return values.flatMap((entry) =>
    typeof entry === "string" || typeof entry === "number"
      ? String(entry).split(/\s+/).filter(Boolean)
      : [],
  );
}

function mergeClassNames(
  rendered: Properties["className"],
  authored: Properties["className"],
): string[] {
  return [...new Set([...getPropertyClassNames(rendered), ...getPropertyClassNames(authored)])];
}

function mergeStyles(rendered: Properties["style"], authored: Properties["style"]): string {
  const renderedStyle = typeof rendered === "string" ? rendered.trim() : "";
  const authoredStyle = typeof authored === "string" ? authored.trim() : "";

  if (renderedStyle.length === 0) return authoredStyle;
  if (authoredStyle.length === 0) return renderedStyle;

  return `${renderedStyle.replace(/;?$/, ";")} ${authoredStyle}`;
}

function mergeProperties(rendered: Properties, authored: Properties): void {
  const className = mergeClassNames(rendered.className, authored.className);
  const style = mergeStyles(rendered.style, authored.style);
  const hasAuthoredClassName = Object.hasOwn(authored, "className");
  const hasAuthoredStyle = Object.hasOwn(authored, "style");

  Object.assign(rendered, authored);

  if (hasAuthoredClassName) rendered.className = className;
  if (hasAuthoredStyle) rendered.style = style;
}

function findElement(children: readonly RootContent[], tagName: string): Element | undefined {
  for (const child of children) {
    if (child.type !== "element") continue;
    if (child.tagName === tagName) return child;

    const nested = findElement(child.children, tagName);
    if (nested) return nested;
  }

  return undefined;
}

function preserveProperties(replacement: RootContent[], parsed: ParsedCodeBlock): void {
  const pre = findElement(replacement, "pre");
  if (!pre) return;

  mergeProperties(pre.properties, parsed.preProperties);

  const code = findElement(pre.children, "code");
  if (code) mergeProperties(code.properties, parsed.codeProperties);
}

async function renderBlock(
  highlighter: Highlighter,
  parsed: ParsedCodeBlock,
  formatter: (language: string | undefined) => Formatter,
): Promise<RootContent[]> {
  // Every other runtime loads whatever a document names, and this plugin is not
  // the place to invent a second rule. A fence naming something that cannot be
  // loaded costs that block its highlighting, not the document its render.
  let resolved = parsed.language;
  if (resolved != null) {
    try {
      await highlighter.loadLanguage(resolved);
    } catch {
      resolved = undefined;
    }
  }

  const html = highlighter.highlight(parsed.code, formatter(resolved));
  const replacement = parseFragment(html);
  preserveProperties(replacement, parsed);

  return replacement;
}

const rehypeLumis: Plugin<[RehypeLumisOptions], Root> = function rehypeLumis(options) {
  const setup = createHighlighter({
    languages: options.languages ?? [],
  });

  return async function transform(tree) {
    const highlighter = await setup;
    const targets: Array<{
      parent: Element | Root;
      index: number;
      parsed: ParsedCodeBlock;
    }> = [];

    visit(tree, "element", (node, index, parent) => {
      if (!parent || index == null || node.tagName !== "pre") {
        return;
      }

      const parsed = parseCodeBlock(node);
      if (!parsed) {
        return;
      }

      targets.push({ parent, index, parsed });
      return "skip";
    });

    const replacements = await Promise.all(
      targets.map(async ({ parsed }) => {
        try {
          return await renderBlock(highlighter, parsed, options.formatter);
        } catch {
          return;
        }
      }),
    );

    for (let i = targets.length - 1; i >= 0; i -= 1) {
      const target = targets[i];
      const replacement = replacements[i];
      if (!target || !replacement) {
        continue;
      }

      target.parent.children.splice(target.index, 1, ...replacement);
    }
  };
};

export default rehypeLumis;
