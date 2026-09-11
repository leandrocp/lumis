//! Lumis-owned overlays composed into the highlight event stream.
//!
//! An [`Annotation`](crate::annotations::Annotation) carries data only the
//! caller understands, so the built-in formatters skip it. A [`Decoration`]
//! carries data Lumis owns, from a closed set every built-in formatter can
//! switch on, so it reaches the same event stream and is rendered rather than
//! skipped.
//!
//! Line highlighting is the first of them. A highlighted line is a line range
//! plus a payload — the same shape as an annotation — and it now travels as
//! [`Decoration::Line`] rather than as a second pass over strings the
//! formatters have already produced.

use crate::annotations::ResolvedAnnotation;
use crate::events::HighlightEvent;
use std::ops::RangeInclusive;

/// An overlay whose data Lumis owns and every built-in formatter understands.
///
/// Delivered as
/// [`DecorationStart`](crate::events::HighlightEvent::DecorationStart) and
/// [`DecorationEnd`](crate::events::HighlightEvent::DecorationEnd), interleaved
/// with the syntax scopes and the caller's annotations.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Decoration {
    /// One rendered line of the document.
    ///
    /// Every line carries one, highlighted or not, because that is what lets a
    /// formatter number the lines it emits. The decoration covers the line's
    /// text *and* the newline that ends it; the last line of a source that does
    /// not end in one covers just the text.
    Line {
        /// The 1-based line number.
        number: usize,
        /// Whether the caller asked for this line to be highlighted.
        highlighted: bool,
    },
}

/// A finite arithmetic progression of 1-based line numbers.
///
/// This is public only so language bindings can preserve a source runtime's
/// stepped-range semantics without expanding the range into one allocation per
/// selected line. Rust's formatter options continue to use
/// [`RangeInclusive<usize>`].
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SteppedLineRange {
    first: usize,
    last: usize,
    step: usize,
}

impl SteppedLineRange {
    /// Normalize an ascending or descending stepped range.
    ///
    /// Returns `None` for a zero step or when the step points away from the end.
    #[doc(hidden)]
    pub fn new(start: usize, end: usize, step: isize) -> Option<Self> {
        let stride = step.unsigned_abs();
        if stride == 0 {
            return None;
        }

        let (first, last) = if step > 0 {
            if start > end {
                return None;
            }
            let span = end - start;
            (start, start + span / stride * stride)
        } else {
            if start < end {
                return None;
            }
            let span = start - end;
            (start - span / stride * stride, start)
        };

        Some(Self {
            first,
            last,
            step: stride,
        })
    }

    /// Whether `line_number` is one of the range's selected lines.
    #[doc(hidden)]
    pub fn contains(&self, line_number: usize) -> bool {
        (self.first..=self.last).contains(&line_number)
            && (line_number - self.first).is_multiple_of(self.step)
    }
}

/// The lines a formatter was asked to highlight, resolved once.
///
/// Contiguous ranges are clamped, sorted and merged, so testing lines in
/// ascending order walks them with a cursor rather than rescanning. A stepped
/// range stays as three numbers and answers by arithmetic. Neither depends on
/// how many lines the document has, so a selection costs the same whether it
/// covers ten lines or a billion.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LineSelection {
    /// Merged, disjoint, ascending, 1-based and inclusive at both ends.
    spans: Vec<(usize, usize)>,
    /// Only ranges with a stride above 1; a stride of 1 is a span either way round.
    stepped: Vec<SteppedLineRange>,
}

impl LineSelection {
    /// Resolves the plain and stepped ranges a formatter was configured with.
    pub(crate) fn new(ranges: &[RangeInclusive<usize>], stepped: &[SteppedLineRange]) -> Self {
        let mut spans: Vec<(usize, usize)> = ranges
            .iter()
            .map(|range| (*range.start(), *range.end()))
            .chain(
                stepped
                    .iter()
                    .filter(|range| range.step == 1)
                    .map(|range| (range.first, range.last)),
            )
            // Line 0 does not exist, and a reversed range covers nothing.
            .filter_map(|(start, end)| (start.max(1) <= end).then_some((start.max(1), end)))
            .collect();
        spans.sort_unstable();
        spans.dedup_by(|(next_start, next_end), (_, end)| {
            // `dedup_by` drops the first argument when this returns true, so
            // widening the second is how a merged span survives.
            let mergeable = *next_start <= end.saturating_add(1);
            if mergeable {
                *end = (*end).max(*next_end);
            }
            mergeable
        });

        Self {
            spans,
            stepped: stepped
                .iter()
                .filter(|range| range.step > 1)
                .cloned()
                .collect(),
        }
    }

    /// Whether no line is selected.
    pub(crate) fn is_empty(&self) -> bool {
        self.spans.is_empty() && self.stepped.is_empty()
    }

