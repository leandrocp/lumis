//! The lines of a highlight.
//!
//! Every line-based formatter walks the same line-decorated stream: text split
//! at each newline, terminators left out, and every open scope reopened on the
//! next line. [`lines_from_events`] returns those lines as data; the HTML
//! helpers render them.

use crate::decorations::{
    compose_line_decorations_into, rainbow_scope_index, Decoration, LineSelection,
};
use crate::events::HighlightEvent;
use crate::highlights::HIGHLIGHT_NAMES;
use crate::languages::Language;
use std::ops::Range;

/// The language of a stream's first scope, or `plaintext` when it has none.
///
/// A rainbow bracket belongs to no block, so this is the language a line walk
/// gives it, and the one text outside every scope reports.
pub(crate) fn stream_language<'a, T>(events: &'a [HighlightEvent<'_, T>]) -> &'a str {
    events
        .iter()
        .find_map(HighlightEvent::language)
        .unwrap_or(Language::PlainText.id_name())
}

/// One line of [`lines_from_events`].
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub struct Line<'a, T = ()> {
    /// The 1-based line number.
    pub number: usize,
    /// The line's text in source order, without its terminator. A blank line
    /// has none.
    pub tokens: Vec<Token<'a>>,
    /// The data of every annotation covering any part of the line, once each,
    /// in the order they open.
    pub annotations: Vec<&'a T>,
}

impl<T> Clone for Line<'_, T> {
    fn clone(&self) -> Self {
        Self {
            number: self.number,
            tokens: self.tokens.clone(),
            annotations: self.annotations.clone(),
        }
    }
}

/// A run of one line's text under one scope.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token<'a> {
    /// The text, which never holds a line terminator.
    pub text: &'a str,
    /// Where `text` sits in the source, in UTF-8 bytes.
    pub range: Range<usize>,
    /// The innermost open scope, or `""` outside every scope and for a
    /// `scope_index` past [`HIGHLIGHT_NAMES`].
    pub scope: &'static str,
    /// The innermost open scope's language. A rainbow bracket, and text outside
    /// every scope, report the language of the stream's first scope, or
    /// `plaintext` when it has none.
    pub language: &'a str,
}

/// Split highlight events into lines of tokens and the annotations touching them.
///
/// The same lines [`render_lines_from_events`](super::html::render_lines_from_events)
/// renders, as data: content only,
/// a final newline adds no line, and an empty source is one empty line. A
/// scope that crosses a newline gives one token on each line. Nothing here
/// resolves a style; look one up with
/// [`Theme::get_style`](crate::themes::Theme::get_style) on `format!("{scope}.{language}")`, which falls back to
/// parent scopes.
///
/// An annotation lands on every line it covers any part of, and a point on the
/// line holding it. A point at the very end of a source that ends in a newline
/// lands on the last line, since that newline adds none.
pub fn lines_from_events<'a, T>(
    source: &'a str,
    events: &'a [HighlightEvent<'a, T>],
) -> Vec<Line<'a, T>> {
    let decoration_language = stream_language(events);
    let mut languages: Vec<&'a str> = Vec::new();
    for language in events.iter().filter_map(HighlightEvent::language) {
        if !languages.contains(&language) {
            languages.push(language);
        }
    }

    let mut walk = LineEventWalk::new();
    let mut lines = LineCollector {
        source,
        decoration_language,
        languages,
        last_line: source.lines().count().max(1),
        scopes: Vec::new(),
        open_annotations: Vec::new(),
        listed: Vec::new(),
        lines: Vec::new(),
    };

    compose_line_decorations_into(source, events, &LineSelection::default(), &mut |event| {
        lines.annotation(&event);
        walk.push(&event, source, decoration_language, &mut |fragment| {
            lines.fragment(fragment);
        });
    });

    lines.lines
}

