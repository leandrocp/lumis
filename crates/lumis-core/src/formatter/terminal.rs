//! Terminal formatter with ANSI color codes.
//!
//! Works with pre-computed highlight events from any source.

use super::{ansi, Formatter};
use crate::decorations::{compose_line_decorations, Decoration, LineSelection, SteppedLineRange};
use crate::events::HighlightEvent;
use crate::languages::Language;
use crate::themes::{Style, Theme};
use derive_builder::Builder;
use std::io::{self, Write};
use std::ops::RangeInclusive;

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

/// Configuration for highlighting specific lines in terminal output.
///
/// A terminal has no class to hang a stylesheet off, so a highlighted line is
/// painted with a background colour: [`background`](Self::background) when set,
/// otherwise the theme's `highlighted` background. With neither, nothing marks
/// the line.
///
/// Only the background is taken from the theme. A foreground would overwrite
/// the colour every token on the line was already given, which is the one thing
/// a syntax highlighter must not do.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HighlightLines {
    /// List of line ranges to highlight (1-based, inclusive).
    pub lines: Vec<RangeInclusive<usize>>,
    /// The background a highlighted line is painted with.
    pub background: Option<String>,
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
    highlight_lines: Option<HighlightLines>,
    #[builder(setter(skip), default)]
    stepped_highlight_lines: Vec<SteppedLineRange>,
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
        highlight_lines: Option<HighlightLines>,
    ) -> Self {
        Self {
            language,
            theme,
            background,
            width,
            highlight_lines,
            stepped_highlight_lines: Vec::new(),
        }
    }

    /// Supply compact stepped ranges from a language binding.
    #[doc(hidden)]
    pub fn set_stepped_highlight_lines(&mut self, lines: Vec<SteppedLineRange>) {
        self.stepped_highlight_lines = lines;
    }

    fn line_selection(&self) -> LineSelection {
        LineSelection::new(
            self.highlight_lines
                .as_ref()
                .map_or(&[][..], |highlight| &highlight.lines),
            &self.stepped_highlight_lines,
        )
    }

    fn fallback_bg(&self) -> Option<&str> {
        match &self.background {
            Background::Inherit => None,
            Background::Theme => self.theme.as_ref().and_then(Theme::bg),
            Background::Color(color) => Some(color.as_str()),
        }
    }

    /// The theme style of the innermost open scope, if any.
    fn active_style(&self, scope_stack: &[(usize, String)]) -> Option<&Style> {
        let (scope_index, language) = scope_stack.last()?;
        let theme = self.theme.as_ref()?;
        let scope = crate::highlights::HIGHLIGHT_NAMES
            .get(*scope_index)
            .copied()
            .unwrap_or("");

        theme
            .get_style(&format!("{scope}.{language}"))
            .or_else(|| theme.get_style(scope))
    }

    /// Paint one source event, padding each line it ends out to the width.
    ///
    /// Returns the width the current line has reached.
    fn write_source(
        &self,
        output: &mut dyn Write,
        text: &str,
        styled: Option<&Style>,
        line_bg: Option<&str>,
        mut line_width: usize,
    ) -> io::Result<usize> {
        // Without a background there is nothing to pad out to, so the text goes
        // out in one piece and the width is never needed.
        let Some(line_bg) = line_bg else {
            write!(output, "{}", paint_with_background(text, styled, None))?;
            return Ok(line_width);
        };

        for segment in text.split_inclusive('\n') {
            let content = segment.strip_suffix('\n');

            if !content.unwrap_or(segment).is_empty() {
                let painted =
                    paint_with_background(content.unwrap_or(segment), styled, Some(line_bg));
                write!(output, "{painted}")?;
                line_width += display_width(content.unwrap_or(segment));
            }

            if content.is_some() {
                write_line_padding(output, Some(line_bg), self.width, line_width)?;
                writeln!(output)?;
                line_width = 0;
            }
        }

        Ok(line_width)
    }

    /// The background a highlighted line is painted with, if any.
    fn highlight_bg(&self) -> Option<&str> {
        let highlight = self.highlight_lines.as_ref()?;
        if let Some(background) = highlight.background.as_deref() {
            return Some(background);
        }

        self.theme.as_ref()?.get_style("highlighted")?.bg.as_deref()
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            theme: None,
            background: Background::Inherit,
            width: None,
            highlight_lines: None,
            stepped_highlight_lines: Vec::new(),
        }
    }
}

impl<T> Formatter<T> for Terminal {
    fn language(&self) -> Language {
        self.language
    }

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
        let highlight_bg = self.highlight_bg();

