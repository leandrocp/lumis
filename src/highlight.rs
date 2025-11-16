//! Core highlighting API that abstracts away tree-sitter complexity.
//!
//! This module provides a high-level interface for accessing syntax-highlighted tokens.
//! It's particularly useful for building custom formatters.
//!
//! # For Custom Formatter Authors
//!
//! If you're implementing a custom formatter, use [`highlight_iter()`] to get styled tokens:
//!
//! ```rust,no_run
//! use autumnus::{formatter::Formatter, highlight::highlight_iter};
//! use std::io::{self, Write};
//!
//! # struct MyFormatter { language: autumnus::languages::Language, theme: Option<autumnus::themes::Theme> }
//! impl Formatter for MyFormatter {
//!     fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
//!         let iter = highlight_iter(source, self.language, self.theme.clone())
//!             .map_err(io::Error::other)?;
//!
//!         for (style, text, range) in iter {
//!             // Format tokens however you want!
//!             // style: colors and font modifiers
//!             // text: the actual source text
//!             // range: byte positions in source
//! #           let _ = (style, text, range);
//!         }
//!         Ok(())
//!     }
//! }
//! ```
//!
//! See also:
//! - [`Formatter`](crate::formatter::Formatter) trait documentation
//! - [`formatter::html`](crate::formatter::html) module for HTML-specific helpers
//! - [`formatter::ansi`](crate::formatter::ansi) module for terminal/ANSI-specific helpers
//!
//! # Architecture
//!
//! The highlighting system has two levels of abstraction:
//!
//! 1. **High-level API** - [`Highlighter`] provides stateful highlighting.
//!
//! 2. **Iterator API** - [`HighlightIterator`] provides streaming access.
//!
//! # Examples
//!
//! ## Simple highlighting
//!
//! ```rust
//! use autumnus::highlight::Highlighter;
//! use autumnus::languages::Language;
//! use autumnus::themes;
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! let mut highlighter = Highlighter::new(Language::Rust, Some(theme));
//! let segments = highlighter.highlight(code).unwrap();
//!
//! for (style, text) in segments {
//!     println!("Text: '{}', Color: {:?}", text, style.fg);
//! }
//! ```
//!
//! ## Using the iterator API for streaming
//!
//! ```rust
//! use autumnus::highlight::highlight_iter;
//! use autumnus::languages::Language;
//! use autumnus::themes;
//!
//! let code = "let x = 42;";
//! let theme = themes::get("github_light").unwrap();
//!
//! for (style, text, range) in highlight_iter(code, Language::Rust, Some(theme)).unwrap() {
//!     println!("{}..{}: '{}' with color {:?}", range.start, range.end, text, style.fg);
//! }
//! ```

use crate::constants::HIGHLIGHT_NAMES;
use crate::languages::Language;
use crate::themes::Theme;
use std::ops::Range;
use std::sync::Arc;
use tree_sitter_highlight::{HighlightEvent, Highlighter as TSHighlighter};

/// A styled segment of text.
///
/// This is the primary output type for the highlighting system. It contains
/// styling information (colors, font modifiers) that can be applied to the text.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    /// Foreground color as hex string (e.g., "#ff5555")
    pub fg: Option<String>,
    /// Background color as hex string (e.g., "#282a36")
    pub bg: Option<String>,
    /// Whether the text should be rendered in bold
    pub bold: bool,
    /// Whether the text should be rendered in italic
    pub italic: bool,
    /// Whether the text should be underlined
    pub underline: bool,
    /// Whether the text should have a strikethrough
    pub strikethrough: bool,
}

impl Style {
    /// Create a new style from a theme's style definition
    pub(crate) fn from_theme_style(theme_style: &crate::themes::Style) -> Self {
        Self {
            fg: theme_style.fg.clone(),
            bg: theme_style.bg.clone(),
            bold: theme_style.bold,
            italic: theme_style.italic,
            underline: theme_style.underline,
            strikethrough: theme_style.strikethrough,
        }
    }
}

/// High-level stateful highlighter for syntax highlighting.
///
/// This is the primary API for most users. It manages tree-sitter state internally
/// and provides simple methods for highlighting code.
///
/// # Examples
///
/// ```rust
/// use autumnus::highlight::Highlighter;
/// use autumnus::languages::Language;
/// use autumnus::themes;
///
/// let code = "fn main() {}";
/// let theme = themes::get("dracula").unwrap();
///
/// let mut highlighter = Highlighter::new(Language::Rust, Some(theme));
/// let segments = highlighter.highlight(code).unwrap();
/// ```
pub struct Highlighter {
    language: Language,
    theme: Option<Theme>,
}

