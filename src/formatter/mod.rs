//! Formatter implementations for generating syntax highlighted output.
//!
//! This module provides three different formatters for rendering syntax highlighted code:
//! - [`HtmlInline`] - HTML output with inline CSS styles
//! - [`HtmlLinked`] - HTML output with CSS classes (requires external CSS)
//! - [`Terminal`] - ANSI color codes for terminal output
//!
//! # Builder Pattern
//!
//! Use [`FormatterBuilder`] for creating any formatter type, or [`HtmlFormatterBuilder`]
//! specifically for HTML formatters that implement the [`HtmlFormatter`] trait.
//!
//! # Examples
//!
//! ## Using FormatterBuilder for any formatter type
//!
//! ```rust
//! use autumnus::formatter::{FormatterBuilder, Formatter};
//! use autumnus::{FormatterOption, languages::Language, themes};
//! use std::io::Write;
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! // HTML with inline styles
//! let formatter = FormatterBuilder::new()
//!     .with_source(code)
//!     .with_lang(Language::Rust)
//!     .with_formatter(FormatterOption::HtmlInline {
//!         theme: Some(theme),
//!         pre_class: Some("code-block"),
//!         italic: false,
//!         include_highlights: false,
//!         highlight_lines: None,
//!     })
//!     .build();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Using FormatterBuilder for terminal output
//!
//! ```rust
//! use autumnus::formatter::{FormatterBuilder, Formatter};
//! use autumnus::{FormatterOption, languages::Language, themes};
//!
//! let code = "puts 'Hello from Ruby!'";
//! let theme = themes::get("github_light").unwrap();
//!
//! let formatter = FormatterBuilder::new()
//!     .with_source(code)
//!     .with_lang(Language::Ruby)
//!     .with_formatter(FormatterOption::Terminal {
//!         theme: Some(theme),
//!     })
//!     .build();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let ansi_output = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Using HtmlFormatterBuilder for HTML-specific features
//!
//! ```rust
//! use autumnus::formatter::{HtmlFormatterBuilder, HtmlFormatter};
//! use autumnus::{FormatterOption, languages::Language};
//!
//! let code = "<div>Hello World</div>";
//!
//! let formatter = HtmlFormatterBuilder::new()
//!     .with_source(code)
//!     .with_lang(Language::HTML)
//!     .with_formatter(FormatterOption::HtmlLinked {
//!         pre_class: Some("my-code"),
//!         highlight_lines: None,
//!     })
//!     .build();
//!
//! let mut output = Vec::new();
//! formatter.open_pre_tag(&mut output).unwrap();
//! formatter.open_code_tag(&mut output).unwrap();
//! formatter.highlights(&mut output).unwrap();
//! formatter.closing_tags(&mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Line highlighting with HTML formatters
//!
//! ```rust
//! use autumnus::formatter::{FormatterBuilder, html_inline::{HighlightLines, HighlightLinesStyle}};
//! use autumnus::{FormatterOption, languages::Language, themes};
//!
//! let code = "line 1\nline 2\nline 3\nline 4";
//! let theme = themes::get("catppuccin_mocha").unwrap();
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![1..=1, 3..=4],  // Highlight lines 1, 3, and 4
//!     style: HighlightLinesStyle::Theme,  // Use theme's cursorline style
//! };
//!
//! let formatter = FormatterBuilder::new()
//!     .with_source(code)
//!     .with_lang(Language::PlainText)
//!     .with_formatter(FormatterOption::HtmlInline {
//!         theme: Some(theme),
//!         pre_class: None,
//!         italic: false,
//!         include_highlights: false,
//!         highlight_lines: Some(highlight_lines),
//!     })
//!     .build();
//! ```

// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

use std::io::{self, Write};

pub mod html_inline;
pub use html_inline::HtmlInline;

pub mod html_linkded;
pub use html_linkded::HtmlLinked;

pub mod terminal;
pub use terminal::Terminal;

use crate::languages::Language;
use crate::FormatterOption;

pub trait Formatter: Send + Sync {
    fn format(&self, output: &mut dyn Write) -> io::Result<()>;
    fn highlights(&self, output: &mut dyn Write) -> io::Result<()>;
}

pub trait HtmlFormatter: Formatter {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn open_code_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn closing_tags(&self, output: &mut dyn Write) -> io::Result<()>;
}

pub struct FormatterBuilder<'a> {
    source: Option<&'a str>,
    lang: Option<Language>,
    formatter: Option<FormatterOption<'a>>,
}

impl<'a> FormatterBuilder<'a> {
    pub fn new() -> Self {
        Self {
            source: None,
            lang: None,
            formatter: None,
        }
    }

    pub fn with_source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn with_formatter(mut self, formatter: FormatterOption<'a>) -> Self {
        self.formatter = Some(formatter);
        self
    }

    pub fn build(self) -> Box<dyn Formatter + 'a> {
        let source = self.source.unwrap_or_default();
        let lang = self.lang.unwrap_or_default();
        let formatter = self.formatter.unwrap_or_default();

        match formatter {
            FormatterOption::HtmlInline {
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
            } => Box::new(HtmlInline::new(
                source,
                lang,
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
            )),
            FormatterOption::HtmlLinked {
                pre_class,
                highlight_lines,
            } => Box::new(HtmlLinked::new(source, lang, pre_class, highlight_lines)),
            FormatterOption::Terminal { theme } => Box::new(Terminal::new(source, lang, theme)),
        }
    }
}

impl Default for FormatterBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HtmlFormatterBuilder<'a> {
    source: Option<&'a str>,
    lang: Option<Language>,
    formatter: Option<FormatterOption<'a>>,
}

impl<'a> HtmlFormatterBuilder<'a> {
    pub fn new() -> Self {
        Self {
            source: None,
            lang: None,
            formatter: None,
        }
    }

    pub fn with_source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn with_formatter(mut self, formatter: FormatterOption<'a>) -> Self {
        self.formatter = Some(formatter);
        self
    }

    pub fn build(self) -> Box<dyn HtmlFormatter + 'a> {
        let source = self.source.unwrap_or_default();
        let lang = self.lang.unwrap_or_default();
        let formatter = self.formatter.unwrap_or_default();

        match formatter {
            FormatterOption::HtmlInline {
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
            } => Box::new(HtmlInline::new(
                source,
                lang,
                theme,
                pre_class,
                italic,
                include_highlights,
                highlight_lines,
            )),
            FormatterOption::HtmlLinked {
                pre_class,
                highlight_lines,
            } => Box::new(HtmlLinked::new(source, lang, pre_class, highlight_lines)),
            FormatterOption::Terminal { .. } => {
                panic!("Terminal formatter does not implement HtmlFormatter trait")
            }
        }
    }
}

impl Default for HtmlFormatterBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
