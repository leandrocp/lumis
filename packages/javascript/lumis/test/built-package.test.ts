import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { expect, it } from "vitest";
import { ensureLocalWasm, localLanguagePackageMetadata } from "./wasm.js";

it.each(["js", "cjs"])("the built Node %s entry keeps its Wasm fallback", (extension) => {
  const result = spawnSync(
    process.execPath,
    [
      "--input-type=module",
      "--eval",
      `
        import { readFileSync } from "node:fs";
        const { createHighlighter, runtimeKind } = await import("./dist/index.${extension}");
        const { htmlLinked } = await import("./dist/formatters.${extension}");
        const language = {
          id: "json",
          aliases: [],
          packageName: "@lumis-sh/wasm-json",
          wasm: new Uint8Array(readFileSync(process.argv[1])),
          languagePackage: JSON.parse(process.argv[2]),
        };
        const highlighter = await createHighlighter({ languages: [language] });
        console.log(runtimeKind());
        console.log(highlighter.highlight('{"answer": 42}', htmlLinked({ language })));
      `,
      fileURLToPath(ensureLocalWasm("json")),
      JSON.stringify(localLanguagePackageMetadata("@lumis-sh/wasm-json")),
    ],
    {
      cwd: new URL("../", import.meta.url),
      env: { ...process.env, LUMIS_TEST_RUNTIME: "wasm" },
      encoding: "utf8",
      timeout: 30_000,
    },
  );

  expect(result.error).toBeUndefined();
  expect(result.status, result.stderr).toBe(0);
  expect(result.stdout).toMatch(/^wasm\n/u);
  expect(result.stdout).toContain('class="l-number">42</span>');
});
