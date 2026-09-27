/**
 * A data directory Lumis cannot write, which is how a serverless host usually
 * leaves it: the deployment is mounted read-only. The document's language
 * loads from its installed package there, and so must a language injected
 * inside it.
 *
 * Its own file, because the addon reads the data directory once per process
 * and resolves installed packages from the working directory.
 */
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it, vi } from "vitest";

const project = mkdtempSync(join(tmpdir(), "lumis-read-only-data-dir-"));
// Under a regular file, so nothing can create it, root included.
writeFileSync(join(project, "file"), "");
process.env.LUMIS_DATA_DIR = join(project, "file", "lumis");

const { ensureLocalWasm, localLanguagePackageMetadata } = await import("./wasm.js");

// Shaped the way a published package is: the manifest under `./lumis.json` in
// the export map, and the parser beside it named after `parser.name`.
for (const language of ["html", "css", "javascript"]) {
  const metadata = localLanguagePackageMetadata(`@lumis-sh/wasm-${language}`);
  const root = join(project, "node_modules", metadata.packageName);
  const parser = `${metadata.parser.name}.wasm`;
  mkdirSync(root, { recursive: true });
  writeFileSync(join(root, "lumis.json"), JSON.stringify(metadata));
  writeFileSync(join(root, parser), readFileSync(ensureLocalWasm(language)));
  writeFileSync(join(root, "index.js"), `export default new URL("./${parser}", import.meta.url);`);
  writeFileSync(
    join(root, "package.json"),
    JSON.stringify({
      name: metadata.packageName,
      version: metadata.version,
      type: "module",
      exports: { ".": "./index.js", "./lumis.json": "./lumis.json" },
    }),
  );
}
process.chdir(project);

const { createHighlighter, highlight, runtimeKind } = await import("../src/index.js");
const { htmlLinked } = await import("../src/formatters.js");

describe("a data directory Lumis cannot write", () => {
  // `web-tree-sitter` cannot load a language during the walk that finds it,
  // from any directory, so there is nothing to check there.
  it.runIf(runtimeKind() === "native")(
    "still loads an injected language from its installed package",
    async () => {
      const warn = vi.spyOn(console, "warn");
      const formatter = htmlLinked({ language: "html" });

      // Formatted on a worker, without coming back to JavaScript.
      const style = await highlight("<style>a { color: red }</style>", formatter);
      // Formatted on the main thread, which can call a JavaScript resolver.
      const hl = await createHighlighter({ languages: [] });
      await hl.loadLanguage("html");
      const script = hl.highlight("<script>let answer = 42</script>", formatter);

      expect(style).toContain('class="l-property"');
      expect(script).toContain('class="l-keyword"');
      expect(warn).not.toHaveBeenCalled();
    },
  );
});
