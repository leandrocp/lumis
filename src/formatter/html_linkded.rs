#![allow(unused_must_use)]

use super::Formatter;
use crate::languages::Language;
use crate::constants::CLASSES;
use tree_sitter_highlight::{Error, HighlightEvent};

pub(crate) struct HtmlLinked<'a> {
    lang: Language,
    pre_class: Option<&'a str>,
}

impl<'a> HtmlLinked<'a> {
    pub fn new(lang: Language, pre_class: Option<&'a str>) -> Self {
        Self { lang, pre_class }
    }
}

impl Formatter for HtmlLinked<'_> {
    fn start<W>(&self, writer: &mut W, _: &str)
    where
        W: std::fmt::Write,
    {
        let class = if let Some(pre_class) = self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        write!(
            writer,
            "<pre class=\"{}\"><code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            class,
            self.lang.id_name()
        );
    }

    fn write<W>(
        &self,
        writer: &mut W,
        source: &str,
        events: impl Iterator<Item = Result<HighlightEvent, Error>>,
    ) where
        W: std::fmt::Write,
    {
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
    }

    fn finish<W>(&self, writer: &mut W, _: &str)
    where
        W: std::fmt::Write,
    {
        writer.write_str("</code></pre>");
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
