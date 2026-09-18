//! Internal crate for lumis.
//! Use the [`lumis`](https://crates.io/crates/lumis) crate instead.

#[doc(hidden)]
pub mod annotations;
pub(crate) mod decorations;
#[doc(hidden)]
#[cfg(feature = "rustler")]
pub mod elixir;
#[doc(hidden)]
pub mod events;
pub mod formatter;
pub mod highlights;
pub mod languages;
pub mod themes;
