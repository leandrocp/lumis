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
//! let formatter = HtmlInlineBuilder::new()
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
//! let formatter = HtmlLinkedBuilder::new()
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
//! let formatter = TerminalBuilder::new()
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
//!     style: Some(HighlightLinesStyle::Theme),  // Use theme's highlighted style
//!     class: None,
//! };
//!
//! let formatter = HtmlInlineBuilder::new()
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

pub mod events;

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

/// Core trait for syntax highlighting formatters.
///
/// Implement this trait to create custom formatters that generate syntax-highlighted
/// output in any format you need. The trait is object-safe and requires `Send + Sync`
/// for use in concurrent contexts.
///
/// # Required Methods
///
/// - [`format`](Formatter::format) - Generate formatted output
/// - [`highlights`](Formatter::highlights) - Generate output with syntax highlighting
///
/// For most formatters, both methods can have the same implementation.
///
/// # Creating Custom Formatters
///
/// ## Approach 1: High-Level Token API (Recommended)
///
/// Use [`events::iter_tokens`] for an ergonomic iterator over syntax tokens:
///
/// ```rust
/// use autumnus::formatter::{Formatter, events};
/// use autumnus::languages::Language;
/// use autumnus::themes::Theme;
/// use std::io::{self, Write};
///
/// pub struct MarkdownFormatter<'a> {
///     source: &'a str,
///     lang: Language,
///     theme: Option<&'a Theme>,
/// }
///
/// impl Formatter for MarkdownFormatter<'_> {
///     fn format(&self, output: &mut dyn Write) -> io::Result<()> {
///         for token in events::iter_tokens(self.source, self.lang, self.theme) {
///             match token.scope.as_ref() {
///                 "comment" => write!(output, "*{}*", token.text)?,
///                 "keyword" => write!(output, "**{}**", token.text)?,
///                 "string" => write!(output, "`{}`", token.text)?,
///                 _ => write!(output, "{}", token.text)?,
///             }
///         }
///         Ok(())
///     }
///
///     fn highlights(&self, output: &mut dyn Write) -> io::Result<()> {
///         self.format(output)
///     }
/// }
/// ```
///
/// ## Approach 2: Low-Level Event API (Maximum Control)
///
/// Use [`events::highlight_events`] for direct access to tree-sitter events:
///
/// ```rust
/// use autumnus::formatter::{Formatter, events};
/// use autumnus::languages::Language;
/// use std::io::{self, Write};
/// use tree_sitter_highlight::HighlightEvent;
///
/// pub struct JsonFormatter<'a> {
///     source: &'a str,
///     lang: Language,
/// }
///
/// impl Formatter for JsonFormatter<'_> {
///     fn format(&self, output: &mut dyn Write) -> io::Result<()> {
///         write!(output, "[\"tokens\":[")?;
///         let mut first = true;
///         let mut current_scope = "text";
///
///         for event in events::highlight_events(self.source, self.lang) {
///             match event {
///                 HighlightEvent::HighlightStart(highlight) => {
///                     current_scope = events::scope_name(highlight.0);
///                 }
///                 HighlightEvent::Source { start, end } => {
///                     if !first { write!(output, ",")?; }
///                     first = false;
///                     write!(output, "{{\"text\":\"{}\",\"scope\":\"{}\"}}",
///                         &self.source[start..end], current_scope)?;
///                 }
///                 HighlightEvent::HighlightEnd => {}
///             }
///         }
///         write!(output, "]")?;
///         Ok(())
///     }
///
///     fn highlights(&self, output: &mut dyn Write) -> io::Result<()> {
///         self.format(output)
///     }
/// }
/// ```
///
/// # Using Custom Formatters
///
/// Once you've implemented the `Formatter` trait, pass it to `Options`:
///
/// ```rust
/// use autumnus::{highlight, Options};
/// # use autumnus::formatter::{Formatter, events};
/// # use autumnus::languages::Language;
/// # use std::io::{self, Write};
/// # pub struct MarkdownFormatter<'a> { source: &'a str, lang: Language }
/// # impl Formatter for MarkdownFormatter<'_> {
/// #     fn format(&self, output: &mut dyn Write) -> io::Result<()> { Ok(()) }
/// #     fn highlights(&self, output: &mut dyn Write) -> io::Result<()> { Ok(()) }
/// # }
///
/// let source = "fn main() {}";
/// let my_formatter = MarkdownFormatter {
///     source,
///     lang: Language::Rust,
/// };
///
/// let result = highlight(source, Options {
///     lang_or_file: Some("rust"),
///     formatter: Box::new(my_formatter),
/// });
/// ```
///
/// # Thread Safety
///
/// Formatters must implement `Send + Sync` because they may be used across
/// thread boundaries in concurrent applications. Ensure your formatter's fields
/// are also `Send + Sync`, or use appropriate synchronization primitives.
///
/// # See Also
///
/// - [`events`] module - Event processing APIs for building custom formatters
/// - Built-in formatters: [`HtmlInline`], [`HtmlLinked`], [`Terminal`]
pub trait Formatter: Send + Sync {
    /// Generates formatted output to the provided writer.
    ///
    /// This method should write the syntax-highlighted source code to `output`
    /// in your custom format. The output can be text, HTML, JSON, or any other
    /// format you need.
    ///
    /// # Arguments
    ///
    /// * `output` - A writable destination for the formatted output
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully wrote formatted output
    /// * `Err(io::Error)` - Write operation failed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use autumnus::formatter::Formatter;
    /// use std::io::Write;
    /// # use autumnus::formatter::events;
    /// # use autumnus::languages::Language;
    /// # struct MyFormatter { source: &'static str, lang: Language }
    /// # impl Formatter for MyFormatter {
    /// #     fn format(&self, output: &mut dyn Write) -> std::io::Result<()> {
    /// #         for token in events::iter_tokens(self.source, self.lang, None) {
    /// #             write!(output, "{}", token.text)?;
    /// #         }
    /// #         Ok(())
    /// #     }
    /// #     fn highlights(&self, output: &mut dyn Write) -> std::io::Result<()> { Ok(()) }
    /// # }
    ///
    /// let formatter = MyFormatter {
    ///     source: "fn main() {}",
    ///     lang: Language::Rust,
    /// };
    ///
    /// let mut buffer = Vec::new();
    /// formatter.format(&mut buffer).unwrap();
    /// let result = String::from_utf8(buffer).unwrap();
    /// ```
    fn format(&self, output: &mut dyn Write) -> io::Result<()>;

    /// Generates output with syntax highlighting information.
    ///
    /// For most custom formatters, this method should have the same implementation
    /// as [`format`](Formatter::format). The distinction exists for formatters that
    /// might have different behavior for highlights vs. plain formatting.
    ///
    /// # Arguments
    ///
    /// * `output` - A writable destination for the highlighted output
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully wrote highlighted output
    /// * `Err(io::Error)` - Write operation failed
    fn highlights(&self, output: &mut dyn Write) -> io::Result<()>;
}

pub trait HtmlFormatter: Formatter {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn open_code_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn closing_tags(&self, output: &mut dyn Write) -> io::Result<()>;
}
