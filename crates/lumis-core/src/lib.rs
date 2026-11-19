//! Internal crate for lumis.
//! Use the [`lumis`](https://crates.io/crates/lumis) crate instead.

#[doc(hidden)]
pub mod annotations;
pub mod code_fence;
#[doc(hidden)]
pub mod decorations;
#[doc(hidden)]
pub mod diff_words;
#[doc(hidden)]
#[cfg(feature = "rustler")]
pub mod elixir;
#[doc(hidden)]
pub mod events;
pub mod formatter;
pub mod highlights;
pub mod languages;
pub mod themes;
