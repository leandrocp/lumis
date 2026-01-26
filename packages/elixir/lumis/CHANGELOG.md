# Changelog

## Unreleased

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
