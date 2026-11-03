import { readdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";

const distDir = fileURLToPath(new URL("../dist/", import.meta.url));

/** Every `x instanceof WebAssembly.Module` in the source, in order. */
function moduleInstanceChecks(source) {
  const checks = [];
  const visit = (node) => {
    if (
      ts.isBinaryExpression(node) &&
      node.operatorToken.kind === ts.SyntaxKind.InstanceOfKeyword &&
      ts.isPropertyAccessExpression(node.right) &&
      node.right.getText() === "WebAssembly.Module"
    ) {
      checks.push(node);
    }
    ts.forEachChild(node, visit);
  };
  visit(ts.createSourceFile("bundle.js", source, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS));
  return checks;
}

const files = await readdir(distDir);
const targets = files.filter((file) => /^web-tree-sitter-.*\.(cjs|js)$/u.test(file));
let moduleChecks = 0;

for (const file of targets) {
  const filePath = path.join(distDir, file);
  let source = await readFile(filePath, "utf8");

  source = source.replaceAll('import("fs/promises")', 'import("node:" + "fs/promises")');
  source = source.replaceAll("import('fs/promises')", 'import("node:" + "fs/promises")');
  source = source.replaceAll('import("module")', 'import("node:" + "module")');
  source = source.replaceAll("import('module')", 'import("node:" + "module")');
  source = source.replaceAll(
    'new URL("web-tree-sitter.wasm", import.meta.url).href',
    '"web-tree-sitter.wasm"',
  );
  // Next.js's Edge sandbox hands over `.wasm?module` imports compiled in
  // another realm, where `instanceof` is false for a real module. Only
  // `next start` does this, not Vercel. Drop it once web-tree-sitter checks
  // the tag itself.
  const checks = moduleInstanceChecks(source);
  moduleChecks += checks.length;
  for (const check of checks.toReversed()) {
    const operand = source.slice(check.left.getStart(), check.left.end);
    source = `${source.slice(0, check.getStart())}(Object.prototype.toString.call(${operand}) === "[object WebAssembly.Module]")${source.slice(check.end)}`;
  }

  await writeFile(filePath, source, "utf8");
}

if (moduleChecks === 0) {
  throw new Error("found no `instanceof WebAssembly.Module` check to patch in web-tree-sitter");
}
