import { openSpanTag, spanMultiThemesAttrs } from "../../src/formatter/html.ts";
import type { Theme } from "../../src/types.ts";

/**
 * Two themes that agree on one scope's non-color properties and disagree on the
 * others, matching `test/light-dark.test.ts` and its counterpart in
 * `crates/lumis-core/src/formatter/html.rs`.
 */
const themes: Record<string, Theme> = {
  light: {
    name: "light",
    appearance: "light",
    highlights: {
      normal: { fg: "#111111", bg: "#ffffff" },
      keyword: { fg: "#d73a49", bold: true },
      comment: { fg: "#6a737d", italic: true },
      string: { fg: "#032f62", underline: true },
      variable: { fg: "#24292f" },
    },
  },
  dark: {
    name: "dark",
    appearance: "dark",
    highlights: {
      normal: { fg: "#eeeeee", bg: "#000000" },
      keyword: { fg: "#ff7b72", bold: true },
      comment: { fg: "#8b949e" },
      string: { fg: "#a5d6ff", strikethrough: true },
      variable: { fg: "#c9d1d9" },
    },
  },
};

const spans = ["keyword", "comment", "string", "variable"]
  .map((scope) => {
    const attrs = spanMultiThemesAttrs({
      scope,
      themes,
      defaultTheme: "light-dark()",
      italic: true,
    });
    return `${openSpanTag({ "data-testid": scope, ...attrs })}${scope}</span>`;
  })
  .join("");

document.body.innerHTML = `<pre class="lumis"><code>${spans}</code></pre>`;
