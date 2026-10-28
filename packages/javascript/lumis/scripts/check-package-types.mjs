import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";
import ts from "typescript";

const directory = mkdtempSync(fileURLToPath(new URL("../dist/typecheck-", import.meta.url)));
const entries = {
  root: "",
  node: "/node",
  languages: "/languages",
  nodeLanguages: "/node/languages",
  themes: "/themes",
  formatters: "/formatters",
  html: "/formatters/html",
  ansi: "/formatters/ansi",
  rust: "/langs/rust",
  web: "/bundles/web",
};
const assertions = `
const metadata: languages.LanguageInfo | undefined = languages.get("main.rs");
const theme: themes.Theme | undefined = themes.get("dracula");
const mapped = themes.mapStyles({ name: "custom", appearance: "light", highlights: {} },
  (_scope, style) => ({ ...style, italic: false }));
const css: Promise<string> = import("@lumis-sh/themes").then(({ buildCss }) => buildCss(mapped));
const formatter: root.Formatter = formatters.htmlInline({ language: "rust", theme });
const render: typeof root.highlight = node.highlight;
const load: typeof languages.load = nodeLanguages.load;
const wasm: typeof languages.withWasm = root.withWasm;
const bundled: root.LanguageBundle = web.bundledLanguages;
const language: root.LanguagePackageHandle = rust.default;
void [metadata, formatter, render, load, wasm, bundled, language, html, ansi, css];
// @ts-expect-error An unresolved lookup is not a LanguageInfo.
const required: languages.LanguageInfo = languages.get("unknown");
`;
const formatHost = {
  getCanonicalFileName: (name) => name,
  getCurrentDirectory: () => directory,
  getNewLine: () => "\n",
};

function checkConsumer(file, module, moduleResolution, customConditions = []) {
  const program = ts.createProgram([file], {
    strict: true,
    skipLibCheck: false,
    noEmit: true,
    target: ts.ScriptTarget.ES2022,
    module,
    moduleResolution,
    customConditions,
    types: [],
  });
  const diagnostics = ts.getPreEmitDiagnostics(program);
  if (diagnostics.length > 0) {
    console.error(ts.formatDiagnosticsWithColorAndContext(diagnostics, formatHost));
    process.exitCode = 1;
  }
}

try {
  for (const format of ["mts", "cts"]) {
    const imports = Object.entries(entries).map(([name, entry]) =>
      format === "mts"
        ? `import * as ${name} from "@lumis-sh/lumis${entry}";`
        : `import ${name} = require("@lumis-sh/lumis${entry}");`,
    );
    const file = join(directory, `consumer.${format}`);
    writeFileSync(file, `${imports.join("\n")}\n${assertions}`);
    for (const module of [ts.ModuleKind.Node16, ts.ModuleKind.NodeNext]) {
      checkConsumer(
        file,
        module,
        module === ts.ModuleKind.Node16
          ? ts.ModuleResolutionKind.Node16
          : ts.ModuleResolutionKind.NodeNext,
      );
    }
  }
  const browser = join(directory, "browser.mts");
  writeFileSync(
    browser,
    `
import * as root from "@lumis-sh/lumis";
import * as client from "@lumis-sh/lumis/client";
import * as languages from "@lumis-sh/lumis/languages";
import * as clientLanguages from "@lumis-sh/lumis/client/languages";
import * as themes from "@lumis-sh/lumis/themes";
const load: typeof languages.load = clientLanguages.load;
const highlight: typeof root.highlight = client.highlight;
const metadata: languages.LanguageInfo | undefined = clientLanguages.get("rs");
const theme: themes.Theme | undefined = themes.get("dracula");
void [load, highlight, metadata, theme];
`,
  );
  checkConsumer(browser, ts.ModuleKind.ESNext, ts.ModuleResolutionKind.Bundler, ["browser"]);
  for (const [entry, declaration] of Object.entries({
    "": "index.browser",
    "/client": "index.browser",
    "/languages": "languages.browser",
    "/client/languages": "languages.browser",
    "/themes": "themes.browser",
  })) {
    const resolved = ts.resolveModuleName(
      `@lumis-sh/lumis${entry}`,
      browser,
      { moduleResolution: ts.ModuleResolutionKind.Bundler, customConditions: ["browser"] },
      ts.sys,
    ).resolvedModule?.resolvedFileName;
    if (!resolved?.endsWith(`/${declaration}.d.ts`)) {
      console.error(`Browser entry ${entry || "."} resolved unexpected declarations: ${resolved}`);
      process.exitCode = 1;
    }
  }
} finally {
  rmSync(directory, { recursive: true, force: true });
}
