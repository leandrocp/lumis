# autumnus (DEPRECATED)

**This crate has been renamed to [`lumis`](https://crates.io/crates/lumis).**

## Migration Guide

1. Update your `Cargo.toml`:

```toml
[dependencies]
# Before
autumnus = "0.8"

# After
lumis = "0.1"
```

2. Update your imports:

```rust
// Before
use autumnus::*;

// After
use lumis::*;
```

The API remains the same - only the crate name has changed.

## Why the rename?

The project has been renamed to `lumis` (Latin for "light") to better reflect its purpose as a syntax highlighting library.
