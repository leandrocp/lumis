import { createHighlighter } from "@lumis-sh/lumis";
import { htmlInline, htmlMultiThemes } from "@lumis-sh/lumis/formatters";
import type { LazyLanguage, Theme } from "@lumis-sh/lumis";
import full from "@lumis-sh/wasm-bundle-full";

const DEFAULT_PRE_CLASS =
  "m-0 overflow-x-auto p-5 font-mono text-[13px] leading-relaxed sm:p-6 sm:text-sm";

// The bundle registers every language up front and imports each one's package
// the first time a section asks for it, so a visit downloads only the parsers it
// highlights. A browser has no document-time loading, so a language injected
// into another highlights only once something here has loaded it.
const highlighter = createHighlighter({ languages: [full] });

async function loadLanguage(languageId: string): Promise<LazyLanguage> {
  const language = full[languageId];
  if (!language) {
    throw new Error(`Unknown language: ${languageId}`);
  }
  await (await highlighter).loadLanguage(language);
  return language;
}

export type WorkerRequest =
  | {
      id: number;
      type: "highlight";
      languageId: string;
      theme: Theme;
      source: string;
      preClass?: string;
    }
  | {
      id: number;
      type: "highlightMultiTheme";
      languageId: string;
      lightTheme: Theme;
      darkTheme: Theme;
      source: string;
      preClass?: string;
    }
  | { id: number; type: "loadLanguages"; languageIds: string[] };

export type WorkerResponse =
  | { id: number; type: "result"; html: string }
  | { id: number; type: "done"; loaded: string[] }
  | { id: number; type: "error"; message: string };

async function handleMessage(req: WorkerRequest): Promise<WorkerResponse> {
  switch (req.type) {
    case "highlight": {
      const language = await loadLanguage(req.languageId);
      const html = (await highlighter).highlight(
        req.source,
        htmlInline({
          language,
          theme: req.theme,
          preClass: req.preClass ?? DEFAULT_PRE_CLASS,
          includeHighlights: true,
          italic: false,
        }),
      );
      return { id: req.id, type: "result", html };
    }
    case "highlightMultiTheme": {
      const language = await loadLanguage(req.languageId);
      const html = (await highlighter).highlight(
        req.source,
        htmlMultiThemes({
          language,
          themes: { light: req.lightTheme, dark: req.darkTheme },
          defaultTheme: "light-dark()",
          preClass: req.preClass ?? DEFAULT_PRE_CLASS,
          italic: false,
        }),
      );
      return { id: req.id, type: "result", html };
    }
    case "loadLanguages": {
      // Warm-up is an optimization, so one unavailable parser reports itself and
      // leaves the rest loaded rather than failing the batch.
      const settled = await Promise.allSettled(req.languageIds.map(loadLanguage));
      const failures = settled.flatMap((result) =>
        result.status === "rejected" ? [result.reason] : [],
      );
      if (failures.length > 0) {
        console.warn("Lumis warm-up did not finish; languages load on demand", failures);
      }
      return { id: req.id, type: "done", loaded: (await highlighter).languages };
    }
  }
}

self.onmessage = async (e: MessageEvent<WorkerRequest>) => {
  try {
    const response = await handleMessage(e.data);
    self.postMessage(response);
  } catch (err) {
    self.postMessage({
      id: e.data.id,
      type: "error",
      message: String(err),
    } satisfies WorkerResponse);
  }
};
