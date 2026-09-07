import diff from "../langs/diff.js";
import plaintext from "../langs/plaintext.js";
import { withWasm } from "../src/index.js";
import type {
  CreateHighlighterOptions as BrowserCreateHighlighterOptions,
  Formatter as BrowserFormatter,
} from "../src/index.browser.js";
import type {
  Annotation,
  CreateHighlighterOptions,
  Formatter,
  HighlightEvent,
  Language,
  LanguagePackageHandle,
  LanguageRef,
  PlaintextLanguage,
} from "../src/index.js";

export interface ExtendedLanguage extends Language {
  dialect: string;
}

export const identifierOnlyReference: LanguageRef = { id: "python", aliases: [] };
export const generatedPackageHandle: LanguagePackageHandle = diff;
export const generatedPlaintext: PlaintextLanguage = plaintext;
export const importedHandleWithWasm: LanguagePackageHandle = withWasm(diff, new Uint8Array());

export const extensibleLanguage: ExtendedLanguage = {
  ...diff,
  dialect: "example",
};

export const resolverOptions: CreateHighlighterOptions = {
  languagePackageResolver: (packageName) => `https://packages.example/${packageName}`,
  wasmResolver: (language) => `https://parsers.example/${language}.wasm`,
};
export const browserResolverOptions: BrowserCreateHighlighterOptions = resolverOptions;

// Queries are not part of a language. A parser and the queries written against
// it ship together in a package, so neither `Language` nor any loadable variant
// accepts query text. Reintroducing them has to reintroduce these errors too.
export const languageRejectsQueries: Language = {
  id: "custom-json",
  aliases: [],
  // @ts-expect-error -- queries come from the package, never from the caller.
  highlights: "(string) @string",
};

export const packageHandleRejectsQueries: LanguagePackageHandle = {
  id: "json",
  aliases: ["jsonc"],
  packageName: "@lumis-sh/wasm-json",
  // @ts-expect-error -- package-owned queries come only from lumis.json.
  highlights: undefined,
};

export const plaintextWithUndefinedWasm: PlaintextLanguage = {
  id: "plaintext",
  aliases: ["text", "txt", "plain"],
  // @ts-expect-error -- plaintext never has a parser source.
  wasm: undefined,
};

// Writing a formatter that consumes annotations needs the event types and the
// interface those events are delivered to. Both entry points carry the whole
// set, so a custom formatter never has to reach into `/formatters` for the one
// type the root forgot.
export const annotationFormatter: Formatter<{ id: number }> = {
  language: plaintext,
  render(source: string, events: readonly HighlightEvent<{ id: number }>[]): string {
    const annotated: number[] = [];
    for (const event of events) {
      if (event.type === "annotationStart") annotated.push(event.annotation.data.id);
    }
    return `${source.length}:${annotated.join(",")}`;
  },
};

export const browserAnnotationFormatter: BrowserFormatter<{ id: number }> = annotationFormatter;

export const offsetAnnotation: Annotation<{ id: number }> = {
  range: { type: "offset", start: 0, end: 1 },
  data: { id: 1 },
};

export const positionAnnotation: Annotation<{ id: number }> = {
  range: { type: "position", start: { line: 0, column: 0 }, end: { line: 0, column: 1 } },
  data: { id: 2 },
};
