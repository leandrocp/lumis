import type { Element, ElementContent, Properties, Root, RootContent } from "hast";
import type { Highlighter, LanguageInput } from "@lumis-sh/lumis";
import type { Formatter, HtmlAttrs } from "@lumis-sh/lumis/formatters";
import type { Plugin } from "unified";
import { createHighlighter } from "@lumis-sh/lumis";
import { toInline, withAttrs } from "@lumis-sh/lumis/formatters";
import { openTag } from "@lumis-sh/lumis/formatters/html";
import { fromHtml } from "hast-util-from-html";
import { find, html as htmlSchema } from "property-information";
import { toString } from "hast-util-to-string";
import { visit } from "unist-util-visit";

const LANGUAGE_PREFIX = "language-";

export interface RehypeLumisOptions {
  formatter: (language: string | undefined) => Formatter;
  languages?: LanguageInput[];
  /**
   * Language for unlabeled blocks, or false to leave them untouched.
   * Defaults to "plaintext" in rehype-lumis and false in @lumis-sh/vite.
   */
  defaultLanguage?: string | false;
  /**
   * Also highlight inline `<code>`, outside a `<pre>`, that names its
   * language with a `language-*` class, a `data-language` attribute or a
   * `{:lang}` suffix in its text. Defaults to `false`.
   */
  inlineCode?: boolean;
}

interface ParsedCodeBlock {
  code: string;
  language?: string;
  preProperties: Properties;
  codeProperties: Properties;
  trailingChildren: ElementContent[];
}

function getPropertyString(value: Properties[keyof Properties]): string | undefined {
  return typeof value === "string" && value.length > 0 ? value : undefined;
}

