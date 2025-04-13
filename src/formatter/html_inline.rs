#![allow(unused_must_use)]

use super::{Formatter, HtmlFormatter};
use crate::languages::Language;
use crate::themes::Theme;
use std::io::{self, Write};
use tree_sitter_highlight::Highlighter;

#[derive(Debug)]
pub struct HtmlInline<'a> {
    source: &'a str,
    lang: Language,
    theme: Option<&'a Theme>,
    pre_class: Option<&'a str>,
    italic: bool,
    include_highlights: bool,
}

impl<'a> HtmlInline<'a> {
    pub fn new(
        source: &'a str,
        lang: Language,
        theme: Option<&'a Theme>,
        pre_class: Option<&'a str>,
        italic: bool,
        include_highlights: bool,
    ) -> Self {
        Self {
            source,
            lang,
            theme,
            pre_class,
            italic,
            include_highlights,
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

    pub fn with_theme(mut self, theme: Option<&'a Theme>) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_pre_class(mut self, pre_class: Option<&'a str>) -> Self {
        self.pre_class = pre_class;
        self
    }

    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn with_include_highlights(mut self, include_highlights: bool) -> Self {
        self.include_highlights = include_highlights;
        self
    }
}

impl Default for HtmlInline<'_> {
    fn default() -> Self {
        Self {
            source: "",
            lang: Language::PlainText,
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
        }
    }
}

impl Formatter for HtmlInline<'_> {
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
                let scope = crate::constants::HIGHLIGHT_NAMES[highlight.0];

                if self.include_highlights {
                    output.extend(" data-highlight=\"".as_bytes());
                    output.extend(scope.as_bytes());
                    output.extend(b"\"");
                }

                if let Some(theme) = self.theme {
                    if let Some(style) = theme.get_style(scope) {
                        if self.include_highlights {
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
                output,
                "<span class=\"line\" data-line=\"{}\">{}</span>",
                i + 1,
                line.replace('{', "&lbrace;").replace('}', "&rbrace;")
            )?;
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

impl HtmlFormatter for HtmlInline<'_> {
    fn open_pre_tag(&self, output: &mut dyn Write) -> io::Result<()> {
        let class = if let Some(pre_class) = &self.pre_class {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        write!(
            output,
            "<pre class=\"{}\"{}>",
            class,
            &self
                .theme
                .and_then(|theme| theme.pre_style(" "))
                .map(|pre_style| format!(" style=\"{}\"", pre_style))
                .unwrap_or_default(),
        )
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
    use crate::themes;

    #[test]
    fn test_do_not_append_pre_style_if_missing_theme_style() {
        let formatter = HtmlInline::default();
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl\">"));
    }

    #[test]
    fn test_include_pre_class() {
        let formatter = HtmlInline::new(
            "",
            Language::PlainText,
            None,
            Some("test-pre-class"),
            false,
            false,
        );
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\">"));
    }

    #[test]
    fn test_include_pre_class_with_theme() {
        let theme = themes::get("github_light").unwrap();
        let formatter = HtmlInline::new(
            "",
            Language::PlainText,
            Some(theme),
            Some("test-pre-class"),
            false,
            false,
        );
        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\" style=\"color: #1f2328; background-color: #ffffff;\">"));
    }

    #[test]
    fn test_builder_pattern() {
        let theme = themes::get("github_light").unwrap();
        let formatter = HtmlInline::default()
            .with_lang(Language::Rust)
            .with_theme(Some(theme))
            .with_pre_class(Some("test-pre-class"))
            .with_italic(true)
            .with_include_highlights(true);

        let mut buffer = Vec::new();
        formatter.open_pre_tag(&mut buffer);
        let pre_tag = String::from_utf8(buffer).unwrap();
        assert!(pre_tag.contains("<pre class=\"athl test-pre-class\" style=\"color: #1f2328; background-color: #ffffff;\">"));
    }
}
