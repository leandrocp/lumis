use super::Formatter;

pub trait HtmlFormatter: Formatter {
    fn pre_tag(&self) -> String;
    fn code_tag(&self) -> String;
    fn closing_tags(&self) -> String;
}
