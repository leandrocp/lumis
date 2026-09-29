/**
 * The public contract that must not depend on which runtime Node picked.
 *
 * Node highlights through the Wasmtime addon and falls back to `web-tree-sitter`,
 * and the two resolve parsers completely differently — Rust against its own store,
 * JavaScript against a resolver you can configure. A caller should not be able to
 * tell. `mise run test-javascript` runs this file under both.
 */
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { fileURLToPath, pathToFileURL } from "node:url";
import { join } from "node:path";
import { beforeAll, beforeEach, describe, expect, it, vi } from "vitest";

import type { Language } from "../src/types.js";
import type { LanguagePackage } from "../src/core/languages.js";
import {
  ensureLocalParserWasm,
  ensureLocalParserWasmDataUrl,
  ensureLocalWasm,
  localLanguagePackageMetadata,
  localLanguagePackageResolver,
} from "./wasm.js";

const parityDirectory = mkdtempSync(join(tmpdir(), "lumis-parity-"));
process.env.LUMIS_DATA_DIR = parityDirectory;

let index: typeof import("../src/index.js");
let htmlLinked: typeof import("../src/formatters.js").htmlLinked;

beforeAll(async () => {
  index = await import("../src/index.js");
  ({ htmlLinked } = await import("../src/formatters.js"));
});

beforeEach(() => {
  index.configureLanguagePackageResolver(localLanguagePackageResolver);
});

function packageDataUrl(
  packageName: string,
  highlights: string,
  aliases: string[] = [],
  options: {
    basePackageName?: string;
    grammarName?: string;
    languageId?: string;
    parserBytes?: Uint8Array;
  } = {},
): string {
  const source = localLanguagePackageMetadata(options.basePackageName ?? "@lumis-sh/wasm-json");
  const languageId = options.languageId ?? "dup";
  const metadata: LanguagePackage = {
    ...source,
    packageName,
    parser: {
      ...source.parser,
      grammarName: options.grammarName ?? source.parser.grammarName,
      ...(options.parserBytes
        ? {
            sha256: createHash("sha256").update(options.parserBytes).digest("hex"),
          }
        : {}),
    },
    languages: {
      [languageId]: {
        aliases,
        highlights,
      },
    },
  };
  return `data:application/json;base64,${Buffer.from(JSON.stringify(metadata)).toString("base64")}`;
}

async function rejectedError(promise: Promise<unknown>): Promise<Error> {
  let rejected: unknown;
  try {
    await promise;
  } catch (error) {
    rejected = error;
  }
  expect(rejected).toBeInstanceOf(Error);
  return rejected as Error;
}

function encodeBase64Octet(dataUrl: string): string {
  const comma = dataUrl.indexOf(",");
  const body = dataUrl.slice(comma + 1);
  const octetIndex = body.search(/[+/=]/);
  if (comma < 0 || octetIndex < 0) throw new Error("test data URL has no encodable base64 octet");
  const octet = body[octetIndex];
  return `${dataUrl.slice(0, comma + 1)}${body.slice(0, octetIndex)}%${octet.charCodeAt(0).toString(16)}${body.slice(octetIndex + 1)}`;
}

function injectedLanguages(markdown: Language, json: Language): Language[] {
  return index.runtimeKind() === "native" ? [markdown] : [markdown, json];
}

/// Point the store at an empty directory rather than unsetting it: unset falls
/// back to the platform data directory, which on a developer machine already
/// holds parsers and would quietly stop testing the resolver path.
async function withoutStagedParsers<T>(run: () => Promise<T>): Promise<T> {
  const previous = process.env.LUMIS_DATA_DIR;
  process.env.LUMIS_DATA_DIR = mkdtempSync(join(tmpdir(), "lumis-empty-store-"));
  try {
    return await run();
  } finally {
    if (previous === undefined) {
      delete process.env.LUMIS_DATA_DIR;
    } else {
      process.env.LUMIS_DATA_DIR = previous;
    }
  }
}

