//! Highlight event types for the rendering pipeline.
//!
//! These events represent the output of syntax highlighting (from tree-sitter or any other source)
//! in a format that is independent of tree-sitter's C FFI types. Formatters in lumis-core consume
//! these events to produce HTML, terminal output, etc.

use std::ops::Range;

/// Re-exported beside the event that carries it for convenient matching.
pub use crate::decorations::Decoration;

/// A single step in rendering syntax-highlighted source.
///
/// This enum mirrors tree-sitter's `HighlightEvent` but uses plain Rust types,
/// making it usable without any tree-sitter dependency. Lumis can enrich the
/// stream with caller-provided annotations and built-in decorations before a
/// formatter consumes it.
///
/// Lumis adds event kinds as it grows, so a formatter matches the ones it
/// renders and ignores the rest. That is what the built-in formatters do with
/// annotations, which they cannot render without knowing the caller's data.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum HighlightEvent<'a, T = ()> {
    /// A highlight scope begins.
    ///
    /// `scope_index` is an index into the `HIGHLIGHT_NAMES` array.
    /// `language` is the language name this highlight belongs to (e.g., "rust", "markdown").
    Start {
        scope_index: usize,
        language: String,
    },
    /// A range of the source text that should be included in the output.
    ///
    /// `start` and `end` are byte offsets into the source string.
    Source { start: usize, end: usize },
    /// A highlight scope ends.
    End,
    /// A caller-provided annotation begins.
    AnnotationStart {
        /// The resolved half-open offset range, measured in UTF-8 bytes.
        range: Range<usize>,
        /// The caller-owned data interpreted by the formatter.
        data: &'a T,
    },
    /// The current caller-provided annotation ends.
    AnnotationEnd,
    /// A Lumis-owned decoration begins.
    ///
    /// Unlike an annotation, its payload is a closed set every built-in
    /// formatter understands, so the built-ins render these rather than skip
    /// them.
    DecorationStart {
        /// What Lumis is marking here.
        decoration: Decoration,
    },
    /// The current Lumis-owned decoration ends.
    DecorationEnd,
}

impl<T> Clone for HighlightEvent<'_, T> {
    fn clone(&self) -> Self {
        match self {
            Self::Start {
                scope_index,
                language,
            } => Self::Start {
                scope_index: *scope_index,
                language: language.clone(),
            },
            Self::Source { start, end } => Self::Source {
                start: *start,
                end: *end,
            },
            Self::End => Self::End,
            Self::AnnotationStart { range, data } => Self::AnnotationStart {
                range: range.clone(),
                data: *data,
            },
            Self::AnnotationEnd => Self::AnnotationEnd,
            Self::DecorationStart { decoration } => Self::DecorationStart {
                decoration: *decoration,
            },
            Self::DecorationEnd => Self::DecorationEnd,
        }
    }
}

impl<T> HighlightEvent<'_, T> {
    /// The scope name a `Start` opens, e.g. `keyword.function`.
    ///
    /// JavaScript's `HighlightEvent` carries the name directly; Rust carries an
    /// index into [`HIGHLIGHT_NAMES`](crate::highlights::HIGHLIGHT_NAMES)
    /// because resolving it per event costs more than the formatters need. This
    /// resolves it, so a custom formatter reads the same value in both.
    ///
    /// Returns `None` for every other variant, and for a `scope_index` outside
    /// `HIGHLIGHT_NAMES` — an event built by hand rather than by highlighting.
    pub fn scope(&self) -> Option<&'static str> {
        match self {
            Self::Start { scope_index, .. } => crate::highlights::HIGHLIGHT_NAMES
                .get(*scope_index)
                .copied(),
            _ => None,
        }
    }

    /// The language a `Start` belongs to, e.g. `rust`.
    ///
    /// Returns `None` for every other variant.
    pub fn language(&self) -> Option<&str> {
        match self {
            Self::Start { language, .. } => Some(language),
            _ => None,
        }
    }
}

/// Builds a highlight event stream that carries no scope it is about to reopen
/// and no source range it is about to continue.
///
/// tree-sitter emits one `HighlightStart`/`Source`/`HighlightEnd` triple per
/// capture, so a run of neighbouring tokens that share a scope arrives as that
/// triple repeated. Every formatter then writes one element per token:
/// 100,000 unclosed `[` become 100,000 copies of
/// `<span class="l-punctuation-bracket">[</span>`, which is 44 bytes of output
/// per byte of input. Closing a scope only to reopen the identical one renders
/// the same way as leaving it open, so this drops the pair, and the token text
/// merges into the run already inside it.
///
/// Both merges are local: a `Start` merges only when the event just written is
/// the `End` of an identical scope, and a `Source` merges only when it
/// continues the previous one. Nesting, ordering and the byte ranges the stream
/// covers are unchanged.
pub struct Coalescing<'a, T = ()> {
    events: Vec<HighlightEvent<'a, T>>,
    open: Vec<usize>,
    reopenable: Option<usize>,
}

