use super::Formatter;

pub trait HtmlFormatter: Formatter {
    fn write_pre_tag(&self) -> String;

    fn write_code_tag(&self) -> String;

    fn write_closing_tags(&self) -> String;
}
