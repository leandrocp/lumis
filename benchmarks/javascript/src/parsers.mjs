import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

// A parser package's grammar, which the benchmarks pair with Lumis's own
// definition through `withWasm`. The package's default export is its language,
// so the bytes come from the `.wasm` it ships beside it.
export async function parserBytes(id) {
  const url = import.meta.resolve(`@lumis-sh/wasm-${id}/tree-sitter-${id}.wasm`);
  return new Uint8Array(await readFile(fileURLToPath(url)));
}
