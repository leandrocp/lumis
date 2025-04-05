use super::Formatter;

pub trait HtmlFormatter: Formatter {
    fn write_pre_tag<W>(&self, _writer: &mut W)
    where
        W: std::fmt::Write;

    fn write_code_tag<W>(&self, _writer: &mut W)
    where
        W: std::fmt::Write;

    fn write_closing_tags<W>(&self, _writer: &mut W)
    where
        W: std::fmt::Write;
}
