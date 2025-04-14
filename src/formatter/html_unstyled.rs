#![allow(unused_must_use)]

use super::{Formatter, HtmlFormatter};
use crate::languages::Language;

#[derive(Clone, Debug)]
pub struct HtmlUnstyled<'a> {
    source: &'a str,
    lang: Language,
    pre_class: Option<&'a str>,
}

impl<'a> HtmlUnstyled<'a> {
    pub fn new(source: &'a str, lang: Language, pre_class: Option<&'a str>) -> Self {
        Self {
            source,
            lang,
            pre_class,
        }
    }

    pub fn with_source(mut self, source: &'a str) -> Self {
        self.source = source;
        self
    }

    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = lang;
        self
    }

    pub fn with_pre_class(mut self, pre_class: Option<&'a str>) -> Self {
        self.pre_class = pre_class;
        self
    }
}

impl Default for HtmlUnstyled<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            pre_class: None,
        }
    }
}

impl HtmlFormatter for HtmlUnstyled<'_> {
    fn open_pre_tag(&self) -> String {
        let class = if let Some(pre_class) = &self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        format!("<pre class=\"{}\">", class)
    }

    fn open_code_tag(&self) -> String {
        format!(
            "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            self.lang.id_name()
        )
    }

    fn closing_tags(&self) -> String {
        "</code></pre>".to_string()
    }
}

impl Formatter for HtmlUnstyled<'_> {
    fn highlights(&self) -> String {
        self.source.to_string()
    }

    fn format<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "{}", &self.open_pre_tag())?;
        write!(writer, "{}", &self.open_code_tag())?;
        write!(writer, "{}", &self.highlights())?;
        write!(writer, "{}", &self.closing_tags())?;
        Ok(())
    }
}
