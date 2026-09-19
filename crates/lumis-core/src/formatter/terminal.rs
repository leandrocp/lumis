//! Terminal formatter with ANSI color codes.
//!
//! Works with pre-computed highlight events from any source.

use super::{ansi, check_source_ranges, source_text, Formatter};
use crate::decorations::{
    compose_line_decorations, gutter_width, last_line_number, rainbow_scope_index, Decoration,
    LineSelection, SteppedLineRange,
};
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
    line_numbers: bool,
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
        line_numbers: bool,
    ) -> Self {
        Self {
            language,
            theme,
            background,
            width,
            highlight_lines,
            stepped_highlight_lines: Vec::new(),
            line_numbers,
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

    /// The Neovim number-column style for this line.
    fn gutter_style(&self, highlighted: bool) -> Option<&Style> {
        let scope = if highlighted {
            "line_number.highlighted"
        } else {
            "line_number"
        };

        self.theme.as_ref()?.get_style(scope)
    }
}

/// Write one line's number, right-aligned in `width` columns and followed by a
/// space, and return the columns it took.
fn write_gutter(
    output: &mut dyn Write,
    number: usize,
    width: usize,
    style: Option<&Style>,
    line_bg: Option<&str>,
) -> io::Result<usize> {
    let gutter = format!("{number:>width$} ");
    let columns = gutter.chars().count();
    write!(output, "{}", paint_with_background(&gutter, style, line_bg))?;

    Ok(columns)
}

struct RenderState<'a> {
    scope_stack: Vec<(usize, String)>,
    line_width: usize,
    line_bg: Option<&'a str>,
    pending_number: Option<(usize, bool)>,
    decorations: Vec<Decoration>,
}

impl<'a> RenderState<'a> {
    fn new(line_bg: Option<&'a str>) -> Self {
        Self {
            scope_stack: Vec::new(),
            line_width: 0,
            line_bg,
            pending_number: None,
            decorations: Vec::new(),
        }
    }

    fn start_decoration(
        &mut self,
        decoration: Decoration,
        fallback_bg: Option<&'a str>,
        highlight_bg: Option<&'a str>,
        gutter: Option<usize>,
        language: Language,
    ) {
        self.decorations.push(decoration);
        match decoration {
            Decoration::Line {
                number,
                highlighted,
            } => {
                self.line_bg = if highlighted {
                    highlight_bg.or(fallback_bg)
                } else {
                    fallback_bg
                };
                self.line_width = 0;
                self.pending_number = gutter.is_some().then_some((number, highlighted));
            }
            Decoration::RainbowBracket { depth } => self
                .scope_stack
                .push((rainbow_scope_index(depth), language.id_name().to_string())),
        }
    }