    /// A cursor for testing line numbers in ascending order.
    fn cursor(&self) -> LineCursor<'_> {
        LineCursor {
            selection: self,
            index: 0,
        }
    }
}

/// Tests ascending line numbers against a [`LineSelection`].
struct LineCursor<'a> {
    selection: &'a LineSelection,
    index: usize,
}

impl LineCursor<'_> {
    /// Whether `line` is selected.
    ///
    /// `line` must not go backwards between calls, which is how the composer
    /// visits lines.
    fn contains(&mut self, line: usize) -> bool {
        let spans = &self.selection.spans;
        while self.index < spans.len() && spans[self.index].1 < line {
            self.index += 1;
        }

        spans
            .get(self.index)
            .is_some_and(|(start, end)| *start <= line && line <= *end)
            || self
                .selection
                .stepped
                .iter()
                .any(|range| range.contains(line))
    }
}

/// One layer held open across a line boundary.
enum OpenLayer<'a, T> {
    Syntax {
        scope_index: usize,
        language: String,
    },
    Annotation(ResolvedAnnotation<'a, T>),
}

impl<'a, T> OpenLayer<'a, T> {
    fn open_event(&self) -> HighlightEvent<'a, T> {
        match self {
            Self::Syntax {
                scope_index,
                language,
            } => HighlightEvent::Start {
                scope_index: *scope_index,
                language: language.clone(),
            },
            Self::Annotation(annotation) => HighlightEvent::AnnotationStart {
                annotation: annotation.clone(),
            },
        }
    }

    fn close_event(&self) -> HighlightEvent<'a, T> {
        match self {
            Self::Syntax { .. } => HighlightEvent::End,
            Self::Annotation(_) => HighlightEvent::AnnotationEnd,
        }
    }
}

/// Compose one [`Decoration::Line`] per rendered line into `events`.
///
/// Lines are the outermost layer, so every syntax scope and caller annotation
/// still open at a newline is closed before the line ends and reopened on the
/// next one. That is the same close-and-reopen
/// [`compose_annotations`](crate::annotations::compose_annotations) does for a
/// scope an annotation cuts across, and it is why a formatter can write the
/// stream straight out instead of assembling lines and wrapping them
/// afterwards.
///
/// The returned stream always holds at least one line: an empty document is one
/// empty line, the same line a caller sees numbered `1`.
///
/// Line decorations already present in `events` are dropped rather than nested,
/// so composing twice gives the same answer as composing once.
pub(crate) fn compose_line_decorations<'a, T>(
    source: &str,
    events: &[HighlightEvent<'a, T>],
    selection: &LineSelection,
) -> Vec<HighlightEvent<'a, T>> {
    // One decoration pair per line, plus a close and reopen of every layer that
    // crosses one. The syntax events themselves pass through.
    let mut output = Vec::with_capacity(events.len() + 2);
    let mut layers: Vec<OpenLayer<'a, T>> = Vec::new();
    let mut cursor = selection.cursor();
    let mut line = 1usize;

    output.push(line_start(line, &mut cursor));

    for event in events {
        match event {
            HighlightEvent::Start {
                scope_index,
                language,
            } => {
                output.push(event.clone());
                layers.push(OpenLayer::Syntax {
                    scope_index: *scope_index,
                    language: language.clone(),
                });
            }
            HighlightEvent::End => {
                if matches!(layers.last(), Some(OpenLayer::Syntax { .. })) {
                    layers.pop();
                    output.push(HighlightEvent::End);
                }
            }
            HighlightEvent::AnnotationStart { annotation } => {
                output.push(event.clone());
                layers.push(OpenLayer::Annotation(annotation.clone()));
            }
            HighlightEvent::AnnotationEnd => {
                if matches!(layers.last(), Some(OpenLayer::Annotation(_))) {
                    layers.pop();
                    output.push(HighlightEvent::AnnotationEnd);
                }
            }
            HighlightEvent::Source { start, end } => {
                split_source(
                    &mut output,
                    source,
                    *start,
                    *end,
                    &layers,
                    &mut line,
                    &mut cursor,
                );
            }
            // A stream that already carries lines is re-composed, not nested.
            HighlightEvent::DecorationStart { .. } | HighlightEvent::DecorationEnd => {}
        }
    }

    // An unbalanced input stream would otherwise leave a scope open past the
    // last line, which no formatter can close.
    close_layers(&mut output, &layers);
    output.push(HighlightEvent::DecorationEnd);

    output
}

fn line_start<'a, T>(line: usize, cursor: &mut LineCursor<'_>) -> HighlightEvent<'a, T> {
    HighlightEvent::DecorationStart {
        decoration: Decoration::Line {
            number: line,
            highlighted: cursor.contains(line),
        },
    }
}

