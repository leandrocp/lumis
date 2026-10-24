import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import * as browserEntry from "../src/index.browser.js";
import * as nodeEntry from "../src/index.js";
import * as nodeLanguages from "../src/languages.js";
import * as browserLanguages from "../src/languages.browser.js";
import * as html from "../src/formatter/html.js";
import * as browserThemes from "../src/themes/index.js";
import * as nodeThemes from "../src/themes/node.js";

type Spelling = string | string[];

interface Capability {
  name: string;
  spelling?: Record<string, Spelling>;
}

interface Manifest {
  capabilities: Capability[];
  runtime_only: Record<string, Record<string, string>>;
  deprecated: Record<string, Record<string, string>>;
  waived: Record<string, Record<string, string>>;
}

const manifest: Manifest = JSON.parse(
  readFileSync(new URL("../../../../fixtures/api.json", import.meta.url), "utf8"),
);

const lookupCases: {
  input: string;
  expected: string | null;
  parser: string | null;
  packageName: string | null;
}[] = JSON.parse(
  readFileSync(new URL("../../../../fixtures/language-lookup.json", import.meta.url), "utf8"),
);

/** The two published entries, each the `.` of a different condition. */
const entries = {
  node: { root: nodeEntry, languages: nodeLanguages, themes: nodeThemes },
  browser: { root: browserEntry, languages: browserLanguages, themes: browserThemes },
};

function exportsFor(entry: keyof typeof entries): Record<string, unknown> {
  const { root, languages, themes } = entries[entry];
  return {
    ...root,
    ...Object.fromEntries(
      Object.entries(languages).map(([name, value]) => [`languages.${name}`, value]),
    ),
    ...Object.fromEntries(Object.entries(themes).map(([name, value]) => [`themes.${name}`, value])),
  };
}

function toCamel(name: string): string {
  return name.replaceAll(/_([a-z])/gu, (_, letter: string) => letter.toUpperCase());
}

function jsNames(capability: Capability): string[] {
  const spelling = capability.spelling?.javascript;
  if (spelling === undefined) return [toCamel(capability.name)];
  return Array.isArray(spelling) ? spelling : [spelling];
}

/**
 * Every name JavaScript is required to offer: the canonical set minus what
 * `waived` exempts it from.
 */
function requiredNames(): string[] {
  const waived = new Set(sectionNames(manifest.waived));
  return manifest.capabilities
    .filter((capability) => !waived.has(capability.name))
    .flatMap((capability) => jsNames(capability));
}

function exportedNames(entry: keyof typeof entries): string[] {
  return Object.keys(exportsFor(entry)).sort();
}

function sectionNames(section: Record<string, Record<string, string>>): string[] {
  return Object.keys(section.javascript ?? {}).filter((name) => !name.startsWith("$"));
}

describe("top-level API manifest", () => {
  it("loads the shared language lookup corpus", () => {
    expect(lookupCases.length).toBeGreaterThanOrEqual(27);
  });

  for (const entry of ["node", "browser"] as const) {
    it.each(lookupCases)(
      `${entry} resolves language metadata for $input`,
      ({ input, expected, parser, packageName }) => {
        const language = entries[entry].languages.get(input);
        const listed = entries[entry].languages.available().find(({ id }) => id === expected);
        expect(language?.id ?? null).toBe(expected);
        expect(language?.parser ?? null).toBe(parser);
        expect(language?.packageName ?? null).toBe(packageName);
        expect(listed).toEqual(language);
      },
    );

    it(`${entry} exports every capability in the manifest`, () => {
      const exported = new Set(exportedNames(entry));
      const missing = requiredNames().filter((name) => !exported.has(name));

      expect(missing, `${entry} is missing these entry points`).toEqual([]);
    });

    it(`${entry} exports nothing the manifest does not account for`, () => {
      const accounted = new Set([
        ...requiredNames(),
        ...sectionNames(manifest.runtime_only),
        ...sectionNames(manifest.deprecated),
        // A waived name is accounted for, so a runtime that grows one fails
        // `has no waiver left standing` alone rather than here as well.
        ...sectionNames(manifest.waived),
      ]);
      const unaccounted = exportedNames(entry).filter((name) => !accounted.has(name));

      expect(
        unaccounted,
        "add them to fixtures/api.json and to the other runtimes, or classify them",
      ).toEqual([]);
    });

    it(`${entry} still exports every runtime_only entry point`, () => {
      const exported = new Set(exportedNames(entry));
      const gone = sectionNames(manifest.runtime_only).filter((name) => !exported.has(name));

      expect(gone, "drop the entry").toEqual([]);
    });

    it(`${entry} has no waiver left standing`, () => {
      const exported = new Set(exportedNames(entry));
      const kept = sectionNames(manifest.waived).filter((name) => exported.has(name));

      expect(kept, "JavaScript offers these; drop the waiver").toEqual([]);
    });
  }

  it("exports the same names from both entries", () => {
    expect(exportedNames("browser")).toEqual(exportedNames("node"));
  });

  for (const entry of ["node", "browser"] as const) {
    it(`${entry} keeps every deprecated root alias identical to its replacement`, () => {
      const exported = { ...exportsFor(entry), "html.sanitizeThemeName": html.sanitizeThemeName };
      const aliases = manifest.deprecated.javascript ?? {};
      expect(Object.keys(aliases).length).toBeGreaterThan(0);
      for (const [oldName, replacement] of Object.entries(aliases)) {
        expect(exported[replacement], replacement).toBeTypeOf("function");
        expect(exported[oldName], oldName).toBe(exported[replacement]);
      }
    });
  }
});
