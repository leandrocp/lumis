#![allow(unused_must_use)]

use super::Formatter;
use crate::languages::Language;
use crate::{constants::HIGHLIGHT_NAMES, themes::Theme};
use tree_sitter_highlight::{Error, HighlightEvent};

pub struct HtmlInline<'a> {
    lang: Language,
    theme: Option<&'a Theme>,
    pre_class: Option<&'a str>,
    italic: bool,
    include_highlights: bool,
}

impl<'a> HtmlInline<'a> {
    pub fn new(
        lang: Language,
        theme: Option<&'a Theme>,
        pre_class: Option<&'a str>,
        italic: bool,
        include_highlights: bool,
    ) -> Self {
        Self {
            lang,
            theme,
            pre_class,
            italic,
            include_highlights,
        }
    }

    pub fn pre_tag(&self) -> String {
        let class = if let Some(pre_class) = self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        format!(
            "<pre class=\"{}\"{}>",
            class,
            &self
                .theme
                .as_ref()
                .and_then(|theme| theme.pre_style(" "))
                .map(|pre_style| format!(" style=\"{}\"", pre_style))
                .unwrap_or_default(),
        )
    }

    pub fn code_tag(&self) -> String {
        format!(
            "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            self.lang.id_name()
        )
    }
}

impl Formatter for HtmlInline<'_> {
    fn start<W>(&self, writer: &mut W, _: &str)
    where
        W: std::fmt::Write,
    {
        write!(writer, "{}{}", self.pre_tag(), self.code_tag());
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

        let (highlight_attr, include_highlights) = if self.include_highlights {
            (" data-highlight=\"", true)
        } else {
            ("", false)
        };

        renderer
            .render(events, source.as_bytes(), &move |highlight, output| {
                let scope = HIGHLIGHT_NAMES[highlight.0];

                if include_highlights {
                    output.extend(highlight_attr.as_bytes());
                    output.extend(scope.as_bytes());
                    output.extend(b"\"");
                }

                if let Some(theme) = &self.theme {
                    if let Some(style) = theme.get_style(scope) {
                        if include_highlights {
                            output.extend(b" ");
                        }

                        output.extend(b"style=\"");
                        output.extend(style.css(self.italic, " ").as_bytes());
                        output.extend(b"\"");
                    }
                }
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

impl Default for HtmlInline<'_> {
    fn default() -> Self {
        Self {
            lang: Language::PlainText,
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::themes;

    use super::*;

    #[test]
    fn test_do_not_append_pre_style_if_missing_theme_style() {
        let formatter = HtmlInline::default();
        let mut buffer = String::new();
        formatter.start(&mut buffer, "");

        assert!(buffer.as_str().contains("<pre class=\"athl\">"));
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlInline::new(
            Language::PlainText,
            None,
            Some("test-pre-class"),
            false,
            false,
        );
        let mut buffer = String::new();
        formatter.start(&mut buffer, "");

        assert!(buffer
            .as_str()
            .contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_include_pre_class_with_theme() {
        let theme = themes::get("github_light").unwrap();
        let formatter = HtmlInline::new(
            Language::PlainText,
            Some(theme),
            Some("test-pre-class"),
            false,
            false,
        );
        let mut buffer = String::new();
        formatter.start(&mut buffer, "");

        assert!(buffer
            .as_str()
            .contains("<pre class=\"athl test-pre-class\" style=\"color: #1f2328; background-color: #ffffff;\">"));
    }
}
