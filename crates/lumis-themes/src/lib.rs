//! Theme data shared by Lumis runtimes.

/// Returns the theme packs linked into this application.
pub fn packs() -> impl Iterator<Item = &'static str> {
    let default = std::iter::once(include_str!("../data/lumis-themes.json"));
    #[cfg(feature = "copyleft")]
    let default = default.chain(std::iter::once(lumis_themes_copyleft_data::PACK));
    default
}
