# Changelog

## Unreleased

### Changed

- Rename CSS class from `athl` to `lumis` for consistency with the project name
- Rename CSS class from `athl-themes` to `lumis-themes` for multi-theme formatter
- Change default CSS variable prefix from `--athl` to `--lumis`

## 0.1.0 - 2026-01-27

First release of `lumis`, a renamed version of the `autumn` package.

### Migration from autumn

Update your `mix.exs`:

```elixir
# Before
{:autumn, "~> 0.6"}

# After
{:lumis, "~> 0.1"}
```

Update your imports:

```elixir
# Before
alias Autumn
alias Autumn.Theme

# After
alias Lumis
alias Lumis.Theme
```