impl Highlighter {
    /// Create a new highlighter for the given language and optional theme.
    ///
    /// # Arguments
    ///
    /// * `language` - The programming language to highlight
    /// * `theme` - Optional theme for styling. If None, segments will have empty styles.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use autumnus::highlight::Highlighter;
    /// use autumnus::languages::Language;
    /// use autumnus::themes;
    ///
    /// // With theme
    /// let theme = themes::get("dracula").unwrap();
    /// let highlighter = Highlighter::new(Language::Rust, Some(theme));
    ///
    /// // Without theme (styles will be empty)
    /// let highlighter = Highlighter::new(Language::JavaScript, None);
    /// ```
    pub fn new(language: Language, theme: Option<Theme>) -> Self {
        Self { language, theme }
    }

    /// Highlight the entire source code and return styled segments.
    ///
    /// This is the main entry point for highlighting. It returns a vector of
    /// (Style, &str) tuples representing styled segments of the source code.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to highlight
    ///
    /// # Returns
    ///
    /// A vector of (`Arc<Style>`, `&str`) tuples where:
    /// - `Arc<Style>` contains the styling information (colors, modifiers) in a shared reference
    /// - `&str` is a slice of the original source text
    ///
    /// # Errors
    ///
    /// Returns an error string if tree-sitter highlighting fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use autumnus::highlight::Highlighter;
    /// use autumnus::languages::Language;
    ///
    /// let code = "fn main() { println!(\"Hello\"); }";
    /// let mut highlighter = Highlighter::new(Language::Rust, None);
    ///
    /// let segments = highlighter.highlight(code).unwrap();
    /// for (style, text) in segments {
    ///     print!("{}", text);  // Print the highlighted code
    /// }
    /// ```
    pub fn highlight<'a>(&mut self, source: &'a str) -> Result<Vec<(Arc<Style>, &'a str)>, String> {
        let mut ts_highlighter = TSHighlighter::new();
        let events = ts_highlighter
            .highlight(
                self.language.config(),
                source.as_bytes(),
                None,
                |injected| Some(Language::guess(Some(injected), "").config()),
            )
            .map_err(|e| format!("Failed to generate highlight events: {:?}", e))?;

        let mut result = Vec::new();
        let mut current_style = Arc::new(Style::default());

        for event in events {
            let event = event.map_err(|e| format!("Failed to get highlight event: {:?}", e))?;

            match event {
                HighlightEvent::HighlightStart(idx) => {
                    let scope = HIGHLIGHT_NAMES[idx.0];

                    current_style = if let Some(ref theme) = self.theme {
                        Arc::new(
                            theme
                                .get_style(scope)
                                .map(Style::from_theme_style)
                                .unwrap_or_default(),
                        )
                    } else {
                        Arc::new(Style::default())
                    };
                }
                HighlightEvent::Source { start, end } => {
                    let text = &source[start..end];
                    if !text.is_empty() {
                        result.push((Arc::clone(&current_style), text));
                    }
                }
                HighlightEvent::HighlightEnd => {
                    current_style = Arc::new(Style::default());
                }
            }
        }

        Ok(result)
    }

    /// Highlight a single line of code (incremental highlighting).
    ///
    /// This method is designed for editors and other tools that need to highlight
    /// code line-by-line while maintaining parse state across lines.
    ///
    /// **Note:** This is a placeholder for future incremental highlighting support.
    /// Currently, it falls back to highlighting the entire source.
    ///
    /// # Arguments
    ///
    /// * `line` - A single line of source code to highlight
    ///
    /// # Returns
    ///
    /// A vector of (Style, &str) tuples for the styled segments in this line.
    #[allow(dead_code)]
    pub fn highlight_line<'a>(
        &mut self,
        line: &'a str,
    ) -> Result<Vec<(Arc<Style>, &'a str)>, String> {
        // For now, just highlight the entire line as a single unit
        // Future: maintain parse state across lines for true incremental highlighting
        self.highlight(line)
    }
}

/// Iterator for lazy, streaming syntax highlighting with position information.
///
/// This provides a lower-level API that yields styled segments with byte positions.
/// Note: This currently pre-computes all segments but provides an iterator interface
/// for compatibility with streaming use cases.
///
/// # Examples
///
/// ```rust
/// use autumnus::highlight::highlight_iter;
/// use autumnus::languages::Language;
///
/// let code = "let x = 42;";
///
/// for (style, text, range) in highlight_iter(code, Language::Rust, None).unwrap() {
///     println!("{}..{}: '{}'", range.start, range.end, text);
/// }
/// ```
pub struct HighlightIterator<'a> {
    segments: Vec<(Arc<Style>, &'a str, Range<usize>)>,
    index: usize,
}

