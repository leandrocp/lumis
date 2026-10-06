/**
 * Corpus checks over every generated query predicate, compiled with `RegExp`.
 *
 * `web-tree-sitter` builds a `RegExp` eagerly when it parses a `#match?`
 * predicate, so an invalid or engine-specific pattern makes `new Query(...)`
 * throw and takes the whole language down. This runs without a parser, so it
 * covers every language rather than only the ones whose WASM happens to be
 * installed at the matching revision.
 *
 * The Rust half of the same guarantee lives in
 * `crates/lumis-build/tests/processed_queries.rs`.
 */
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const PROCESSED_QUERIES_DIR = fileURLToPath(
  new URL("../../../../queries/processed/", import.meta.url),
);
const OPERATORS = ["#match?", "#not-match?", "#any-match?", "#any-not-match?"];

interface Predicate {
  file: string;
  line: number;
  operator: string;
  regex: string;
}

function scmFiles(directory: string): string[] {
  const files: string[] = [];
  for (const entry of readdirSync(directory)) {
    const path = join(directory, entry);
    if (statSync(path).isDirectory()) {
      files.push(...scmFiles(path));
    } else if (entry.endsWith(".scm")) {
      files.push(path);
    }
  }
  return files.sort();
}

/**
 * Resolve the escapes Tree-sitter applies when it parses a query string, so the
 * pattern is compiled exactly as `web-tree-sitter` receives it.
 */
function unescapeQueryString(raw: string): string {
  let result = "";
  for (let index = 0; index < raw.length; index += 1) {
    if (raw[index] !== "\\") {
      result += raw[index];
      continue;
    }
    index += 1;
    const escaped = raw[index];
    if (escaped === "\\" || escaped === '"') result += escaped;
    else if (escaped === "n") result += "\n";
    else if (escaped === "r") result += "\r";
    else if (escaped === "t") result += "\t";
    else if (escaped === undefined) result += "\\";
    else result += `\\${escaped}`;
  }
  return result;
}

function nextQueryString(text: string): { raw: string; rest: string } | undefined {
  const start = text.indexOf('"');
  if (start === -1) return undefined;
  for (let index = start + 1; index < text.length; index += 1) {
    if (text[index] === "\\") index += 1;
    else if (text[index] === '"') {
      return { raw: text.slice(start + 1, index), rest: text.slice(index + 1) };
    }
  }
  return undefined;
}

function regexPredicates(): Predicate[] {
  const predicates: Predicate[] = [];

  for (const file of scmFiles(PROCESSED_QUERIES_DIR)) {
    const lines = readFileSync(file, "utf8").split("\n");
    lines.forEach((line, index) => {
      if (line.trimStart().startsWith(";")) return;

      let rest = line;
      for (;;) {
        const remaining = rest;
        const found = OPERATORS.map((operator) => ({
          operator,
          offset: remaining.indexOf(operator),
        }))
          .filter(({ offset }) => offset !== -1)
          .sort((a, b) => a.offset - b.offset || b.operator.length - a.operator.length)[0];
        if (!found) break;

        rest = rest.slice(found.offset + found.operator.length);
        const argument = nextQueryString(rest);
        if (!argument) break;
        rest = argument.rest;

        predicates.push({
          file: file.slice(PROCESSED_QUERIES_DIR.length),
          line: index + 1,
          operator: found.operator,
          regex: unescapeQueryString(argument.raw),
        });
      }
    });
  }

  return predicates;
}

const predicates = regexPredicates();

