# Changelog

## [0.2.0] - Unreleased

First release of `lumis`, a renamed and restructured version of `autumnus`.

### Changed

- Crate renamed from `autumnus` to `lumis`
- CLI binary renamed from `autumnus` to `lumis`
- Restructured as Cargo workspace

### Migration from autumnus

Update your `Cargo.toml`:

```toml
# Before
[dependencies]
autumnus = "0.8"

# After
[dependencies]
lumis = "0.2"
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
