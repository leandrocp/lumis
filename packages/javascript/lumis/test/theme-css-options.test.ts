import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
import { buildCss, type BuildCssOptions } from "@lumis-sh/themes";
import ts from "typescript";
import { expect, it } from "vitest";
import manifest from "../../../../fixtures/theme-css-options.json";

const defaults: Required<BuildCssOptions> = {
  layout: true,
  enableItalic: true,
  scope: "",
  containerSelector: ".lumis",
  containerStyle: [],
};

function camel(name: string): string {
  return name.replaceAll(/_([a-z])/gu, (_, letter: string) => letter.toUpperCase());
}

const theme: Parameters<typeof buildCss>[0] = { ...manifest.theme, appearance: "dark" };

it("pins CSS option names and defaults in both directions", () => {
  const expected = Object.fromEntries(
    manifest.options.map((option) => [camel(option.name), option.default]),
  );
  expect(defaults).toEqual(expected);
  expect(buildCss(theme)).toBe(buildCss(theme, defaults));
  const path = new URL("../../themes/src/css.ts", import.meta.url);
  const source = ts.createSourceFile(
    path.pathname,
    readFileSync(path, "utf8"),
    ts.ScriptTarget.Latest,
  );
  const declaration = source.statements.find(
    (node): node is ts.InterfaceDeclaration =>
      ts.isInterfaceDeclaration(node) && node.name.text === "BuildCssOptions",
  );
  expect(declaration).toBeDefined();
  expect(declaration?.members.map((member) => member.name?.getText(source)).sort()).toEqual(
    Object.keys(defaults).sort(),
  );
});

it("renders the shared CSS cases", () => {
  expect(manifest.cases.length).toBeGreaterThanOrEqual(6);
  for (const entry of manifest.cases) {
    const options: BuildCssOptions = {
      layout: entry.options.layout,
      enableItalic: entry.options.enable_italic,
      scope: entry.options.scope,
      containerSelector: entry.options.container_selector,
      containerStyle: entry.options.container_style?.map((pair): [string, string] => {
        const [property, value] = pair;
        if (pair.length !== 2 || property === undefined || value === undefined)
          throw new Error("expected a property/value pair");
        return [property, value];
      }),
    };
    expect(buildCss(theme, options), entry.name).toBe(entry.css);
  }
});

it("keeps layout in every prebuilt stylesheet", () => {
  const root = new URL("../../../../css/", import.meta.url);
  const require = createRequire(import.meta.url);
  const files = readdirSync(root).filter((file) => file.endsWith(".css"));
  expect(files.length).toBeGreaterThan(200);
  for (const file of files) {
    const css = readFileSync(new URL(file, root), "utf8");
    expect(css).toContain("@layer lumis {");
    expect(readFileSync(require.resolve(`@lumis-sh/themes/css/${file}`), "utf8")).toBe(css);
  }
});
