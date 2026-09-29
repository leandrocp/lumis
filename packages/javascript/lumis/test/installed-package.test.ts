/**
 * Loading a parser out of `node_modules`, which is what makes a closed set
 * usable rather than merely strict.
 *
 * Its own directory and its own working directory, because resolution happens
 * from the *project*, not from wherever Lumis itself is installed: resolving
 * relative to Lumis would find whatever parser version Lumis happens to carry.
 */
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { cacheKey } from "../src/core/languages.js";
import { writeCachedWasm } from "../src/runtime/node-cache.js";
import {
  ensureLocalWasm,
  installLocalPackages,
  localPackageLanguage,
  localLanguagePackageMetadata,
} from "./wasm.js";

const project = mkdtempSync(join(tmpdir(), "lumis-installed-"));
const packageRoot = join(project, "node_modules", "@lumis-sh", "wasm-json");
const previousCwd = process.cwd();

beforeAll(() => {
  const wasm = readFileSync(ensureLocalWasm("json"));
  mkdirSync(packageRoot, { recursive: true });

  // Shaped the way a published package is: the manifest under `./lumis.json`
  // in the export map, and the parser beside it named after `parser.name`.
  const metadata = structuredClone(localLanguagePackageMetadata("@lumis-sh/wasm-json"));
  metadata.parser.name = "tree-sitter-json";
  metadata.parser.sha256 = createHash("sha256").update(wasm).digest("hex");

  writeFileSync(join(packageRoot, "lumis.json"), JSON.stringify(metadata));
  writeFileSync(join(packageRoot, "tree-sitter-json.wasm"), wasm);
  writeFileSync(
    join(packageRoot, "package.json"),
    JSON.stringify({
      name: "@lumis-sh/wasm-json",
      version: metadata.version,
      type: "module",
      exports: {
        "./lumis.json": "./lumis.json",
        "./tree-sitter-json.wasm": "./tree-sitter-json.wasm",
      },
    }),
  );
  writeFileSync(
    join(project, "package.json"),
    JSON.stringify({ name: "host", dependencies: { "@lumis-sh/wasm-json": "^0.26" } }),
  );

  // pnpm links only the project's direct dependencies into its node_modules,
  // and keeps the languages a bundle depends on beside the bundle.
  const bundle = join(project, "node_modules", "@lumis-sh", "wasm-bundle-web");
  installLocalPackages(bundle, ["css"]);
  writeFileSync(join(bundle, "index.js"), "export default {};\n");
  writeFileSync(
    join(bundle, "package.json"),
    JSON.stringify({
      name: "@lumis-sh/wasm-bundle-web",
      type: "module",
      exports: { ".": "./index.js" },
      dependencies: { "@lumis-sh/wasm-css": "^0.26" },
    }),
  );

  // Installed with a digest its parser does not match.
  const diffManifest = installLocalPackages(project, ["diff"])["@lumis-sh/wasm-diff"];
  const diff = JSON.parse(readFileSync(diffManifest, "utf8")) as { parser: { sha256: string } };
  diff.parser.sha256 = "0".repeat(64);
  writeFileSync(diffManifest, JSON.stringify(diff));

  // Installed, but from a Tree-sitter series this build does not support.
  const luaManifest = installLocalPackages(project, ["lua"])["@lumis-sh/wasm-lua"];
  const lua = JSON.parse(readFileSync(luaManifest, "utf8")) as { version: string };
  lua.version = "0.27.0";
  writeFileSync(luaManifest, JSON.stringify(lua));

  process.env.LUMIS_DATA_DIR = mkdtempSync(join(tmpdir(), "lumis-installed-data-"));
  process.chdir(project);
});

afterAll(() => {
  process.chdir(previousCwd);
});

