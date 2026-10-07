use super::{available_themes, Appearance, Theme};
use crate::formatter::ansi::hex_to_rgb;
use std::collections::BTreeSet;

type Rgb = (u8, u8, u8);

const CORE_SCOPES: [&str; 10] = [
    "keyword",
    "string",
    "comment",
    "function",
    "type",
    "constant",
    "number",
    "variable",
    "operator",
    "punctuation",
];
const MAX_BACKGROUND_DISTANCE: u64 = 6_400;
const BACKGROUND_TOLERANCE: u64 = 400;

/// Colors reported by a terminal, in eight-bit RGB.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct TerminalColors {
    /// The background color. Required for automatic theme selection.
    pub background: Option<Rgb>,
    /// The default text color, when available.
    pub foreground: Option<Rgb>,
    /// ANSI palette entries 0–15. Selection uses the accent colors at 1–6.
    pub ansi: [Option<Rgb>; 16],
}

/// Choose a built-in theme from the colors reported by a terminal.
///
/// Returns `None` without a background or a sufficiently close theme whose
/// default text has at least 3:1 contrast against that background.
/// Monochrome themes (at most three distinct core syntax colors) are excluded.
/// Appearance follows foreground versus background luminance, or whichever of
/// black and white contrasts better when the foreground is absent or identical.
///
/// Backgrounds must be within 6,400 squared redmean distance. With foreground
/// or ANSI accents available, candidates within 400 of the nearest background
/// compete by foreground distance, then ANSI 1–6 distance to syntax colors,
/// then background distance and name. Otherwise the closest background wins.
/// These distances correspond to about 27 and 7 levels per RGB channel on gray.
pub fn choose_theme(colors: &TerminalColors) -> Option<&'static Theme> {
    let background = colors.background?;
    let appearance = appearance(colors, background);
    let candidates: Vec<_> = available_themes()
        .filter_map(|theme| Candidate::new(theme, background, appearance))
        .collect();
    let nearest = candidates.iter().map(|c| c.background_distance).min()?;
    let tolerance = if colors.foreground.is_some() || colors.ansi[1..7].iter().any(Option::is_some)
    {
        BACKGROUND_TOLERANCE
    } else {
        0
    };
    candidates
        .into_iter()
        .filter(|c| c.background_distance <= nearest + tolerance)
        .min_by_key(|c| {
            (
                colors
                    .foreground
                    .map_or(0, |fg| color_distance(fg, c.foreground)),
                palette_distance(colors, c.theme),
                c.background_distance,
                &c.theme.name,
            )
        })
        .map(|c| c.theme)
}

struct Candidate<'a> {
    theme: &'a Theme,
    foreground: Rgb,
    background_distance: u64,
}

impl<'a> Candidate<'a> {
    fn new(theme: &'a Theme, background: Rgb, appearance: Appearance) -> Option<Self> {
        let foreground = hex_to_rgb(theme.fg()?)?;
        let background_distance = color_distance(background, hex_to_rgb(theme.bg()?)?);
        if theme.appearance != appearance
            || background_distance > MAX_BACKGROUND_DISTANCE
            || core_colors(theme).len() <= 3
            || contrast(foreground, background) < 3.0
        {
            return None;
        }
        Some(Self {
            theme,
            foreground,
            background_distance,
        })
    }
}

fn core_colors(theme: &Theme) -> BTreeSet<Rgb> {
    CORE_SCOPES
        .iter()
        .filter_map(|scope| {
            theme
                .get_style(scope)
                .and_then(|style| style.fg.as_deref())
                .or_else(|| theme.fg())
                .and_then(hex_to_rgb)
        })
        .collect()
}

fn palette_distance(colors: &TerminalColors, theme: &Theme) -> u64 {
    let syntax: Vec<_> = theme
        .highlights
        .iter()
        .filter(|(scope, _)| scope.as_str() != "normal")
        .filter_map(|(_, style)| style.fg.as_deref().and_then(hex_to_rgb))
        .collect();
    colors.ansi[1..7]
        .iter()
        .flatten()
        .map(|accent| {
            syntax
                .iter()
                .map(|color| color_distance(*accent, *color))
                .min()
                .unwrap_or(u64::MAX / 6)
        })
        .sum()
}

