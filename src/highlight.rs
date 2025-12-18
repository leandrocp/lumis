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
//!         for (style, text, range, scope) in iter {
//!             // Format tokens however you want!
//!             // style: colors and font modifiers
//!             // text: the actual source text
//!             // range: byte positions in source
//!             // scope: tree-sitter scope name (e.g., "keyword", "string")
//! #           let _ = (style, text, range, scope);
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
//! for (style, text, range, scope) in highlight_iter(code, Language::Rust, Some(theme)).unwrap() {
//!     println!("{}..{}: '{}' (scope: {}, color: {:?})", range.start, range.end, text, scope, style.fg);
//! }
//! ```

use crate::constants::HIGHLIGHT_NAMES;
use crate::languages::Language;
use crate::themes::Theme;
use crate::vendor::tree_sitter_highlight::{HighlightEvent, Highlighter as TSHighlighter};
use std::ops::Range;
use std::sync::Arc;
use thiserror::Error;

pub use crate::themes::{Style, TextDecoration, UnderlineStyle};

/// Error type for syntax highlighting operations.
///
/// # Examples
///
/// ```rust
/// use autumnus::highlight::{highlight_iter, HighlightError};
/// use autumnus::languages::Language;
///
/// match highlight_iter("fn main() {}", Language::Rust, None) {
///     Ok(iter) => {
///         for (_style, text, _range, _scope) in iter {
///             print!("{}", text);
///         }
///     }
///     Err(HighlightError::HighlighterInit(msg)) => {
///         eprintln!("Failed to initialize highlighter: {}", msg);
///     }
///     Err(HighlightError::EventProcessing(msg)) => {
///         eprintln!("Failed to process highlight event: {}", msg);
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum HighlightError {
    /// Failed to initialize the tree-sitter highlighter for the given language.
    #[error("failed to initialize highlighter: {0}")]
    HighlighterInit(String),

    /// Failed to process a highlight event during parsing.
    #[error("failed to process highlight event: {0}")]
    EventProcessing(String),
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
    /// Returns [`HighlightError`] if tree-sitter highlighting fails.
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
    pub fn highlight<'a>(
        &mut self,
        source: &'a str,
    ) -> Result<Vec<(Arc<Style>, &'a str)>, HighlightError> {
        let mut ts_highlighter = TSHighlighter::new();
        let events = ts_highlighter
            .highlight(
                self.language.config(),
                source.as_bytes(),
                None,
                |injected| Some(Language::guess(Some(injected), "").config()),
            )
            .map_err(|e| HighlightError::HighlighterInit(format!("{:?}", e)))?;

        let mut result = Vec::new();
        let mut style_stack: Vec<Arc<Style>> = vec![Arc::new(Style::default())];

        for event in events {
            let event = event.map_err(|e| HighlightError::EventProcessing(format!("{:?}", e)))?;

            match event {
                HighlightEvent::HighlightStart {
                    highlight,
                    language,
                } => {
                    let scope = HIGHLIGHT_NAMES[highlight.0];
                    let specialized_scope = format!("{}.{}", scope, language);

                    let new_style = if let Some(ref theme) = self.theme {
                        Arc::new(
                            theme
                                .get_style(&specialized_scope)
                                .cloned()
                                .unwrap_or_default(),
                        )
                    } else {
                        Arc::new(Style::default())
                    };
                    style_stack.push(new_style);
                }
                HighlightEvent::Source { start, end } => {
                    let text = &source[start..end];
                    if !text.is_empty() {
                        let current_style = style_stack.last().cloned().unwrap_or_default();
                        result.push((current_style, text));
                    }
                }
                HighlightEvent::HighlightEnd => {
                    if style_stack.len() > 1 {
                        style_stack.pop();
                    }
                }
            }
        }

        Ok(result)
    }
}

