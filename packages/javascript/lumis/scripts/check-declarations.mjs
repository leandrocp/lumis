// The rollup rewrites import aliases back to their exported names without
// seeing local ones, so sources that typecheck can still ship declarations that
// do not. This checks them the way a consumer with `skipLibCheck: false` does,
// including what they import: web-tree-sitter's declarations do not typecheck
// without `@types/emscripten`, so its types stay out of the public API.
import { readFile, readdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

const distDir = fileURLToPath(new URL("../dist/", import.meta.url));
const api = JSON.parse(
  await readFile(new URL("../../../../fixtures/api.json", import.meta.url), "utf8"),
);

function checkDeprecations(checker, module, fileName) {
  const exported = checker.getExportsOfModule(module);
  for (const [name, replacement] of Object.entries(api.deprecated.javascript)) {
    const symbol = resolvedSymbol(
      checker,
      exported.find((entry) => entry.name === name),
    );
    const deprecated = symbol?.getJsDocTags(checker).find((tag) => tag.name === "deprecated");
    if (!deprecated?.text?.some((part) => part.text.includes("@lumis-sh/lumis/"))) {
      console.error(
        `${fileName}: ${name} must retain @deprecated with the import replacing ${replacement}`,
      );
      process.exitCode = 1;
    }
  }
}

function resolvedSymbol(checker, symbol) {
  return (
    symbol && (symbol.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(symbol) : symbol)
  );
}

function checkItalicDeprecations(checker, module, fileName, names) {
  const exported = checker.getExportsOfModule(module);
  for (const name of names) {
    const symbol = resolvedSymbol(
      checker,
      exported.find((entry) => entry.name === name),
    );
    const property = symbol && checker.getDeclaredTypeOfSymbol(symbol).getProperty("italic");
    const deprecated = property?.getJsDocTags(checker).find((tag) => tag.name === "deprecated");
    if (!deprecated?.text?.some((part) => part.text.includes("mapStyles"))) {
      throw new Error(`${fileName}: ${name}.italic must retain its mapStyles deprecation`);
    }
  }
}

const options = {
  strict: true,
  skipLibCheck: false,
  noEmit: true,
  target: ts.ScriptTarget.ES2022,
  module: ts.ModuleKind.ESNext,
  moduleResolution: ts.ModuleResolutionKind.Bundler,
  types: [],
};

const formatHost = {
  getCanonicalFileName: (fileName) => fileName,
  getCurrentDirectory: () => ts.sys.getCurrentDirectory(),
  getNewLine: () => ts.sys.newLine,
};

const files = await readdir(distDir, { recursive: true });

// One program per module format: web-tree-sitter declares the same ambient
// module in its `.d.ts` and `.d.cts`, and no consumer loads both.
for (const extension of [".d.ts", ".d.cts"]) {
  const rootNames = files
    .filter((file) => file.endsWith(extension))
    .map((file) => path.join(distDir, file));
  if (rootNames.length === 0) {
    throw new Error(`No ${extension} files in ${distDir}`);
  }

  const program = ts.createProgram(rootNames, options);
  const diagnostics = ts.getPreEmitDiagnostics(program);

  if (diagnostics.length > 0) {
    console.error(ts.formatDiagnosticsWithColorAndContext(diagnostics, formatHost));
    process.exitCode = 1;
  }

  // An entry point missing from tsconfig.build.json still ships, exporting nothing.
  const checker = program.getTypeChecker();
  for (const fileName of rootNames) {
    const module = checker.getSymbolAtLocation(program.getSourceFile(fileName));
    if (!module || checker.getExportsOfModule(module).length === 0) {
      console.error(`${fileName} exports nothing. Is its entry point in tsconfig.build.json?`);
      process.exitCode = 1;
    }
    if (module && /^index(?:\.browser)?\.d\.(?:c)?ts$/u.test(path.basename(fileName))) {
      checkDeprecations(checker, module, fileName);
    }
    if (module && /^formatters\.d\.(?:c)?ts$/u.test(path.basename(fileName))) {
      checkItalicDeprecations(checker, module, fileName, [
        "HtmlInlineOptions",
        "HtmlMultiThemesOptions",
      ]);
    }
    if (
      module &&
      path.dirname(fileName) === path.join(distDir, "formatters") &&
      /^html\.d\.(?:c)?ts$/u.test(path.basename(fileName))
    ) {
      checkItalicDeprecations(checker, module, fileName, [
        "SpanInlineOptions",
        "SpanMultiThemesOptions",
      ]);
    }
  }
}
