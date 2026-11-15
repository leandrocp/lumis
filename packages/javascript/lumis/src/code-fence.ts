/**
 * The options a code fence's meta sets. A field is `undefined` when the meta
 * does not set it, or sets it to an empty or invalid value.
 */
export interface CodeFenceMeta {
  /** `theme=github_dark`: the theme to highlight with. */
  theme?: string;
  /** `pre_class="a b"`: classes to add to the `<pre>` element. */
  preClass?: string;
  /** `highlight_lines="1,3-5"`: 1-based inclusive `[start, end]` line ranges. */
  highlightLines?: Array<[number, number]>;
  /**
   * `highlight_lines_style="background: yellow"`, or `"theme"` for the theme's
   * own highlight color.
   */
  highlightLinesStyle?: string;
  /** `highlight_lines_class=emphasis`: a class for highlighted lines. */
  highlightLinesClass?: string;
  /**
   * `include_highlights`, `include_highlights=true` or
   * `include_highlights=false`: whether to name each token's highlight.
   */
  includeHighlights?: boolean;
  /** Every other word in the meta, in order. */
  unrecognized: MetaAttribute[];
}

/** A word in a code fence's meta that {@link parseCodeFenceMeta} does not recognise. */
export interface MetaAttribute {
  /** The text before the first `=`, or the whole word when it has none. */
  name: string;
  /** The text after the first `=`, which may be empty. */
  value?: string;
}

const ASCII_WHITESPACE = new Set([" ", "\t", "\n", "\f", "\r"]);

/**
 * Read the options in a code fence's meta, the info string after its language.
 *
 * Words are separated by ASCII whitespace and split into a name and a value at
 * their first `=`. Double or single quotes group text with spaces in it and are
 * removed; inside double quotes `\"` and `\\` escape, and any other backslash is
 * kept. A quote with no closing match is ordinary text.
 *
 * When a recognised name appears more than once, the last one wins. A malformed
 * part of `highlight_lines` is dropped on its own, so one typo does not turn the
 * rest off.
 *
 * ```ts
 * import { parseCodeFenceMeta } from '@lumis-sh/lumis'
 *
 * const meta = parseCodeFenceMeta('highlight_lines="1, 3-5" theme=github_dark title="app.ex"')
 * meta.theme          // "github_dark"
 * meta.highlightLines // [[1, 1], [3, 5]]
 * meta.unrecognized   // [{ name: "title", value: "app.ex" }]
 * ```
 */
export function parseCodeFenceMeta(meta: string): CodeFenceMeta {
  const parsed: CodeFenceMeta = { unrecognized: [] };

  for (const word of words(meta)) {
    const equals = word.indexOf("=");
    if (equals === -1) {
      set(parsed, word, undefined);
    } else {
      set(parsed, word.slice(0, equals), word.slice(equals + 1));
    }
  }

  return parsed;
}

type TextField = "theme" | "preClass" | "highlightLinesStyle" | "highlightLinesClass";

const TEXT_FIELDS = new Map<string, TextField>([
  ["theme", "theme"],
  ["pre_class", "preClass"],
  ["highlight_lines_style", "highlightLinesStyle"],
  ["highlight_lines_class", "highlightLinesClass"],
]);

function set(parsed: CodeFenceMeta, name: string, value: string | undefined): void {
  const field = TEXT_FIELDS.get(name);

  if (field !== undefined) {
    parsed[field] = value === "" ? undefined : value;
  } else if (name === "highlight_lines") {
    parsed.highlightLines = value === undefined ? undefined : lenientLineRanges(value);
  } else if (name === "include_highlights") {
    parsed.includeHighlights = flag(value);
  } else {
    parsed.unrecognized.push(value === undefined ? { name } : { name, value });
  }
}

function flag(value: string | undefined): boolean | undefined {
  if (value === undefined || value === "true") return true;
  if (value === "false") return false;
  return undefined;
}

function* words(meta: string): Generator<string> {
  let word = "";
  let index = 0;

  while (index < meta.length) {
    const ch = meta.charAt(index);
    index += 1;

    if (ch === '"' || ch === "'") {
      const closed = quoted(meta, index, ch);
      if (closed === undefined) {
        word += ch;
      } else {
        word += closed.content;
        index = closed.next;
      }
    } else if (ASCII_WHITESPACE.has(ch)) {
      if (word !== "") yield word;
      word = "";
    } else {
      word += ch;
    }
  }

  if (word !== "") yield word;
}

/**
 * The text from `start` up to the `quote` that closes it, and the index after
 * that quote, or `undefined` when nothing closes it.
 */
function quoted(
  meta: string,
  start: number,
  quote: string,
): { content: string; next: number } | undefined {
  let content = "";
  let index = start;

  while (index < meta.length) {
    const ch = meta.charAt(index);
    if (ch === quote) return { content, next: index + 1 };

    const escaped = meta.charAt(index + 1);
    if (quote === '"' && ch === "\\" && (escaped === '"' || escaped === "\\")) {
      content += escaped;
      index += 2;
    } else {
      content += ch;
      index += 1;
    }
  }

  return undefined;
}

function lenientLineRanges(spec: string): Array<[number, number]> | undefined {
  const ranges = spec
    .split(",")
    .map((part) => trimAscii(part))
    .filter((part) => part !== "")
    .map((part) => lineRange(part))
    .filter((range) => range !== undefined);

  return ranges.length > 0 ? ranges : undefined;
}

function lineRange(part: string): [number, number] | undefined {
  const dash = part.indexOf("-");
  const start = lineNumber(dash === -1 ? part : part.slice(0, dash));
  const end = dash === -1 ? start : lineNumber(part.slice(dash + 1));

  if (start === undefined || end === undefined) return undefined;
  if (start === 0 || end === 0 || start > end) return undefined;
  return [start, end];
}

/** A line number as Rust's `usize` parses one: an optional `+`, then ASCII digits. */
function lineNumber(text: string): number | undefined {
  const trimmed = trimAscii(text);
  const digits = trimmed.startsWith("+") ? trimmed.slice(1) : trimmed;
  if (!/^[0-9]+$/u.test(digits)) return undefined;

  const value = Number(digits);
  return Number.isSafeInteger(value) ? value : undefined;
}

function trimAscii(text: string): string {
  let start = 0;
  let end = text.length;
  while (start < end && ASCII_WHITESPACE.has(text.charAt(start))) start += 1;
  while (end > start && ASCII_WHITESPACE.has(text.charAt(end - 1))) end -= 1;
  return text.slice(start, end);
}