function propertyClassNames(value: Properties[keyof Properties]): string[] {
  if (typeof value === "string") {
    return value.split(/\s+/u).filter(Boolean);
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
    .find(
      (className) =>
        className.startsWith(LANGUAGE_PREFIX) && className.length > LANGUAGE_PREFIX.length,
    )
    ?.slice(LANGUAGE_PREFIX.length);
}

function getLanguageFromProperties(node: Element): string | undefined {
  return (
    getPropertyString(node.properties.dataLanguage) ??
    getPropertyString(node.properties["data-language"]) ??
    getPropertyString(node.properties.language)
  );
}

function skipsHighlighting(...nodes: Element[]): boolean {
  return nodes
    .flatMap((node) => getClassNames(node))
    .some((className) => className === "nohighlight" || className === "no-highlight");
}

function parseCodeBlock(
  node: Element,
  defaultLanguage: string | false,
): ParsedCodeBlock | undefined {
  const head = node.children[0];
  if (!head || head.type !== "element" || head.tagName !== "code") {
    return undefined;
  }

  if (skipsHighlighting(node, head)) {
    return undefined;
  }

  const language =
    getLanguageFromClassNames(head) ??
    getLanguageFromClassNames(node) ??
    getLanguageFromProperties(node) ??
    defaultLanguage;

  if (language === false) {
    return undefined;
  }

  return {
    code: toString(head),
    language,
    preProperties: node.properties,
    codeProperties: head.properties,
    trailingChildren: node.children.slice(1),
  };
}

interface ParsedInlineCode {
  code: string;
  language: string;
  codeProperties: Properties;
}

// `{:.name}` is how rehype-pretty-code names a highlight scope rather than a
// language, so a name starting with a dot is not a marker.
const LANGUAGE_NAME = /^[\w+#-][\w+#.-]*$/u;

/** Split a trailing `{:lang}` marker off inline code, as `` `x = 1{:python}` `` writes it. */
function splitLanguageSuffix(text: string): { code: string; language: string } | undefined {
  if (!text.endsWith("}")) return undefined;

  const open = text.lastIndexOf("{:");
  if (open <= 0) return undefined;

  const language = text.slice(open + 2, -1);
  return LANGUAGE_NAME.test(language) ? { code: text.slice(0, open), language } : undefined;
}

function parseInlineCode(node: Element): ParsedInlineCode | undefined {
  if (skipsHighlighting(node)) return undefined;

  const text = toString(node);
  const language = getLanguageFromClassNames(node) ?? getLanguageFromProperties(node);
  const parsed = language === undefined ? splitLanguageSuffix(text) : { code: text, language };

  return parsed && { ...parsed, codeProperties: node.properties };
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
function propertyToAttr(
  name: string,
  value: Properties[keyof Properties],
): [string, HtmlAttrs[string]] {
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
    // oxlint-disable-next-line unicorn/no-useless-undefined -- return type distinguishes a skipped node from normal traversal.
    return undefined;
  });
  if (!pre) return;

  const codeIndex = pre.children.findIndex(
    (node) => node.type === "element" && node.tagName === "code",
  );
  const trailingIndex = codeIndex === -1 ? pre.children.length : codeIndex + 1;
  pre.children.splice(trailingIndex, 0, ...parsed.trailingChildren);
}

// Every other runtime loads whatever a document names, and this plugin is not
// the place to invent a second rule. A fence naming something that cannot be
// loaded costs that block its highlighting, not the document its render.
async function loadLanguage(
  highlighter: Highlighter,
  language: string | undefined,
): Promise<string | undefined> {
  if (language == null) return undefined;
  try {
    await highlighter.loadLanguage(language);
    return language;
  } catch {
    return undefined;
  }
}

async function renderBlock(
  highlighter: Highlighter,
  parsed: ParsedCodeBlock,
  formatter: (language: string | undefined) => Formatter,
): Promise<RootContent[]> {
  const resolved = await loadLanguage(highlighter, parsed.language);

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

async function renderInline(
  highlighter: Highlighter,
  parsed: ParsedInlineCode,
  formatter: (language: string | undefined) => Formatter,
): Promise<RootContent[] | undefined> {
  const resolved = await loadLanguage(highlighter, parsed.language);
  const inline = toInline(formatter(resolved), propertiesToAttrs(parsed.codeProperties));
  if (!inline) return undefined;

  const spans = highlighter.highlight(parsed.code, inline.formatter);
  return parseFragment(`${openTag("code", inline.attrs)}${spans}</code>`);
}

type Render = () => Promise<RootContent[] | undefined>;

function planRender(
  node: Element,
  parent: Element | Root,
  highlighter: Highlighter,
  options: RehypeLumisOptions,
): Render | undefined {
  if (node.tagName === "pre") {
    const parsed = parseCodeBlock(node, options.defaultLanguage ?? "plaintext");
    return parsed && (() => renderBlock(highlighter, parsed, options.formatter));
  }

  // A `<code>` inside a `<pre>` is a block, highlighted or declined above.
  const inParent = parent.type === "element" && parent.tagName === "pre";
  if (options.inlineCode !== true || node.tagName !== "code" || inParent) return undefined;

  const parsed = parseInlineCode(node);
  return parsed && (() => renderInline(highlighter, parsed, options.formatter));
}

const rehypeLumis: Plugin<[RehypeLumisOptions], Root> = function rehypeLumis(options) {
  const setup = createHighlighter({
    languages: options.languages ?? [],
  });

  return async function transform(tree) {
    const highlighter = await setup;
    const targets: Array<{ parent: Element | Root; index: number; render: Render }> = [];

    visit(tree, "element", (node, index, parent) => {
      if (!parent || index == null) {
        // oxlint-disable-next-line unicorn/no-useless-undefined -- explicit no-op result keeps visitor returns consistent.
        return undefined;
      }

      const render = planRender(node, parent, highlighter, options);
      if (!render) {
        // oxlint-disable-next-line unicorn/no-useless-undefined -- explicit no-op result keeps visitor returns consistent.
        return undefined;
      }

      targets.push({ parent, index, render });
      return "skip";
    });

    const replacements = await Promise.all(
      targets.map(async ({ render }) => {
        try {
          return await render();
        } catch {
          // oxlint-disable-next-line unicorn/no-useless-undefined -- failed blocks are intentionally skipped.
          return undefined;
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