fn close_layers<'a, T>(output: &mut Vec<HighlightEvent<'a, T>>, layers: &[OpenLayer<'a, T>]) {
    for layer in layers.iter().rev() {
        output.push(layer.close_event());
    }
}

fn reopen_layers<'a, T>(output: &mut Vec<HighlightEvent<'a, T>>, layers: &[OpenLayer<'a, T>]) {
    for layer in layers {
        output.push(layer.open_event());
    }
}

/// Emit `start..end`, ending a line at every newline it contains.
///
/// The newline stays inside the line it ends, so concatenating the `Source`
/// events of the composed stream still reproduces the source exactly.
fn split_source<'a, T>(
    output: &mut Vec<HighlightEvent<'a, T>>,
    source: &str,
    start: usize,
    end: usize,
    layers: &[OpenLayer<'a, T>],
    line: &mut usize,
    cursor: &mut LineCursor<'_>,
) {
    let mut cursor_offset = start.min(source.len());
    let end = end.min(source.len()).max(cursor_offset);

    while cursor_offset < end {
        let Some(newline) = source.as_bytes()[cursor_offset..end]
            .iter()
            .position(|byte| *byte == b'\n')
        else {
            break;
        };

        let line_end = cursor_offset + newline + 1;
        output.push(HighlightEvent::Source {
            start: cursor_offset,
            end: line_end,
        });
        close_layers(output, layers);
        output.push(HighlightEvent::DecorationEnd);

        *line += 1;
        output.push(line_start(*line, cursor));
        reopen_layers(output, layers);

        cursor_offset = line_end;
    }

    if cursor_offset < end {
        output.push(HighlightEvent::Source {
            start: cursor_offset,
            end,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotations::{compose_annotations, Annotation};

    fn selection(ranges: impl IntoIterator<Item = RangeInclusive<usize>>) -> LineSelection {
        LineSelection::new(&ranges.into_iter().collect::<Vec<_>>(), &[])
    }

    fn lines<T>(events: &[HighlightEvent<'_, T>]) -> Vec<(usize, bool)> {
        events
            .iter()
            .filter_map(|event| match event {
                HighlightEvent::DecorationStart {
                    decoration:
                        Decoration::Line {
                            number,
                            highlighted,
                        },
                } => Some((*number, *highlighted)),
                _ => None,
            })
            .collect()
    }

    fn rendered<T>(source: &str, events: &[HighlightEvent<'_, T>]) -> String {
        let mut out = String::new();
        for event in events {
            if let HighlightEvent::Source { start, end } = event {
                out.push_str(&source[*start..*end]);
            }
        }
        out
    }

    #[test]
    fn an_empty_document_is_one_line() {
        let events: [HighlightEvent<'_>; 0] = [];

        let composed = compose_line_decorations("", &events, &LineSelection::default());

        assert_eq!(lines(&composed), [(1, false)]);
        assert_eq!(composed.len(), 2, "a start and an end: {composed:?}");
    }

    #[test]
    fn a_trailing_newline_opens_one_more_line() {
        let source = "a\n";
        let events = [HighlightEvent::<()>::Source { start: 0, end: 2 }];

        let composed = compose_line_decorations(source, &events, &LineSelection::default());

        assert_eq!(lines(&composed), [(1, false), (2, false)]);
        assert_eq!(rendered(source, &composed), source);
    }

    #[test]
    fn source_events_still_reproduce_the_source() {
        let source = "one\ntwo\nthree";
        let events = [HighlightEvent::<()>::Source {
            start: 0,
            end: source.len(),
        }];

        let composed =
            compose_line_decorations(source, &events, &selection(std::iter::once(2..=2)));

        assert_eq!(lines(&composed), [(1, false), (2, true), (3, false)]);
        assert_eq!(rendered(source, &composed), source);
    }

    #[test]
    fn a_scope_crossing_a_newline_is_closed_and_reopened() {
        let source = "a\nb";
        let events = [
            HighlightEvent::<()>::Start {
                scope_index: 1,
                language: "rust".to_string(),
            },
            HighlightEvent::Source { start: 0, end: 3 },
            HighlightEvent::End,
        ];

        let composed = compose_line_decorations(source, &events, &LineSelection::default());

        assert_eq!(
            composed,
            [
                HighlightEvent::DecorationStart {
                    decoration: Decoration::Line {
                        number: 1,
                        highlighted: false
                    }
                },
                HighlightEvent::Start {
                    scope_index: 1,
                    language: "rust".to_string(),
                },
                HighlightEvent::Source { start: 0, end: 2 },
                HighlightEvent::End,
                HighlightEvent::DecorationEnd,
                HighlightEvent::DecorationStart {
                    decoration: Decoration::Line {
                        number: 2,
                        highlighted: false
                    }
                },
                HighlightEvent::Start {
                    scope_index: 1,
                    language: "rust".to_string(),
                },
                HighlightEvent::Source { start: 2, end: 3 },
                HighlightEvent::End,
                HighlightEvent::DecorationEnd,
            ]
        );
    }

    #[test]
    fn an_annotation_crossing_a_newline_is_closed_and_reopened() {
        let source = "ab\ncd";
        let syntax = [HighlightEvent::Source { start: 0, end: 5 }];
        let annotations = [Annotation::new(1..4, "span").unwrap()];
        let events = compose_annotations(source, &syntax, &annotations).unwrap();

        let composed = compose_line_decorations(source, &events, &LineSelection::default());

        let starts = composed
            .iter()
            .filter(|event| matches!(event, HighlightEvent::AnnotationStart { .. }))
            .count();
        let ends = composed
            .iter()
            .filter(|event| matches!(event, HighlightEvent::AnnotationEnd))
            .count();

        assert_eq!((starts, ends), (2, 2));
        assert_eq!(rendered(source, &composed), source);
    }

    #[test]
    fn an_unbalanced_stream_still_closes_before_the_last_line_ends() {
        let events = [HighlightEvent::<()>::Start {
            scope_index: 1,
            language: "rust".to_string(),
        }];

        let composed = compose_line_decorations("", &events, &LineSelection::default());

        assert_eq!(
            composed.last(),
            Some(&HighlightEvent::DecorationEnd),
            "{composed:?}"
        );
        assert!(
            matches!(composed[composed.len() - 2], HighlightEvent::End),
            "{composed:?}"
        );
    }

    #[test]
    fn composing_twice_gives_the_same_stream() {
        let source = "a\nb\n";
        let events = [HighlightEvent::<()>::Source { start: 0, end: 4 }];

        let once = compose_line_decorations(source, &events, &selection(std::iter::once(1..=1)));
        let twice = compose_line_decorations(source, &once, &selection(std::iter::once(1..=1)));

        assert_eq!(once, twice);
    }

    #[test]
    fn overlapping_ranges_merge_and_line_zero_is_dropped() {
        let selection = selection([0..=2, 2..=4, 8..=9]);

        assert_eq!(selection.spans, [(1, 4), (8, 9)]);
    }

    #[test]
    fn adjacent_ranges_merge() {
        assert_eq!(selection([1..=2, 3..=4]).spans, [(1, 4)]);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn a_reversed_range_selects_nothing() {
        assert!(selection(std::iter::once(5..=3)).is_empty());
    }

    #[test]
    fn a_selection_costs_nothing_for_a_range_larger_than_the_document() {
        let huge = selection(std::iter::once(1..=usize::MAX));
        let mut cursor = huge.cursor();

        assert!(cursor.contains(1));
        assert!(cursor.contains(1_000_000_000));
    }

    #[test]
    fn a_stride_of_one_is_a_span_either_way_round() {
        let ascending = SteppedLineRange::new(1, 1_000_000_000, 1).unwrap();
        let descending = SteppedLineRange::new(1_000_000_000, 1, -1).unwrap();

        let selection = LineSelection::new(&[], &[ascending, descending]);

        assert_eq!(selection.spans, [(1, 1_000_000_000)]);
        assert!(selection.stepped.is_empty());
    }

    #[test]
    fn stepped_ranges_stay_compact() {
        let ascending = SteppedLineRange::new(1, 1_000_000_000, 2).unwrap();
        let descending = SteppedLineRange::new(1_000_000_000, 1, -3).unwrap();
        let selection = LineSelection::new(&[], &[ascending, descending]);
        let mut cursor = selection.cursor();

        assert_eq!(
            (1..=6)
                .map(|line| cursor.contains(line))
                .collect::<Vec<_>>(),
            [true, false, true, true, true, false],
        );
    }

    #[test]
    fn a_descending_stepped_range_keeps_the_lines_the_step_lands_on() {
        let unaligned = SteppedLineRange::new(10, 2, -3).unwrap();

        assert!(unaligned.contains(4));
        assert!(!unaligned.contains(2));
    }

    #[test]
    fn many_ranges_merge_before_any_line_is_tested() {
        let ranges: Vec<_> = (1..=20_000).step_by(2).map(|line| line..=line).collect();
        let selection = selection(ranges);
        let mut cursor = selection.cursor();

        assert_eq!(selection.spans.len(), 10_000);
        assert_eq!(
            (1..=20_000).filter(|line| cursor.contains(*line)).count(),
            10_000
        );
    }
}

#[cfg(test)]
#[path = "decorations_parity.rs"]
mod parity;
