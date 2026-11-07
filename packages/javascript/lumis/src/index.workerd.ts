/** Syntax highlighting with Tree-sitter and Neovim themes, on hosts that only run precompiled WebAssembly. */

import treeSitterRuntime from "./web-tree-sitter.wasm";
import { setRuntimeModule } from "./runtime/browser.js";

setRuntimeModule(treeSitterRuntime);

export * from "./index.browser.js";
