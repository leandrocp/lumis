// https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

mod html;
pub use html::*;

mod html_inline;
pub use html_inline::*;

mod html_linkded;
pub use html_linkded::*;

mod terminal;
pub use terminal::*;

use crate::languages::Language;
use crate::themes::Theme;
use crate::FormatterOption;
use tree_sitter_highlight::{Error, HighlightEvent, Highlighter};

pub trait Formatter {
    fn highlights(
        &self,
        source: &str,
        events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    ) -> String;
}

pub fn write_formatted<W>(
    writer: &mut W,
    source: &str,
    lang: Language,
    formatter: FormatterOption,
    theme: Option<&Theme>,
) where
    W: std::fmt::Write,
{
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(lang.config(), source.as_bytes(), None, |injected| {
            Some(Language::guess(injected, "").config())
        })
        .expect("failed to generate highlight events");

    match formatter {
        FormatterOption::HtmlInline {
            pre_class,
            italic,
            include_highlights,
        } => {
            let formatter = HtmlInline::new(lang, theme, pre_class, italic, include_highlights);
            let _ = write!(writer, "{}", formatter.pre_tag());
            let _ = write!(writer, "{}", formatter.code_tag());
            let _ = write!(writer, "{}", formatter.highlights(source, events));
            let _ = write!(writer, "{}", formatter.closing_tags());
        }
        FormatterOption::HtmlLinked { pre_class } => {
            let formatter = HtmlLinked::new(lang, pre_class);
            let _ = write!(writer, "{}", formatter.pre_tag());
            let _ = write!(writer, "{}", formatter.code_tag());
            let _ = write!(writer, "{}", formatter.highlights(source, events));
            let _ = write!(writer, "{}", formatter.closing_tags());
        }
        FormatterOption::Terminal => {
            let formatter = Terminal::new(theme);
            let _ = write!(writer, "{}", formatter.highlights(source, events));
        }
    }
}
