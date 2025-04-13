// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

mod html_inline;
use std::io::{self, Write};

pub use html_inline::*;

mod html_linkded;
pub use html_linkded::*;

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

pub fn build_formatter<'a>(
    source: &'a str,
    lang: Language,
    options: FormatterOption<'a>,
) -> Box<dyn Formatter + 'a> {
    match options {
        FormatterOption::HtmlInline {
            theme,
            pre_class,
            italic,
            include_highlights,
        } => Box::new(HtmlInline::new(
            source,
            lang,
            theme,
            pre_class,
            italic,
            include_highlights,
        )),
        FormatterOption::HtmlLinked { pre_class } => {
            Box::new(HtmlLinked::new(source, lang, pre_class))
        }
        FormatterOption::Terminal { theme } => Box::new(Terminal::new(source, lang, theme)),
    }
}

pub fn build_html_formatter<'a>(
    source: &'a str,
    lang: Language,
    options: FormatterOption<'a>,
) -> Box<dyn HtmlFormatter + 'a> {
    match options {
        FormatterOption::HtmlInline {
            theme,
            pre_class,
            italic,
            include_highlights,
        } => Box::new(HtmlInline::new(
            source,
            lang,
            theme,
            pre_class,
            italic,
            include_highlights,
        )),
        FormatterOption::HtmlLinked { pre_class } => {
            Box::new(HtmlLinked::new(source, lang, pre_class))
        }
        FormatterOption::Terminal { .. } => {
            panic!("Terminal formatter does not implement HtmlFormatter trait")
        }
    }
}

pub fn open_pre_tag(html_formatter: &impl HtmlFormatter, output: &mut dyn Write) -> io::Result<()> {
    html_formatter.open_pre_tag(output)
}

pub fn open_code_tag(
    html_formatter: &impl HtmlFormatter,
    output: &mut dyn Write,
) -> io::Result<()> {
    html_formatter.open_code_tag(output)
}

pub fn closing_tags(html_formatter: &impl HtmlFormatter, output: &mut dyn Write) -> io::Result<()> {
    html_formatter.closing_tags(output)
}
