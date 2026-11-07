/** Syntax highlighting with Tree-sitter and Neovim themes, on Vercel's Edge runtime. */

import treeSitterRuntime from "./web-tree-sitter.wasm?module";
import { setRuntimeModule } from "./runtime/browser.js";

setRuntimeModule(treeSitterRuntime);

export * from "./index.browser.js";
