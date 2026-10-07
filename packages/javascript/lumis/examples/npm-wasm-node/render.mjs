import { writeFile } from "node:fs/promises";
import { createHighlighter } from "@lumis-sh/lumis";
import { htmlInline } from "@lumis-sh/lumis/formatters";
import githubLight from "@lumis-sh/themes/github_light";
import { elixir } from "@lumis-sh/wasm-elixir";

const source = `defmodule Lumis.Release do
  def manifest(env) do
    %{target: env, profile: :release, artifact: "lumis-#{env}.beam"}
  end

  def print(env) do
    env
    |> manifest()
    |> IO.inspect(label: "release")
  end
end`;

const highlighter = await createHighlighter({ languages: [elixir] });
const code = highlighter.highlight(source, htmlInline({ language: elixir, theme: githubLight }));

const html = `<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>npm-wasm-node + Lumis</title>
    <style>
      body { font-family: sans-serif; margin: 2rem; background: #f6f7f9; }
      main { max-width: 960px; margin: 0 auto; }
      pre.lumis { overflow-x: auto; }
    </style>
  </head>
  <body>
    <main>
      <h1>npm-wasm-node + Lumis</h1>
      <p>This example runs in Node and imports <code>@lumis-sh/wasm-elixir</code> directly.</p>
      ${code}
    </main>
  </body>
</html>
`;

await writeFile(new URL("./output.html", import.meta.url), html);
console.log("Wrote packages/javascript/lumis/examples/npm-wasm-node/output.html");
