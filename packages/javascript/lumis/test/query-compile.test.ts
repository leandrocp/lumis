/**
 * Compile every processed query against its real grammar.
 *
 * This test must never report success while checking nothing. It previously
 * `return`ed from the test body whenever a parser was missing or at the wrong
 * revision, which silently skipped 77 of 115 languages and let an invalid regex
 * reach the branch (see REVIEW.md §1 and §2).
 *
 * Coverage is now enforced two ways:
 *
 * - every language that has a usable parser has its queries compiled, and then
 *   run over that language's `samples/` file: the parser must load, the sample
 *   must parse, and the highlights query must execute, since predicates and
 *   directives only run against a real tree. The captures themselves are not
 *   inspected; conformance fixtures cover output;
 * - every language that has no usable parser must be listed in
 *   `unverified-parsers.json`, and that list can only shrink.
 *
 * Parsers resolve in this order, so the check prefers the artifact that ships but
 * is never blocked by the release cycle:
 *
 * 1. the installed `@lumis-sh/wasm-*` package, when its recorded parser revision
 *    matches `languages.toml`
 * 2. `$LUMIS_DATA_DIR/parsers/<name>.wasm`
 * 3. `tmp/wasm/build/<name>.wasm`, the output of `mise run wasm-build`
 * 4. `fixtures/parsers/<name>.wasm`, committed for grammars CI cannot build
 *
 * `mise run test-queries` builds every parser, then requires complete coverage.
 * A parser release runs this against only the parser it just built, with
 * `LUMIS_QUERY_PARSERS=built`, before anything is published.
 *
 * Not part of `pnpm test`. Every grammar this file loads stays compiled in V8
 * for the life of the process -- web-tree-sitter gives no way to free a
 * Language -- so running all 115 in one process exhausts a CI runner's memory.
 * `.github/workflows/queries.yml` shards the parser work twelve ways against
 * parsers built from languages.toml and four against what npm publishes, then
 * starts a fresh test process for each batch of four selected languages.
 */
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseToml } from "smol-toml";
import { beforeAll, describe, expect, it } from "vitest";
import { Language as TSLanguage, Parser, Query } from "web-tree-sitter";

const QUERY_KINDS = ["highlights", "injections", "locals"] as const;

const workspaceRoot = fileURLToPath(new URL("../../../../", import.meta.url));
const bundleRequire = createRequire(
  createRequire(import.meta.url).resolve("@lumis-sh/wasm-bundle-full"),
);

interface ParserEntry {
  rev?: string;
  wasm_name?: string;
  query_name?: string;
  aliases?: string[];
}

const languagesToml = parseToml(readFileSync(join(workspaceRoot, "languages.toml"), "utf8")) as {
  parsers?: Record<string, ParserEntry>;
};
const parsers = Object.entries(languagesToml.parsers ?? {});

const unverified = JSON.parse(
  readFileSync(new URL("./unverified-parsers.json", import.meta.url), "utf8"),
) as {
  reason: string;
  languages: string[];
};
const waived = new Set(unverified.languages);

function wasmName(id: string, entry: ParserEntry): string {
  return entry.wasm_name ?? `tree-sitter-${id}`;
}

function packageName(parser: string): string {
  return `@lumis-sh/wasm-${parser.replace(/^tree-sitter-/, "")}`;
}

/** The parser revision an installed package was built from. */
function installedRevision(wasmPath: string): string | undefined {
  const packageDirectory = dirname(wasmPath);
  const manifestPath = join(packageDirectory, "lumis.json");
  if (existsSync(manifestPath)) {
    const languagePackage = JSON.parse(readFileSync(manifestPath, "utf8")) as {
      parser?: { revision?: string };
    };
    if (languagePackage.parser?.revision) return languagePackage.parser.revision;
  }

  // Packages published before `lumis.json` kept the revision in package.json.
  const packageJson = JSON.parse(readFileSync(join(packageDirectory, "package.json"), "utf8")) as {
    lumis?: { rev?: string };
  };
  return packageJson.lumis?.rev;
}

