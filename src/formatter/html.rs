use super::Formatter;

pub trait HtmlFormatter: Formatter {
    fn open_pre_tag(&self) -> String;
    fn open_code_tag(&self) -> String;
    fn closing_tags(&self) -> String;
}