/// What [`lines_from_events`] builds from one line-decorated stream.
struct LineCollector<'a, T> {
    source: &'a str,
    decoration_language: &'a str,
    /// Every language the input names. The composed stream carries copies, and
    /// a token borrows the original instead.
    languages: Vec<&'a str>,
    /// The number of content lines. A later one is the empty line after a
    /// final newline, whose points belong to this one.
    last_line: usize,
    scopes: Vec<(&'static str, &'a str)>,
    /// The range of every annotation open right now, innermost last.
    open_annotations: Vec<Range<usize>>,
    /// How many annotations with each range the current line lists already.
    listed: Vec<(Range<usize>, usize)>,
    lines: Vec<Line<'a, T>>,
}

impl<'a, T> LineCollector<'a, T> {
    /// Record an annotation the first time it opens on the current line.
    ///
    /// Composition closes and reopens an annotation at every line boundary and
    /// around every annotation that opens inside it, but never opens one twice
    /// at once. So two annotations sharing a range are always open together,
    /// and the n-th open one with a range is the n-th distinct one. A point
    /// never reopens, so each one is new. That keys on what every runtime
    /// can compare, without asking whether two annotations' data are equal.
    fn annotation(&mut self, event: &HighlightEvent<'a, T>) {
        match event {
            HighlightEvent::AnnotationStart { range, data } => {
                self.open_annotations.push(range.clone());
                let open = self
                    .open_annotations
                    .iter()
                    .filter(|open| *open == range)
                    .count();
                let index = self
                    .listed
                    .iter()
                    .position(|(listed, _)| listed == range)
                    .unwrap_or_else(|| {
                        self.listed.push((range.clone(), 0));
                        self.listed.len() - 1
                    });
                let listed = &mut self.listed[index].1;
                if range.is_empty() || open > *listed {
                    *listed += 1;
                    if let Some(line) = self.lines.last_mut() {
                        line.annotations.push(*data);
                    }
                }
            }
            HighlightEvent::AnnotationEnd => {
                self.open_annotations.pop();
            }
            _ => {}
        }
    }

    fn fragment(&mut self, fragment: LineFragment<'_>) {
        match fragment {
            LineFragment::OpenLine { number, .. } if number <= self.last_line => {
                self.listed.clear();
                self.lines.push(Line {
                    number,
                    tokens: Vec::new(),
                    annotations: Vec::new(),
                });
            }
            LineFragment::OpenLine { .. } | LineFragment::Close => {}
            LineFragment::Text(range) if range.is_empty() => {}
            LineFragment::Text(range) => {
                let (scope, language) = self
                    .scopes
                    .last()
                    .copied()
                    .unwrap_or(("", self.decoration_language));
                if let Some(line) = self.lines.last_mut() {
                    line.tokens.push(Token {
                        text: &self.source[range.clone()],
                        range,
                        scope,
                        language,
                    });
                }
            }
            LineFragment::SpanOpen(scope_index, language) => {
                let language = self
                    .languages
                    .iter()
                    .copied()
                    .find(|known| *known == language)
                    .unwrap_or(self.decoration_language);
                self.scopes.push((
                    HIGHLIGHT_NAMES.get(scope_index).copied().unwrap_or(""),
                    language,
                ));
            }
            LineFragment::SpanClose => {
                self.scopes.pop();
            }
        }
    }
}

/// One step of a line-decorated event stream.
pub(crate) enum LineFragment<'a> {
    /// A line begins.
    OpenLine { number: usize, highlighted: bool },
    /// The current content-only line ends.
    Close,
    /// Where unescaped source text sits, never spanning a line boundary.
    Text(Range<usize>),
    /// A syntax or built-in decoration scope begins.
    SpanOpen(usize, &'a str),
    /// The innermost syntax or built-in decoration scope ends.
    SpanClose,
}

