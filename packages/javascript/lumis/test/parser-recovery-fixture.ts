import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { ensureLocalWasm, localLanguagePackageMetadata } from "./wasm.js";
import type { Language } from "../src/types.js";

export function recoveryLanguages(): Language[] {
  const require = createRequire(import.meta.url);
  const broken = readFileSync(require.resolve("test-wasm-php-trap/tree-sitter-php.wasm"));
  if (
    createHash("sha256").update(broken).digest("hex") !==
    "78400ce8fbdcb88d39318f910127065f1d7e3c98bb38163fcd67a6c2f4cd9596"
  ) {
    throw new Error("Parser trap fixture is not the published PHP 0.26.4 binary");
  }
  return ["php", "json", "markdown", "markdown_inline"].map((id) => {
    // Keep these exact bytes even if the project has a newer parser installed.
    const packageName = `@test/recovery-${id}`;
    const languagePackage = {
      ...localLanguagePackageMetadata(`@lumis-sh/wasm-${id}`),
      packageName,
    };
    const wasm = id === "php" ? broken : readFileSync(ensureLocalWasm(id));
    return { id, aliases: [], packageName, languagePackage, wasm: new Uint8Array(wasm) };
  });
}
