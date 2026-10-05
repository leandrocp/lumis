/**
 * Print the languages and parsers `languages.toml` declares.
 *
 * Usage: node scripts/list-parsers.mjs [--parsers|--pairs]
 *   default    language ids, one per line
 *   --parsers  tree-sitter parser names, deduplicated, for `mise run wasm-build`
 *   --pairs    `<language>\t<parser>`, for callers that must keep the two in step
 *
 * Several languages share one parser, so the language list and the parser list have
 * different lengths. Anything that shards this work must shard `--pairs` and derive
 * both sides from it; sharding the two lists independently puts a language in one
 * shard and its parser in another.
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { parse as parseToml } from "smol-toml";

const workspaceRoot = fileURLToPath(new URL("../../../../", import.meta.url));

const { parsers = {} } = parseToml(readFileSync(join(workspaceRoot, "languages.toml"), "utf8"));

const wantParsers = process.argv.includes("--parsers");
const wantPairs = process.argv.includes("--pairs");
const output = new Set();

for (const [id, entry] of Object.entries(parsers)) {
  const parser = entry.wasm_name ?? `tree-sitter-${id}`;
  if (wantPairs) output.add(`${id}\t${parser}`);
  else output.add(wantParsers ? parser : id);
}

for (const value of [...output].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0))) console.log(value);
