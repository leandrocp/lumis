import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { sep } from "node:path";
import { buildCss, type ThemeData } from "@lumis-sh/themes";
import githubLight from "@lumis-sh/themes/github_light";
import { describe, expect, it } from "vitest";
import { scopeToClass } from "../src/formatter/html.js";
import { HIGHLIGHT_NAMES } from "../src/highlights.js";

const require = createRequire(import.meta.url);

const sample: ThemeData = {
  name: "test",
  appearance: "dark",
  revision: "3e976b4",
  highlights: {
    normal: { fg: "red", bg: "green" },
    keyword: { fg: "blue", italic: true },
    line_number: { fg: "silver" },
    "line_number.highlighted": { fg: "white", bold: true },
    "tag.attribute": { bg: "gray", bold: true },
  },
};

describe("buildCss", () => {
  it("renders the default stylesheet", () => {
    const expected = `/* test
 * revision: 3e976b4
 */
.lumis {
  color: red;
  background-color: green;
}
.l-keyword {
  color: blue;
  font-style: italic;
}
.l-line-number:not(.l-line-number-highlighted) {
  color: silver;
}
.l-line-number-highlighted {
  color: white;
  font-weight: bold;
}
.l-tag-attribute {
  background-color: gray;
  font-weight: bold;
}
`;

    expect(buildCss(sample)).toBe(expected);
  });

  it("scopes selectors and applies container style", () => {
    const expected = `/* test
 * revision: 3e976b4
 */
html[data-theme="dark"] .lumis {
  color: red;
  background-color: var(--color-grey-900);
  border-radius: 0.375rem;
}
html[data-theme="dark"] .l-keyword {
  color: blue;
  font-style: italic;
}
html[data-theme="dark"] .l-line-number:not(.l-line-number-highlighted) {
  color: silver;
}
html[data-theme="dark"] .l-line-number-highlighted {
  color: white;
  font-weight: bold;
}
html[data-theme="dark"] .l-tag-attribute {
  background-color: gray;
  font-weight: bold;
}
`;

    expect(
      buildCss(sample, {
        scope: 'html[data-theme="dark"]',
        containerSelector: ".lumis",
        containerStyle: [
          ["background-color", "var(--color-grey-900)"],
          ["border-radius", "0.375rem"],
        ],
      }),
    ).toBe(expected);
  });

  it("omits italic styles when disabled", () => {
    const css = buildCss(sample, { enableItalic: false });

    expect(css).toContain(".l-keyword {\n  color: blue;\n}");
    expect(css).not.toContain("font-style: italic;");
  });

  it("uses the regular line-number rule as the highlighted fallback", () => {
    const fallbackTheme: ThemeData = {
      name: "fallback",
      appearance: "dark",
      highlights: { line_number: { fg: "silver" } },
    };

    const css = buildCss(fallbackTheme);

    expect(css).toContain(".l-line-number {\n  color: silver;\n}");
    expect(css).not.toContain(".l-line-number:not(");
  });

  it("matches the bundled stylesheet for the default config", () => {
    const bundled = readFileSync(require.resolve("@lumis-sh/themes/css/github_light"), "utf-8");

    expect(buildCss(githubLight)).toBe(bundled);
  });

  // `buildCss` spells a class from the scope; the HTML formatters read the
  // generated `CLASSES` table. Two derivations of one name drift, and this is
  // where they did: replacing `_` alongside `.` moved the nine scopes holding
  // one — `attribute.c_sharp`, `module.c_sharp` and every `*.markdown_inline`
  // scope — onto classes no element carries, with nothing to fail. This is the
  // only file where both packages are in scope.
  it("writes the classes the formatters write", () => {
    const allScopes: ThemeData = {
      name: "all-scopes",
      appearance: "dark",
      highlights: Object.fromEntries(HIGHLIGHT_NAMES.map((scope) => [scope, { fg: "red" }])),
    };

    const underscored = HIGHLIGHT_NAMES.filter((scope) => scope.includes("_"));
    expect(underscored.length).toBeGreaterThanOrEqual(9);

    const css = buildCss(allScopes);

    for (const scope of HIGHLIGHT_NAMES) {
      if (scope === "normal") continue;
      expect(css, `${scope} has no rule for the class the formatters write`).toContain(
        `\n.${scopeToClass(scope)} {\n`,
      );
    }
  });
});

// `./css/*` maps onto `./dist/css/*.css`, so it appends the extension itself and
// the specifier every doc page shows, with one, resolved to `github_light.css.css`.
// Both forms are mapped now, and this fails if either stops resolving or lands
// on the wrong asset.
describe("bundled asset specifiers", () => {
  it.each([
    ["@lumis-sh/themes/css/github_light.css", "dist/css/github_light.css"],
    ["@lumis-sh/themes/css/github_light", "dist/css/github_light.css"],
    ["@lumis-sh/themes/json/github_light.json", "dist/json/github_light.json"],
    ["@lumis-sh/themes/json/github_light", "dist/json/github_light.json"],
  ])("resolves %s", (specifier, target) => {
    const resolved = require.resolve(specifier).replaceAll(sep, "/");

    expect(resolved.endsWith(`/${target}`)).toBe(true);
    expect(readFileSync(resolved, "utf-8").length).toBeGreaterThan(0);
  });
});
