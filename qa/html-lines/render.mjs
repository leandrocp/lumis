import {
  htmlInline,
  htmlLinked,
  htmlMultiThemes,
} from "../../packages/javascript/lumis/src/formatters.ts";
import manifest from "./cases.json" with { type: "json" };

export function renderCases(highlighter, language, theme) {
  const output = {};
  for (const sample of manifest.cases) {
    for (const name of manifest.formatters) {
      for (const lineNumbers of manifest.lineNumbers) {
        const options = {
          language,
          lineNumbers,
          italic: true,
          highlightLines: { lines: manifest.highlightLines, class: "l-highlighted" },
        };
        const formatters = new Map([
          ["html-inline", htmlInline({ ...options, theme })],
          ["html-linked", htmlLinked(options)],
          [
            "html-multi-themes",
            htmlMultiThemes({ ...options, themes: { dark: theme }, defaultTheme: "dark" }),
          ],
        ]);
        const formatter = formatters.get(name);
        if (!formatter) throw new Error(`Unknown QA formatter ${name}`);
        const key = `${sample.id}/${name}/${lineNumbers ? "numbered" : "plain"}`;
        output[key] = highlighter.highlight(sample.source, formatter);
      }
    }
  }
  return output;
}
