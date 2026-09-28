/**
 * A data directory Lumis cannot write, which is how a serverless host usually
 * leaves it: the deployment is mounted read-only. The document's language
 * loads from its installed package there, and so must a language injected
 * inside it.
 *
 * Its own file, because the addon reads the data directory once per process
 * and resolves installed packages from the working directory.
 */
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it, vi } from "vitest";

const project = mkdtempSync(join(tmpdir(), "lumis-read-only-data-dir-"));
// Under a regular file, so nothing can create it, root included.
writeFileSync(join(project, "file"), "");
process.env.LUMIS_DATA_DIR = join(project, "file", "lumis");

const { installLocalPackages } = await import("./wasm.js");
installLocalPackages(project, ["html", "css", "javascript"]);
process.chdir(project);

const { createHighlighter, highlight, runtimeKind } = await import("../src/index.js");
const { htmlLinked } = await import("../src/formatters.js");

describe("a data directory Lumis cannot write", () => {
  it("loads the document's language from its installed package", async () => {
    const hl = await createHighlighter({ languages: [] });
    await hl.loadLanguage("css");

    expect(hl.highlight("a { color: red }", htmlLinked({ language: "css" }))).toContain(
      'class="l-property"',
    );
  });

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
