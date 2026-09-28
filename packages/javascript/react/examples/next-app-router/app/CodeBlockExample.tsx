"use client";

import { createHighlighter } from "@lumis-sh/lumis/client";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import { CodeBlock } from "@lumis-sh/react";
import githubDark from "@lumis-sh/themes/github_dark";
import web from "@lumis-sh/wasm-bundle-web";

const highlighter = createHighlighter({ languages: [web] });

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
