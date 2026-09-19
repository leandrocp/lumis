import type MarkdownIt from "markdown-it";
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
import { withAttrs } from "@lumis-sh/lumis/formatters";

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

/**
 * A fence's own attributes, as `markdown-it-attrs` and friends leave them on
 * the token.
 *
 * Those plugins set `token.attrs` from a core rule, so they are here whether or
 * not this plugin owns the `fence` renderer. Their own renderer, the one that
 * would place them, stands down as soon as it sees a custom `fence` rule, which
 * is why nothing else has put them anywhere by the time this runs.
 */
function fenceAttrs(attrs: Array<[string, string]> | null): HtmlAttrs | undefined {
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

export function fromHighlighter(highlighter: Highlighter, options: MarkdownItLumisOptions) {
  return function installMarkdownItLumis(md: MarkdownIt): void {
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
