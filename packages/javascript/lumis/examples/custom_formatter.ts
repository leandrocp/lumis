import { createHighlighter, highlightIter } from "../src/index.ts";
import { type Formatter } from "../src/formatters.ts";
import {
  closingTags,
  escape,
  openCodeTag,
  openPreTag,
  openSpanTag,
  styleToCss,
  wrapLine,
} from "../src/formatter/html.ts";
import html from "../langs/html.ts";
import javascript from "../langs/javascript.ts";
import dracula from "@lumis-sh/themes/dracula";

class InteractiveDocsFormatter implements Formatter {
  constructor(readonly language = html) {}

  render(source: string): string {
    let tokenId = 0;
    const lines = [""];

    highlightIter(source, this.language, dracula, (text, language, range, scope, style) => {
      const parts = text.split("\n");
      for (let i = 0; i < parts.length; i += 1) {
        const part = parts[i] ?? "";
        if (part.length > 0) {
          const escaped = escape(part);
          if (scope) {
            lines[lines.length - 1] += `${openSpanTag({
              class: "tok",
              tabindex: 0,
              "data-token-id": String(++tokenId),
              "data-scope": scope,
              "data-language": language,
              "data-start": String(range.start),
              "data-end": String(range.end),
              "data-fg": style?.fg,
              "data-bg": style?.bg,
              style: styleToCss(style, { italic: true }),
            })}${escaped}</span>`;
          } else {
            lines[lines.length - 1] += escaped;
          }
        }
        if (i < parts.length - 1) lines.push("");
      }
    });

    const body = lines.map((line, index) => wrapLine(index + 1, line)).join("");
    return `${openPreTag({ preClass: "docs-demo" })}${openCodeTag(this.language)}${body}${closingTags()}`;
  }
}

const code = `<article class="profile-card">
  <h2>User profile</h2>
  <script>
    async function loadUserProfile(userId) {
      const response = await fetch(\`/api/users/\${userId}\`)
      if (!response.ok) throw new Error('Failed to load user profile')

      return response.json()
    }
  </script>
</article>`;

const hl = await createHighlighter({ languages: [html, javascript] });

console.log(hl.highlight(code, new InteractiveDocsFormatter()));