describe("a parser installed in the project", () => {
  it("is found through the package's ./lumis.json export", async () => {
    const { nodeRuntime } = await import("../src/runtime/node.js");

    const resolved = await nodeRuntime.resolveInstalledManifest?.("@lumis-sh/wasm-json");

    expect(resolved?.pathname).toContain("@lumis-sh/wasm-json/lumis.json");
  });

  // The entry point a parser package exports is the WASM *bytes* in Node, so a
  // URL was never there to resolve the manifest against. Reading one off it is
  // the defect that made installed packages unusable.
  it("does not depend on the package's default export being a URL", async () => {
    const { nodeRuntime } = await import("../src/runtime/node.js");

    const resolved = await nodeRuntime.resolveInstalledManifest?.("@lumis-sh/wasm-json");
    expect(resolved).toBeDefined();

    const manifest: unknown = JSON.parse(readFileSync(resolved!, "utf8"));
    expect((manifest as { packageName: string }).packageName).toBe("@lumis-sh/wasm-json");
  });

  it("answers nothing for a package that is not installed", async () => {
    const { nodeRuntime } = await import("../src/runtime/node.js");

    await expect(
      nodeRuntime.resolveInstalledManifest?.("@lumis-sh/wasm-haskell"),
    ).resolves.toBeUndefined();
  });

  it("is found through the bundle that brought it", async () => {
    const { nodeRuntime } = await import("../src/runtime/node.js");

    const resolved = await nodeRuntime.resolveInstalledManifest?.("@lumis-sh/wasm-css");

    expect(resolved?.pathname).toContain(
      "wasm-bundle-web/node_modules/@lumis-sh/wasm-css/lumis.json",
    );
  });

  it("loads a language the bundle brought", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const hl = await createHighlighter({ languages: [] });

    await hl.loadLanguage("css");

    expect(hl.highlight("a { color: red }", htmlLinked({ language: "css" }))).toContain(
      'class="l-',
    );
  });

  // The package manager already checked what it installed.
  it("loads the parser as installed, without checking it again", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const hl = await createHighlighter({ languages: [] });

    await hl.loadLanguage("diff");

    expect(hl.highlight("- old\n+ new", htmlLinked({ language: "diff" }))).toContain('class="l-');
  });

  it("is refused outside the supported version range", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const hl = await createHighlighter({ languages: [] });

    await expect(hl.loadLanguage("lua")).rejects.toThrow(
      /@lumis-sh\/wasm-lua@0\.27\.0 does not satisfy the supported range/,
    );
  });

  // A caller's resolver says where packages come from, as a native walk
  // already honors for an injected language.
  it("gives way to a resolver the caller configured", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const installed = localLanguagePackageMetadata("@lumis-sh/wasm-json");
    const numbersOnly = {
      ...installed,
      languages: { json: { aliases: [], highlights: "(number) @number" } },
    };
    const hl = await createHighlighter({
      languages: [],
      languagePackageResolver: () =>
        `data:application/json;base64,${Buffer.from(JSON.stringify(numbersOnly)).toString("base64")}`,
    });
    await hl.loadLanguage("json");

    const html = hl.highlight('["text", 42]', htmlLinked({ language: "json" }));

    expect(html).toContain('class="l-number"');
    expect(html).not.toContain('class="l-string"');
  });

  it("keeps each resolver's manifest to the highlighters that use it", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const installed = localLanguagePackageMetadata("@lumis-sh/wasm-json");
    const resolving = (highlights: string) => () =>
      `data:application/json;base64,${Buffer.from(
        JSON.stringify({ ...installed, languages: { json: { aliases: [], highlights } } }),
      ).toString("base64")}`;
    // The default path first, so its manifest is the one a shared cache would hold.
    const plain = await createHighlighter({ languages: [] });
    await plain.loadLanguage("json");
    const strings = await createHighlighter({
      languages: [],
      languagePackageResolver: resolving("(string) @string"),
    });
    const numbers = await createHighlighter({
      languages: [],
      languagePackageResolver: resolving("(number) @number"),
    });
    await strings.loadLanguage("json");
    await numbers.loadLanguage("json");

    const fromStrings = strings.highlight('["text", 42]', htmlLinked({ language: "json" }));
    const fromNumbers = numbers.highlight('["text", 42]', htmlLinked({ language: "json" }));

    expect(fromStrings).toContain('class="l-string"');
    expect(fromStrings).not.toContain('class="l-number"');
    expect(fromNumbers).toContain('class="l-number"');
    expect(fromNumbers).not.toContain('class="l-string"');
  });

  // The manifest decides the bytes. An installed parser that does not match it
  // is passed over, not treated as the answer.
  it("takes the parser a resolver's manifest names over the installed one", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const lua = new Uint8Array(readFileSync(ensureLocalWasm("lua")));
    const manifest = {
      ...localLanguagePackageMetadata("@lumis-sh/wasm-json"),
      // The installed package has a parser by this name, with other bytes.
      parser: {
        name: "tree-sitter-json",
        grammarName: "lua",
        sha256: createHash("sha256").update(lua).digest("hex"),
      },
      // `identifier` exists in Lua and not in JSON, so this compiles only
      // against the parser the manifest names.
      languages: { json: { aliases: [], highlights: "(identifier) @variable" } },
    };
    // Where the default WASM resolver finds those bytes, with no network.
    await writeCachedWasm(
      cacheKey({
        ...manifest.parser,
        packageName: manifest.packageName,
        version: manifest.version,
      }),
      lua,
    );
    const hl = await createHighlighter({
      languages: [],
      languagePackageResolver: () =>
        `data:application/json;base64,${Buffer.from(JSON.stringify(manifest)).toString("base64")}`,
    });
    await hl.loadLanguage("json");

    expect(hl.highlight("x = 42", htmlLinked({ language: "json" }))).toContain(
      'class="l-variable"',
    );
  });

  // `import json from "@lumis-sh/wasm-json"` works in Node as it does in a
  // browser. On the addon it is the installed package, so it takes the
  // installed path and a native walk still loads what it injects.
  it("loads a language its package exports", async () => {
    const { createHighlighter } = await import("../src/index.js");
    const { htmlLinked } = await import("../src/formatters.js");
    const json = localPackageLanguage("json");
    const hl = await createHighlighter({ languages: [json] });

    expect(hl.highlight('["text", 42]', htmlLinked({ language: json }))).toContain(
      'class="l-string"',
    );
  });
});
