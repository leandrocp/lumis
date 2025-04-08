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
use crate::FormatterOption;

pub trait Formatter {
    fn highlights(&self) -> String;
}

pub fn write_formatted<W>(
    writer: &mut W,
    source: &str,
    lang: Language,
    formatter: FormatterOption,
) -> std::fmt::Result
where
    W: std::fmt::Write,
{
    match formatter {
        FormatterOption::HtmlInline {
            pre_class,
            italic,
            include_highlights,
            theme,
        } => {
            let formatter = HtmlInline::new(source, lang, FormatterOption::HtmlInline {
                pre_class,
                italic,
                include_highlights,
                theme,
            });
            write!(writer, "{}", formatter.open_pre_tag())?;
            write!(writer, "{}", formatter.open_code_tag())?;
            write!(writer, "{}", formatter.highlights())?;
            write!(writer, "{}", formatter.closing_tags())?;
        }
        FormatterOption::HtmlLinked { pre_class, theme } => {
            let formatter = HtmlLinked::new(source, lang, FormatterOption::HtmlLinked { pre_class, theme });
            write!(writer, "{}", formatter.open_pre_tag())?;
            write!(writer, "{}", formatter.open_code_tag())?;
            write!(writer, "{}", formatter.highlights())?;
            write!(writer, "{}", formatter.closing_tags())?;
        }
        FormatterOption::Terminal { theme } => {
            let formatter = Terminal::new(source, lang, FormatterOption::Terminal { theme });
            write!(writer, "{}", formatter.highlights())?;
        }
    }

    Ok(())
}