describe("runtime parity", () => {
  it("runs under the runtime the environment asked for", () => {
    const requested = process.env.LUMIS_TEST_RUNTIME;
    if (requested === "native" || requested === "wasm") {
      expect(index.runtimeKind()).toBe(requested);
    } else {
      expect(["native", "wasm"]).toContain(index.runtimeKind());
    }
  });

  it("normalizes only ASCII case in language ids", async () => {
    const upper: Language = { id: "Ä", aliases: [], packageName: "@test/upper" };
    const lower: Language = { id: "ä", aliases: [], packageName: "@test/lower" };
    const hl = await index.createHighlighter({
      languages: [upper, lower],
      languagePackageResolver: (packageName) =>
        packageDataUrl(
          packageName,
          packageName === "@test/upper" ? "(string) @string" : "(number) @number",
          [],
          { languageId: packageName === "@test/upper" ? "Ä" : "ä" },
        ),
      wasmResolver: () => ensureLocalWasm("json"),
    });

    expect(hl.languages).toEqual(expect.arrayContaining(["Ä", "ä"]));
    const upperHtml = hl.highlight('{"answer": 42}', htmlLinked({ language: upper }));
    const lowerHtml = hl.highlight('{"answer": 42}', htmlLinked({ language: lower }));
    expect(upperHtml).toContain('class="l-string"');
    expect(upperHtml).not.toContain('class="l-number"');
    expect(lowerHtml).toContain('class="l-number"');
    expect(lowerHtml).not.toContain('class="l-string"');
  }, 30_000);

  /**
   * Queries were removed from the public surface: a parser and the queries
   * written against it are released together in a package. TypeScript stops
   * this at compile time, but plain JavaScript callers get no types, so the
   * boundary has to reject it rather than drop the field silently.
   */
  it.each([
    ["queries with no package", { id: "custom", aliases: [], highlights: "(string) @string" }],
    [
      "queries beside a package",
      {
        id: "json",
        aliases: [],
        packageName: "@lumis-sh/wasm-json",
        highlights: "(string) @string",
      },
    ],
  ])(
    "rejects %s",
    async (_name, language) => {
      await expect(
        index.createHighlighter({ languages: [language as unknown as Language] }),
      ).rejects.toThrow(/incomplete or conflicting load definition/);
    },
    30_000,
  );

  it("keeps package-backed definitions with the same public id isolated", async () => {
    const strings: Language = { id: "dup", aliases: [], packageName: "@test/strings" };
    const numbers: Language = { id: "dup", aliases: [], packageName: "@test/numbers" };
    const create = (language: Language, query: string) =>
      index.createHighlighter({
        languages: [language],
        languagePackageResolver: (packageName) => packageDataUrl(packageName, query),
        wasmResolver: () => ensureLocalWasm("json"),
      });

    const first = await create(strings, "(string) @string");
    const second = await create(numbers, "(number) @number");
    const firstHtml = first.highlight('{"a": 1}', htmlLinked({ language: strings }));
    const secondHtml = second.highlight('{"a": 1}', htmlLinked({ language: numbers }));

    expect(firstHtml).toContain('class="l-string"');
    expect(firstHtml).not.toContain('class="l-number"');
    expect(secondHtml).toContain('class="l-number"');
    expect(secondHtml).not.toContain('class="l-string"');
  }, 30_000);

  it("rejects package grammar metadata before loading and accepts corrected metadata", async () => {
    const basePackageName = "@lumis-sh/wasm-comment";
    const source = localLanguagePackageMetadata(basePackageName);
    const highlights = source.languages.comment.highlights;
    const { Language: TreeSitterLanguage } = await import("web-tree-sitter");
    const load = vi.spyOn(TreeSitterLanguage, "load");
    const moduleExports = vi.spyOn(WebAssembly.Module, "exports");
    const create = (id: string, packageName: string, grammarName: string) =>
      index.createHighlighter({
        languages: [{ id, aliases: [], packageName }],
        languagePackageResolver: () =>
          packageDataUrl(packageName, highlights, [], {
            basePackageName,
            grammarName,
            languageId: id,
          }),
        wasmResolver: (_language, wasm) => ensureLocalParserWasm("comment", wasm.name),
      });

    try {
      const loadsBefore = load.mock.calls.length;
      const exportsBefore = moduleExports.mock.calls.length;
      const firstError = await rejectedError(
        create("wrong-grammar-first", "@test/wrong-grammar-first", "not_comment"),
      );
      const secondError = await rejectedError(
        create("wrong-grammar-second", "@test/wrong-grammar-second", "not_comment"),
      );

      for (const error of [firstError, secondError]) {
        expect(error.message).toMatch(/parser grammar/);
        expect(error.message).toContain("not_comment");
        expect(error.message).toContain("comment");
      }
      if (index.runtimeKind() === "wasm") {
        expect(load.mock.calls.length - loadsBefore).toBe(0);
        expect(moduleExports.mock.calls.length - exportsBefore).toBe(0);
      }

      const corrected = await create("wrong-grammar-first", "@test/corrected-grammar", "comment");
      expect(corrected.languages).toContain("wrong-grammar-first");
      if (index.runtimeKind() === "wasm") {
        expect(load.mock.calls.length - loadsBefore).toBe(1);
        expect(moduleExports.mock.calls.length - exportsBefore).toBe(0);
      }
    } finally {
      load.mockRestore();
      moduleExports.mockRestore();
    }
  }, 30_000);

  it("does not treat a non-function tree-sitter export as a parser grammar", async () => {
    const parserBytes = new Uint8Array([
      0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x06, 0x06, 0x01, 0x7f, 0x00, 0x41, 0x00,
      0x0b, 0x07, 0x15, 0x01, 0x11, 0x74, 0x72, 0x65, 0x65, 0x5f, 0x73, 0x69, 0x74, 0x74, 0x65,
      0x72, 0x5f, 0x64, 0x65, 0x63, 0x6f, 0x79, 0x03, 0x00,
    ]);
    expect(WebAssembly.Module.exports(new WebAssembly.Module(parserBytes))).toEqual([
      { name: "tree_sitter_decoy", kind: "global" },
    ]);
    const packageName = "@test/non-function-grammar";
    const languageId = "non-function-grammar";
    const { Language: TreeSitterLanguage } = await import("web-tree-sitter");
    const load = vi.spyOn(TreeSitterLanguage, "load");

    try {
      const loadsBefore = load.mock.calls.length;
      const error = await rejectedError(
        index.createHighlighter({
          languages: [{ id: languageId, aliases: [], packageName, wasm: parserBytes }],
          languagePackageResolver: () =>
            packageDataUrl(packageName, "(string) @string", [], {
              grammarName: "decoy",
              languageId,
              parserBytes,
            }),
        }),
      );

      expect(error.message).toMatch(/parser grammar/);
      if (index.runtimeKind() === "wasm") {
        expect(load.mock.calls.length - loadsBefore).toBe(0);
      }
    } finally {
      load.mockRestore();
    }
  }, 30_000);

  it("registers the canonical id and aliases supplied by a language package", async () => {
    const language: Language = {
      id: "package-alias",
      aliases: [],
      packageName: "@test/aliases",
    };
    const hl = await index.createHighlighter({
      languages: [language],
      languagePackageResolver: (packageName) =>
        packageDataUrl(packageName, "(string) @string", ["package-alias"]),
      wasmResolver: () => ensureLocalWasm("json"),
    });

    expect(hl.languages).toContain("dup");
    expect(hl.highlight('{"a": 1}', htmlLinked({ language }))).toContain('class="l-string"');
  }, 30_000);

  it("reads parser bytes a per-highlighter resolver returns", async () => {
    const { default: diff } = await import("../langs/diff.ts");
    const seen: string[] = [];

    const hl = await withoutStagedParsers(() =>
      index.createHighlighter({
        languages: [diff],
        wasmResolver: (language, wasm) => {
          seen.push(language);
          return ensureLocalParserWasm(language, wasm.name);
        },
      }),
    );

    expect(seen).toContain("diff");
    expect(hl.highlight("- old\n+ new", htmlLinked({ language: diff }))).toContain(
      'class="language-diff"',
    );
  }, 30_000);

  it("loads a resolver-backed injected language in the same native walk", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const { default: json } = await import("../langs/json.ts");
    const seen: string[] = [];
    const native = index.runtimeKind() === "native";
    const hl = await withoutStagedParsers(() =>
      index.createHighlighter({
        languages: injectedLanguages(markdown, json),
        wasmResolver: (language, wasm) => {
          seen.push(language);
          return ensureLocalParserWasm(language, wasm.name);
        },
      }),
    );

    expect(seen).toContain("markdown");
    if (native) {
      expect(seen).not.toContain("json");
      expect(hl.languages).not.toContain("json");
    } else {
      expect(hl.languages).toContain("json");
    }

    const html = hl.highlight('```JSON\n{"answer": 42}\n```\n', htmlLinked({ language: markdown }));

    if (native) expect(seen).toContain("json");
    expect(html).toContain('class="l-number"');
  }, 30_000);

  it("rejects reentrant native highlighting from a resolver instead of deadlocking", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const { default: json } = await import("../langs/json.ts");
    const source = '```json\n{"answer": 42}\n```\n';
    let hl: Awaited<ReturnType<typeof index.createHighlighter>> | undefined;
    let reentrantError: unknown;

    hl = await index.createHighlighter({
      languages: injectedLanguages(markdown, json),
      wasmResolver: (language, wasm) => {
        if (language === "json" && hl && reentrantError === undefined) {
          try {
            hl.highlight(source, htmlLinked({ language: markdown }));
          } catch (error) {
            reentrantError = error;
          }
        }
        return ensureLocalParserWasm(language, wasm.name);
      },
    });

    const html = hl.highlight(source, htmlLinked({ language: markdown }));
    if (index.runtimeKind() === "native") {
      expect(String(reentrantError)).toContain(
        "native highlighting cannot be called from a language resolver callback",
      );
    } else {
      expect(reentrantError).toBeUndefined();
    }
    expect(html).toContain('class="l-number"');
  }, 30_000);

  it("reads file resolver URLs without treating their query or fragment as path bytes", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const { default: json } = await import("../langs/json.ts");
    const packagePath = join(parityDirectory, "json.lumis.json");
    writeFileSync(packagePath, JSON.stringify(localLanguagePackageMetadata("@lumis-sh/wasm-json")));
    const packageUrl = pathToFileURL(packagePath);
    packageUrl.search = "?cache=test";
    packageUrl.hash = "#package";
    const parserUrl = new URL(ensureLocalWasm("json"));
    parserUrl.search = "?cache=test";
    parserUrl.hash = "#parser";

    const hl = await index.createHighlighter({
      languages: injectedLanguages(markdown, json),
      languagePackageResolver: (packageName) =>
        packageName === "@lumis-sh/wasm-json"
          ? packageUrl
          : localLanguagePackageResolver(packageName),
      wasmResolver: (language, wasm) =>
        language === "json" ? parserUrl : ensureLocalParserWasm(language, wasm.name),
    });

    expect(
      hl.highlight('```json\n{"answer": 42}\n```\n', htmlLinked({ language: markdown })),
    ).toContain('class="l-number"');
  }, 30_000);

  it("percent-decodes base64 data resolver URLs before decoding them", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const { default: json } = await import("../langs/json.ts");
    const packageUrl = encodeBase64Octet(localLanguagePackageResolver("@lumis-sh/wasm-json"));
    const parserUrl = encodeBase64Octet(ensureLocalParserWasmDataUrl("json", "tree-sitter-json"));

    const hl = await index.createHighlighter({
      languages: injectedLanguages(markdown, json),
      languagePackageResolver: (packageName) =>
        packageName === "@lumis-sh/wasm-json"
          ? packageUrl
          : localLanguagePackageResolver(packageName),
      wasmResolver: (language, wasm) =>
        language === "json" ? parserUrl : ensureLocalParserWasm(language, wasm.name),
    });

    expect(
      hl.highlight('```json\n{"answer": 42}\n```\n', htmlLinked({ language: markdown })),
    ).toContain('class="l-number"');
  }, 30_000);

  it("accepts forgiving-base64 data resolver URLs without padding", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const { default: json } = await import("../langs/json.ts");
    const packageUrl = localLanguagePackageResolver("@lumis-sh/wasm-json").replace(/=+$/, "");
    const parserUrl = ensureLocalParserWasmDataUrl("json", "tree-sitter-json").replace(/=+$/, "");

    const hl = await index.createHighlighter({
      languages: injectedLanguages(markdown, json),
      languagePackageResolver: (packageName) =>
        packageName === "@lumis-sh/wasm-json"
          ? packageUrl
          : localLanguagePackageResolver(packageName),
      wasmResolver: (language, wasm) =>
        language === "json" ? parserUrl : ensureLocalParserWasm(language, wasm.name),
    });

    expect(
      hl.highlight('```json\n{"answer": 42}\n```\n', htmlLinked({ language: markdown })),
    ).toContain('class="l-number"');
  }, 30_000);

  it("treats base64 as a data URL marker only when it is last", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const { default: json } = await import("../langs/json.ts");
    const metadata = JSON.stringify(localLanguagePackageMetadata("@lumis-sh/wasm-json"));
    const packageUrl = `data:application/json;base64;parameter,${encodeURIComponent(metadata)}`;

    const hl = await index.createHighlighter({
      languages: injectedLanguages(markdown, json),
      languagePackageResolver: (packageName) =>
        packageName === "@lumis-sh/wasm-json"
          ? packageUrl
          : localLanguagePackageResolver(packageName),
      wasmResolver: (language, wasm) => ensureLocalParserWasm(language, wasm.name),
    });

    expect(
      hl.highlight('```json\n{"answer": 42}\n```\n', htmlLinked({ language: markdown })),
    ).toContain('class="l-number"');
  }, 30_000);

  it("supports blob resolver URLs when the injected language is preloaded", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const { default: json } = await import("../langs/json.ts");
    const packageUrl = URL.createObjectURL(
      new Blob([JSON.stringify(localLanguagePackageMetadata("@lumis-sh/wasm-json"))]),
    );
    const parserUrl = URL.createObjectURL(
      new Blob([readFileSync(fileURLToPath(ensureLocalWasm("json")))]),
    );

    try {
      const hl = await index.createHighlighter({
        languages: [markdown, json],
        languagePackageResolver: (packageName) =>
          packageName === "@lumis-sh/wasm-json"
            ? packageUrl
            : localLanguagePackageResolver(packageName),
        wasmResolver: (language, wasm) =>
          language === "json" ? parserUrl : ensureLocalParserWasm(language, wasm.name),
      });

      expect(
        hl.highlight('```json\n{"answer": 42}\n```\n', htmlLinked({ language: markdown })),
      ).toContain('class="l-number"');
    } finally {
      URL.revokeObjectURL(packageUrl);
      URL.revokeObjectURL(parserUrl);
    }
  }, 30_000);

  it("rejects explicit parser bytes for another grammar", async () => {
    const { default: json } = await import("../langs/json.ts");

    await expect(
      index.createHighlighter({
        languages: [{ ...json, wasm: ensureLocalWasm("markdown") }],
      }),
    ).rejects.toThrow(/parser grammar/);
  }, 30_000);

  /** An unavailable injection costs one block and is reported to the caller. */
  it("reports an injected language it could not load, rather than failing or hiding it", async () => {
    const { default: markdown } = await import("../langs/markdown.ts");
    const warnings: string[] = [];
    const warn = console.warn;
    console.warn = (message: unknown) => warnings.push(String(message));

    try {
      const hl = await index.createHighlighter({
        languages: [{ ...markdown, wasm: ensureLocalWasm("markdown") }],
        wasmResolver: (language, wasm) => {
          if (language === "erlang") throw new Error("no erlang parser in this test");
          return ensureLocalParserWasm(language, wasm.name);
        },
      });

      const html = hl.highlight(
        "```erlang\nplain\n```\n\n```no-such-language\nplain\n```\n",
        htmlLinked({ language: markdown }),
      );

      // The document still highlights; only the two fences are plain.
      expect(html).toContain('class="language-markdown"');
      expect(warnings.join("\n")).toContain("erlang");
      // "Load it up front" is not advice that can apply to a name which is not
      // a language, and injection queries produce those routinely: html asks
      // for the raw `<script type=...>` value.
      expect(warnings.join("\n")).not.toContain("no-such-language");
    } finally {
      console.warn = warn;
    }
  }, 30_000);

  it("applies a globally configured resolver to later highlighters", async () => {
    const { default: css } = await import("../langs/css.ts");
    const seen: string[] = [];

    index.configureWasmResolver((language, wasm) => {
      seen.push(language);
      return ensureLocalParserWasm(language, wasm.name);
    });

    const hl = await withoutStagedParsers(() => index.createHighlighter({ languages: [css] }));

    expect(seen).toContain("css");
    expect(hl.highlight("a { color: red }", htmlLinked({ language: css }))).toContain(
      'class="language-css"',
    );
  }, 30_000);
});
