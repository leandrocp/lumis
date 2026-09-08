//! Terminal formatter with ANSI color codes.
//!
//! Works with pre-computed highlight events from any source.

use super::{ansi, Formatter};
use crate::events::HighlightEvent;
use crate::languages::Language;
use crate::themes::{Style, Theme};
use derive_builder::Builder;
use std::io::{self, Write};

/// Background fill behavior for terminal output.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Background {
    /// Inherit the output environment's current background unless a token style sets one.
    #[default]
    Inherit,
    /// Reuse the theme's `normal` background color as the fallback background.
    Theme,
    /// Use an explicit fallback background color.
    Color(String),
}

/// Terminal formatter for syntax highlighting with ANSI color codes.
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct Terminal {
    #[builder(setter(custom))]
    language: Language,
    theme: Option<Theme>,
    background: Background,
    width: Option<usize>,
}

impl TerminalBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    #[deprecated(note = "use `.language(...)` instead")]
    pub fn lang(&mut self, language: Language) -> &mut Self {
        self.language(language)
    }
}

impl Terminal {
    pub fn new(
        language: Language,
        theme: Option<Theme>,
        background: Background,
        width: Option<usize>,
    ) -> Self {
        Self {
            language,
            theme,
            background,
            width,
        }
    }

    fn fallback_bg(&self) -> Option<&str> {
        match &self.background {
            Background::Inherit => None,
            Background::Theme => self.theme.as_ref().and_then(Theme::bg),
            Background::Color(color) => Some(color.as_str()),
        }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            theme: None,
            background: Background::Inherit,
            width: None,
        }
    }
}

impl<T> Formatter<T> for Terminal {
    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let source_bytes = source.as_bytes();
        let mut scope_stack: Vec<(usize, String)> = Vec::new();
        let mut line_width = 0usize;
        let fallback_bg = self.fallback_bg();

        for event in events {
            match event {
                HighlightEvent::Source { start, end } => {
                    if *start > *end || *end > source_bytes.len() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "invalid source range: {}..{} (len={})",
                                start,
                                end,
                                source_bytes.len()
                            ),
                        ));
                    }
                    let text = std::str::from_utf8(&source_bytes[*start..*end])
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    let styled = scope_stack.last().and_then(|(scope_index, language)| {
                        let theme = self.theme.as_ref()?;
                        let scope = crate::highlights::HIGHLIGHT_NAMES
                            .get(*scope_index)
                            .copied()
                            .unwrap_or("");
                        let specialized = format!("{scope}.{language}");
                        theme
                            .get_style(&specialized)
                            .or_else(|| theme.get_style(scope))
                    });

                    if fallback_bg.is_none() {
                        write!(output, "{}", paint_with_background(text, styled, None))?;
                        continue;
                    }

                    for segment in text.split_inclusive('\n') {
                        let has_newline = segment.ends_with('\n');
                        let content = if has_newline {
                            &segment[..segment.len() - 1]
                        } else {
                            segment
                        };

                        if !content.is_empty() {
                            write!(
                                output,
                                "{}",
                                paint_with_background(content, styled, fallback_bg)
                            )?;
                            line_width += display_width(content);
                        }

                        if has_newline {
                            write_line_padding(output, fallback_bg, self.width, line_width)?;
                            writeln!(output)?;
                            line_width = 0;
                        }
                    }
                }
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => scope_stack.push((*scope_index, language.clone())),
                HighlightEvent::End => {
                    scope_stack.pop();
                }
                HighlightEvent::AnnotationStart { .. } | HighlightEvent::AnnotationEnd => {}
            }
        }

        if !source.ends_with('\n') {
            write_line_padding(output, fallback_bg, self.width, line_width)?;
        }

        Ok(())
    }
}

fn paint_with_background(text: &str, style: Option<&Style>, fallback_bg: Option<&str>) -> String {
    match (style, fallback_bg) {
        (Some(style), Some(fallback_bg)) if style.bg.is_none() => {
            let mut style = style.clone();
            style.bg = Some(fallback_bg.to_string());
            ansi::paint(text, &style)
        }
        (Some(style), _) => ansi::paint(text, style),
        (None, Some(fallback_bg)) => ansi::paint(
            text,
            &Style {
                bg: Some(fallback_bg.to_string()),
                ..Default::default()
            },
        ),
        (None, None) => text.to_string(),
    }
}

fn write_line_padding(
    output: &mut dyn Write,
    fallback_bg: Option<&str>,
    width: Option<usize>,
    line_width: usize,
) -> io::Result<()> {
    let Some(fallback_bg) = fallback_bg else {
        return Ok(());
    };
    let Some(width) = width else {
        return Ok(());
    };

    if line_width >= width {
        return Ok(());
    }

    let padding = " ".repeat(width - line_width);
    write!(
        output,
        "{}",
        paint_with_background(&padding, None, Some(fallback_bg))
    )
}

