import type { Language, Parser, Query } from "web-tree-sitter";
import treeSitterWasmBinary from "../tree-sitter-wasm.js";

// Declared here rather than derived from the generated module, which a fresh
// checkout does not have until `build:runtime-wasm` runs. Derived, every type
// below collapsed to `any` and the type-aware lint stopped checking this path.
export interface TreeSitterBinding {
  Parser: Omit<typeof Parser, "init"> & {
    new (): Parser;
    init(options: { wasmModule: WebAssembly.Module }): Promise<void>;
  };
  Language: { load(module: WebAssembly.Module): Promise<Language> };
  Query: typeof Query;
}

let prepared: Promise<() => Promise<TreeSitterBinding>> | undefined;

/** Compile once asynchronously; replacement engines only instantiate. */
export function prepareTreeSitter(): Promise<() => Promise<TreeSitterBinding>> {
  prepared ??= Promise.all([
    import("../generated/web-tree-sitter.js"),
    WebAssembly.compile(treeSitterWasmBinary),
  ]).then(
    ([{ createBinding }, wasmModule]) =>
      async () => {
        const binding = createBinding();
        await binding.Parser.init({ wasmModule });
        return binding;
      },
    (error: unknown) => {
      // A bundled chunk can fail to load while offline; let the next call retry.
      prepared = undefined;
      throw error;
    },
  );
  return prepared;
}
