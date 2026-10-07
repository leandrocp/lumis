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

const output = document.querySelector("#output");
const error = document.querySelector("#error");

try {
  const highlighter = await createHighlighter({ languages: [elixir] });
  output.innerHTML = highlighter.highlight(
    source,
    htmlInline({ language: elixir, theme: githubLight }),
  );
} catch (err) {
  error.style.display = "block";
  error.textContent = err instanceof Error ? (err.stack ?? err.message) : String(err);
  output.textContent = "Failed to load the Elixir parser.";
}
