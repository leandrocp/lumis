//! Highlight event types for the rendering pipeline.
//!
//! These events represent the output of syntax highlighting (from tree-sitter or any other source)
//! in a format that is independent of tree-sitter's C FFI types. Formatters in lumis-core consume
//! these events to produce HTML, terminal output, etc.

use crate::annotations::ResolvedAnnotation;

/// Re-exported so the one decoration a public event carries is nameable: the
/// module it lives in is internal to this crate.
pub use crate::decorations::Decoration;

/// A single step in rendering syntax-highlighted source.
///
/// This enum mirrors tree-sitter's `HighlightEvent` but uses plain Rust types,
/// making it usable without any tree-sitter dependency. Lumis can enrich the
/// stream with caller-provided events before a formatter consumes it.
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
        /// The annotation resolved to the offset range consumed by formatters.
        annotation: ResolvedAnnotation<'a, T>,
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
            Self::AnnotationStart { annotation } => Self::AnnotationStart {
                annotation: annotation.clone(),
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