/**
 * Whether the *published* package can verify this language.
 *
 * The waiver describes the state of npm, so it must be judged only against the
 * installed package. Building a parser locally makes a language verifiable
 * without making its package published, and conflating the two would force the
 * waiver to be both full and empty depending on whether `tmp/wasm/build` exists.
 */
function publishedParser(
  id: string,
  entry: ParserEntry,
): { path: string } | { unavailable: string } {
  const parser = wasmName(id, entry);

  let installed: string;
  try {
    installed = bundleRequire.resolve(`${packageName(parser)}/${parser}.wasm`);
  } catch {
    return { unavailable: "parser package is not installed" };
  }
  if (!existsSync(installed)) return { unavailable: "installed parser file is missing" };

  const expected = entry.rev;
  if (expected) {
    const actual = installedRevision(installed);
    if (actual !== expected) {
      return {
        unavailable: `installed package is at rev ${
          actual?.slice(0, 8) ?? "unknown"
        }, languages.toml pins ${expected.slice(0, 8)}`,
      };
    }
  }

  return { path: installed };
}

/**
 * Locate any parser built from the revision `languages.toml` pins.
 *
 * The published package wins when it is usable, so the check exercises the
 * artifact that actually ships. A locally built parser is the fallback for a
 * package that is missing or lagging, which keeps query validation from being
 * blocked by the release cycle. It needs no revision check because it was
 * produced from the pinned revision.
 */
