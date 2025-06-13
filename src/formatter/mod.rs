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
//! use autumnus::{HtmlInlineBuilder, languages::Language, themes};
//! use std::io::Write;
//!
//! let code = "fn main() { println!(\"Hello\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! // HTML with inline styles
//! let formatter = HtmlInlineBuilder::new()
//!     .source(code)
//!     .lang(Language::Rust)
//!     .theme(theme)
//!     .pre_class("code-block")
//!     .italic(false)
//!     .include_highlights(false)
//!     .build();
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
//! use autumnus::{HtmlLinkedBuilder, languages::Language};
//!
//! let code = "<div>Hello World</div>";
//!
//! let formatter = HtmlLinkedBuilder::new()
//!     .source(code)
//!     .lang(Language::HTML)
//!     .pre_class("my-code")
//!     .build();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Using TerminalBuilder
//!
//! ```rust
//! use autumnus::{TerminalBuilder, languages::Language, themes};
//!
//! let code = "puts 'Hello from Ruby!'";
//! let theme = themes::get("github_light").unwrap();
//!
//! let formatter = TerminalBuilder::new()
//!     .source(code)
//!     .lang(Language::Ruby)
//!     .theme(theme)
//!     .build();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let ansi_output = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Line highlighting with HTML formatters
//!
//! ```rust
//! use autumnus::{HtmlInlineBuilder, languages::Language, themes};
//! use autumnus::formatter::html_inline::{HighlightLines, HighlightLinesStyle};
//!
//! let code = "line 1\nline 2\nline 3\nline 4";
//! let theme = themes::get("catppuccin_mocha").unwrap();
//!
//! let highlight_lines = HighlightLines {
//!     lines: vec![1..=1, 3..=4],  // Highlight lines 1, 3, and 4
//!     style: HighlightLinesStyle::Theme,  // Use theme's cursorline style
//! };
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .source(code)
//!     .lang(Language::PlainText)
//!     .theme(theme)
//!     .include_highlights(false)
//!     .highlight_lines(highlight_lines)
//!     .build();
//! ```

// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

use std::io::{self, Write};

pub mod html_inline;
pub use html_inline::HtmlInline;

pub mod html_linked;
pub use html_linked::HtmlLinked;

pub mod terminal;
pub use terminal::Terminal;

use crate::languages::Language;

pub trait Formatter: Send + Sync {
    fn format(&self, output: &mut dyn Write) -> io::Result<()>;
    fn highlights(&self, output: &mut dyn Write) -> io::Result<()>;
}

pub trait HtmlFormatter: Formatter {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn open_code_tag(&self, output: &mut dyn Write) -> io::Result<()>;
    fn closing_tags(&self, output: &mut dyn Write) -> io::Result<()>;
}

pub struct HtmlInlineBuilder<'a> {
    source: Option<&'a str>,
    lang: Option<Language>,
    theme: Option<&'a crate::themes::Theme>,
    pre_class: Option<&'a str>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<html_inline::HighlightLines>,
}

impl<'a> HtmlInlineBuilder<'a> {
    pub fn new() -> Self {
        Self {
            source: None,
            lang: None,
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
        }
    }

    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }

    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn theme(mut self, theme: &'a crate::themes::Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn pre_class(mut self, pre_class: &'a str) -> Self {
        self.pre_class = Some(pre_class);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn include_highlights(mut self, include_highlights: bool) -> Self {
        self.include_highlights = include_highlights;
        self
    }

    pub fn highlight_lines(mut self, highlight_lines: html_inline::HighlightLines) -> Self {
        self.highlight_lines = Some(highlight_lines);
        self
    }

    pub fn build(self) -> Box<dyn HtmlFormatter + 'a> {
        let source = self.source.unwrap_or_default();
        let lang = self.lang.unwrap_or_default();
        Box::new(HtmlInline::new(
            source,
            lang,
            self.theme,
            self.pre_class,
            self.italic,
            self.include_highlights,
            self.highlight_lines,
        ))
    }
}

pub struct HtmlLinkedBuilder<'a> {
    source: Option<&'a str>,
    lang: Option<Language>,
    pre_class: Option<&'a str>,
    highlight_lines: Option<html_linked::HighlightLines>,
}

impl<'a> HtmlLinkedBuilder<'a> {
    pub fn new() -> Self {
        Self {
            source: None,
            lang: None,
            pre_class: None,
            highlight_lines: None,
        }
    }

    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }

    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn pre_class(mut self, pre_class: &'a str) -> Self {
        self.pre_class = Some(pre_class);
        self
    }

    pub fn highlight_lines(mut self, highlight_lines: html_linked::HighlightLines) -> Self {
        self.highlight_lines = Some(highlight_lines);
        self
    }

    pub fn build(self) -> Box<dyn HtmlFormatter + 'a> {
        let source = self.source.unwrap_or_default();
        let lang = self.lang.unwrap_or_default();
        Box::new(HtmlLinked::new(
            source,
            lang,
            self.pre_class,
            self.highlight_lines,
        ))
    }
}

pub struct TerminalBuilder<'a> {
    source: Option<&'a str>,
    lang: Option<Language>,
    theme: Option<&'a crate::themes::Theme>,
}

impl<'a> TerminalBuilder<'a> {
    pub fn new() -> Self {
        Self {
            source: None,
            lang: None,
            theme: None,
        }
    }

    pub fn source(mut self, source: &'a str) -> Self {
        self.source = Some(source);
        self
    }

    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }

    pub fn theme(mut self, theme: &'a crate::themes::Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn build(self) -> Box<dyn Formatter + 'a> {
        let source = self.source.unwrap_or_default();
        let lang = self.lang.unwrap_or_default();
        Box::new(Terminal::new(source, lang, self.theme))
    }
}
