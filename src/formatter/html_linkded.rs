#![allow(unused_must_use)]

use super::{Formatter, HtmlFormatter};
use crate::constants::CLASSES;
use crate::languages::Language;
use std::io::{self, Write};
use tree_sitter_highlight::Highlighter;

#[derive(Debug)]
pub struct HtmlLinked<'a> {
    source: &'a str,
    lang: Language,
    pre_class: Option<&'a str>,
}

impl<'a> HtmlLinked<'a> {
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

impl Default for HtmlLinked<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            pre_class: None,
        }
    }
}

impl Formatter for HtmlLinked<'_> {
    fn highlights(&self, output: &mut dyn Write) -> io::Result<()> {
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

        for (i, line) in renderer.lines().enumerate() {
            write!(
                output,
                "<span class=\"line\" data-line=\"{}\">{}</span>",
                i + 1,
                line.replace('{', "&lbrace;").replace('}', "&rbrace;")
            );
        }
        Ok(())
    }

    fn format(&self, output: &mut dyn Write) -> io::Result<()> {
        let mut buffer = Vec::new();
        self.open_pre_tag(&mut buffer)?;
        self.open_code_tag(&mut buffer)?;
        self.highlights(&mut buffer)?;
        self.closing_tags(&mut buffer)?;
        write!(output, "{}", &String::from_utf8(buffer).unwrap())?;
        Ok(())
    }
}

impl HtmlFormatter for HtmlLinked<'_> {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        let class = if let Some(pre_class) = self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        write!(output, "<pre class=\"{}\">", class)
    }

    fn open_code_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        write!(
            output,
            "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            self.lang.id_name()
        )
    }

    fn closing_tags(&self, output: &mut dyn Write) -> io::Result<()> {
        output.write_all(b"</code></pre>")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlLinked::new("", Language::PlainText, Some("test-pre-class"));
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_code_tag_with_language() {
        let formatter = HtmlLinked::new("", Language::Rust, None);
        let mut buffer = Vec::new();
        formatter.open_code_tag(&mut buffer);
        let code_tag = String::from_utf8(buffer).unwrap();
        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }

    #[test]
    fn test_builder_pattern() {
        let formatter = HtmlLinked::default()
            .with_lang(Language::Rust)
            .with_pre_class(Some("test-pre-class"));

        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));

        let mut buffer = Vec::new();
        formatter.open_code_tag(&mut buffer);
        let code_tag = String::from_utf8(buffer).unwrap();
        assert!(code_tag.contains("<code class=\"language-rust\" translate=\"no\" tabindex=\"0\">"));
    }
}