/// Walk a line-decorated stream, handing each step to `on_fragment`.
///
/// Source terminators are omitted from the line content. Caller annotations
/// are skipped, which is what a built-in formatter does with data it has never seen.
pub(crate) fn write_line_events<T, F>(
    events: &[HighlightEvent<'_, T>],
    source: &str,
    decoration_language: &str,
    mut on_fragment: F,
) where
    F: FnMut(LineFragment<'_>),
{
    let mut walk = LineEventWalk::new();
    for event in events {
        walk.push(event, source, decoration_language, &mut on_fragment);
    }
}

/// [`write_line_events`] one event at a time.
///
/// The walk tracks open decorations while a formatter composes its stream lazily.
pub(crate) struct LineEventWalk {
    decorations: Vec<Decoration>,
}

impl LineEventWalk {
    pub(crate) const fn new() -> Self {
        Self {
            decorations: Vec::new(),
        }
    }

    pub(crate) fn push<T, F>(
        &mut self,
        event: &HighlightEvent<'_, T>,
        source: &str,
        decoration_language: &str,
        on_fragment: &mut F,
    ) where
        F: FnMut(LineFragment<'_>),
    {
        let decorations = &mut self.decorations;
        match event {
            HighlightEvent::DecorationStart { decoration } => {
                decorations.push(*decoration);
                match decoration {
                    Decoration::Line {
                        number,
                        highlighted,
                    } => {
                        on_fragment(LineFragment::OpenLine {
                            number: *number,
                            highlighted: *highlighted,
                        });
                    }
                    Decoration::RainbowBracket { depth } => on_fragment(LineFragment::SpanOpen(
                        rainbow_scope_index(*depth),
                        decoration_language,
                    )),
                }
            }
            HighlightEvent::DecorationEnd => match decorations.pop() {
                Some(Decoration::Line { .. }) => on_fragment(LineFragment::Close),
                Some(Decoration::RainbowBracket { .. }) => on_fragment(LineFragment::SpanClose),
                None => {}
            },
            HighlightEvent::Start {
                scope_index,
                language,
            } => on_fragment(LineFragment::SpanOpen(*scope_index, language)),
            HighlightEvent::End => on_fragment(LineFragment::SpanClose),
            HighlightEvent::Source { start, end } => {
                let range = source_range(source, *start, *end);
                let text = strip_line_ending(
                    &source[range.clone()],
                    source.as_bytes().get(*end) == Some(&b'\n'),
                );
                on_fragment(LineFragment::Text(range.start..range.start + text.len()));
            }
            HighlightEvent::AnnotationStart { .. } | HighlightEvent::AnnotationEnd => {}
        }
    }
}

fn strip_line_ending(text: &str, followed_by_lf: bool) -> &str {
    if let Some(content) = text.strip_suffix('\n') {
        content.strip_suffix('\r').unwrap_or(content)
    } else if followed_by_lf {
        text.strip_suffix('\r').unwrap_or(text)
    } else {
        text
    }
}

/// The largest range of `source` fully inside `start..end`.
///
/// A formatter can build its own events rather than replaying the ones Lumis
/// handed it, so these offsets are caller data. Out of range is clamped, and an
/// offset landing inside a multi-byte character moves to the boundary that keeps
/// the range smaller, because `&source[start..end]` would otherwise panic on a
/// range that split one. Reversed offsets give an empty range.
fn source_range(source: &str, start: usize, end: usize) -> Range<usize> {
    let start = ceil_char_boundary(source, start.min(source.len()));
    let end = floor_char_boundary(source, end.min(source.len())).max(start);

    start..end
}

fn floor_char_boundary(source: &str, mut index: usize) -> usize {
    while index > 0 && !source.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceil_char_boundary(source: &str, mut index: usize) -> usize {
    while index < source.len() && !source.is_char_boundary(index) {
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A formatter can build its own events, so a `Source` range is caller data
    /// and `&source[start..end]` used to panic on one that split a character.
    #[test]
    fn test_source_range_never_panics_on_caller_offsets() {
        let source = "éx";

        assert_eq!(&source[source_range(source, 0, 1)], "", "end splits 'é'");
        assert_eq!(&source[source_range(source, 1, 2)], "", "start splits 'é'");
        assert_eq!(&source[source_range(source, 1, 3)], "x", "start splits 'é'");
        assert_eq!(&source[source_range(source, 0, 2)], "é");
        assert_eq!(&source[source_range(source, 0, 3)], "éx");
        assert_eq!(
            &source[source_range(source, 0, 99)],
            "éx",
            "end past the source"
        );
        assert_eq!(
            &source[source_range(source, 99, 99)],
            "",
            "start past the source"
        );
        assert_eq!(&source[source_range(source, 3, 0)], "", "reversed");
    }
}
