import { closest, get, mapStyles } from "../../dist/themes.browser.js";
import { spanInline, spanLinked } from "../../dist/formatters/html.js";
import { buildCss } from "@lumis-sh/themes";
import cases from "../../../../../fixtures/theme-choice-cases.json";

const output = document.querySelector("#output");
if (!output) throw new Error("missing output element");
output.textContent = JSON.stringify(cases.map((fixture) => closest(fixture)?.name ?? null));

const original = get("github_light");
if (!original) throw new Error("github_light is missing");
const selective = mapStyles(original, (scope, style) => ({
  ...style,
  italic: scope === "markup.raw.block" ? false : style.italic,
}));
const upright = mapStyles(original, (_scope, style) => ({ ...style, italic: false }));
const samples = [
  ["markup.raw.block", "const answer = 42;"],
  ["markup.italic", "Markdown emphasis"],
] as const;

for (const [name, theme] of Object.entries({ original, selective, upright })) {
  const section = document.createElement("section");
  section.id = name;
  const title = document.createElement("h2");
  title.textContent = name;
  section.append(title);
  for (const mode of ["linked", "inline"] as const) {
    const label = document.createElement("h3");
    label.textContent = mode;
    const pre = document.createElement("pre");
    pre.className = `lumis ${mode}`;
    pre.innerHTML = samples
      .map(([scope, text]) =>
        mode === "linked"
          ? spanLinked(text, scope)
          : spanInline(text, { scope, theme, italic: true, includeHighlights: true }),
      )
      .join("\n");
    section.append(label, pre);
  }
  const style = document.createElement("style");
  style.textContent = buildCss(theme, { scope: `#${name}`, layout: false });
  section.append(style);
  document.body.append(section);
}
