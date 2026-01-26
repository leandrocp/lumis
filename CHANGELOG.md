# Changelog

## Unreleased

### Removed

- Remove `elixir-nif` feature. The Elixir/Rustler bridge code is now maintained in the Elixir package's NIF crate.

## 0.1.0 - 2026-01-23

First release of `lumis`, a renamed and restructured version of `autumnus`.

### Migration from autumnus

Update your `Cargo.toml`:

```toml
# Before
[dependencies]
autumnus = "0.8"

# After
[dependencies]
lumis = "0.1"
```

Update your imports:

```rust
// Before
use autumnus::*;

// After
use lumis::*;
```

The API remains the same as `autumnus` v0.8.0 - only the crate and binary names have changed.

A deprecated `autumnus` v0.9.0 crate re-exports all types from `lumis` with deprecation warnings to facilitate migration.
