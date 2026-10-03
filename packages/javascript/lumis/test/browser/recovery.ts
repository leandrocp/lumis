import type { Language } from "../../src/types.js";
import { exerciseParserRecovery } from "../parser-recovery.js";

const { createHighlighter } = await (new URLSearchParams(location.search).has("bundle")
  ? import("../../dist/index.browser.js")
  : import("../../src/index.browser.ts"));

export type RecoveryInput = Omit<Language, "wasm"> & { wasm: number[] };
declare global {
  interface Window {
    runParserRecovery(inputs: RecoveryInput[]): ReturnType<typeof exerciseParserRecovery>;
  }
}
window.runParserRecovery = (inputs) =>
  exerciseParserRecovery(
    createHighlighter,
    inputs.map((input) => ({ ...input, wasm: new Uint8Array(input.wasm) })),
  );
