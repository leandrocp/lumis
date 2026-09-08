//! Formatter implementations for generating syntax highlighted output.
//!
//! This module provides four different formatters for rendering syntax highlighted code:
//! - [`html_inline`] - HTML output with inline CSS styles (single theme)
//! - [`html_multi_themes`] - HTML output with inline CSS styles (multiple themes)
//! - [`html_linked`] - HTML output with CSS classes (requires external CSS)
//! - [`terminal`] - ANSI color codes for terminal output
//! - [`bbcode`] - `BBCode` scoped output using highlight scope names as tags
//!
//! # Builder Pattern
//!
//! Each formatter has a dedicated builder that provides a type-safe, ergonomic API:
//! - [`HtmlInlineBuilder`] - Create HTML formatters with inline CSS styles
//! - [`HtmlMultiThemesBuilder`] - Create HTML formatters with multiple theme support
//! - [`HtmlLinkedBuilder`] - Create HTML formatters with CSS classes
//! - [`TerminalBuilder`] - Create terminal formatters with ANSI colors
//! - [`BBCodeScopedBuilder`] - Create `BBCode` scoped formatters using highlight scope names as tags
//!
//! Builders are exported at the crate root for convenient access:
//! ```rust
//! use lumis::{HtmlInlineBuilder, HtmlMultiThemesBuilder, HtmlLinkedBuilder, TerminalBuilder, BBCodeScopedBuilder};
//! ```
//!
//! # Examples
//!
//! ## Using `HtmlInlineBuilder`
//!
//! ```rust
//! use lumis::{HtmlInlineBuilder, languages::Language, themes, formatters::Formatter};
//! use std::io::Write;
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! // HTML with inline styles
//! let formatter = HtmlInlineBuilder::new()
//!     .language(Language::Rust)
//!     .theme(Some(theme))
//!     .pre_class(Some("code-block".to_string()))
//!     .italic(false)
//!     .include_highlights(false)
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Using `HtmlMultiThemesBuilder`
//!
//! ```rust
//! use lumis::{HtmlMultiThemesBuilder, languages::Language, themes, formatters::Formatter};
//! use std::collections::HashMap;
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//!
//! let mut themes_map = HashMap::new();
//! themes_map.insert("light".to_string(), themes::get("github_light").unwrap());
//! themes_map.insert("dark".to_string(), themes::get("github_dark").unwrap());
//!
//! // HTML with multiple theme support using CSS variables
//! let formatter = HtmlMultiThemesBuilder::new()
//!     .language(Language::Rust)
//!     .themes(themes_map)
//!     .default_theme("light")
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Using `HtmlLinkedBuilder`
//!
//! ```rust
//! use lumis::{HtmlLinkedBuilder, languages::Language, formatters::Formatter};
//! use std::io::Write;
//!
//! let code = "<div>Hello World</div>";
//!
//! let formatter = HtmlLinkedBuilder::new()
//!     .language(Language::HTML)
//!     .pre_class(Some("my-code".to_string()))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Using `TerminalBuilder`
//!
//! ```rust
//! use lumis::{TerminalBuilder, languages::Language, themes, formatters::Formatter};
//! use std::io::Write;
//!
//! let code = "puts 'Hello from Ruby!'";
//! let theme = themes::get("github_light").unwrap();
//!
//! let formatter = TerminalBuilder::new()
//!     .language(Language::Ruby)
//!     .theme(Some(theme))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! lumis::write_highlight(&mut output, code, formatter).unwrap();
//! let ansi_output = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Line highlighting with HTML formatters
//!
//! ```rust
//! use lumis::{HtmlInlineBuilder, languages::Language, themes, formatters::Formatter};
//! use lumis::formatters::html_inline::{HighlightLines, HighlightLinesStyle};
//! use std::io::Write;
//!
//! let code = "line 1\nline 2\nline 3\nline 4";
//! let theme = themes::get("catppuccin_mocha").unwrap();
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![1..=1, 3..=4],  // Highlight lines 1, 3, and 4
//!     style: Some(HighlightLinesStyle::Theme),  // Use theme's highlighted style
//!     class: None,
//! };
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .language(Language::PlainText)
//!     .theme(Some(theme))
//!     .include_highlights(false)
//!     .highlight_lines(Some(highlight_lines))
//!     .build()
//!     .unwrap();
//! ```
//!
//! # Custom Formatters
//!
//! Implement the [`Formatter`] trait to create custom output formats.
//! Use [`highlight_iter()`](crate::highlight::highlight_iter) for streaming token access
//! and the [`html`] / [`ansi`] helper modules to build output consistently with the
//! built-in formatters.
//!
//! See the [crate examples](https://github.com/leandrocp/lumis/tree/main/crates/lumis/examples)
//! for custom formatter implementations.

// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

use crate::languages::Language;
use lumis_core::events::HighlightEvent;
use std::io::{self, Write};

pub mod ansi;
pub mod html;

pub mod html_inline;
pub use html_inline::{HtmlInline, HtmlInlineBuilder};

pub mod html_multi_themes;
pub use html_multi_themes::{HtmlMultiThemes, HtmlMultiThemesBuilder};

pub mod html_linked;
pub use html_linked::{HtmlLinked, HtmlLinkedBuilder};

pub mod terminal;
pub use terminal::Background as TerminalBackground;
pub use terminal::{Terminal, TerminalBuilder};

pub mod bbcode;
pub use bbcode::{BBCodeScoped, BBCodeScopedBuilder};

#[deprecated(note = "use `formatters::html::HtmlElement` instead")]
pub use lumis_core::formatter::HtmlElement;

pub(crate) fn map_inline_highlight_lines(
    highlight_lines: html_inline::HighlightLines,
) -> lumis_core::formatter::html_inline::HighlightLines {
    lumis_core::formatter::html_inline::HighlightLines {
        lines: highlight_lines.lines,
        style: highlight_lines.style.map(|style| match style {
            html_inline::HighlightLinesStyle::Theme => {
                lumis_core::formatter::html_inline::HighlightLinesStyle::Theme
            }
            html_inline::HighlightLinesStyle::Style(style) => {
                lumis_core::formatter::html_inline::HighlightLinesStyle::Style(style)
            }
        }),
        class: highlight_lines.class,
    }
}

/// Trait for implementing custom syntax highlighting formatters.
///
/// The `Formatter` trait allows custom output formats to consume Lumis's
/// unified syntax and annotation event stream.
///
/// For HTML formatters, see the [`html`] module for helper functions
/// that handle HTML generation, escaping, and styling.
///
/// For terminal/ANSI formatters, see the [`ansi`] module for helper functions
/// that handle ANSI escape sequences and color conversion.
///
/// # Creating Custom Formatters
///
/// Minimal formatter that writes the highlighted source:
///
/// ```rust
/// use lumis::{
///     events::HighlightEvent,
///     formatters::Formatter,
///     languages::Language,
///     write_highlight,
/// };
/// use std::io::{self, Write};
///
/// struct SourceFormatter;
///
/// impl Formatter for SourceFormatter {
///     fn language(&self) -> Language {
///         Language::Rust
///     }
///
///     fn render(
///         &self,
///         source: &str,
///         events: &[HighlightEvent<'_>],
///         output: &mut dyn Write,
///     ) -> io::Result<()> {
///         for event in events {
///             if let HighlightEvent::Source { start, end } = event {
///                 output.write_all(&source.as_bytes()[*start..*end])?;
///             }
///         }
///         Ok(())
///     }
/// }
///
/// let mut output = Vec::new();
/// write_highlight(
///     &mut output,
///     "let answer = 42;",
///     SourceFormatter,
/// )?;
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// # See Also
///
/// - [`highlight`](mod@crate::highlight) module - High-level API for accessing styled tokens
/// - [`highlight_iter()`](crate::highlight::highlight_iter) - Streaming callback API
/// - [Crate examples](https://github.com/leandrocp/lumis/tree/main/crates/lumis/examples) - Custom formatter implementations
pub trait Formatter<T = ()>: Send + Sync {
    /// Returns the source language this formatter highlights.
    fn language(&self) -> Language;

    /// Renders the unified syntax and annotation event stream.
    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()>;
}

impl<T> Formatter<T> for Box<dyn Formatter<T>> {
    fn language(&self) -> Language {
        (**self).language()
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        (**self).render(source, events, output)
    }
}

impl<T, F> Formatter<T> for &F
where
    F: Formatter<T> + ?Sized,
{
    fn language(&self) -> Language {
        (**self).language()
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        (**self).render(source, events, output)
    }
}
