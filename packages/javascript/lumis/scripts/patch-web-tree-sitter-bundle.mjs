import { readdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const distDir = fileURLToPath(new URL("../dist/", import.meta.url));

const files = await readdir(distDir);
const targets = files.filter((file) => /^web-tree-sitter-.*\.(cjs|js)$/u.test(file));

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

  await writeFile(filePath, source, "utf8");
}
