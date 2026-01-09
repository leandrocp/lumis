//! ANSI/Terminal helpers for creating custom terminal formatters.
//!
//! This module provides utilities to make it easy to create custom terminal formatters
//! without dealing with tree-sitter or termcolor internals directly.
//!
//! # Example: Simple Terminal Formatter
//!
//! ```rust
//! use autumnus::{ansi, languages::Language, themes};
//! use std::io::Write;
//!
//! let code = "fn main() {}";
//! let theme = themes::get("dracula").ok();
//! let lang = Language::Rust;
//!
//! let mut output = Vec::new();
//! for (ansi_text, _range) in ansi::highlight_iter_with_ansi(code, lang, theme).unwrap() {
//!     write!(&mut output, "{}", ansi_text).unwrap();
//! }
//! ```
//!
//! See also:
//! - [`Formatter`](crate::formatter::Formatter) trait documentation
//! - [`examples/custom_terminal_formatter.rs`](https://github.com/leandrocp/autumnus/blob/main/examples/custom_terminal_formatter.rs)

use crate::highlight::{highlight_iter, HighlightError, Style};
use crate::languages::Language;
use crate::themes::Theme;
use std::ops::Range;

/// ANSI reset sequence to clear all formatting.
///
/// Use this to reset terminal colors and styles back to default.
pub const ANSI_RESET: &str = "\u{1b}[0m";

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
/// use autumnus::ansi::hex_to_rgb;
///
/// assert_eq!(hex_to_rgb("#ff5555"), Some((255, 85, 85)));
/// assert_eq!(hex_to_rgb("ff5555"), Some((255, 85, 85)));
/// assert_eq!(hex_to_rgb("invalid"), None);
/// ```
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
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
/// use autumnus::ansi::rgb_to_ansi;
///
/// let fg = rgb_to_ansi(255, 85, 85, false);
/// assert_eq!(fg, "\u{1b}[38;2;255;85;85m");
///
/// let bg = rgb_to_ansi(40, 42, 54, true);
/// assert_eq!(bg, "\u{1b}[48;2;40;42;54m");
/// ```
pub fn rgb_to_ansi(r: u8, g: u8, b: u8, is_background: bool) -> String {
    if is_background {
        format!("\u{1b}[48;2;{};{};{}m", r, g, b)
    } else {
        format!("\u{1b}[38;2;{};{};{}m", r, g, b)
    }
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
/// use autumnus::{ansi::style_to_ansi, highlight::Style};
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
    let mut codes = Vec::new();

    if let Some(fg) = &style.fg {
        if let Some((r, g, b)) = hex_to_rgb(fg) {
            codes.push(rgb_to_ansi(r, g, b, false));
        }
    }

    if let Some(bg) = &style.bg {
        if let Some((r, g, b)) = hex_to_rgb(bg) {
            codes.push(rgb_to_ansi(r, g, b, true));
        }
    }

    if style.bold {
        codes.push("\u{1b}[1m".to_string());
    }

    if style.italic {
        codes.push("\u{1b}[3m".to_string());
    }

    use crate::themes::UnderlineStyle;
    match style.text_decoration.underline {
        UnderlineStyle::None => {}
        UnderlineStyle::Solid => codes.push("\u{1b}[4m".to_string()),
        UnderlineStyle::Wavy => codes.push("\u{1b}[4:3m".to_string()),
        UnderlineStyle::Double => codes.push("\u{1b}[4:2m".to_string()),
        UnderlineStyle::Dotted => codes.push("\u{1b}[4:4m".to_string()),
        UnderlineStyle::Dashed => codes.push("\u{1b}[4:5m".to_string()),
    }

    if style.text_decoration.strikethrough {
        codes.push("\u{1b}[9m".to_string());
    }

    codes.join("")
}

/// Wrap text with ANSI color codes based on a Style.
///
/// Applies ANSI escape sequences to the text and adds a reset sequence at the end.
/// When the style includes a background color, resets are inserted before newlines
/// to prevent the background from extending across the entire terminal line width.
///
/// # Arguments
///
/// * `text` - The text to wrap
/// * `style` - The styling to apply
///
/// # Returns
///
/// Text wrapped with ANSI codes and reset sequence.
///
/// # Examples
///
/// ```rust
/// use autumnus::{ansi::wrap_with_ansi, highlight::Style};
///
/// let style = Style {
///     fg: Some("#8be9fd".to_string()),
///     ..Default::default()
/// };
///
/// let wrapped = wrap_with_ansi("fn", &style);
/// assert_eq!(wrapped, "\u{1b}[0m\u{1b}[38;2;139;233;253mfn\u{1b}[0m");
/// ```
pub fn wrap_with_ansi(text: &str, style: &Style) -> String {
    let ansi_codes = style_to_ansi(style);

    if ansi_codes.is_empty() {
        text.to_string()
    } else if style.bg.is_some() {
        // When there's a background color, we need to reset before newlines
        // to prevent the background from extending across the entire line width
        let mut result = String::with_capacity(text.len() + ansi_codes.len() * 2 + 10);
        result.push_str(ANSI_RESET);
        result.push_str(&ansi_codes);

        for (i, ch) in text.char_indices() {
            if ch == '\n' {
                // Reset before the newline, then output newline, then reapply style
                result.push_str(ANSI_RESET);
                result.push('\n');
                // Only reapply style if there's more content after this newline
                if i + 1 < text.len() {
                    result.push_str(&ansi_codes);
                }
            } else {
                result.push(ch);
            }
        }

        // Final reset if text doesn't end with newline
        if !text.ends_with('\n') {
            result.push_str(ANSI_RESET);
        }

        result
    } else {
        format!("{}{}{}{}", ANSI_RESET, ansi_codes, text, ANSI_RESET)
    }
}

/// Iterator over highlighted tokens with ANSI codes pre-applied.
///
/// Returns tuples of `(ansi_wrapped_text, byte_range)` for each token.
pub struct AnsiIterator<'a> {
    segments: Vec<(String, Range<usize>)>,
    index: usize,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Iterator for AnsiIterator<'a> {
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
/// use autumnus::{ansi::highlight_iter_with_ansi, languages::Language, themes};
/// use std::io::Write;
///
/// let code = "fn main() {}";
/// let theme = themes::get("dracula").ok();
///
/// let mut output = Vec::new();
/// for (ansi_text, _range) in highlight_iter_with_ansi(code, Language::Rust, theme).unwrap() {
///     write!(&mut output, "{}", ansi_text).unwrap();
/// }
/// ```
pub fn highlight_iter_with_ansi(
    source: &str,
    language: Language,
    theme: Option<Theme>,
) -> Result<AnsiIterator<'_>, HighlightError> {
    let mut segments = Vec::new();

    highlight_iter(source, language, theme, |text, range, _scope, style| {
        let wrapped = wrap_with_ansi(text, style);
        segments.push((wrapped, range));
        Ok::<_, std::io::Error>(())
    })?;

    Ok(AnsiIterator {
        segments,
        index: 0,
        _phantom: std::marker::PhantomData,
    })
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
    fn test_wrap_with_ansi() {
        let style = Style {
            fg: Some("#8be9fd".to_string()),
            ..Default::default()
        };
        let result = wrap_with_ansi("fn", &style);
        assert!(result.starts_with("\u{1b}[0m\u{1b}[38;2;139;233;253m"));
        assert!(result.contains("fn"));
        assert!(result.ends_with("\u{1b}[0m"));
    }

    #[test]
    fn test_wrap_with_ansi_empty_style() {
        let style = Style::default();
        let result = wrap_with_ansi("text", &style);
        assert_eq!(result, "text");
    }
}
