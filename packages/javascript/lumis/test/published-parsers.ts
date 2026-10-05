/**
 * The parsers `languages.toml` pins, and whether the installed `@lumis-sh/wasm-*`
 * package for each was built from that pin. Shared by `query-compile.test.ts`,
 * which prefers a published parser when it is usable, and
 * `unverified-parsers.test.ts`, which holds the waiver to what npm publishes.
 */
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseToml } from "smol-toml";

export const workspaceRoot = fileURLToPath(new URL("../../../../", import.meta.url));
const bundleRequire = createRequire(
  createRequire(import.meta.url).resolve("@lumis-sh/wasm-bundle-full"),
);

export interface ParserEntry {
  rev?: string;
  wasm_name?: string;
  query_name?: string;
  aliases?: string[];
}

const languagesToml = parseToml(readFileSync(join(workspaceRoot, "languages.toml"), "utf8")) as {
  parsers?: Record<string, ParserEntry>;
};
export const parsers = Object.entries(languagesToml.parsers ?? {});

export function wasmName(id: string, entry: ParserEntry): string {
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
export function publishedParser(
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
