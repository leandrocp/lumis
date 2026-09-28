"use client";

import { createHighlighter, withWasmBundle } from "@lumis-sh/lumis/client";
import { bundledLanguages } from "@lumis-sh/lumis/bundles/web";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import { CodeBlock } from "@lumis-sh/react";
import githubDark from "@lumis-sh/themes/github_dark";
import { bundledPackages } from "@lumis-sh/wasm-bundle-web";

const highlighter = createHighlighter({
  languages: [withWasmBundle(bundledLanguages, bundledPackages)],
});

export function CodeBlockExample() {
  return (
    <CodeBlock
      highlighter={highlighter}
      formatter={htmlInline({ language: "tsx", theme: githubDark })}
    >
      {`export function Button() {
  return <button type="button">Click me</button>
}`}
    </CodeBlock>
  );
}
