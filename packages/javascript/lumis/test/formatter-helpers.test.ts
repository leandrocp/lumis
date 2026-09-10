/**
 * JavaScript's half of the cross-runtime formatter helper check.
 *
 * `fixtures/formatter-helpers.json` lists the helper capabilities every runtime
 * must offer a custom formatter. JavaScript can read its own exports, so unlike
 * the Rust half this reflects rather than calls:
 *
 * - `exports every helper in the manifest` fails on a capability JavaScript
 *   lacks.
 * - `exports nothing the manifest does not account for` fails on an export in
 *   neither the manifest's helper set, `runtime_only`, nor `deprecated`. This is
 *   the one that catches drift: JavaScript grew 20 helpers Rust never got before
 *   anything checked (#1381).
 * - `marks every deprecation the manifest claims` reads the source for the
 *   `@deprecated` tag, which does not survive to run time.
 *
 * Helpers are camelCase here and snake_case in the manifest; `toCamel` bridges
 * that, and `spelling.javascript` overrides it where the two do not line up.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import * as ansi from "../src/formatter/ansi.js";
import * as html from "../src/formatter/html.js";

interface ManifestHelper {
  name: string;
  spelling?: Record<string, string>;
}

interface Manifest {
  modules: Record<string, { helpers: ManifestHelper[] }>;
  runtime_only: Record<string, Record<string, Record<string, string>>>;
  deprecated: Record<string, Record<string, { runtimes?: string[] }>>;
  waived: Record<string, unknown>;
}

const manifest: Manifest = JSON.parse(
  readFileSync(new URL("../../../../fixtures/formatter-helpers.json", import.meta.url), "utf8"),
);

const modules: Record<string, Record<string, unknown>> = { html, ansi };

// A helper defined in one file and re-exported from another is one helper, so
// every file behind a module is read for the `@deprecated` tag.
const sources: Record<string, string[]> = {
  html: ["../src/formatter/html.ts"],
  ansi: ["../src/formatter/ansi.ts", "../src/formatter/ansi-core.ts"],
};

function toCamel(name: string): string {
  return name.replaceAll(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());
}

function jsName(helper: ManifestHelper): string {
  return helper.spelling?.javascript ?? toCamel(helper.name);
}

function exportedNames(module: string): string[] {
  return Object.keys(modules[module] ?? {}).sort();
}

/** Names carrying a `@deprecated` JSDoc tag on their `export` in `module`'s sources. */
function deprecatedInSource(module: string): Set<string> {
  const marked = new Set<string>();
  const pattern =
    /\/\*\*(?:(?!\*\/)[\s\S])*?@deprecated[\s\S]*?\*\/\s*export\s+(?:async\s+)?function\s+(\w+)/g;

  for (const source of sources[module] ?? []) {
    const text = readFileSync(new URL(source, import.meta.url), "utf8");
    for (const match of text.matchAll(pattern)) {
      marked.add(match[1]);
    }
  }

  return marked;
}

function sectionNames(section: Record<string, Record<string, unknown>>, module: string): string[] {
  return Object.keys(section.javascript?.[module] ?? {}).filter((name) => !name.startsWith("$"));
}

function deprecatedNames(module: string): string[] {
  return (
    Object.entries(manifest.deprecated[module] ?? {})
      .filter(([name]) => !name.startsWith("$"))
      .filter(([, entry]) => entry.runtimes?.includes("javascript"))
      // A key is canonical snake_case where every runtime has the helper and the
      // JavaScript-only spelling where only JavaScript does; `toCamel` leaves the
      // latter alone.
      .map(([name]) => toCamel(name))
  );
}

describe("formatter helper manifest", () => {
  it("covers the modules JavaScript publishes", () => {
    expect(Object.keys(manifest.modules).sort()).toEqual(Object.keys(modules).sort());
  });

  for (const [module, entry] of Object.entries(manifest.modules)) {
    it(`${module} exports every helper in the manifest`, () => {
      const missing = entry.helpers
        .map(jsName)
        .filter((name) => !Object.hasOwn(modules[module] ?? {}, name));

      expect(missing).toEqual([]);
    });

    it(`${module} exports nothing the manifest does not account for`, () => {
      const accounted = new Set([
        ...entry.helpers.map(jsName),
        ...sectionNames(manifest.runtime_only, module),
        ...deprecatedNames(module),
      ]);

      const unaccounted = exportedNames(module).filter((name) => !accounted.has(name));

      expect(
        unaccounted,
        "add them to fixtures/formatter-helpers.json and to the other runtimes, or classify them",
      ).toEqual([]);
    });

    it(`${module} marks every deprecation the manifest claims`, () => {
      const marked = deprecatedInSource(module);
      const unmarked = deprecatedNames(module).filter((name) => !marked.has(name));

      expect(unmarked).toEqual([]);
    });

    it(`${module} still exports every runtime_only helper`, () => {
      const gone = sectionNames(manifest.runtime_only, module).filter(
        (name) => !Object.hasOwn(modules[module] ?? {}, name),
      );

      expect(gone, "drop the entry").toEqual([]);
    });
  }

  it("has no waiver left standing", () => {
    const waivers = Object.keys(manifest.waived).filter((key) => !key.startsWith("$"));

    expect(waivers).toEqual([]);
  });
});
