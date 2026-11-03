"use client";

import { createHighlighter } from "@lumis-sh/lumis/client";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import { CodeBlock, InlineCode } from "@lumis-sh/react";
import githubDark from "@lumis-sh/themes/github_dark";
import web from "@lumis-sh/wasm-bundle-web";

const highlighter = createHighlighter({ languages: [web] });
const formatter = htmlInline({ language: "tsx", theme: githubDark });

export function CodeBlockExample() {
  return (
    <>
      <CodeBlock highlighter={highlighter} formatter={formatter}>
        {`export function Button() {
  return <button type="button">Click me</button>
}`}
      </CodeBlock>
      <p>
        Render it with{" "}
        <InlineCode highlighter={highlighter} formatter={formatter}>
          {"<Button />"}
        </InlineCode>{" "}
        anywhere in your app.
      </p>
    </>
  );
}
