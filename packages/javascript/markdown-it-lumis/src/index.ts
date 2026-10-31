import type { MarkdownIt, Token } from "markdown-it";
import type {
  Highlighter,
  Language,
  LanguageDefinition,
  LanguageInput,
  LanguageRef,
  LazyLanguage,
} from "@lumis-sh/lumis";
import type { Formatter, HtmlAttrs } from "@lumis-sh/lumis/formatters";
import { createHighlighter } from "@lumis-sh/lumis";
import { toInline, withAttrs } from "@lumis-sh/lumis/formatters";
import { openTag } from "@lumis-sh/lumis/formatters/html";

export interface MarkdownItLumisOptions {
  formatter: (language: string | undefined) => Formatter;
  languages?: Array<LanguageInput | LanguageRef>;
  /**
   * Put a fence's attributes on the `<pre>` tag rather than the `<code>` tag.
   * Defaults to `true`.
   *
   * `markdown-it-attrs` and `@mdit/plugin-attrs` both default to `<pre>` under
   * this same name, so a document written against either keeps rendering the
   * way it did. markdown-it's own fence renderer puts them on `<code>`; pass
   * `false` for that.
   */
  fenceAttrsOnPre?: boolean;
  /**
   * Also highlight inline code that names its language, with a `{:lang}`
   * suffix as in `` `const x = 1{:javascript}` ``, or with a `language-*`
   * class or `data-language` attribute that a plugin such as
   * `markdown-it-attrs` put on it. Defaults to `false`.
   */
  inlineCode?: boolean;
}

type FenceRenderer = NonNullable<MarkdownIt["renderer"]["rules"]["fence"]>;
type InlineCodeRenderer = NonNullable<MarkdownIt["renderer"]["rules"]["code_inline"]>;

const LANGUAGE_PREFIX = "language-";

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

function attrsLanguage(classes: string[], attrs: HtmlAttrs | undefined): string | undefined {
  const fromClass = classes
    .find((name) => name.startsWith(LANGUAGE_PREFIX) && name.length > LANGUAGE_PREFIX.length)
    ?.slice(LANGUAGE_PREFIX.length);
  const dataLanguage = attrs?.["data-language"];

  return fromClass ?? (typeof dataLanguage === "string" && dataLanguage ? dataLanguage : undefined);
}

function parseInlineCode(
  content: string,
  attrs: HtmlAttrs | undefined,
): { code: string; language: string } | undefined {
  const classes = typeof attrs?.class === "string" ? attrs.class.split(/\s+/u) : [];
  if (classes.some((name) => name === "nohighlight" || name === "no-highlight")) return undefined;

  const language = attrsLanguage(classes, attrs);
  return language === undefined ? splitLanguageSuffix(content) : { code: content, language };
}

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
  const language = info.trim().split(/\s+/u, 1)[0];
  return language && language.length > 0 ? language : undefined;
}

/**
 * A fence's own attributes, as `markdown-it-attrs` and friends leave them on
 * the token.
 *
 * Those plugins set `token.attrs` from a core rule, so they are here whether or
 * not this plugin owns the `fence` renderer. Their own renderer, the one that
 * would place them, stands down as soon as it sees a custom `fence` rule, which
 * is why nothing else has put them anywhere by the time this runs.
 */
function fenceAttrs(attrs: Token["attrs"]): HtmlAttrs | undefined {
  if (!attrs || attrs.length === 0) return undefined;
  return Object.fromEntries(attrs);
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

function installInlineCode(
  md: MarkdownIt,
  highlighter: Highlighter,
  formatter: MarkdownItLumisOptions["formatter"],
): void {
  const defaultInlineCode = md.renderer.rules.code_inline;

  function renderDefault(...args: Parameters<InlineCodeRenderer>): string {
    if (defaultInlineCode) return defaultInlineCode(...args);
    const [tokens, idx, opts, _env, self] = args;
    return self.renderToken(tokens, idx, opts);
  }

  md.renderer.rules.code_inline = function codeInline(tokens, idx, opts, env, self) {
    const token = tokens[idx];
    const authored = token && fenceAttrs(token.attrs);
    const parsed = token && parseInlineCode(token.content, authored);
    if (!parsed) return renderDefault(tokens, idx, opts, env, self);

    try {
      const inline = toInline(formatter(parsed.language), authored);
      if (!inline) return renderDefault(tokens, idx, opts, env, self);

      const spans = highlighter.highlight(parsed.code, inline.formatter);
      return `${openTag("code", inline.attrs)}${spans}</code>`;
    } catch {
      return renderDefault(tokens, idx, opts, env, self);
    }
  };
}

export function fromHighlighter(highlighter: Highlighter, options: MarkdownItLumisOptions) {
  return function installMarkdownItLumis(md: MarkdownIt): void {
    if (options.inlineCode === true) installInlineCode(md, highlighter, options.formatter);

    const defaultFence = md.renderer.rules.fence;

    md.renderer.rules.fence = function fence(tokens, idx, opts, env, self) {
      const token = tokens[idx];
      if (!token) {
        return renderDefaultFence(defaultFence, tokens, idx, opts, env, self);
      }

      const language = getLanguageName(token.info);
      const authored = fenceAttrs(token.attrs);

      try {
        const formatter = options.formatter(language);
        const withFenceAttrs =
          authored === undefined
            ? formatter
            : withAttrs(
                formatter,
                options.fenceAttrsOnPre === false
                  ? { codeAttrs: authored }
                  : { preAttrs: authored },
              );

        return highlighter.highlight(token.content, withFenceAttrs);
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
