import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { parse as parseToml } from "smol-toml";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { parseLanguagesToml } from "../scripts/languages-toml.js";

const root = resolve(import.meta.dirname, "../../../..");
const config = parseLanguagesToml(parseToml(readFileSync(join(root, "languages.toml"), "utf8")));
const bundles = Object.entries(config.bundles ?? {});
const out = mkdtempSync(join(tmpdir(), "lumis-wasm-bundles-"));

beforeAll(() => {
  execFileSync(
    process.execPath,
    ["--import", "tsx", "scripts/build-wasm-bundles.ts", "--out", out],
    { cwd: join(root, "packages/javascript") },
  );
});

afterAll(() => rmSync(out, { recursive: true, force: true }));

describe("generated WASM bundle packages", () => {
  it("checks every declared bundle", () => {
    expect(bundles.length).toBeGreaterThanOrEqual(5);
  });

  it.each(bundles)(
    "%s depends on and exports exactly its declared parsers",
    async (name, bundle) => {
      const ids =
        bundle.parsers === "all"
          ? Object.keys(config.parsers).filter((id) => !bundle.exclude?.includes(id))
          : bundle.parsers;
      const dependencies = Object.fromEntries(
        ids.map((id) => {
          const parser = config.parsers[id]?.wasm_name ?? `tree-sitter-${id}`;
          return [`@lumis-sh/wasm-${parser.replace(/^tree-sitter-/u, "")}`, expect.any(String)];
        }),
      );
      const dir = join(out, `wasm-bundle-${name}`);
      const manifest: unknown = JSON.parse(readFileSync(join(dir, "package.json"), "utf8"));
      expect(manifest).toHaveProperty("dependencies", dependencies);

      const generated = await import(pathToFileURL(join(dir, "index.js")).href);
      expect(generated.default[Symbol.for("@lumis-sh/package-url")]).toBe(
        pathToFileURL(join(dir, "index.js")).href,
      );
      expect(manifest).toHaveProperty("lumis.bundleFormat", 3);
      expect(Object.keys(generated.default).sort()).toEqual([...ids].sort());
      expect(dependencies).not.toHaveProperty("@lumis-sh/wasm-plaintext");
      expect(Object.hasOwn(dependencies, "@lumis-sh/wasm-diff")).toBe(name === "full");
    },
  );
});
