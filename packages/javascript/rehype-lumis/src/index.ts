import type { Element, ElementContent, Properties, Root, RootContent } from "hast";
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
  language?: string;
  preProperties: Properties;
  codeProperties: Properties;
  trailingChildren: ElementContent[];
}

function getPropertyString(value: unknown): string | undefined {
  return typeof value === "string" && value.length > 0 ? value : undefined;
}

function propertyClassNames(value: unknown): string[] {
  if (typeof value === "string") {
    return value.split(/\s+/).filter(Boolean);
  }

  if (!Array.isArray(value)) {
    return [];
  }

  return value.filter((entry): entry is string => typeof entry === "string");
}

function getClassNames(node: Element): string[] {
  return propertyClassNames(node.properties.className);
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
    language,
    preProperties: node.properties,
    codeProperties: head.properties,
    trailingChildren: node.children.slice(1),
  };
}

function parseFragment(html: string): RootContent[] {
  return fromHtml(html, { fragment: true }).children;
}

function mergedStyle(generated: unknown, authored: unknown): string | undefined {
  const generatedStyle = typeof generated === "string" ? generated.trim() : "";
  const authoredStyle = typeof authored === "string" ? authored.trim() : "";

  if (generatedStyle.length === 0) return authoredStyle || undefined;
  if (authoredStyle.length === 0) return generatedStyle;

  return `${generatedStyle.replace(/;$/, "")}; ${authoredStyle}`;
}

function mergeProperties(generated: Properties, authored: Properties): Properties {
  const merged = { ...generated, ...authored };
  const className = [
    ...new Set([
      ...propertyClassNames(generated.className),
      ...propertyClassNames(authored.className),
    ]),
  ];
  const style = mergedStyle(generated.style, authored.style);

  if (className.length > 0) merged.className = className;
  if (style) merged.style = style;

  return merged;
}

function mergeAuthoredProperties(replacement: RootContent[], parsed: ParsedCodeBlock): void {
  let pre: Element | undefined;
  visit({ type: "root", children: replacement }, "element", (node) => {
    if (!pre && node.tagName === "pre") {
      pre = node;
      return "skip";
    }
  });
  if (!pre) return;

  const codeIndex = pre.children.findIndex(
    (node) => node.type === "element" && node.tagName === "code",
  );
  const code = pre.children[codeIndex];

  pre.properties = mergeProperties(pre.properties, parsed.preProperties);
  if (code?.type === "element") {
    code.properties = mergeProperties(code.properties, parsed.codeProperties);
  }
  const trailingIndex = codeIndex === -1 ? pre.children.length : codeIndex + 1;
  pre.children.splice(trailingIndex, 0, ...parsed.trailingChildren);
}

async function renderBlock(
  highlighter: Highlighter,
  code: string,
  language: string | undefined,
  formatter: (language: string | undefined) => Formatter,
): Promise<RootContent[]> {
  // Every other runtime loads whatever a document names, and this plugin is not
  // the place to invent a second rule. A fence naming something that cannot be
  // loaded costs that block its highlighting, not the document its render.
  let resolved = language;
  if (resolved != null) {
    try {
      await highlighter.loadLanguage(resolved);
    } catch {
      resolved = undefined;
    }
  }

  const html = highlighter.highlight(code, formatter(resolved));

  return parseFragment(html);
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
          const replacement = await renderBlock(
            highlighter,
            parsed.code,
            parsed.language,
            options.formatter,
          );
          mergeAuthoredProperties(replacement, parsed);
          return replacement;
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
