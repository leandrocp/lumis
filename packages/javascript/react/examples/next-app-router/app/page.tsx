import { htmlInline } from "@lumis-sh/lumis/formatters";
import { renderCodeBlock, renderInlineCode } from "@lumis-sh/react/server";
import githubLight from "@lumis-sh/themes/github_light";
import { CodeBlockExampleLazy } from "./CodeBlockExampleLazy";

const mainStyle = {
  fontFamily: "system-ui, sans-serif",
  margin: "0 auto",
  maxWidth: 880,
  padding: "48px 24px",
};
const spacerStyle = { height: 32 };

export default async function Page() {
  const serverBlock = await renderCodeBlock({
    children: `export async function Article() {
  const post = await fetch("https://example.com/api/post").then((res) => res.json())
  return <article>{post.title}</article>
}`,
    formatter: htmlInline({
      language: "tsx",
      theme: githubLight,
    }),
  });
  const serverInline = await renderInlineCode({
    children: "await fetch(url)",
    formatter: htmlInline({
      language: "tsx",
      theme: githubLight,
    }),
  });

  return (
    <main style={mainStyle}>
      <h1>Lumis React with Next.js App Router</h1>
      <p>
        The first block is rendered on the server with <code>@lumis-sh/react/server</code>.
      </p>
      {serverBlock}
      <p>Inline code renders on the server too, as in {serverInline}.</p>

      <div style={spacerStyle} />

      <p>
        The second block is a client component using <code>CodeBlock</code>, followed by{" "}
        <code>InlineCode</code>.
      </p>
      <CodeBlockExampleLazy />
    </main>
  );
}