impl<T> Default for Coalescing<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Coalescing<'a, T> {
    /// An empty stream.
    pub const fn new() -> Self {
        Self {
            events: Vec::new(),
            open: Vec::new(),
            reopenable: None,
        }
    }

    /// Open `scope_index` for `language`, reopening the run just closed when it
    /// is the same scope.
    pub fn start(&mut self, scope_index: usize, language: String) {
        if let Some(index) = self.reopenable.take() {
            if matches!(
                &self.events[index],
                HighlightEvent::Start {
                    scope_index: open_scope,
                    language: open_language,
                } if *open_scope == scope_index && *open_language == language
            ) {
                self.events.pop();
                self.open.push(index);
                return;
            }
        }

        self.open.push(self.events.len());
        self.events.push(HighlightEvent::Start {
            scope_index,
            language,
        });
    }

    /// Close the innermost open scope.
    pub fn end(&mut self) {
        self.reopenable = self.open.pop();
        self.events.push(HighlightEvent::End);
    }

    /// Emit `start..end` of the source, extending the previous range when this
    /// one continues it.
    pub fn source(&mut self, start: usize, end: usize) {
        self.reopenable = None;
        if let Some(HighlightEvent::Source {
            end: previous_end, ..
        }) = self.events.last_mut()
        {
            if *previous_end == start {
                *previous_end = end;
                return;
            }
        }

        self.events.push(HighlightEvent::Source { start, end });
    }

    /// The collected stream.
    pub fn finish(self) -> Vec<HighlightEvent<'a, T>> {
        self.events
    }
}

#[cfg(test)]
mod coalescing_tests {
    use super::*;

    fn scopes_and_text(events: &[HighlightEvent<'_>]) -> Vec<String> {
        events
            .iter()
            .map(|event| match event {
                HighlightEvent::Start { scope_index, .. } => format!("<{scope_index}"),
                HighlightEvent::End => ">".to_string(),
                HighlightEvent::Source { start, end } => format!("{start}..{end}"),
                _ => "?".to_string(),
            })
            .collect()
    }

    #[test]
    fn a_scope_reopened_with_nothing_between_stays_open() {
        let mut stream = Coalescing::<()>::new();
        stream.start(1, "json".to_string());
        stream.source(0, 1);
        stream.end();
        stream.start(1, "json".to_string());
        stream.source(1, 2);
        stream.end();

        assert_eq!(scopes_and_text(&stream.finish()), ["<1", "0..2", ">"]);
    }

    #[test]
    fn a_different_scope_or_language_opens_its_own_run() {
        let mut stream = Coalescing::<()>::new();
        stream.start(1, "json".to_string());
        stream.source(0, 1);
        stream.end();
        stream.start(2, "json".to_string());
        stream.source(1, 2);
        stream.end();
        stream.start(2, "html".to_string());
        stream.source(2, 3);
        stream.end();

        assert_eq!(
            scopes_and_text(&stream.finish()),
            ["<1", "0..1", ">", "<2", "1..2", ">", "<2", "2..3", ">"]
        );
    }

    #[test]
    fn text_between_two_runs_of_one_scope_keeps_them_apart() {
        let mut stream = Coalescing::<()>::new();
        stream.start(1, "json".to_string());
        stream.source(0, 1);
        stream.end();
        stream.source(1, 2);
        stream.start(1, "json".to_string());
        stream.source(2, 3);
        stream.end();

        assert_eq!(
            scopes_and_text(&stream.finish()),
            ["<1", "0..1", ">", "1..2", "<1", "2..3", ">"]
        );
    }

    #[test]
    fn a_source_range_that_does_not_continue_the_last_one_is_its_own_event() {
        let mut stream = Coalescing::<()>::new();
        stream.source(0, 1);
        stream.source(2, 3);

        assert_eq!(scopes_and_text(&stream.finish()), ["0..1", "2..3"]);
    }

    #[test]
    fn an_inner_scope_reopens_without_disturbing_the_one_around_it() {
        let mut stream = Coalescing::<()>::new();
        stream.start(1, "json".to_string());
        stream.start(2, "json".to_string());
        stream.source(0, 1);
        stream.end();
        stream.start(2, "json".to_string());
        stream.source(1, 2);
        stream.end();
        stream.end();

        assert_eq!(
            scopes_and_text(&stream.finish()),
            ["<1", "<2", "0..2", ">", ">"]
        );
    }

    #[test]
    fn closing_an_outer_scope_after_an_inner_one_does_not_reopen_the_inner() {
        let mut stream = Coalescing::<()>::new();
        stream.start(1, "json".to_string());
        stream.start(2, "json".to_string());
        stream.source(0, 1);
        stream.end();
        stream.end();
        stream.start(2, "json".to_string());
        stream.source(1, 2);
        stream.end();

        assert_eq!(
            scopes_and_text(&stream.finish()),
            ["<1", "<2", "0..1", ">", ">", "<2", "1..2", ">"]
        );
    }
}
