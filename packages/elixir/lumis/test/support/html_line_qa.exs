root = Path.expand("../../../../..", __DIR__)
manifest = root |> Path.join("qa/html-lines/cases.json") |> File.read!() |> JSON.decode!()
:ok = Lumis.Languages.load(manifest["language"])

formatters = %{
  "html-inline" => :html_inline,
  "html-linked" => :html_linked,
  "html-multi-themes" => :html_multi_themes
}

output =
  for sample <- manifest["cases"],
      name <- manifest["formatters"],
      numbered <- manifest["lineNumbers"],
      into: %{} do
    common = [
      language: manifest["language"],
      line_numbers: numbered,
      highlight_lines: %{lines: manifest["highlightLines"], class: "l-highlighted"}
    ]

    options =
      case name do
        "html-inline" ->
          common ++ [theme: manifest["theme"], italic: true]

        "html-linked" ->
          common

        "html-multi-themes" ->
          common ++ [themes: [dark: manifest["theme"]], default_theme: "dark", italic: true]
      end

    key = Enum.join([sample["id"], name, if(numbered, do: "numbered", else: "plain")], "/")
    {key, Lumis.highlight!(sample["source"], formatter: {Map.fetch!(formatters, name), options})}
  end

path = Path.join(root, "target/html-line-qa/elixir.json")
File.mkdir_p!(Path.dirname(path))
File.write!(path, JSON.encode!(output))
IO.puts("Rendered Elixir QA cases")
