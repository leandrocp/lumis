#![allow(unused_must_use)]

use super::Formatter;
use crate::constants::CLASSES;
use crate::languages::Language;
use tree_sitter_highlight::{Error, HighlightEvent};

pub struct HtmlLinked<'a> {
    lang: Language,
    pre_class: Option<&'a str>,
}

impl<'a> HtmlLinked<'a> {
    pub fn new(lang: Language, pre_class: Option<&'a str>) -> Self {
        Self { lang, pre_class }
    }

    pub fn pre_tag(&self) -> String {
        let class = if let Some(pre_class) = self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        format!("<pre class=\"{}\">", class)
    }

    pub fn code_tag(&self) -> String {
        format!(
            "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            self.lang.id_name()
        )
    }
}

impl Default for HtmlLinked<'_> {
    fn default() -> Self {
        Self {
            lang: Language::PlainText,
            pre_class: None,
        }
    }
}

impl Formatter for HtmlLinked<'_> {
    fn write<W>(
        &self,
        writer: &mut W,
        source: &str,
        events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    ) where
        W: std::fmt::Write,
    {
        write!(writer, "{}{}", self.pre_tag(), self.code_tag());

        let mut renderer = tree_sitter_highlight::HtmlRenderer::new();

        renderer
            .render(events, source.as_bytes(), &move |highlight, output| {
                let class = CLASSES[highlight.0];

                output.extend(b"class=\"");
                output.extend(class.as_bytes());
                output.extend(b"\"");
            })
            .expect("failed to render highlight events");

        for (i, line) in renderer.lines().enumerate() {
            write!(
                writer,
                "<span class=\"line\" data-line=\"{}\">{}</span>",
                i + 1,
                line.replace('{', "&lbrace;").replace('}', "&rbrace;")
            );
        }

        writer.write_str("</code></pre>");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pre_tag() {
        let formatter = HtmlLinked::default();
        let mut buffer = String::new();
        formatter.write(&mut buffer, "", std::iter::empty());

        assert!(buffer.as_str().contains("<pre class=\"athl\">"));
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new(Language::PlainText, Some("test-pre-class"));
        let mut buffer = String::new();
        formatter.write(&mut buffer, "", std::iter::empty());

        assert!(buffer
            .as_str()
            .contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new(Language::Rust, None);
        let mut buffer = String::new();
        formatter.write(&mut buffer, "", std::iter::empty());

        assert!(buffer
            .as_str()
            .contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }
}
