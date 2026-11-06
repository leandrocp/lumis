pub(crate) mod langs;
pub(crate) mod themes;
pub(crate) mod type_;

use rune::El;

/// The three scenes back to back, each with its own clock.
pub(crate) fn reel(t: f64) -> El {
    if t < type_::LEN {
        type_::view(t)
    } else if t < type_::LEN + themes::LEN {
        themes::view(t - type_::LEN)
    } else {
        langs::view(t - type_::LEN - themes::LEN)
    }
}
