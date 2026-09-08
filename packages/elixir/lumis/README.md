# Lumis

<!-- MDOC -->

<p align="center">
  Syntax highlighter powered by Tree-sitter and Neovim themes.
</p>

<p align="center">
  <a href="https://lumis.sh">https://lumis.sh</a>
</p>

<div align="center">
  <a href="https://hex.pm/packages/lumis">
    <img alt="Hex Version" src="https://img.shields.io/hexpm/v/lumis">
  </a>

  <a href="https://hexdocs.pm/lumis">
    <img alt="Hex Docs" src="http://img.shields.io/badge/hex.pm-docs-green.svg?style=flat">
  </a>

  <a href="https://opensource.org/licenses/MIT">
    <img alt="MIT" src="https://img.shields.io/hexpm/l/lumis">
  </a>
</div>

## Features

- **110+ Tree-sitter languages** - Fast, accurate, and updated syntax parsing
- **250+ built-in Neovim themes** - Updated and curated themes from the Neovim community
- **Built-in formatters** - HTML (inline/linked), Terminal (ANSI), Multi-theme (light/dark), BBCode
- **Custom formatters** - Build your own output
- **Language auto-detection** - File extension, shebang, and emacs-mode support
- **Line highlighting** - Mark and style individual lines, with custom HTML wrappers
- **Streaming-friendly** - Handles incomplete code
- **Load parsers on demand** - Verified and cached, including injected languages

## Installation

```elixir
def deps do
  [
    {:lumis, "~> 0.7"}
  ]
end
```

## Usage

```elixir
iex> Lumis.highlight!("Atom.to_string(:elixir)", formatter: {:html_inline, language: "elixir", theme: "github_light"})
```

The language is optional — Lumis detects it from the source, a filename, or a
shebang. The theme is optional too, but there is no default: without one,
`:html_inline` emits spans with no colors. Themes are named:
`theme: "github_light"`, or a `Lumis.Theme` struct built from your own JSON.

Formatters decide the output: `:html_inline`, `:html_linked`,
`:html_multi_themes`, `:terminal`, `:bbcode_scoped`, or your own.

## Parsers

Highlighting downloads, verifies and loads whatever a document needs, including
languages injected inside it, and caches them for every later request. Loading is
global to the VM, so only the first process pays.

```elixir
# move the download off the first request
Lumis.Languages.load(["elixir", "html", "javascript", "css"])
```

## Application startup

Warm parsers from your application's `start/2` so production does not download
or compile them on the first request:

```elixir
def start(_type, _args) do
  Lumis.Languages.async_load(~w(elixir html javascript css))
  Supervisor.start_link(children(), strategy: :one_for_one, name: MyApp.Supervisor)
end
```

It returns immediately, so the boot never waits on the network, and a failed
warm-up is logged rather than able to stop the application from starting.

See the [deployment guide](https://lumis.hexdocs.pm/deployment.html) for the
full lifecycle example, bundles, the standalone CLI, and custom cache directories.

The NIF is precompiled. Set `LUMIS_BUILD=1` to build it from source instead, or
`LUMIS_USE_LEGACY_ARTIFACTS=1` to take the legacy-CPU variant on a machine
without the newer instruction sets.

It downloads from GitHub Releases, mirrored to Cloudflare R2. Set
`config :lumis, artifact_source: :cloudflare` or `LUMIS_ARTIFACT_SOURCE=cloudflare`
to use the mirror when GitHub is down, see
[where the precompiled NIF comes from](https://lumis.sh/docs/usage/elixir#where-the-precompiled-nif-comes-from).

## Documentation

- [Elixir integration](https://lumis.sh/docs/usage/elixir) — configuration, releases, Phoenix
- [Formatters](https://lumis.sh/docs/formatters) — every formatter and its options
- [Annotations](https://lumis.sh/docs/formatters/annotations) — compose your own ranges into the event stream
- [Themes](https://lumis.sh/docs/themes) — the theme list, custom themes, CSS files
- [Languages](https://lumis.sh/docs/reference/languages) — what is supported and how detection works
- [Line highlighting](https://lumis.sh/docs/recipes/line-highlighting)
- [Recipes](https://lumis.sh/docs/recipes) — LiveView rendering, light/dark, injected languages

API reference: [hexdocs.pm/lumis](https://hexdocs.pm/lumis).

## Acknowledgements

* [Makeup](https://hex.pm/packages/makeup) for setting up the baseline and for the inspiration
* [Inkjet](https://crates.io/crates/inkjet) for the Rust implementation up to v0.2 and for the inspiration
