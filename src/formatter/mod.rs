// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

use std::io::{self, Write};

pub mod html_inline;
pub use html_inline::HighlightLines as HtmlInlineHighlightLines;
pub use html_inline::HtmlInline;

pub mod html_linkded;
pub use html_linkded::HighlightLines as HtmlLinkedHighlightLines;
pub use html_linkded::HtmlLinked;

mod terminal;
pub use terminal::*;

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
