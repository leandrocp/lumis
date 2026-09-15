//! ANSI/Terminal helpers for creating custom terminal formatters.
//!
//! This module provides utilities for ANSI color code generation
//! independent of tree-sitter.

use crate::themes::{Style, UnderlineStyle};

/// ANSI reset sequence to clear all formatting.
pub const ANSI_RESET: &str = "\u{1b}[0m";

/// Convert a hex color string to RGB tuple.
///
/// A color is exactly six ASCII hex digits, optionally preceded by `#`, and
/// anything else is `None`. The digit check is what makes that true:
/// `u8::from_str_radix` accepts a leading sign, so component-wise parsing alone
/// read `"ff+79c"` as `(255, 7, 156)`.
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

/// Generate ANSI color escape sequence from RGB values.
pub fn rgb_to_ansi(r: u8, g: u8, b: u8, is_background: bool) -> String {
    if is_background {
        format!("\u{1b}[48;2;{r};{g};{b}m")
    } else {
        format!("\u{1b}[38;2;{r};{g};{b}m")
    }
}

/// Convert a Style to ANSI escape sequences.
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

/// Render text with ANSI color codes based on a Style.
pub fn paint(text: &str, style: &Style) -> String {
    let ansi_codes = style_to_ansi(style);

    if ansi_codes.is_empty() {
        text.to_string()
    } else if style.bg.is_some() {
        let mut result = String::with_capacity(text.len() + ansi_codes.len() * 2 + 10);
        result.push_str(ANSI_RESET);
        result.push_str(&ansi_codes);

        for (i, ch) in text.char_indices() {
            if ch == '\n' {
                result.push_str(ANSI_RESET);
                result.push('\n');
                if i + 1 < text.len() {
                    result.push_str(&ansi_codes);
                }
            } else {
                result.push(ch);
            }
        }

        if !text.ends_with('\n') {
            result.push_str(ANSI_RESET);
        }

        result
    } else {
        format!("{ANSI_RESET}{ansi_codes}{text}{ANSI_RESET}")
    }
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
    }

    #[test]
    fn test_hex_to_rgb_invalid() {
        assert_eq!(hex_to_rgb("invalid"), None);
        assert_eq!(hex_to_rgb("#fff"), None);
        assert_eq!(hex_to_rgb("aéaaa"), None);
        assert_eq!(hex_to_rgb(""), None);
    }

    #[test]
    fn test_hex_to_rgb_rejects_a_signed_component() {
        assert_eq!(hex_to_rgb("ff+79c"), None);
        assert_eq!(hex_to_rgb("+f+f+f"), None);
    }

    #[test]
    fn test_rgb_to_ansi_foreground() {
        assert_eq!(rgb_to_ansi(255, 85, 85, false), "\u{1b}[38;2;255;85;85m");
    }

    #[test]
    fn test_rgb_to_ansi_background() {
        assert_eq!(rgb_to_ansi(40, 42, 54, true), "\u{1b}[48;2;40;42;54m");
    }

    #[test]
    fn test_style_to_ansi_with_bold() {
        let style = Style {
            bold: true,
            ..Default::default()
        };
        assert_eq!(style_to_ansi(&style), "\u{1b}[1m");
    }

    #[test]
    fn test_paint_empty_style() {
        let style = Style::default();
        assert_eq!(paint("text", &style), "text");
    }

    #[test]
    fn test_paint_fg() {
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
    fn test_paint_multiline_without_background() {
        let style = Style {
            fg: Some("#8be9fd".to_string()),
            ..Default::default()
        };
        let result = paint("a\nb", &style);

        assert_eq!(result, "\u{1b}[0m\u{1b}[38;2;139;233;253ma\nb\u{1b}[0m");
    }

    #[test]
    fn test_paint_multiline_with_background_resets_before_newline() {
        let style = Style {
            fg: Some("#8be9fd".to_string()),
            bg: Some("#282a36".to_string()),
            ..Default::default()
        };
        let result = paint("a\nb", &style);

        assert_eq!(
            result,
            "\u{1b}[0m\u{1b}[38;2;139;233;253m\u{1b}[48;2;40;42;54ma\u{1b}[0m\n\u{1b}[38;2;139;233;253m\u{1b}[48;2;40;42;54mb\u{1b}[0m"
        );
    }
}
