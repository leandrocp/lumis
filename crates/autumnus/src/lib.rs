//! # DEPRECATED
//!
//! This crate has been renamed to [`lumis`](https://crates.io/crates/lumis).
//! Please update your dependencies.
//!
//! ## Migration
//!
//! Update your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! # Before
//! autumnus = "0.8"
//!
//! # After
//! lumis = "0.2"
//! ```
//!
//! Update your imports:
//!
//! ```ignore
//! // Before
//! use autumnus::*;
//!
//! // After
//! use lumis::*;
//! ```

#![deprecated(
    since = "0.9.0",
    note = "autumnus has been renamed to lumis. Replace `autumnus` with `lumis` in your Cargo.toml."
)]

pub use lumis::*;
