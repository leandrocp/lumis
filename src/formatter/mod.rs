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
use tree_sitter_highlight::{Error, HighlightEvent};

pub trait Formatter {
    fn write_highlights<W>(
        &self,
        _writer: &mut W,
        _source: &str,
        events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    ) where
        W: std::fmt::Write;
}

pub fn write_formatted<W>(
    writer: &mut W,
    source: &str,
    events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    lang: Language,
    formatter: FormatterOption,
    theme: Option<&Theme>,
) where
    W: std::fmt::Write,
{
    match formatter {
        FormatterOption::HtmlInline {
            pre_class,
            italic,
            include_highlights,
        } => {
            let formatter = HtmlInline::new(
                lang,
                theme,
                pre_class.as_deref(),
                italic,
                include_highlights,
            );
            formatter.write_pre_tag(writer);
            formatter.write_code_tag(writer);
            formatter.write_highlights(writer, source, events);
            formatter.write_closing_tags(writer);
        }
        FormatterOption::HtmlLinked { pre_class } => {
            let formatter = HtmlLinked::new(lang, pre_class.as_deref());
            formatter.write_pre_tag(writer);
            formatter.write_code_tag(writer);
            formatter.write_highlights(writer, source, events);
            formatter.write_closing_tags(writer);
        }
        FormatterOption::Terminal => {
            let formatter = Terminal::new(theme);
            formatter.write_highlights(writer, source, events);
        }
    }
}