    fn end_decoration(&mut self) {
        if matches!(
            self.decorations.pop(),
            Some(Decoration::RainbowBracket { .. })
        ) {
            self.scope_stack.pop();
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
            highlight_lines: None,
            stepped_highlight_lines: Vec::new(),
            line_numbers: false,
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
        let fallback_bg = self.fallback_bg();
        let highlight_bg = self.highlight_bg();

        // A terminal writes one line after another whether or not it is told
        // which lines to mark or number, so the line decorations are only worth
        // composing when one of the two was asked for. Without them the stream,
        // and the output, are exactly what they were.
        let selection = self.line_selection();
        let composed;
        let events: &[HighlightEvent<'_, T>] = if selection.is_empty() && !self.line_numbers {
            events
        } else {
            check_source_ranges(source_bytes, events)?;
            composed = compose_line_decorations(source, events, &selection);
            &composed
        };
        // The gutter is padded to the widest number it will show, which is only
        // known once the lines are.
        let gutter = self
            .line_numbers
            .then(|| gutter_width(last_line_number(events)));
        let mut state = RenderState::new(fallback_bg);

        for event in events {
            match event {
                HighlightEvent::Source { start, end } => {
                    self.write_event_source(
                        output,
                        source_bytes,
                        (*start, *end),
                        fallback_bg,
                        gutter,
                        &mut state,
                    )?;
                }
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => state.scope_stack.push((*scope_index, language.clone())),
                HighlightEvent::End => {
                    state.scope_stack.pop();
                }
                HighlightEvent::DecorationStart { decoration } => {
                    state.start_decoration(
                        *decoration,
                        fallback_bg,
                        highlight_bg,
                        gutter,
                        self.language,
                    );
                }
                HighlightEvent::DecorationEnd => state.end_decoration(),
                // Caller annotations carry data this formatter has never seen.
                HighlightEvent::AnnotationStart { .. } | HighlightEvent::AnnotationEnd => {}
            }
        }

        if !source.ends_with('\n') {
            write_line_padding(output, state.line_bg, self.width, state.line_width)?;
        }

        Ok(())
    }
}

impl Terminal {
    fn write_event_source(
        &self,
        output: &mut dyn Write,
        source: &[u8],
        range: (usize, usize),
        fallback_bg: Option<&str>,
        gutter: Option<usize>,
        state: &mut RenderState<'_>,
    ) -> io::Result<()> {
        let text = source_text(source, range.0, range.1)?;
        if let (false, Some((number, highlighted)), Some(width)) =
            (text.is_empty(), state.pending_number, gutter)
        {
            // Neovim draws the number column with `CursorLineNr` alone, so
            // `CursorLine` does not reach it: a highlighted line's background
            // starts at its text.
            state.line_width = write_gutter(
                output,
                number,
                width,
                self.gutter_style(highlighted),
                fallback_bg,
            )?;
            state.pending_number = None;
        }
        let styled = self.active_style(&state.scope_stack);
        state.line_width =
            self.write_source(output, text, styled, state.line_bg, state.line_width)?;
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
    use std::fmt::Write as _;

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
            false,
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
            false,
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
            false,
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
            false,
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
            false,
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
            false,
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
            false,
        );

        assert_eq!(render_lines(&formatter, "a\nb"), "a\nb");
    }

    fn numbered(theme: Option<Theme>, width: Option<usize>) -> Terminal {
        Terminal::new(
            Language::PlainText,
            theme,
            Background::Inherit,
            width,
            None,
            true,
        )
    }

    /// The gutter is padded to the widest number the render will show, which is
    /// only known once the lines are.
    #[test]
    fn every_number_is_padded_to_the_widest_one() {
        let source = (1..=10).fold(String::new(), |mut acc, n| {
            let _ = writeln!(acc, "{n}");
            acc
        });

        let rendered = render_lines(&numbered(None, None), &source);

        assert_eq!(rendered.lines().next(), Some(" 1 1"));
        assert!(rendered.contains("10 10"), "{rendered}");
    }

    /// A source ending in a newline opens one more line, and a terminal writes
    /// nothing for it. Numbering it would leave a bare number after the output.
    #[test]
    fn the_line_a_trailing_newline_opens_carries_no_number() {
        let rendered = render_lines(&numbered(None, None), "a\n");

        assert_eq!(rendered, "1 a\n");
    }

    /// A blank line in the middle is still a line the terminal writes, so it is
    /// still numbered.
    #[test]
    fn a_blank_line_keeps_its_number() {
        let rendered = render_lines(&numbered(None, None), "a\n\nb");

        assert_eq!(rendered, "1 a\n2 \n3 b");
    }

    #[test]
    fn an_empty_document_renders_nothing() {
        assert_eq!(render_lines(&numbered(None, None), ""), "");
    }

    /// The gutter takes columns, so the padding that fills a line out to `width`
    /// has to count them.
    #[test]
    fn the_gutter_counts_towards_the_width() {
        let formatter = Terminal::new(
            Language::PlainText,
            None,
            Background::Color("#282a36".to_string()),
            Some(6),
            None,
            true,
        );

        let rendered = render_lines(&formatter, "ab");

        // "1 ab" is four columns of the six, so two are padded.
        assert_eq!(
            rendered,
            "\u{1b}[0m\u{1b}[48;2;40;42;54m1 \u{1b}[0m\u{1b}[0m\u{1b}[48;2;40;42;54mab\u{1b}[0m\u{1b}[0m\u{1b}[48;2;40;42;54m  \u{1b}[0m"
        );
    }

