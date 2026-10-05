import { readdirSync, readFileSync } from "node:fs";
import type { Budget, HighlightRange, HtmlStructure } from "../src/types.js";

export type SerializableHighlightEvent =
  | { type: "start"; scope: string; language: string }
  | { type: "source"; start: number; end: number }
  | { type: "end" }
  | {
      type: "decorationStart";
      decoration: { type: "rainbowBracket"; depth: number };
    }
  | { type: "decorationEnd" }
  | { type: "annotationStart"; range: HighlightRange; data: unknown }
  | { type: "annotationEnd" };

export interface FixtureAnnotation {
  range: HighlightRange;
  data: unknown;
}

export interface ConformanceFixture {
  name: string;
  language: string;
  theme: string;
  htmlMultiThemesOptions?: HtmlMultiThemesFixture;
  rainbowBrackets: boolean;
  annotations?: FixtureAnnotation[];
  /** What the three HTML outputs write around the tokens. */
  structure: HtmlStructure;
  /** The limits the fixture renders under. An absent limit keeps its default. */
  budget?: Budget;
  /** Runtimes that cannot run the fixture yet, each with the reason. */
  skip?: Record<string, string>;
  events: SerializableHighlightEvent[];
  source: string;
  htmlInline: string;
  htmlLinked: string;
  htmlMultiThemes: string;
  bbcode: string;
  terminal: string;
}

export interface HtmlMultiThemesFixture {
  themes: Record<string, string>;
  /** Absent means CSS-variables-only mode, which is its own rendering branch. */
  defaultTheme?: string;
  highlightLines?: number[];
}

interface FixtureMetadata {
  name: string;
  language: string;
  theme: string;
  htmlMultiThemes?: HtmlMultiThemesFixture;
  rainbowBrackets?: boolean;
  annotations?: FixtureAnnotation[];
  structure?: HtmlStructure;
  budget?: Budget;
  skip?: Record<string, string>;
  events: SerializableHighlightEvent[];
}

export function loadConformanceFixtures(): ConformanceFixture[] {
  const baseDir = new URL("../../../../fixtures/conformance/", import.meta.url);

  return (
    readdirSync(baseDir, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      // oxlint-disable-next-line oxc/no-map-spread -- mutating the parsed fixture would alias it.
      .map((entry) => {
        const fixtureDir = new URL(`${entry.name}/`, baseDir);
        const metadata: FixtureMetadata = JSON.parse(
          readFileSync(new URL("fixture.json", fixtureDir), "utf8"),
        );

        return {
          ...metadata,
          htmlMultiThemesOptions: metadata.htmlMultiThemes,
          rainbowBrackets: metadata.rainbowBrackets ?? false,
          structure: metadata.structure ?? "block",
          source: readFileSync(new URL("source.txt", fixtureDir), "utf8"),
          htmlInline: readFileSync(new URL("html-inline.html", fixtureDir), "utf8"),
          htmlLinked: readFileSync(new URL("html-linked.html", fixtureDir), "utf8"),
          htmlMultiThemes: readFileSync(new URL("html-multi-themes.html", fixtureDir), "utf8"),
          bbcode: readFileSync(new URL("bbcode.txt", fixtureDir), "utf8"),
          terminal: readFileSync(new URL("terminal.txt", fixtureDir), "utf8"),
        };
      })
      .sort((a, b) => a.name.localeCompare(b.name))
  );
}