describe("processed query predicates", () => {
  it("covers the whole catalog", () => {
    expect(predicates.length).toBeGreaterThan(200);
  });

  it("compiles every predicate regex with RegExp", () => {
    const failures = predicates.flatMap(({ file, line, operator, regex }) => {
      try {
        // oxlint-disable-next-line eslint/require-unicode-regexp, no-new -- mirror runtime semantics without flags; construction tests validity.
        new RegExp(regex);
        return [];
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        return [`${file}:${line} (${operator} ${JSON.stringify(regex)}): ${message}`];
      }
    });

    expect(failures).toEqual([]);
  });

  it("never nests a character class", () => {
    // `[[A-Z]]` is `[A-Z]` for the Rust regex crate but `[` or `A-Z` followed by a
    // literal `]` for RegExp, so the two runtimes would silently disagree.
    const failures = predicates
      .filter(({ regex }) => hasNestedCharacterClass(regex))
      .map(({ file, line, regex }) => `${file}:${line} ${JSON.stringify(regex)}`);

    expect(failures).toEqual([]);
  });

  it("never uses an inline flag group", () => {
    // The Rust regex crate honours `(?i)`; RegExp does not.
    const failures = predicates
      .filter(({ regex }) => regex.includes("(?"))
      .map(({ file, line, regex }) => `${file}:${line} ${JSON.stringify(regex)}`);

    expect(failures).toEqual([]);
  });

  it("leaves no Lua predicate in the generated queries", () => {
    const failures = scmFiles(PROCESSED_QUERIES_DIR)
      .filter((file) => readFileSync(file, "utf8").includes("lua-match?"))
      .map((file) => file.slice(PROCESSED_QUERIES_DIR.length));

    expect(failures).toEqual([]);
  });
});

describe("documented defects stay fixed", () => {
  it("compiles the Clojure threading-macro predicate", () => {
    // REVIEW.md §1.1: this was `^*?\>[\^>].*`, which RegExp rejects with
    // "Nothing to repeat", so the whole Clojure query failed to compile.
    const clojure = predicates.find(
      ({ file, regex }) => file.startsWith("clojure/") && regex.includes("->"),
    );
    expect(clojure).toBeDefined();
    expect(clojure?.regex).toBe("^->[^>][\\s\\S]*");
    // oxlint-disable-next-line eslint/require-unicode-regexp -- match runtime RegExp semantics without flags.
    expect(new RegExp(clojure!.regex).test("->foo")).toBe(true);
  });

  it("keeps the documentation-comment predicate negated", () => {
    // REVIEW.md §1.2: `[^*]` had become `[\^*]`, inverting the predicate.
    const documentation = predicates.find(
      ({ file, regex }) => file === "javascript/highlights.scm" && regex.includes("[*][*]"),
    );
    expect(documentation?.regex).toBe("^/[*][*][^*][\\s\\S]*[*]/$");
    // oxlint-disable-next-line eslint/require-unicode-regexp -- match runtime RegExp semantics without flags.
    const compiled = new RegExp(documentation!.regex);
    expect(compiled.test("/** hi */")).toBe(true);
    expect(compiled.test("/*** hi */")).toBe(false);
    // REVIEW.md §1.4: Lua's `.` crosses newlines, regex `.` does not.
    expect(compiled.test("/**\n * hi\n */")).toBe(true);
  });

  it("matches uppercase types the same way Rust does", () => {
    // REVIEW.md §1.3: `^[[A-Z]]` matched in Rust and never in JavaScript.
    const cpp = predicates.find(
      ({ file, regex }) => file === "cpp/highlights.scm" && regex === "^[A-Z]",
    );
    expect(cpp).toBeDefined();
    // oxlint-disable-next-line eslint/require-unicode-regexp -- match runtime RegExp semantics without flags.
    expect(new RegExp(cpp!.regex).test("Foo")).toBe(true);
  });

  it("treats a mid-pattern dollar sign as a literal", () => {
    // REVIEW.md §1.4: `^$env:` anchored at end and could never match.
    const powershell = predicates.find(
      ({ file, regex }) => file === "powershell/highlights.scm" && regex.includes("env:"),
    );
    expect(powershell?.regex).toBe("^\\$env:");
    // oxlint-disable-next-line eslint/require-unicode-regexp -- match runtime RegExp semantics without flags.
    expect(new RegExp(powershell!.regex).test("$env:PATH")).toBe(true);
  });
});

/** Detect an unescaped `[` inside a character class. */
function hasNestedCharacterClass(regex: string): boolean {
  let depth = 0;
  for (let index = 0; index < regex.length; index += 1) {
    const character = regex[index];

    if (character === "\\") {
      index += 1;
    } else if (character === "[") {
      if (depth === 1) return true;
      depth = 1;
      index = skipClassPrefix(regex, index);
    } else if (character === "]" && depth === 1) {
      depth = 0;
    }
  }
  return false;
}

// `^` right after `[` negates the class, and a `]` right after that is a
// literal; neither opens or closes anything.
function skipClassPrefix(regex: string, index: number): number {
  let next = index;
  if (regex[next + 1] === "^") next += 1;
  if (regex[next + 1] === "]") next += 1;
  return next;
}