function resolveParser(id: string, entry: ParserEntry): { path: string } | { unavailable: string } {
  const parser = wasmName(id, entry);
  // A release asks about the parser it is about to publish, which an installed
  // package at the same revision must not answer for.
  if (parserSource === "built") {
    const built = join(workspaceRoot, "tmp", "wasm", "build", `${parser}.wasm`);
    return existsSync(built) ? { path: built } : { unavailable: "not built in tmp/wasm/build" };
  }

  const fromPackage = publishedParser(id, entry);
  if ("path" in fromPackage) return fromPackage;
  // `compile-published` asks what npm ships can do, so a locally built or
  // committed parser must not answer for it.
  if (parserSource === "published") return fromPackage;

  const sourceDirectory = process.env.LUMIS_DATA_DIR;
  if (sourceDirectory) {
    const prepared = join(sourceDirectory, "parsers", `${parser}.wasm`);
    if (existsSync(prepared)) return { path: prepared };
  }

  const built = join(workspaceRoot, "tmp", "wasm", "build", `${parser}.wasm`);
  if (existsSync(built)) return { path: built };

  // Last, because a grammar that builds anywhere should be checked as built
  // rather than as whatever was committed months ago.
  const committed = join(workspaceRoot, "fixtures", "parsers", `${parser}.wasm`);
  if (existsSync(committed)) return { path: committed };

  return fromPackage;
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

/** Restrict the run to one language, so CI can shard the parser builds. */
const only = process.env.LUMIS_QUERY_LANGUAGES?.split(",")
  .map((value) => value.trim())
  .filter(Boolean);
const selected = only?.length ? parsers.filter(([id]) => only.includes(id)) : parsers;
const parserIds = new Set(parsers.map(([id]) => id));
const unknownSelections = only?.filter((id) => !parserIds.has(id)) ?? [];
const batchLimit = Number(process.env.LUMIS_QUERY_BATCH_LIMIT);

/**
 * Set when every parser was built from `languages.toml` first, which means the
 * run must reach complete coverage and the waiver must not excuse anything.
 */
const requireCompleteCoverage = process.env.LUMIS_QUERY_COVERAGE === "complete";

/**
 * `published` judges only what npm ships, `built` only what `tmp/wasm/build`
 * holds. Unset, the first usable parser in the order above answers.
 */
const parserSource = process.env.LUMIS_QUERY_PARSERS;

const resolved = new Map<string, { path: string } | { unavailable: string }>(
  selected.map(([id, entry]) => [id, resolveParser(id, entry)]),
);
const published = new Map<string, { path: string } | { unavailable: string }>(
  parsers.map(([id, entry]) => [id, publishedParser(id, entry)]),
);
const verifiable = selected.filter(([id]) => "path" in resolved.get(id)!);
const unavailable = selected.filter(([id]) => "unavailable" in resolved.get(id)!);

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

  // A misspelled source would quietly fall back to the installed package, and a
  // release would then vouch for a parser it never loaded.
  it("recognizes the requested parser source", () => {
    expect([undefined, "published", "built"]).toContain(parserSource);
  });

  it("keeps the selected-language batch within its configured limit", () => {
    const limitIsValid =
      process.env.LUMIS_QUERY_BATCH_LIMIT === undefined ||
      (Number.isSafeInteger(batchLimit) && batchLimit > 0 && selected.length <= batchLimit);
    expect(limitIsValid).toBe(true);
  });

  it("reports coverage", () => {
    console.info(
      `query-compile: ${verifiable.length}/${selected.length} languages verified, ` +
        `${unavailable.length} without a usable parser`,
    );
    expect(verifiable.length + unavailable.length).toBe(selected.length);
  });

  it.runIf(requireCompleteCoverage)("verifies every selected language", () => {
    // `mise run test-queries` builds every parser first, so a gap here means a
    // parser build failed rather than a package lagging behind.
    const missing = unavailable.map(
      ([id]) => `${id}: ${(resolved.get(id) as { unavailable: string }).unavailable}`,
    );
    expect(missing, "every parser should have been built from languages.toml").toEqual([]);
  });

  // One test per language, not two, because a grammar is loaded per test and
  // web-tree-sitter gives no way to free one. Splitting these compiled every
  // shard's parsers twice, which exhausted V8's zone memory on a runner.
  it.each(verifiable)(
    "compiles and runs every query for %s",
    async (id, entry) => {
      const parser = resolved.get(id)!;
      expect(parser).toHaveProperty("path");
      const grammar = await TSLanguage.load(readFileSync((parser as { path: string }).path));

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

      // Predicates and directives only run against a tree, so compiling is not
      // enough on its own.
      const sample = samplePath(id, entry.aliases ?? []);
      expect(sample, `no samples/ file for ${id}`).toBeDefined();

      const instance = new Parser();
      instance.setLanguage(grammar);
      const tree = instance.parse(readFileSync(sample!, "utf8"));
      expect(tree, `${id} sample did not parse`).not.toBeNull();
      compiled.get("highlights")?.captures(tree!.rootNode);

      for (const query of compiled.values()) query.delete();
      tree!.delete();
      instance.delete();
    },
    30_000,
  );
});

/**
 * The waiver records which published packages cannot verify their queries. It is
 * judged against npm only, so it stays meaningful whether or not parsers were
 * built locally.
 */
describe("unverified parser waiver", () => {
  const unpublished = parsers.filter(([id]) => "unavailable" in published.get(id)!);

  it("lists every language whose published package cannot verify it", () => {
    // A new gap must be declared. Otherwise a parser bump silently drops a
    // language out of coverage, which is how the §1 defects escaped review.
    const undeclared = unpublished
      .filter(([id]) => !waived.has(id))
      .map(([id]) => `${id}: ${(published.get(id) as { unavailable: string }).unavailable}`);

    expect(
      undeclared,
      "add these to test/unverified-parsers.json, or publish the parser packages",
    ).toEqual([]);
  });

  it("has no stale entries", () => {
    // The list can only shrink. Once a package publishes at the pinned revision,
    // its waiver must go.
    const stale = parsers
      .filter(([id]) => waived.has(id) && "path" in published.get(id)!)
      .map(([id]) => id);

    expect(stale, "these packages are published at the pinned rev, remove them").toEqual([]);
  });

  it("names only real languages", () => {
    const known = new Set(parsers.map(([id]) => id));
    expect(unverified.languages.filter((id) => !known.has(id))).toEqual([]);
  });
});