        // A terminal writes one line after another whether or not it is told
        // which lines to mark, so the line decorations are only worth composing
        // when there is something to mark. Without them the stream, and the
        // output, are exactly what they were.
        let selection = self.line_selection();
        let composed;
        let events: &[HighlightEvent<'_, T>] = if selection.is_empty() {
            events
        } else {
            composed = compose_line_decorations(source, events, &selection);
            &composed
        };
        let mut line_bg = fallback_bg;

        for event in events {
            match event {
                HighlightEvent::Source { start, end } => {
                    let text = source_text(source_bytes, *start, *end)?;
                    let styled = self.active_style(&scope_stack);
                    line_width = self.write_source(output, text, styled, line_bg, line_width)?;
                }
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => scope_stack.push((*scope_index, language.clone())),
                HighlightEvent::End => {
                    scope_stack.pop();
                }
                HighlightEvent::DecorationStart {
                    decoration: Decoration::Line { highlighted, .. },
                } => {
                    line_bg = if *highlighted {
                        highlight_bg.or(fallback_bg)
                    } else {
                        fallback_bg
                    };
                    line_width = 0;
                }
                // Caller annotations carry data this formatter has never seen.
                HighlightEvent::AnnotationStart { .. }
                | HighlightEvent::AnnotationEnd
                | HighlightEvent::DecorationEnd => {}
            }
        }

        if !source.ends_with('\n') {
            write_line_padding(output, line_bg, self.width, line_width)?;
        }

        Ok(())
    }
}

/// The source slice an event names, or an error when it names one that is not
/// there. A formatter can be handed events it did not produce.
fn source_text(source_bytes: &[u8], start: usize, end: usize) -> io::Result<&str> {
    if start > end || end > source_bytes.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "invalid source range: {start}..{end} (len={})",
                source_bytes.len()
            ),
        ));
    }

    std::str::from_utf8(&source_bytes[start..end])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
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
            None,
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
            None,
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

    fn theme_with_highlighted_background(bg: &str) -> Theme {
        Theme {
            name: "test".to_string(),
            highlights: BTreeMap::from([(
                "highlighted".to_string(),
                Style {
                    bg: Some(bg.to_string()),
                    ..Default::default()
                },
            )]),
            ..Default::default()
        }
    }

    fn render_lines(formatter: &Terminal, source: &str) -> String {
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source {
            start: 0,
            end: source.len(),
        }];
        let mut output = Vec::new();
        formatter.render(source, &events, &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    #[test]
    fn highlighted_lines_take_the_theme_background() {
        let formatter = Terminal::new(
            Language::PlainText,
            Some(theme_with_highlighted_background("#3a3a3a")),
            Background::Inherit,
            None,
            Some(HighlightLines {
                lines: std::iter::once(2..=2).collect(),
                background: None,
            }),
        );

        assert_eq!(
            render_lines(&formatter, "a\nb\nc"),
            "a\n\u{1b}[0m\u{1b}[48;2;58;58;58mb\u{1b}[0m\nc"
        );
    }

    #[test]
    fn an_explicit_background_wins_over_the_theme() {
        let formatter = Terminal::new(
            Language::PlainText,
            Some(theme_with_highlighted_background("#3a3a3a")),
            Background::Inherit,
            None,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                background: Some("#ff0000".to_string()),
            }),
        );

        assert!(render_lines(&formatter, "a\nb").contains("\u{1b}[48;2;255;0;0m"));
    }

    /// A highlighted line pads out to the width so its background reaches the
    /// edge, which is the whole point of marking the line.
    #[test]
    fn a_highlighted_line_pads_to_the_width() {
        let formatter = Terminal::new(
            Language::PlainText,
            None,
            Background::Inherit,
            Some(4),
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                background: Some("#282a36".to_string()),
            }),
        );

        assert_eq!(
            render_lines(&formatter, "a\nb"),
            "\u{1b}[0m\u{1b}[48;2;40;42;54ma\u{1b}[0m\u{1b}[0m\u{1b}[48;2;40;42;54m   \u{1b}[0m\nb"
        );
    }

    /// Without a theme `highlighted` style and without a background, the option
    /// has nothing to paint with and the line is left alone.
    #[test]
    fn nothing_marks_the_line_without_a_colour_to_mark_it_with() {
        let formatter = Terminal::new(
            Language::PlainText,
            None,
            Background::Inherit,
            None,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                background: None,
            }),
        );

        assert_eq!(render_lines(&formatter, "a\nb"), "a\nb");
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
