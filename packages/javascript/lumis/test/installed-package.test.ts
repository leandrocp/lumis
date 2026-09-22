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
import { localLanguagePackageMetadata, ensureLocalWasm } from "./wasm.js";

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
  metadata.parser.size = wasm.byteLength;

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
});
