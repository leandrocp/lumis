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

pub trait Formatter {
    fn highlights(&self) -> String;
}

pub fn write_formatted<W>(
    writer: &mut W,
    source: &str,
    lang: Language,
    formatter: FormatterOption,
    theme: Option<&Theme>,
) -> std::fmt::Result
where
    W: std::fmt::Write,
{
    match formatter {
        FormatterOption::HtmlInline {
            pre_class,
            italic,
            include_highlights,
        } => {
            let formatter =
                HtmlInline::new(source, lang, theme, pre_class, italic, include_highlights);
            write!(writer, "{}", formatter.pre_tag())?;
            write!(writer, "{}", formatter.code_tag())?;
            write!(writer, "{}", formatter.highlights())?;
            write!(writer, "{}", formatter.closing_tags())?;
        }
        FormatterOption::HtmlLinked { pre_class } => {
            let formatter = HtmlLinked::new(source, lang, pre_class);
            write!(writer, "{}", formatter.pre_tag())?;
            write!(writer, "{}", formatter.code_tag())?;
            write!(writer, "{}", formatter.highlights())?;
            write!(writer, "{}", formatter.closing_tags())?;
        }
        FormatterOption::Terminal => {
            let formatter = Terminal::new(source, lang, theme);
            write!(writer, "{}", formatter.highlights())?;
        }
    }

    Ok(())
}
