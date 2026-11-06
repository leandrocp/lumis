pub(crate) mod annotate;
pub(crate) mod langs;
pub(crate) mod themes;
pub(crate) mod type_;

use rune::El;

/// The scenes back to back, each with its own clock.
pub(crate) fn reel(t: f64) -> El {
    let annotate_at = type_::LEN + themes::LEN;
    if t < type_::LEN {
        type_::view(t)
    } else if t < annotate_at {
        themes::view(t - type_::LEN)
    } else if t < annotate_at + annotate::LEN {
        annotate::view(t - annotate_at)
    } else {
        langs::view(t - annotate_at - annotate::LEN)
    }
}
