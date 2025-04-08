#![allow(unused_must_use)]

use super::{Formatter, HtmlFormatter};
use crate::constants::CLASSES;
use crate::languages::Language;
use crate::FormatterOption;
use tree_sitter_highlight::Highlighter;

#[derive(Clone, Debug)]
pub struct HtmlLinked<'a> {
    source: &'a str,
    lang: Language,
    options: FormatterOption<'a>,
}

impl<'a> HtmlLinked<'a> {
    pub fn new(source: &'a str, lang: Language, options: FormatterOption<'a>) -> Self {
        Self {
            source,
            lang,
            options,
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

    pub fn with_options(mut self, options: FormatterOption<'a>) -> Self {
        self.options = options;
        self
    }
}

impl Default for HtmlLinked<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            options: FormatterOption::HtmlLinked { pre_class: None },
        }
    }
}

impl HtmlFormatter for HtmlLinked<'_> {
    fn open_pre_tag(&self) -> String {
        let class = if let FormatterOption::HtmlLinked { pre_class, .. } = &self.options {
            if let Some(pre_class) = pre_class {
                format!("athl {}", pre_class)
            } else {
                "athl".to_string()
            }
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

impl Formatter for HtmlLinked<'_> {
    fn highlights(&self) -> String {
        let mut highlighter = Highlighter::new();
        let events = highlighter
            .highlight(
                self.lang.config(),
                self.source.as_bytes(),
                None,
                |injected| Some(Language::guess(injected, "").config()),
            )
            .expect("failed to generate highlight events");

        let mut renderer = tree_sitter_highlight::HtmlRenderer::new();

        renderer
            .render(events, self.source.as_bytes(), &move |highlight, output| {
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
        let pre_tag = formatter.open_pre_tag();

        assert!(pre_tag.contains("<pre class=\"athl\">"));
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new(
            "",
            Language::PlainText,
            FormatterOption::HtmlLinked {
                pre_class: Some("test-pre-class"),
            },
        );
        let pre_tag = formatter.open_pre_tag();

        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new(
            "",
            Language::Rust,
            FormatterOption::HtmlLinked { pre_class: None },
        );
        let code_tag = formatter.open_code_tag();

        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinked::default()
            .with_lang(Language::Rust)
            .with_options(FormatterOption::HtmlLinked {
                pre_class: Some("test-class"),
            });

        let pre_tag = formatter.open_pre_tag();
        let code_tag = formatter.open_code_tag();

        assert!(pre_tag.contains("<pre class=\"athl test-class\">"));
        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }
}
