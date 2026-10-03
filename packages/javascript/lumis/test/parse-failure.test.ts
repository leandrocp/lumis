import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { expect, it } from "vitest";
import { createHighlighter } from "../src/index.js";
import { htmlLinked } from "../src/formatters.js";
import php from "../langs/php.ts";
import { localLanguagePackageMetadata } from "./wasm.js";

it("reports a real scanner trap as a parse failure", async () => {
  const wasm = readFileSync(
    new URL("../../../../fixtures/failing-parsers/php-0.26.4.wasm", import.meta.url),
  );
  const metadata = structuredClone(localLanguagePackageMetadata(php.packageName));
  metadata.parser.sha256 = createHash("sha256").update(wasm).digest("hex");
  const language = { ...php, wasm, languagePackage: metadata };
  const highlighter = await createHighlighter({ languages: [language] });
  const format = htmlLinked({ language });
  expect(highlighter.highlight('<?php $a = "x $b";', format)).toContain("l-variable");
  expect(() => highlighter.highlight("<?php\n$a = <<<END\nEND;\n", format)).toThrow(
    /parser returned no tree for language 'php'/,
  );
}, 30_000);
