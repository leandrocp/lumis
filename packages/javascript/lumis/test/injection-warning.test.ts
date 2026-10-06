import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, beforeAll, describe, expect, it, vi } from "vitest";
import { installLocalPackages } from "./wasm.js";

const project = mkdtempSync(join(tmpdir(), "lumis-injection-warning-"));
const previousCwd = process.cwd();
const previousNodePath = process.env.NODE_PATH;
let index: typeof import("../src/index.js");
let htmlLinked: typeof import("../src/formatters.js").htmlLinked;
let highlighter: Awaited<ReturnType<typeof index.createHighlighter>>;

beforeAll(async () => {
  const manifests = installLocalPackages(project, [
    "html",
    "javascript",
    "markdown",
    "markdown_inline",
    "lua",
  ]);
  const luaPath = manifests["@lumis-sh/wasm-lua"];
  const lua = JSON.parse(readFileSync(luaPath, "utf8")) as { version: string };
  lua.version = "0.27.0";
  writeFileSync(luaPath, JSON.stringify(lua));
  writeFileSync(join(project, "package.json"), JSON.stringify({ name: "injection-warning" }));

  // Vitest's NODE_PATH would make the workspace's parsers available to this project.
  delete process.env.NODE_PATH;
  const { _initPaths } = await import("node:module");
  (_initPaths as () => void)();
  process.env.LUMIS_DATA_DIR = mkdtempSync(join(tmpdir(), "lumis-injection-warning-store-"));
  process.chdir(project);
  index = await import("../src/index.js");
  ({ htmlLinked } = await import("../src/formatters.js"));
  highlighter = await index.createHighlighter({ languages: [] });
  for (const language of ["html", "javascript", "markdown", "markdown_inline"]) {
    await highlighter.loadLanguage(language);
  }
}, 30_000);

afterAll(async () => {
  process.chdir(previousCwd);
  if (previousNodePath === undefined) delete process.env.NODE_PATH;
  else process.env.NODE_PATH = previousNodePath;
  const { _initPaths } = await import("node:module");
  (_initPaths as () => void)();
});

describe("unavailable injected languages", () => {
  it("runs under the requested runtime", () => {
    expect(index.runtimeKind()).toBe(process.env.LUMIS_TEST_RUNTIME);
  });

  it.each([
    ["css", "@lumis-sh/wasm-css"],
    ["ejs", "@lumis-sh/wasm-embedded-template"],
    ["C++", "@lumis-sh/wasm-cpp"],
    ["lua", "@lumis-sh/wasm-lua"],
  ])("gives runtime-specific advice for %s", (language, packageName) => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    try {
      const root = language === "css" ? "html" : "markdown";
      const plain = language === "css" ? "body { color: red }" : "plain";
      const source =
        language === "css"
          ? `<style>${plain}</style>\n<script>console.log(1)</script>`
          : `\`\`\`${language}\n${plain}\n\`\`\`\n`;
      const formatter = htmlLinked({ language: root });
      const output = highlighter.highlight(source, formatter);

      expect(output).toContain(`class="language-${root}"`);
      expect(output).toContain(plain);
      expect(warn).toHaveBeenCalledTimes(1);
      const warning = String(warn.mock.calls[0][0]);
      expect(warning).toContain(`Lumis could not load "${language}"`);
      if (index.runtimeKind() === "native") {
        // oxlint-disable-next-line vitest/no-conditional-expect -- both runtimes assert their distinct installation/loading guidance.
        expect(warning).toContain(
          `add ${packageName} to its dependencies, or update it if it is there`,
        );
        // oxlint-disable-next-line vitest/no-conditional-expect -- both runtimes assert their distinct installation/loading guidance.
        expect(warning).not.toContain("Load it up front");
      } else {
        // oxlint-disable-next-line vitest/no-conditional-expect -- both runtimes assert their distinct installation/loading guidance.
        expect(warning).toContain("Load it up front");
        // oxlint-disable-next-line vitest/no-conditional-expect -- both runtimes assert their distinct installation/loading guidance.
        expect(warning).not.toContain("add @lumis-sh/wasm-");
      }

      expect(highlighter.highlight(source, formatter)).toBe(output);
      expect(warn).toHaveBeenCalledTimes(1);
    } finally {
      warn.mockRestore();
    }
  });

  it("keeps highlighting available injections without warning about unknown names", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    try {
      const output = highlighter.highlight(
        '<script type="module">console.log(1)</script>',
        htmlLinked({ language: "html" }),
      );
      expect(output).toContain('class="l-number"');
      expect(warn).not.toHaveBeenCalled();
    } finally {
      warn.mockRestore();
    }
  });
});
