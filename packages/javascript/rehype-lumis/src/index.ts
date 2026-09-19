import type { Element, ElementContent, Properties, Root, RootContent } from "hast";
import type { Highlighter, LanguageInput } from "@lumis-sh/lumis";
import type { Formatter, HtmlAttrs } from "@lumis-sh/lumis/formatters";
import type { Plugin } from "unified";
import { createHighlighter } from "@lumis-sh/lumis";
import { withAttrs } from "@lumis-sh/lumis/formatters";
import { fromHtml } from "hast-util-from-html";
import { find, html as htmlSchema } from "property-information";
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

/**
 * A hast property as the attribute HTML spells it.
 *
 * The authored side of a code block is hast, and the merge that has to happen
 * to it lives in the formatter, which speaks attributes. `property-information`
 * owns that mapping, including how a list-valued property is joined, so this
 * does not have a table of its own.
 */
function propertyToAttr(name: string, value: Properties[string]): [string, HtmlAttrs[string]] {
  const info = find(htmlSchema, name);

  if (Array.isArray(value)) {
    return [info.attribute, value.join(info.commaSeparated ? ", " : " ")];
  }

  return [info.attribute, value];
}

function propertiesToAttrs(properties: Properties): HtmlAttrs | undefined {
  const entries = Object.entries(properties)
    .filter(([, value]) => value != null && value !== false)
    .map(([name, value]) => propertyToAttr(name, value));

  return entries.length > 0 ? Object.fromEntries(entries) : undefined;
}

/**
 * Re-attach whatever the authored `<pre>` held after its `<code>`.
 *
 * Properties go through the formatter, but a sibling element does not: a copy
 * button or a caption another plugin put inside the `<pre>` has no attribute
 * to become, and Lumis has no reason to know about it.
 */
function restoreTrailingChildren(replacement: RootContent[], parsed: ParsedCodeBlock): void {
  if (parsed.trailingChildren.length === 0) return;

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
  const trailingIndex = codeIndex === -1 ? pre.children.length : codeIndex + 1;
  pre.children.splice(trailingIndex, 0, ...parsed.trailingChildren);
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

  // The authored properties go to the formatter rather than onto the hast it
  // produces, so `class` unions and `style` appends exactly once, where Rust
  // defines those rules, instead of a second time here.
  const rendered = highlighter.highlight(
    parsed.code,
    withAttrs(formatter(resolved), {
      preAttrs: propertiesToAttrs(parsed.preProperties),
      codeAttrs: propertiesToAttrs(parsed.codeProperties),
    }),
  );

  const replacement = parseFragment(rendered);
  restoreTrailingChildren(replacement, parsed);

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
