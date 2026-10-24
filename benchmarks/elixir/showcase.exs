repo_dir = Path.expand("../..", __DIR__)
generated_dir = Path.join(repo_dir, "benchmarks/showcase/generated")
System.put_env("LUMIS_BUILD", "1")

System.put_env(
  "CARGO_PATCH_CRATES_IO_LUMIS_WASM_RUNTIME_PATH",
  Path.join(repo_dir, "crates/lumis-wasm-runtime")
)

System.put_env(
  "CARGO_PATCH_CRATES_IO_LUMIS_CORE_PATH",
  Path.join(repo_dir, "crates/lumis-core")
)

Mix.install(
  [
    {:lumis, path: Path.join(repo_dir, "packages/elixir/lumis")},
    {:lumis_themes, path: Path.join(repo_dir, "packages/elixir/themes"), override: true},
    {:rustler, "~> 0.38"}
  ],
  lockfile: Path.expand("mix.lock", __DIR__)
)

Code.require_file("runtime.exs", __DIR__)
Lumis.BenchmarkRuntime.configure(repo_dir)

documents =
  generated_dir
  |> Path.join("assets/documents.json")
  |> File.read!()
  |> JSON.decode!()

themes =
  generated_dir
  |> Path.join("assets/themes.json")
  |> File.read!()
  |> JSON.decode!()

for document <- documents, theme <- themes do
  :ok = Lumis.Languages.load(document["load"])
  source = File.read!(Path.join(generated_dir, "assets/#{document["file"]}"))

  output =
    Lumis.highlight!(source,
      formatter: {:html_inline, language: document["language"], theme: theme["lumis"]}
    )

  unless byte_size(output) > byte_size(source) and String.contains?(output, "<pre") and
           String.contains?(output, "<span") do
    raise "Lumis Elixir did not produce highlighted HTML for #{document["id"]}"
  end

  path =
    Path.join(generated_dir, "fragments/#{document["id"]}/#{theme["id"]}/lumis-elixir.html")

  File.mkdir_p!(Path.dirname(path))
  File.write!(path, output)
end