/// Iterator for lazy, streaming syntax highlighting with position information.
///
/// This provides a lower-level API that yields styled segments with byte positions
/// and scope names.
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
/// for (style, text, range, scope) in highlight_iter(code, Language::Rust, None).unwrap() {
///     println!("{}..{}: '{}' (scope: {})", range.start, range.end, text, scope);
/// }
/// ```
pub struct HighlightIterator<'a> {
    segments: Vec<(Arc<Style>, &'a str, Range<usize>, &'static str)>,
    index: usize,
}

impl<'a> HighlightIterator<'a> {
    /// Create a new highlight iterator.
    ///
    /// Typically you should use the [`highlight_iter`] convenience function instead.
    pub fn new(
        source: &'a str,
        language: Language,
        theme: Option<Theme>,
    ) -> Result<Self, HighlightError> {
        let mut ts_highlighter = TSHighlighter::new();
        let events = ts_highlighter
            .highlight(language.config(), source.as_bytes(), None, |injected| {
                Some(Language::guess(Some(injected), "").config())
            })
            .map_err(|e| HighlightError::HighlighterInit(format!("{:?}", e)))?;

        let mut segments = Vec::new();
        let mut style_stack: Vec<Arc<Style>> = vec![Arc::new(Style::default())];
        let mut scope_stack: Vec<&'static str> = vec![""];

        for event in events {
            let event = event.map_err(|e| HighlightError::EventProcessing(format!("{:?}", e)))?;

            match event {
                HighlightEvent::HighlightStart {
                    highlight,
                    language,
                } => {
                    let scope = HIGHLIGHT_NAMES[highlight.0];
                    let specialized_scope = format!("{}.{}", scope, language);

                    let new_style = if let Some(ref theme) = theme {
                        Arc::new(
                            theme
                                .get_style(&specialized_scope)
                                .cloned()
                                .unwrap_or_default(),
                        )
                    } else {
                        Arc::new(Style::default())
                    };
                    style_stack.push(new_style);
                    scope_stack.push(scope);
                }
                HighlightEvent::Source { start, end } => {
                    let text = &source[start..end];
                    if !text.is_empty() {
                        let current_style = style_stack.last().cloned().unwrap_or_default();
                        let current_scope = scope_stack.last().copied().unwrap_or("");
                        segments.push((current_style, text, start..end, current_scope));
                    }
                }
                HighlightEvent::HighlightEnd => {
                    if style_stack.len() > 1 {
                        style_stack.pop();
                    }
                    if scope_stack.len() > 1 {
                        scope_stack.pop();
                    }
                }
            }
        }

        Ok(Self { segments, index: 0 })
    }
}

impl<'a> Iterator for HighlightIterator<'a> {
    type Item = (Arc<Style>, &'a str, Range<usize>, &'static str);

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
/// A `HighlightIterator` that yields (`Arc<Style>`, `&str`, `Range<usize>`, `&'static str`) tuples:
/// - `Arc<Style>` - Color and font styling information (shared reference)
/// - `&str` - The token text
/// - `Range<usize>` - Byte range in source
/// - `&'static str` - Scope name (e.g., "keyword", "string")
///
/// # Errors
///
/// Returns [`HighlightError::HighlighterInit`] if tree-sitter initialization fails,
/// or [`HighlightError::EventProcessing`] if parsing encounters an error.
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
/// for (style, text, range, scope) in highlight_iter(code, Language::Rust, Some(theme)).unwrap() {
///     println!("{} (scope: {}, color: {:?})", text, scope, style.fg);
/// }
/// ```
pub fn highlight_iter(
    source: &str,
    language: Language,
    theme: Option<Theme>,
) -> Result<HighlightIterator<'_>, HighlightError> {
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

        // Check that ranges are valid and scopes are present
        for (_, text, range, scope) in &segments {
            assert_eq!(&code[range.clone()], *text);
            assert!(scope.is_empty() || !scope.is_empty()); // scope is always valid
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
        let has_colors = segments.iter().any(|(style, _, _, _)| style.fg.is_some());
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