fn display_width(text: &str) -> usize {
    text.chars()
        .map(|ch| match ch {
            '\t' => 4,
            _ => 1,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn theme_with_background(bg: &str) -> Theme {
        Theme {
            name: "test".to_string(),
            highlights: BTreeMap::from([(
                "normal".to_string(),
                Style {
                    bg: Some(bg.to_string()),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        }
    }

    fn theme_with_scope_style(scope: &str, style: Style) -> Theme {
        Theme {
            name: "test".to_string(),
            highlights: BTreeMap::from([(scope.to_string(), style)]),
            ..Default::default()
        }
    }

    #[test]
    fn paint_with_background_applies_background_to_unstyled_text() {
        let painted = paint_with_background("abc", None, Some("#282a36"));

        assert_eq!(painted, "\u{1b}[0m\u{1b}[48;2;40;42;54mabc\u{1b}[0m");
    }

    #[test]
    fn paint_with_background_uses_fallback_when_style_has_no_background() {
        let style = Style {
            fg: Some("#8be9fd".to_string()),
            bold: true,
            ..Default::default()
        };

        let painted = paint_with_background("fn", Some(&style), Some("#282a36"));

        assert!(painted.contains("\u{1b}[38;2;139;233;253m"));
        assert!(painted.contains("\u{1b}[48;2;40;42;54m"));
        assert!(painted.contains("\u{1b}[1m"));
    }

    #[test]
    fn paint_with_background_preserves_explicit_style_background() {
        let style = Style {
            fg: Some("#8be9fd".to_string()),
            bg: Some("#ff0000".to_string()),
            ..Default::default()
        };

        let painted = paint_with_background("fn", Some(&style), Some("#282a36"));

        assert!(painted.contains("\u{1b}[48;2;255;0;0m"));
        assert!(!painted.contains("\u{1b}[48;2;40;42;54m"));
    }

    #[test]
    fn paint_with_background_resets_around_newlines() {
        let style = Style {
            fg: Some("#8be9fd".to_string()),
            ..Default::default()
        };

        let painted = paint_with_background("a\nb", Some(&style), Some("#282a36"));

        assert_eq!(
            painted,
            "\u{1b}[0m\u{1b}[38;2;139;233;253m\u{1b}[48;2;40;42;54ma\u{1b}[0m\n\u{1b}[38;2;139;233;253m\u{1b}[48;2;40;42;54mb\u{1b}[0m"
        );
    }

    #[test]
    fn render_pads_lines_to_width_with_custom_background() {
        let formatter = Terminal::new(
            Language::PlainText,
            None,
            Background::Color("#282a36".to_string()),
            Some(5),
        );
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source { start: 0, end: 2 }];
        let mut output = Vec::new();

        formatter.render("hi", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "\u{1b}[0m\u{1b}[48;2;40;42;54mhi\u{1b}[0m\u{1b}[0m\u{1b}[48;2;40;42;54m   \u{1b}[0m"
        );
    }

    #[test]
    fn render_pads_each_line_before_newline() {
        let formatter = Terminal::new(
            Language::PlainText,
            None,
            Background::Color("#282a36".to_string()),
            Some(4),
        );
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source { start: 0, end: 3 }];
        let mut output = Vec::new();

        formatter.render("a\nb", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "\u{1b}[0m\u{1b}[48;2;40;42;54ma\u{1b}[0m\u{1b}[0m\u{1b}[48;2;40;42;54m   \u{1b}[0m\n\u{1b}[0m\u{1b}[48;2;40;42;54mb\u{1b}[0m\u{1b}[0m\u{1b}[48;2;40;42;54m   \u{1b}[0m"
        );
    }

    #[test]
    fn render_uses_theme_background_when_requested() {
        let formatter = Terminal::new(
            Language::PlainText,
            Some(theme_with_background("#282a36")),
            Background::Theme,
            None,
        );
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source { start: 0, end: 2 }];
        let mut output = Vec::new();

        formatter.render("hi", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "\u{1b}[0m\u{1b}[48;2;40;42;54mhi\u{1b}[0m"
        );
    }

    #[test]
    fn render_preserves_styled_newline_when_background_is_inherited() {
        let scope = crate::highlights::HIGHLIGHT_NAMES[0];
        let formatter = Terminal::new(
            Language::PlainText,
            Some(theme_with_scope_style(
                scope,
                Style {
                    fg: Some("#ffffff".to_string()),
                    ..Default::default()
                },
            )),
            Background::Inherit,
            None,
        );
        let events: [HighlightEvent<'_, ()>; 3] = [
            HighlightEvent::Start {
                scope_index: 0,
                language: "plaintext".to_string(),
            },
            HighlightEvent::Source { start: 0, end: 2 },
            HighlightEvent::End,
        ];
        let mut output = Vec::new();

        formatter.render("a\n", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "\u{1b}[0m\u{1b}[38;2;255;255;255ma\n\u{1b}[0m"
        );
    }
}
