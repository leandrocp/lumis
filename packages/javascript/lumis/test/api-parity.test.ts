/**
 * JavaScript's half of the cross-runtime top-level API check.
 *
 * `fixtures/api.json` lists the entry points every runtime must offer a caller.
 * JavaScript can read its own exports, so unlike the Rust half this reflects
 * rather than calls:
 *
 * - `exports every capability in the manifest` fails on a capability JavaScript
 *   lacks. That is the direction Elixir failed silently for `highlightEvents`
 *   until #1543.
 * - `exports nothing the manifest does not account for` fails on a value export
 *   in neither the manifest's capability set nor `runtime_only`. This is the one
 *   that catches drift: an entry point added here and nowhere else has to be
 *   classified before it can ship.
 * - `still exports every runtime_only entry point` fails on a `runtime_only`
 *   name that is gone, so the list cannot outlive its reasons.
 *
 * Both published entries are checked. `.` resolves to `index.ts` under Node and
 * `index.browser.ts` in a browser, and `./client` is `index.browser.ts`
 * everywhere, so a capability present in one and not the other is a gap for
 * whichever half of the audience lands on the wrong file.
 *
 * Capabilities are camelCase here and snake_case in the manifest; `toCamel`
 * bridges that, and `spelling.javascript` overrides it where the two do not line
 * up. Types are not checked: they are erased at run time, and their shape is
 * per-runtime anyway.
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import * as browserEntry from "../src/index.browser.js";
import * as nodeEntry from "../src/index.js";

type Spelling = string | string[];

interface Capability {
  name: string;
  spelling?: Record<string, Spelling>;
}

interface Manifest {
  capabilities: Capability[];
  runtime_only: Record<string, Record<string, string>>;
  waived: Record<string, Record<string, string>>;
}

const manifest: Manifest = JSON.parse(
  readFileSync(new URL("../../../../fixtures/api.json", import.meta.url), "utf8"),
);

/** The two published entries, each the `.` of a different condition. */
const entries: Record<string, Record<string, unknown>> = {
  "index.ts": nodeEntry,
  "index.browser.ts": browserEntry,
};

function toCamel(name: string): string {
  return name.replaceAll(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());
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

/** Value exports. `import * as` omits types, so this is the callable surface. */
function exportedNames(entry: string): string[] {
  return Object.keys(entries[entry] ?? {}).sort();
}

function sectionNames(section: Record<string, Record<string, string>>): string[] {
  return Object.keys(section.javascript ?? {}).filter((name) => !name.startsWith("$"));
}

describe("top-level API manifest", () => {
  it("spells every capability inside the package index", () => {
    // Rust and Elixir reach some capabilities through another module and prove
    // those by calling them. Every JavaScript one is an index export today, so
    // this file only reflects. A spelling naming a home elsewhere needs a call
    // here instead, and would otherwise be looked for as an export and missed.
    const elsewhere = manifest.capabilities
      .flatMap(jsNames)
      .filter((name) => name.includes(".") || name.includes("::"));

    expect(
      elsewhere,
      "these live outside the package index; add a call for them rather than relying on the scan",
    ).toEqual([]);
  });

  for (const entry of Object.keys(entries)) {
    it(`${entry} exports every capability in the manifest`, () => {
      const exported = new Set(exportedNames(entry));
      const missing = requiredNames().filter((name) => !exported.has(name));

      expect(missing, `${entry} is missing these entry points`).toEqual([]);
    });

    it(`${entry} exports nothing the manifest does not account for`, () => {
      const accounted = new Set([
        ...requiredNames(),
        ...sectionNames(manifest.runtime_only),
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
    const [first, ...rest] = Object.keys(entries);

    for (const entry of rest) {
      expect(
        exportedNames(entry),
        `${first} and ${entry} disagree; a caller resolving one of them is missing what the ` +
          "other has, and the manifest cannot tell which half of the audience is right",
      ).toEqual(exportedNames(first ?? ""));
    }
  });
});
