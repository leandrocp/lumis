import type MarkdownIt from "markdown-it";
import type { Element, Properties, RootContent } from "hast";
import type {
  Highlighter,
  Language,
  LanguageDefinition,
  LanguageInput,
  LanguageRef,
  LazyLanguage,
} from "@lumis-sh/lumis";
import type { Formatter } from "@lumis-sh/lumis/formatters";
import { createHighlighter } from "@lumis-sh/lumis";
import { fromHtml } from "hast-util-from-html";
import { toHtml } from "hast-util-to-html";

export interface MarkdownItLumisOptions {
  formatter: (language: string | undefined) => Formatter;
  languages?: Array<LanguageInput | LanguageRef>;
}

type FenceRenderer = NonNullable<MarkdownIt["renderer"]["rules"]["fence"]>;

function renderDefaultFence(
  defaultFence: FenceRenderer | undefined,
  ...args: Parameters<FenceRenderer>
): string {
  if (defaultFence) {
    return defaultFence(...args);
  }

  const [tokens, idx, opts, _env, self] = args;
  return self.renderToken(tokens, idx, opts);
}

function getLanguageName(info: string): string | undefined {
  const language = info.trim().split(/\s+/, 1)[0];
  return language && language.length > 0 ? language : undefined;
}

function getClassNames(value: Properties["className"]): string[] {
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
  return [...new Set([...getClassNames(rendered), ...getClassNames(authored)])];
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

function openingTagEnd(html: string, start: number): number | undefined {
  let quote: '"' | "'" | undefined;

  for (let index = start; index < html.length; index += 1) {
    const character = html[index];
    if (quote) {
      if (character === quote) quote = undefined;
      continue;
    }
    if (character === '"' || character === "'") {
      quote = character;
      continue;
    }
    if (character === ">") return index + 1;
  }

  return undefined;
}

function renderOpeningTag(element: Element): string {
  const html = toHtml({ ...element, children: [] });
  const end = openingTagEnd(html, 0);
  return end == null ? html : html.slice(0, end);
}

function preserveFenceAttributes(html: string, renderedAttributes: string): string {
  if (renderedAttributes.length === 0) return html;

  const authoredFragment = fromHtml(`<code${renderedAttributes}></code>`, { fragment: true });
  const authoredCode = findElement(authoredFragment.children, "code");
  if (!authoredCode) return html;

  const renderedFragment = fromHtml(html, { fragment: true });
  const pre = findElement(renderedFragment.children, "pre");
  const code = pre ? findElement(pre.children, "code") : undefined;
  const start = code?.position?.start.offset;
  if (!code || start == null) return html;

  const end = openingTagEnd(html, start);
  if (end == null) return html;

  mergeProperties(code.properties, authoredCode.properties);
  // Keep the formatter's highlighted content byte-for-byte intact. The parsed
  // positions let this survive wrappers or other structure around `<code>`.
  return `${html.slice(0, start)}${renderOpeningTag(code)}${html.slice(end)}`;
}

function splitLanguages(entries: Array<LanguageInput | LanguageRef>): {
  inputs: LanguageInput[];
  refs: LanguageRef[];
} {
  const inputs: LanguageInput[] = [];
  const refs: LanguageRef[] = [];

  for (const entry of entries) {
    if (typeof entry === "string") {
      refs.push(entry);
      continue;
    }
    if (isLazyLanguage(entry)) {
      inputs.push({ [entry.id]: entry });
      refs.push(entry);
      continue;
    }
    if (isLanguageDefinition(entry)) {
      if (isLanguage(entry)) inputs.push(entry);
      refs.push(entry);
      continue;
    }
    if (hasLanguageMetadata(entry)) {
      throw new TypeError("Invalid markdown-it-lumis language metadata");
    }
    inputs.push(entry);
  }

  return { inputs, refs };
}

function isLanguageDefinition(value: LanguageInput | LanguageRef): value is LanguageDefinition {
  return (
    typeof value === "object" &&
    value !== null &&
    "id" in value &&
    typeof value.id === "string" &&
    value.id.length > 0 &&
    "aliases" in value &&
    Array.isArray(value.aliases) &&
    value.aliases.every((alias) => typeof alias === "string")
  );
}

function isLanguage(value: LanguageInput | LanguageRef): value is Language {
  return (
    isLanguageDefinition(value) &&
    (value.id === "plaintext" || ("packageName" in value && typeof value.packageName === "string"))
  );
}

function isLazyLanguage(value: LanguageInput | LanguageRef): value is LazyLanguage {
  return (
    typeof value === "function" &&
    "id" in value &&
    typeof value.id === "string" &&
    value.id.length > 0 &&
    "aliases" in value &&
    Array.isArray(value.aliases) &&
    value.aliases.every((alias) => typeof alias === "string")
  );
}

function hasLanguageMetadata(value: LanguageInput | LanguageRef): boolean {
  return (
    ((typeof value === "object" && value !== null) || typeof value === "function") &&
    ("id" in value || "aliases" in value)
  );
}

export function fromHighlighter(highlighter: Highlighter, options: MarkdownItLumisOptions) {
  return function installMarkdownItLumis(md: MarkdownIt): void {
    const defaultFence = md.renderer.rules.fence;

    md.renderer.rules.fence = function fence(tokens, idx, opts, env, self) {
      const token = tokens[idx];
      if (!token) {
        return renderDefaultFence(defaultFence, tokens, idx, opts, env, self);
      }

      const language = getLanguageName(token.info);

      try {
        const html = highlighter.highlight(token.content, options.formatter(language));
        return preserveFenceAttributes(html, self.renderAttrs(token));
      } catch {
        return renderDefaultFence(defaultFence, tokens, idx, opts, env, self);
      }
    };
  };
}

export default async function markdownItLumis(options: MarkdownItLumisOptions) {
  const { inputs: languageInputs, refs: languageRefs } = splitLanguages(options.languages ?? []);

  const highlighter = await createHighlighter({
    languages: languageInputs,
  });

  await Promise.all(languageRefs.map((language) => highlighter.loadLanguage(language)));

  return fromHighlighter(highlighter, options);
}