    /// Neovim draws the number column with `CursorLineNr` alone, so `CursorLine`
    /// does not reach it. A highlighted line's background starts at its text.
    #[test]
    fn a_highlighted_line_does_not_paint_its_gutter() {
        let formatter = Terminal::new(
            Language::PlainText,
            None,
            Background::Inherit,
            None,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                background: Some("#ff0000".to_string()),
            }),
            true,
        );

        let rendered = render_lines(&formatter, "a\nb");

        assert_eq!(
            rendered, "1 \u{1b}[0m\u{1b}[48;2;255;0;0ma\u{1b}[0m\n2 b",
            "the gutter is outside the highlight: {rendered:?}"
        );
    }

    #[test]
    fn the_gutter_uses_the_line_number_style() {
        let theme = theme_with_scope_style(
            "line_number",
            Style {
                fg: Some("#6272a4".to_string()),
                italic: true,
                ..Default::default()
            },
        );

        let rendered = render_lines(&numbered(Some(theme), None), "a");

        assert_eq!(
            rendered,
            "\u{1b}[0m\u{1b}[38;2;98;114;164m\u{1b}[3m1 \u{1b}[0ma"
        );
    }

    #[test]
    fn a_highlighted_gutter_uses_the_cursor_line_number_style() {
        let theme = Theme {
            name: "test".to_string(),
            highlights: BTreeMap::from([
                (
                    "line_number".to_string(),
                    Style {
                        fg: Some("#6272a4".to_string()),
                        ..Default::default()
                    },
                ),
                (
                    "line_number.highlighted".to_string(),
                    Style {
                        fg: Some("#ff79c6".to_string()),
                        bold: true,
                        ..Default::default()
                    },
                ),
            ]),
            ..Default::default()
        };
        let formatter = Terminal::new(
            Language::PlainText,
            Some(theme),
            Background::Inherit,
            None,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                background: None,
            }),
            true,
        );

        let rendered = render_lines(&formatter, "a\nb");

        assert!(
            rendered.starts_with("\u{1b}[0m\u{1b}[38;2;255;121;198m\u{1b}[1m1 \u{1b}[0ma"),
            "the highlighted gutter should use CursorLineNr: {rendered:?}"
        );
        assert!(
            rendered.contains("\n\u{1b}[0m\u{1b}[38;2;98;114;164m2 \u{1b}[0mb"),
            "the regular gutter should use LineNr: {rendered:?}"
        );
    }

    #[test]
    fn a_highlighted_gutter_falls_back_to_the_line_number_style() {
        let theme = theme_with_scope_style(
            "line_number",
            Style {
                fg: Some("#6272a4".to_string()),
                ..Default::default()
            },
        );
        let formatter = Terminal::new(
            Language::PlainText,
            Some(theme),
            Background::Inherit,
            None,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                background: None,
            }),
            true,
        );

        let rendered = render_lines(&formatter, "a");

        assert_eq!(rendered, "\u{1b}[0m\u{1b}[38;2;98;114;164m1 \u{1b}[0ma");
    }

    /// Line composition clips a `Source` to the document, so asking for
    /// highlighting must not turn a range this formatter refuses into
    /// truncated output.
    #[test]
    fn an_out_of_range_source_is_refused_with_or_without_highlighting() {
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source { start: 0, end: 99 }];

        for highlight_lines in [
            None,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
                background: Some("#ff0000".to_string()),
            }),
        ] {
            let formatter = Terminal::new(
                Language::PlainText,
                None,
                Background::Inherit,
                None,
                highlight_lines,
                false,
            );
            let mut output = Vec::new();

            let error = formatter
                .render("a\nb", &events, &mut output)
                .expect_err("an out-of-range source range is invalid data");

            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        }
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
            false,
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
