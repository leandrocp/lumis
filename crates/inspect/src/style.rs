//! Where a byte's colours come from: each capture's own theme entry, and the style the
//! terminal and HTML formatters paint once the enclosing captures fill in what the inner
//! ones leave unset.

use lumis_core::highlights::HIGHLIGHT_NAMES;
use lumis_core::themes::{Style, Theme, UnderlineStyle};
use rune::{Color, Span};

/// A capture's own entry in the theme.
pub(crate) struct Own {
    /// What the formatters look up: the scope suffixed with its language.
    pub(crate) query: String,
    /// The theme key that answered, after dropping suffixes from `query`.
    pub(crate) matched: Option<String>,
    pub(crate) style: Option<Style>,
}

pub(crate) fn own(theme: &Theme, scope_index: usize, language: &str) -> Own {
    let query = format!("{}.{language}", HIGHLIGHT_NAMES[scope_index]);
    let found = theme
        .resolve_style(&query)
        .map(|(matched, style)| (matched.to_string(), style.clone()));
    let (matched, style) = found.unzip();
    Own {
        query,
        matched,
        style,
    }
}

/// The painted style, and for each property the index of the capture it came from.
#[derive(Default)]
pub(crate) struct Effective {
    pub(crate) style: Style,
    pub(crate) fg: Option<usize>,
    pub(crate) bg: Option<usize>,
    pub(crate) bold: Option<usize>,
    pub(crate) italic: Option<usize>,
    pub(crate) underline: Option<usize>,
    pub(crate) strikethrough: Option<usize>,
}

/// `scopes` outermost first, as the highlight opened them.
pub(crate) fn effective(theme: &Theme, scopes: &[(usize, String)]) -> Effective {
    let mut out = Effective::default();
    for (index, (scope_index, language)) in scopes.iter().enumerate().rev() {
        let Some(style) = own(theme, *scope_index, language).style else {
            continue;
        };
        let decoration = &style.text_decoration;
        let first = |slot: &mut Option<usize>, set: bool| {
            if set && slot.is_none() {
                *slot = Some(index);
            }
        };
        first(&mut out.fg, style.fg.is_some());
        first(&mut out.bg, style.bg.is_some());
        first(&mut out.bold, style.bold);
        first(&mut out.italic, style.italic);
        first(
            &mut out.underline,
            decoration.underline != UnderlineStyle::None,
        );
        first(&mut out.strikethrough, decoration.strikethrough);
        out.style.inherit(&style);
    }
    out
}

pub(crate) fn color(hex: Option<&str>) -> Option<Color> {
    rune::color::hex(hex?)
}

pub(crate) fn span(
    theme: &Theme,
    bytes: std::ops::Range<usize>,
    scopes: &[(usize, String)],
) -> Span {
    let style = effective(theme, scopes).style;
    let mut span = Span::new(bytes.start, bytes.end);
    span.color = color(style.fg.as_deref());
    if let Some(bg) = color(style.bg.as_deref()) {
        span = span.bg(bg, 0.0);
    }
    if style.bold {
        span = span.bold();
    }
    if style.italic {
        span = span.italic();
    }
    if style.text_decoration.underline != UnderlineStyle::None {
        span.underline = span.color.or_else(|| color(theme.fg()));
    }
    span
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use lumis_core::highlights::HIGHLIGHT_NAMES;
    use lumis_core::themes::{Appearance, Style, Theme};

    use super::effective;

    fn scope(name: &str) -> usize {
        HIGHLIGHT_NAMES.iter().position(|n| *n == name).unwrap()
    }

    fn style(fg: Option<&str>, bg: Option<&str>, bold: bool) -> Style {
        Style {
            fg: fg.map(str::to_string),
            bg: bg.map(str::to_string),
            bold,
            ..Style::default()
        }
    }

    #[test]
    fn inner_captures_win_and_outer_ones_fill_the_rest() {
        let theme = Theme::new(
            "test".into(),
            Appearance::Dark,
            String::new(),
            BTreeMap::from([
                (
                    "comment".into(),
                    style(Some("#111111"), Some("#222222"), true),
                ),
                ("keyword".into(), style(Some("#333333"), None, false)),
                ("keyword.rust".into(), style(Some("#444444"), None, false)),
            ]),
        );
        let scopes = [
            (scope("comment"), "rust".to_string()),
            (scope("string"), "rust".to_string()),
            (scope("keyword"), "rust".to_string()),
        ];
        let painted = effective(&theme, &scopes);
        assert_eq!(painted.style.fg.as_deref(), Some("#444444"));
        assert_eq!(painted.fg, Some(2));
        assert_eq!(painted.style.bg.as_deref(), Some("#222222"));
        assert_eq!(painted.bg, Some(0));
        assert!(painted.style.bold);
        assert_eq!(painted.bold, Some(0));
        assert_eq!(painted.italic, None);

        let python = [(scope("keyword"), "python".to_string())];
        assert_eq!(
            effective(&theme, &python).style.fg.as_deref(),
            Some("#333333")
        );
    }
}
