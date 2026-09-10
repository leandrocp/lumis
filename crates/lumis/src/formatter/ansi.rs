//! ANSI/Terminal helpers for creating custom terminal formatters.
//!
//! This module provides utilities to make it easy to create custom terminal formatters
//! without dealing with tree-sitter or termcolor internals directly.
//!
//! # Example: Simple Terminal Formatter
//!
//! ```rust
//! use lumis::{ansi::paint, highlight::highlight_iter, languages::Language, themes};
//! use std::io::Write;
//!
//! let code = "fn main() {}";
//! let theme = themes::get("dracula").ok();
//! let lang = Language::Rust;
//!
//! let mut output = Vec::new();
//! highlight_iter(code, lang, theme, |text, _language, _range, _scope, style| {
//!     write!(&mut output, "{}", paint(text, style))
//! })
//! .unwrap();
//! ```
//!
//! See also:
//! - [`Formatter`](crate::formatters::Formatter) trait documentation
//! - [`crates/lumis/examples/custom_terminal_formatter.rs`](https://github.com/leandrocp/lumis/blob/main/crates/lumis/examples/custom_terminal_formatter.rs)

use crate::highlight::{highlight_iter, HighlightError, Style};
use crate::languages::Language;
use crate::themes::Theme;
use std::ops::Range;

/// ANSI reset sequence to clear all formatting.
///
/// Use this to reset terminal colors and styles back to default.
pub const ANSI_RESET: &str = lumis_core::formatter::ansi::ANSI_RESET;

/// Convert a hex color string to RGB tuple.
///
/// # Arguments
///
/// * `hex` - Hex color string (with or without '#' prefix)
///
/// # Returns
///
/// `Some((r, g, b))` tuple of u8 values if parsing succeeds, `None` otherwise.
///
/// # Examples
///
/// ```rust
/// use lumis::ansi::hex_to_rgb;
///
/// assert_eq!(hex_to_rgb("#ff5555"), Some((255, 85, 85)));
/// assert_eq!(hex_to_rgb("ff5555"), Some((255, 85, 85)));
/// assert_eq!(hex_to_rgb("invalid"), None);
/// ```
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    lumis_core::formatter::ansi::hex_to_rgb(hex)
}

/// Generate ANSI color escape sequence from RGB values.
///
/// # Arguments
///
/// * `r, g, b` - RGB color components (0-255)
/// * `is_background` - true for background color, false for foreground
///
/// # Returns
///
/// ANSI escape sequence string for the specified color.
///
/// # Examples
///
/// ```rust
/// use lumis::ansi::rgb_to_ansi;
///
/// let fg = rgb_to_ansi(255, 85, 85, false);
/// assert_eq!(fg, "\u{1b}[38;2;255;85;85m");
///
/// let bg = rgb_to_ansi(40, 42, 54, true);
/// assert_eq!(bg, "\u{1b}[48;2;40;42;54m");
/// ```
pub fn rgb_to_ansi(r: u8, g: u8, b: u8, is_background: bool) -> String {
    lumis_core::formatter::ansi::rgb_to_ansi(r, g, b, is_background)
}

/// Convert a Style to ANSI escape sequences.
///
/// Combines all style attributes (foreground, background, bold, italic, etc.)
/// into a single ANSI escape sequence string.
///
/// # Arguments
///
/// * `style` - The Style to convert
///
/// # Returns
///
/// ANSI escape sequence string combining all style attributes.
///
/// # Examples
///
/// ```rust
/// use lumis::{ansi::style_to_ansi, highlight::Style};
///
/// let style = Style {
///     fg: Some("#ff79c6".to_string()),
///     bg: Some("#282a36".to_string()),
///     bold: true,
///     italic: false,
///     ..Default::default()
/// };
///
/// let ansi = style_to_ansi(&style);
/// ```
pub fn style_to_ansi(style: &Style) -> String {
    lumis_core::formatter::ansi::style_to_ansi(style)
}

/// Render text with ANSI color codes based on a Style.
///
/// Applies ANSI escape sequences to the text and adds a reset sequence at the end.
/// When the style includes a background color, resets are inserted before newlines
/// to prevent the background from extending across the entire terminal line width.
///
/// # Arguments
///
/// * `text` - The text to render
/// * `style` - The styling to apply
///
/// # Returns
///
/// Text rendered with ANSI codes and reset sequence.
///
/// # Examples
///
/// ```rust
/// use lumis::{ansi::paint, highlight::Style};
///
/// let style = Style {
///     fg: Some("#8be9fd".to_string()),
///     ..Default::default()
/// };
///
/// let painted = paint("fn", &style);
/// assert_eq!(painted, "\u{1b}[0m\u{1b}[38;2;139;233;253mfn\u{1b}[0m");
/// ```
pub fn paint(text: &str, style: &Style) -> String {
    lumis_core::formatter::ansi::paint(text, style)
}

/// Wrap text with ANSI color codes based on a Style.
///
/// Deprecated: use [`paint()`] instead.
#[deprecated(note = "use `paint(...)` instead")]
pub fn wrap_with_ansi(text: &str, style: &Style) -> String {
    paint(text, style)
}

