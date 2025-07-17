//! Formatter implementations for generating syntax highlighted output.
//!
//! This module provides three different formatters for rendering syntax highlighted code:
//! - [`html_inline`] - HTML output with inline CSS styles
//! - [`html_linked`] - HTML output with CSS classes (requires external CSS)
//! - [`terminal`] - ANSI color codes for terminal output
//!
//! # Builder Pattern
//!
//! Each formatter has a dedicated builder that provides a type-safe, ergonomic API:
//! - [`HtmlInlineBuilder`] - Create HTML formatters with inline CSS styles
//! - [`HtmlLinkedBuilder`] - Create HTML formatters with CSS classes
//! - [`TerminalBuilder`] - Create terminal formatters with ANSI colors
//!
//! Builders are exported at the crate root for convenient access:
//! ```rust
//! use autumnus::{HtmlInlineBuilder, HtmlLinkedBuilder, TerminalBuilder};
//! ```
//!
//! # Examples
//!
//! ## Using HtmlInlineBuilder
//!
//! ```rust
//! use autumnus::{HtmlInlineBuilder, languages::Language, themes, formatter::Formatter};
//! use std::io::Write;
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! // HTML with inline styles
//! let formatter = HtmlInlineBuilder::default()
//!     .source(code)
//!     .lang(Language::Rust)
//!     .theme(Some(theme))
//!     .pre_class(Some("code-block"))
//!     .italic(false)
//!     .include_highlights(false)
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//!
//! ## Using HtmlLinkedBuilder
//!
//! ```rust
//! use autumnus::{HtmlLinkedBuilder, languages::Language, formatter::Formatter};
//! use std::io::Write;
//!
//! let code = "<div>Hello World</div>";
//!
//! let formatter = HtmlLinkedBuilder::default()
//!     .source(code)
//!     .lang(Language::HTML)
//!     .pre_class(Some("my-code"))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Using TerminalBuilder
//!
//! ```rust
//! use autumnus::{TerminalBuilder, languages::Language, themes, formatter::Formatter};
//! use std::io::Write;
//!
//! let code = "puts 'Hello from Ruby!'";
//! let theme = themes::get("github_light").unwrap();
//!
//! let formatter = TerminalBuilder::default()
//!     .source(code)
//!     .lang(Language::Ruby)
//!     .theme(Some(theme))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let ansi_output = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Line highlighting with HTML formatters
//!
//! ```rust
//! use autumnus::{HtmlInlineBuilder, languages::Language, themes, formatter::Formatter};
//! use autumnus::formatter::html_inline::{HighlightLines, HighlightLinesStyle};
//! use std::io::Write;
//!
//! let code = "line 1\nline 2\nline 3\nline 4";
//! let theme = themes::get("catppuccin_mocha").unwrap();
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![1..=1, 3..=4],  // Highlight lines 1, 3, and 4
//!     style: HighlightLinesStyle::Theme,  // Use theme's cursorline style
//! };
//!
//! let formatter = HtmlInlineBuilder::default()
//!     .source(code)
//!     .lang(Language::PlainText)
//!     .theme(Some(theme))
//!     .include_highlights(false)
//!     .highlight_lines(Some(highlight_lines))
//!     .build()
//!     .unwrap();
//! ```

// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

use std::io::{self, Write};

pub mod html_inline;
pub use html_inline::{HtmlInline, HtmlInlineBuilder};

pub mod html_linked;
pub use html_linked::{HtmlLinked, HtmlLinkedBuilder};

pub mod terminal;
pub use terminal::{Terminal, TerminalBuilder};

/// Configuration for wrapping the formatted output with custom HTML elements.
///
/// This struct allows you to specify opening and closing HTML tags that will wrap
/// the entire code block. This is useful for adding custom containers, sections,
/// or other structural elements around the formatted code.
///
/// # Examples
///
/// Wrapping with a div element:
/// ```rust
/// use autumnus::formatter::HtmlElement;
///
/// let header = HtmlElement {
///     open_tag: "<div class=\"code-wrapper\">".to_string(),
///     close_tag: "</div>".to_string(),
/// };
/// ```
///
/// Wrapping with a section element with attributes:
/// ```rust
/// use autumnus::formatter::HtmlElement;
///
/// let header = HtmlElement {
///     open_tag: "<section class=\"highlight\" data-lang=\"rust\">".to_string(),
///     close_tag: "</section>".to_string(),
/// };
/// ```
#[derive(Clone, Debug)]
pub struct HtmlElement {
    /// The opening HTML tag that will be placed before the formatted code.
    ///
    /// This should be a complete HTML opening tag, including any attributes.
    /// Example: `"<div class=\"wrapper\" id=\"code-block\">"`
    pub open_tag: String,
    /// The closing HTML tag that will be placed after the formatted code.
    ///
    /// This should be the corresponding closing tag for the opening tag.
    /// Example: `"</div>"`
    pub close_tag: String,
}

pub trait Formatter: Send + Sync {
    fn format(&self, output: &mut dyn Write) -> io::Result<()>;
    fn highlights(&self, output: &mut dyn Write) -> io::Result<()>;
}

pub trait HtmlFormatter: Formatter {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn open_code_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn closing_tags(&self, output: &mut dyn Write) -> io::Result<()>;
}
