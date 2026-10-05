/**
 * Check every parser `languages.toml` pins: it loads, parses its language's
 * `samples/` file, and compiles and runs every processed query against that
 * tree. Predicates and directives only run against a real tree, so compiling
 * alone is not enough. The captures are not inspected; conformance fixtures
 * cover output.
 *
 * The parser is always one built from the pinned revision, never a published
 * package, so a parser bump is checked before it is released and nothing here
 * depends on what npm has. A selected language without a built parser fails:
 * this test must never report success while checking nothing. It once
 * `return`ed early instead, which silently skipped 77 of 115 languages and let
 * an invalid regex reach the branch (see REVIEW.md §1 and §2).
 *
 * `mise run test-queries` builds every parser and checks them all.
 * `mise run wasm-check <parser>` checks one, and a release runs it before
 * publishing.
 *
 * Not part of `pnpm test`. Every grammar this file loads stays compiled in V8
 * for the life of the process -- web-tree-sitter gives no way to free a
 * Language -- so running all 115 in one process exhausts a CI runner's memory.
 * `.github/workflows/queries.yml` shards the parser builds twelve ways, then
 * starts a fresh test process for each batch of four selected languages.
 */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseToml } from "smol-toml";
import { beforeAll, describe, expect, it } from "vitest";
import { Language as TSLanguage, Parser, Query } from "web-tree-sitter";

const QUERY_KINDS = ["highlights", "injections", "locals"] as const;

const workspaceRoot = fileURLToPath(new URL("../../../../", import.meta.url));

interface ParserEntry {
  wasm_name?: string;
  query_name?: string;
  aliases?: string[];
}

const languagesToml = parseToml(readFileSync(join(workspaceRoot, "languages.toml"), "utf8")) as {
  parsers?: Record<string, ParserEntry>;
};
const parsers = Object.entries(languagesToml.parsers ?? {});

function wasmName(id: string, entry: ParserEntry): string {
  return entry.wasm_name ?? `tree-sitter-${id}`;
}

/**
 * The parser built from the revision `languages.toml` pins: the output of
 * `mise run wasm-build`, or the copy committed under `fixtures/parsers/` for a
 * grammar CI cannot build. A build wins, so a grammar that builds anywhere is
 * checked as built rather than as whatever was committed months ago.
 */
function parserPath(id: string, entry: ParserEntry): string | undefined {
  const parser = wasmName(id, entry);
  return [
    join(workspaceRoot, "tmp", "wasm", "build", `${parser}.wasm`),
    join(workspaceRoot, "fixtures", "parsers", `${parser}.wasm`),
  ].find((path) => existsSync(path));
}

function queryPath(entry: ParserEntry, id: string, kind: string): string {
  return join(workspaceRoot, "queries", "processed", entry.query_name ?? id, `${kind}.scm`);
}

const samplesDir = join(workspaceRoot, "samples");
const sampleFiles = readdirSync(samplesDir).filter(
  (name) => name !== "README.md" && name !== "LICENSE.md",
);

/**
 * The website keys samples by the filename before the first dot. A few predate
 * their language id, so an alias and the id without underscores are tried too.
 */
function samplePath(id: string, aliases: string[]): string | undefined {
  const stems = [id, ...aliases, id.replaceAll("_", "")];
  for (const stem of stems) {
    const file = sampleFiles.find((name) => name.slice(0, name.indexOf(".")) === stem);
    if (file) return join(samplesDir, file);
  }
  return undefined;
}

/** Restrict the run to some languages, so CI can shard the parser builds. */
const only = process.env.LUMIS_QUERY_LANGUAGES?.split(",")
  .map((value) => value.trim())
  .filter(Boolean);
const selected = only?.length ? parsers.filter(([id]) => only.includes(id)) : parsers;
const parserIds = new Set(parsers.map(([id]) => id));
const unknownSelections = only?.filter((id) => !parserIds.has(id)) ?? [];
const batchLimit = Number(process.env.LUMIS_QUERY_BATCH_LIMIT);

beforeAll(async () => {
  await Parser.init();
});

describe("processed queries compile against their pinned grammar", () => {
  it("has a parser catalog to check", () => {
    expect(parsers.length).toBeGreaterThan(100);
  });

  it("recognizes every requested language", () => {
    expect(unknownSelections).toEqual([]);
  });

  it("keeps the selected-language batch within its configured limit", () => {
    const limitIsValid =
      process.env.LUMIS_QUERY_BATCH_LIMIT === undefined ||
      (Number.isSafeInteger(batchLimit) && batchLimit > 0 && selected.length <= batchLimit);
    expect(limitIsValid).toBe(true);
  });

  // One test per language, not two, because a grammar is loaded per test and
  // web-tree-sitter gives no way to free one. Splitting these compiled every
  // shard's parsers twice, which exhausted V8's zone memory on a runner.
  it.each(selected)(
    "compiles and runs every query for %s",
    async (id, entry) => {
      const parser = parserPath(id, entry);
      expect(
        parser,
        `no parser built from languages.toml for ${id}; run mise run wasm-build ${wasmName(id, entry)}`,
      ).toBeDefined();
      const grammar = await TSLanguage.load(readFileSync(parser!));

      const failures: string[] = [];
      const compiled = new Map<string, Query>();
      for (const kind of QUERY_KINDS) {
        const path = queryPath(entry, id, kind);
        if (!existsSync(path)) continue;
        try {
          compiled.set(kind, new Query(grammar, readFileSync(path, "utf8")));
        } catch (error) {
          failures.push(`${kind}: ${error instanceof Error ? error.message : String(error)}`);
        }
      }
      expect(failures, `${id} queries failed to compile`).toEqual([]);

      const sample = samplePath(id, entry.aliases ?? []);
      expect(sample, `no samples/ file for ${id}`).toBeDefined();

      const instance = new Parser();
      instance.setLanguage(grammar);
      const tree = instance.parse(readFileSync(sample!, "utf8"));
      expect(tree, `${id} sample did not parse`).not.toBeNull();
      for (const query of compiled.values()) query.captures(tree!.rootNode);

      for (const query of compiled.values()) query.delete();
      tree!.delete();
      instance.delete();
    },
    30_000,
  );
});
