// https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

mod html_inline;
pub use html_inline::*;

mod html_linkded;
pub use html_linkded::*;

mod terminal;
pub use terminal::*;

use tree_sitter_highlight::{Error, HighlightEvent};

pub trait Formatter {
    #[inline]
    fn start<W>(&self, _writer: &mut W, _source: &str)
    where
        W: std::fmt::Write,
    {
    }

    fn write<W>(
        &self,
        _writer: &mut W,
        _source: &str,
        events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    ) where
        W: std::fmt::Write;

    #[inline]
    fn finish<W>(&self, _writer: &mut W, _source: &str)
    where
        W: std::fmt::Write,
    {
    }
}
