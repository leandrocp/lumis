import type { createBinding } from "../generated/web-tree-sitter.js";
import treeSitterWasmBinary from "../tree-sitter-wasm.js";

export type TreeSitterBinding = ReturnType<typeof createBinding>;
let prepared: Promise<() => Promise<TreeSitterBinding>> | undefined;

/** Compile once asynchronously; replacement engines only instantiate. */
export function prepareTreeSitter(): Promise<() => Promise<TreeSitterBinding>> {
  prepared ??= Promise.all([
    import("../generated/web-tree-sitter.js"),
    WebAssembly.compile(treeSitterWasmBinary),
  ]).then(([{ createBinding }, wasmModule]) => async () => {
    const binding = createBinding();
    await binding.Parser.init({ wasmModule });
    return binding;
  });
  return prepared;
}
