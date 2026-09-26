// The rollup rewrites import aliases back to their exported names without
// seeing local ones, so sources that typecheck can still ship declarations that
// do not. This checks them the way a consumer with `skipLibCheck: false` does.
import { readdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

const distDir = fileURLToPath(new URL("../dist/", import.meta.url));

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

  // web-tree-sitter's declarations name `EmscriptenModule` without depending on
  // `@types/emscripten`, so only diagnostics in what Lumis ships count here.
  const diagnostics = ts
    .getPreEmitDiagnostics(ts.createProgram(rootNames, options))
    .filter(
      (diagnostic) =>
        !diagnostic.file || path.resolve(diagnostic.file.fileName).startsWith(distDir),
    );

  if (diagnostics.length > 0) {
    console.error(ts.formatDiagnosticsWithColorAndContext(diagnostics, formatHost));
    process.exitCode = 1;
  }
}
