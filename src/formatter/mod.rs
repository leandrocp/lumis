// Originally based on https://github.com/Colonial-Dev/inkjet/tree/da289fa8b68f11dffad176e4b8fabae8d6ac376d/src/formatter

mod html_inline;
pub use html_inline::*;

mod html_linkded;
pub use html_linkded::*;

mod terminal;
pub use terminal::*;

pub trait Formatter {
    fn format<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result;
    fn highlights(&self) -> String;
}

pub trait HtmlFormatter: Formatter {
    fn open_pre_tag(&self) -> String;
    fn open_code_tag(&self) -> String;
    fn closing_tags(&self) -> String;
}