fn appearance(colors: &TerminalColors, background: Rgb) -> Appearance {
    let bg = luminance(background);
    let dark = colors
        .foreground
        .map(luminance)
        .filter(|fg| (fg - bg).abs() > f64::EPSILON)
        .map_or_else(
            || contrast((255, 255, 255), background) >= contrast((0, 0, 0), background),
            |fg| fg > bg,
        );
    if dark {
        Appearance::Dark
    } else {
        Appearance::Light
    }
}

fn luminance(color: Rgb) -> f64 {
    fn linear(channel: u8) -> f64 {
        let value = f64::from(channel) / 255.0;
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linear(color.0) + 0.7152 * linear(color.1) + 0.0722 * linear(color.2)
}

fn contrast(left: Rgb, right: Rgb) -> f64 {
    let left = luminance(left);
    let right = luminance(right);
    (left.max(right) + 0.05) / (left.min(right) + 0.05)
}

fn color_distance(left: Rgb, right: Rgb) -> u64 {
    let red_mean = u64::midpoint(u64::from(left.0), u64::from(right.0));
    let red = i64::from(left.0) - i64::from(right.0);
    let green = i64::from(left.1) - i64::from(right.1);
    let blue = i64::from(left.2) - i64::from(right.2);

    (((512 + red_mean) * red.unsigned_abs().pow(2)) >> 8)
        + 4 * green.unsigned_abs().pow(2)
        + (((767 - red_mean) * blue.unsigned_abs().pow(2)) >> 8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Case {
        name: String,
        background: Option<String>,
        foreground: Option<String>,
        ansi: Option<[Option<String>; 16]>,
        expected: Option<String>,
    }

    fn color(hex: Option<&str>) -> Option<Rgb> {
        hex.map(|hex| hex_to_rgb(hex).expect("valid fixture RGB"))
    }

    #[test]
    fn shared_theme_choice_cases() {
        let cases: Vec<Case> =
            serde_json::from_str(include_str!("../../../../fixtures/theme-choice-cases.json"))
                .unwrap();
        assert!(cases.len() >= 20);
        let mut failures = Vec::new();
        for case in cases {
            let colors = TerminalColors {
                background: color(case.background.as_deref()),
                foreground: color(case.foreground.as_deref()),
                ansi: case.ansi.unwrap_or_default().map(|c| color(c.as_deref())),
            };
            let actual = choose_theme(&colors).map(|theme| theme.name.as_str());
            if actual != case.expected.as_deref() {
                failures.push(format!(
                    "{}: {actual:?}, expected {:?}",
                    case.name, case.expected
                ));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn excluded_monochrome_themes_are_pinned() {
        let expected: Vec<String> = serde_json::from_str(include_str!(
            "../../../../fixtures/theme-choice-excluded.json"
        ))
        .unwrap();
        let actual: Vec<_> = available_themes()
            .filter(|theme| core_colors(theme).len() <= 3)
            .map(|theme| theme.name.clone())
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn foreground_controls_appearance_and_low_contrast_is_rejected() {
        let mut colors = TerminalColors {
            background: Some((102, 102, 102)),
            ..Default::default()
        };
        assert_eq!(
            appearance(&colors, colors.background.unwrap()),
            Appearance::Dark
        );
        colors.foreground = Some((0, 0, 0));
        assert_eq!(
            appearance(&colors, colors.background.unwrap()),
            Appearance::Light
        );
        colors.foreground = colors.background;
        assert_eq!(
            appearance(&colors, colors.background.unwrap()),
            Appearance::Dark
        );
        assert_eq!(
            appearance(&TerminalColors::default(), (0, 153, 255)),
            Appearance::Light
        );

        let mut theme = super::super::get("tokyonight_night").unwrap();
        assert!(Candidate::new(&theme, (26, 27, 38), Appearance::Light).is_none());
        theme.highlights.get_mut("normal").unwrap().fg = Some("#1a1b26".to_string());
        assert!(Candidate::new(&theme, (26, 27, 38), Appearance::Dark).is_none());
    }
}
