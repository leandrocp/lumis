//! Formatter implementations for generating syntax highlighted output.
//!
//! This module re-exports `lumis-core`'s formatters for rendering syntax highlighted code:
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
//! The trait is [`lumis_core::formatter::Formatter`], so the same implementation
//! serves both crates: pass it to [`write_highlight()`](crate::write_highlight),
//! which parses `source` with the grammar
//! [`Formatter::language()`](lumis_core::formatter::Formatter::language) names, or
//! render your own event stream with it directly.
//!
//! ```rust
//! use lumis::{
//!     events::HighlightEvent,
//!     formatters::Formatter,
//!     languages::Language,
//!     write_highlight,
//! };
//! use std::io::{self, Write};
//!
//! struct SourceFormatter;
//!
//! impl Formatter for SourceFormatter {
//!     fn language(&self) -> Language {
//!         Language::Rust
//!     }
//!
//!     fn render(
//!         &self,
//!         source: &str,
//!         events: &[HighlightEvent<'_>],
//!         output: &mut dyn Write,
//!     ) -> io::Result<()> {
//!         for event in events {
//!             if let HighlightEvent::Source { start, end } = event {
//!                 output.write_all(&source.as_bytes()[*start..*end])?;
//!             }
//!         }
//!         Ok(())
//!     }
//! }
//!
//! let mut output = Vec::new();
//! write_highlight(&mut output, "let answer = 42;", SourceFormatter)?;
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! See the [crate examples](https://github.com/leandrocp/lumis/tree/main/crates/lumis/examples)
//! for custom formatter implementations.

// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

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

/// Trait for implementing custom syntax highlighting formatters.
///
/// The `Formatter` trait allows custom output formats to consume Lumis's
/// unified syntax and annotation event stream. It is defined in `lumis-core`
/// and re-exported here, so one implementation serves both crates.
///
/// For HTML formatters, see the [`html`] module for helper functions
/// that handle HTML generation, escaping, and styling.
///
/// For terminal/ANSI formatters, see the [`ansi`] module for helper functions
/// that handle ANSI escape sequences and color conversion.
///
/// For output built line by line in any other format, [`lines_from_events`]
/// splits the event stream into lines of tokens.
///
/// See the [module docs](self#custom-formatters) for a worked example.
///
/// # See Also
///
/// - [`highlight`](mod@crate::highlight) module - High-level API for accessing styled tokens
/// - [`highlight_iter()`](crate::highlight::highlight_iter) - Streaming callback API
/// - [Crate examples](https://github.com/leandrocp/lumis/tree/main/crates/lumis/examples) - Custom formatter implementations
pub use lumis_core::formatter::Formatter;

/// One line of [`lines_from_events`]: its number, its tokens, and the data of
/// the annotations touching it.
pub use lumis_core::formatter::Line;

/// A run of one line's text under one scope, with its byte range, scope and
/// language.
pub use lumis_core::formatter::Token;

/// Split highlight events into lines of tokens, with the annotations each line
/// touches.
///
/// These are the lines [`html::render_lines_from_events`] renders, as data, for
/// output that is not an HTML string: content only, a final newline adds no
/// line, and an empty source is one empty line. A scope that crosses a newline
/// gives one token on each line, and a token's `range` leaves the terminator
/// out.
///
/// A token's `scope` is the innermost open one, or `""` outside every scope. A
/// rainbow bracket reports `punctuation.bracket.rainbow.N`. Nothing here
/// resolves a style: look one up with
/// [`Theme::get_style`](crate::themes::Theme::get_style) on
/// `format!("{scope}.{language}")`, which falls back to the parent scopes the
/// built-in formatters fall back to. Indexing the theme's highlights by `scope`
/// alone misses those.
///
/// A line lists the data of every annotation covering any part of it, once
/// each, in the order they open. A point annotation lands on the line holding
/// it, including a blank one, and a point at the very end of a source that ends
/// in a newline lands on the last line.
///
/// # Example
///
/// ```rust
/// use lumis::{events::HighlightEvent, formatters, highlights::HIGHLIGHT_NAMES};
///
/// let source = "/* a\nb */";
/// let comment = HIGHLIGHT_NAMES.iter().position(|&s| s == "comment").unwrap();
/// let review = "review";
/// let events: Vec<HighlightEvent<'_, &str>> = vec![
///     HighlightEvent::AnnotationStart { range: 0..source.len(), data: &review },
///     HighlightEvent::Start { scope_index: comment, language: "rust".to_string() },
///     HighlightEvent::Source { start: 0, end: source.len() },
///     HighlightEvent::End,
///     HighlightEvent::AnnotationEnd,
/// ];
///
/// let lines = formatters::lines_from_events(source, &events);
///
/// assert_eq!(lines.len(), 2);
/// assert_eq!(lines[1].number, 2);
/// assert_eq!(lines[1].tokens[0].text, "b */");
/// assert_eq!(lines[1].tokens[0].range, 5..9);
/// assert_eq!(lines[1].tokens[0].scope, "comment");
/// assert_eq!(lines[1].annotations, [&review]);
/// ```
pub fn lines_from_events<'a, T>(
    source: &'a str,
    events: &'a [lumis_core::events::HighlightEvent<'a, T>],
) -> Vec<Line<'a, T>> {
    lumis_core::formatter::lines_from_events(source, events)
}