impl<'a> HighlightIterator<'a> {
    /// Create a new highlight iterator.
    ///
    /// Typically you should use the [`highlight_iter`] convenience function instead.
    pub fn new(source: &'a str, language: Language, theme: Option<Theme>) -> Result<Self, String> {
        let mut ts_highlighter = TSHighlighter::new();
        let events = ts_highlighter
            .highlight(language.config(), source.as_bytes(), None, |injected| {
                Some(Language::guess(Some(injected), "").config())
            })
            .map_err(|e| format!("Failed to generate highlight events: {:?}", e))?;

        let mut segments = Vec::new();
        let mut current_style = Arc::new(Style::default());

        for event in events {
            let event = event.map_err(|e| format!("Failed to get highlight event: {:?}", e))?;

            match event {
                HighlightEvent::HighlightStart(idx) => {
                    let scope = HIGHLIGHT_NAMES[idx.0];

                    current_style = if let Some(ref theme) = theme {
                        Arc::new(
                            theme
                                .get_style(scope)
                                .map(Style::from_theme_style)
                                .unwrap_or_default(),
                        )
                    } else {
                        Arc::new(Style::default())
                    };
                }
                HighlightEvent::Source { start, end } => {
                    let text = &source[start..end];
                    if !text.is_empty() {
                        segments.push((Arc::clone(&current_style), text, start..end));
                    }
                }
                HighlightEvent::HighlightEnd => {
                    current_style = Arc::new(Style::default());
                }
            }
        }

        Ok(Self { segments, index: 0 })
    }
}

impl<'a> Iterator for HighlightIterator<'a> {
    type Item = (Arc<Style>, &'a str, Range<usize>);

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

/// Convenience function to create a highlight iterator.
///
/// This is the easiest way to get started with the iterator API.
///
/// # Arguments
///
/// * `source` - The source code to highlight
/// * `language` - The programming language
/// * `theme` - Optional theme for styling
///
/// # Returns
///
/// A `HighlightIterator` that yields (`Style`, `&str`, `Range<usize>`) tuples.
///
/// # Examples
///
/// ```rust
/// use autumnus::highlight::highlight_iter;
/// use autumnus::languages::Language;
/// use autumnus::themes;
///
/// let code = "fn main() {}";
/// let theme = themes::get("dracula").unwrap();
///
/// for (style, text, range) in highlight_iter(code, Language::Rust, Some(theme)).unwrap() {
///     println!("{}..{}: '{}' (fg: {:?})", range.start, range.end, text, style.fg);
/// }
/// ```
pub fn highlight_iter<'a>(
    source: &'a str,
    language: Language,
    theme: Option<Theme>,
) -> Result<HighlightIterator<'a>, String> {
    HighlightIterator::new(source, language, theme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes;

    #[test]
    fn test_highlighter_without_theme() {
        let code = "fn main() {}";
        let mut highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        assert!(!segments.is_empty());
        // Segments should have text but no styling
        for (style, _text) in &segments {
            assert_eq!(style.fg, None);
            assert_eq!(style.bg, None);
        }
    }

    #[test]
    fn test_highlighter_with_theme() {
        let code = "fn main() {}";
        let theme = themes::get("dracula").unwrap();
        let mut highlighter = Highlighter::new(Language::Rust, Some(theme));
        let segments = highlighter.highlight(code).unwrap();

        assert!(!segments.is_empty());

        // At least some segments should have styling
        let has_styling = segments.iter().any(|(style, _)| style.fg.is_some());
        assert!(has_styling, "Expected at least some styled segments");
    }

    #[test]
    fn test_highlight_preserves_source_text() {
        let code = "fn main() { println!(\"Hello\"); }";
        let mut highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        // Concatenating all segments should give back original code
        let reconstructed: String = segments.iter().map(|(_, text)| *text).collect();
        assert_eq!(reconstructed, code);
    }

    #[test]
    fn test_iterator_api() {
        let code = "let x = 42;";
        let iter = highlight_iter(code, Language::Rust, None).unwrap();
        let segments: Vec<_> = iter.collect();

        assert!(!segments.is_empty());

        // Check that ranges are valid
        for (_, text, range) in &segments {
            assert_eq!(&code[range.clone()], *text);
        }
    }

    #[test]
    fn test_iterator_with_theme() {
        let code = "let x = 42;";
        let theme = themes::get("github_light").unwrap();
        let iter = highlight_iter(code, Language::Rust, Some(theme)).unwrap();
        let segments: Vec<_> = iter.collect();

        assert!(!segments.is_empty());

        // At least some segments should have colors
        let has_colors = segments.iter().any(|(style, _, _)| style.fg.is_some());
        assert!(has_colors);
    }

    #[test]
    fn test_empty_source() {
        let code = "";
        let mut highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        assert!(segments.is_empty());
    }

    #[test]
    fn test_multiline_code() {
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let mut highlighter = Highlighter::new(Language::Rust, None);
        let segments = highlighter.highlight(code).unwrap();

        let reconstructed: String = segments.iter().map(|(_, text)| *text).collect();
        assert_eq!(reconstructed, code);
    }
}