/// Iterator over highlighted tokens with ANSI codes pre-applied.
///
/// Returns tuples of `(ansi_wrapped_text, byte_range)` for each token.
pub struct AnsiIterator {
    segments: Vec<(String, Range<usize>)>,
    index: usize,
}

impl Iterator for AnsiIterator {
    type Item = (String, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.segments.len() {
            let result = self.segments[self.index].clone();
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

/// Create an iterator over highlighted tokens with ANSI codes pre-applied.
///
/// This is the most convenient way to build custom terminal formatters.
/// Each token is pre-wrapped with appropriate ANSI escape sequences based
/// on the theme and includes a reset sequence.
///
/// Deprecated: use [`highlight_iter()`](crate::highlight::highlight_iter)
/// with [`paint()`] instead.
///
/// # Arguments
///
/// * `source` - The source code to highlight
/// * `language` - The programming language
/// * `theme` - Optional theme for styling
///
/// # Returns
///
/// An `AnsiIterator` yielding `(ansi_wrapped_text, Range<usize>)` tuples.
///
/// # Examples
///
/// ```rust
/// use lumis::{ansi::paint, highlight::highlight_iter, languages::Language, themes};
/// use std::io::Write;
///
/// let code = "fn main() {}";
/// let theme = themes::get("dracula").ok();
///
/// let mut output = Vec::new();
/// highlight_iter(code, Language::Rust, theme, |text, _language, _range, _scope, style| {
///     write!(&mut output, "{}", paint(text, style))
/// })
/// .unwrap();
/// ```
#[deprecated(note = "use `highlight::highlight_iter(...)` with `ansi::paint(...)` instead")]
pub fn highlight_iter_with_ansi(
    source: &str,
    language: Language,
    theme: Option<Theme>,
) -> Result<AnsiIterator, HighlightError> {
    let mut segments = Vec::new();

    highlight_iter(
        source,
        language,
        theme,
        |text, _language, range, _scope, style| {
            let wrapped = paint(text, style);
            segments.push((wrapped, range));
            Ok::<_, std::io::Error>(())
        },
    )?;

    Ok(AnsiIterator { segments, index: 0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb_with_hash() {
        assert_eq!(hex_to_rgb("#ff5555"), Some((255, 85, 85)));
        assert_eq!(hex_to_rgb("#000000"), Some((0, 0, 0)));
        assert_eq!(hex_to_rgb("#ffffff"), Some((255, 255, 255)));
    }

    #[test]
    fn test_hex_to_rgb_without_hash() {
        assert_eq!(hex_to_rgb("ff5555"), Some((255, 85, 85)));
        assert_eq!(hex_to_rgb("8be9fd"), Some((139, 233, 253)));
    }

    #[test]
    fn test_hex_to_rgb_invalid() {
        assert_eq!(hex_to_rgb("invalid"), None);
        assert_eq!(hex_to_rgb("#fff"), None);
        assert_eq!(hex_to_rgb("aéaaa"), None);
        assert_eq!(hex_to_rgb(""), None);
        assert_eq!(hex_to_rgb("#gggggg"), None);
    }

    #[test]
    fn test_rgb_to_ansi_foreground() {
        let result = rgb_to_ansi(255, 85, 85, false);
        assert_eq!(result, "\u{1b}[38;2;255;85;85m");
    }

    #[test]
    fn test_rgb_to_ansi_background() {
        let result = rgb_to_ansi(40, 42, 54, true);
        assert_eq!(result, "\u{1b}[48;2;40;42;54m");
    }

    #[test]
    fn test_style_to_ansi_with_fg_only() {
        let style = Style {
            fg: Some("#ff79c6".to_string()),
            ..Default::default()
        };
        let result = style_to_ansi(&style);
        assert!(result.contains("\u{1b}[38;2;255;121;198m"));
    }

    #[test]
    fn test_style_to_ansi_with_bold() {
        let style = Style {
            bold: true,
            ..Default::default()
        };
        let result = style_to_ansi(&style);
        assert_eq!(result, "\u{1b}[1m");
    }

    #[test]
    fn test_style_to_ansi_with_multiple() {
        let style = Style {
            fg: Some("#ff5555".to_string()),
            bold: true,
            italic: true,
            ..Default::default()
        };
        let result = style_to_ansi(&style);
        assert!(result.contains("\u{1b}[38;2;255;85;85m"));
        assert!(result.contains("\u{1b}[1m"));
        assert!(result.contains("\u{1b}[3m"));
    }

    #[test]
    fn test_paint() {
        let style = Style {
            fg: Some("#8be9fd".to_string()),
            ..Default::default()
        };
        let result = paint("fn", &style);
        assert!(result.starts_with("\u{1b}[0m\u{1b}[38;2;139;233;253m"));
        assert!(result.contains("fn"));
        assert!(result.ends_with("\u{1b}[0m"));
    }

    #[test]
    fn test_paint_empty_style() {
        let style = Style::default();
        let result = paint("text", &style);
        assert_eq!(result, "text");
    }
}
