#![allow(unused_must_use)]

use super::{Formatter, HtmlFormatter};
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

    pub fn with_lang(mut self, lang: Language) -> Self {
        self.lang = lang;
        self
    }

    pub fn with_pre_class(mut self, pre_class: Option<&'a str>) -> Self {
        self.pre_class = pre_class;
        self
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

impl HtmlFormatter for HtmlLinked<'_> {
    fn pre_tag(&self) -> String {
        let class = if let Some(pre_class) = self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        format!("<pre class=\"{}\">", class)
    }

    fn code_tag(&self) -> String {
        format!(
            "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            self.lang.id_name()
        )
    }

    fn closing_tags(&self) -> String {
        "</code></pre>".to_string()
    }
}

impl Formatter for HtmlLinked<'_> {
    fn highlights(
        &self,
        source: &str,
        events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    ) -> String {
        let mut renderer = tree_sitter_highlight::HtmlRenderer::new();

        renderer
            .render(events, source.as_bytes(), &move |highlight, output| {
                let class = CLASSES[highlight.0];

                output.extend(b"class=\"");
                output.extend(class.as_bytes());
                output.extend(b"\"");
            })
            .expect("failed to render highlight events");

        let mut result = String::new();
        for (i, line) in renderer.lines().enumerate() {
            result.push_str(&format!(
                "<span class=\"line\" data-line=\"{}\">{}</span>",
                i + 1,
                line.replace('{', "&lbrace;").replace('}', "&rbrace;")
            ));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pre_tag() {
        let formatter = HtmlLinked::default();
        let pre_tag = formatter.pre_tag();

        assert!(pre_tag.contains("<pre class=\"athl\">"));
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new(Language::PlainText, Some("test-pre-class"));
        let pre_tag = formatter.pre_tag();

        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new(Language::Rust, None);
        let code_tag = formatter.code_tag();

        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinked::default()
            .with_lang(Language::Rust)
            .with_pre_class(Some("test-class"));

        let pre_tag = formatter.pre_tag();
        let code_tag = formatter.code_tag();

        assert!(pre_tag.contains("<pre class=\"athl test-class\">"));
        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }
}
