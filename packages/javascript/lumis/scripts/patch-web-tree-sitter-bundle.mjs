import { readdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const distDir = fileURLToPath(new URL("../dist/", import.meta.url));

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
  const moduleCheck = /([\w$]+) instanceof WebAssembly\.Module\b/gu;
  moduleChecks += source.match(moduleCheck)?.length ?? 0;
  source = source.replaceAll(
    moduleCheck,
    'Object.prototype.toString.call($1) === "[object WebAssembly.Module]"',
  );

  await writeFile(filePath, source, "utf8");
}

if (moduleChecks === 0) {
  throw new Error("found no `instanceof WebAssembly.Module` check to patch in web-tree-sitter");
}
