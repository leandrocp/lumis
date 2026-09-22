//! Formatter implementations for generating syntax highlighted output.
//!
//! This module provides formatters for rendering syntax highlighted code from
//! pre-computed highlight events. The formatters are independent of tree-sitter
//! and work with [`HighlightEvent`] streams.
//!
//! Available formatters:
//! - [`html_inline`] - HTML with inline CSS styles
//! - [`html_multi_themes`] - HTML with multiple theme support
//! - [`html_linked`] - HTML with CSS classes
//! - [`terminal`] - ANSI color codes for terminal output
//! - [`bbcode`] - `BBCode` scoped output using highlight scope names as tags

use crate::events::HighlightEvent;
use crate::languages::Language;
use std::io::{self, Write};

pub mod ansi;
pub mod html;

pub mod html_inline;
pub use html_inline::{HtmlInline, HtmlInlineBuilder};

pub mod html_multi_themes;
pub use html_multi_themes::{HtmlMultiThemes, HtmlMultiThemesBuilder};

pub mod html_linked;
pub use html_linked::{HtmlLinked, HtmlLinkedBuilder};

pub mod terminal;
pub use terminal::{Background as TerminalBackground, Terminal, TerminalBuilder};

pub mod bbcode;
pub use bbcode::{BBCodeScoped, BBCodeScopedBuilder};

/// The source slice an event names, or an error when it names one that is not
/// there.
///
/// A formatter can be handed events it did not produce, so these offsets are
/// caller data. The HTML formatters clamp them; `terminal` and `bbcode_scoped`
/// refuse them, which is what this is for.
pub(crate) fn source_text(source: &[u8], start: usize, end: usize) -> io::Result<&str> {
    if start > end || end > source.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "invalid source range: {start}..{end} (len={})",
                source.len()
            ),
        ));
    }

    std::str::from_utf8(&source[start..end])
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

/// Reject an out-of-range `Source` before anything clamps it.
///
/// Line composition clips a `Source` to the document, so a formatter that
/// refuses a malformed range has to say so before composing, or the option that
/// turned composition on would quietly turn the error into truncated output.
pub(crate) fn check_source_ranges<T>(
    source: &[u8],
    events: &[HighlightEvent<'_, T>],
) -> io::Result<()> {
    for event in events {
        if let HighlightEvent::Source { start, end } = event {
            source_text(source, *start, *end)?;
        }
    }

    Ok(())
}

/// Configuration for wrapping the formatted output with custom HTML elements.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HtmlElement {
    /// The opening HTML tag that will be placed before the formatted code.
    pub open_tag: String,
    /// The closing HTML tag that will be placed after the formatted code.
    pub close_tag: String,
}

/// Trait for implementing custom syntax highlighting formatters.
///
/// Formatters in lumis-core work with pre-computed highlight events,
/// making them independent of tree-sitter. The `lumis` crate re-exports this
/// trait, so a formatter written against it works with both crates.
///
/// # Example
///
/// ```rust
/// use lumis_core::events::HighlightEvent;
/// use lumis_core::formatter::Formatter;
/// use lumis_core::languages::Language;
/// use std::io::{self, Write};
///
/// struct SourceFormatter;
///
/// impl Formatter for SourceFormatter {
///     fn language(&self) -> Language {
///         Language::Rust
///     }
///
///     fn render(
///         &self,
///         source: &str,
///         events: &[HighlightEvent<'_>],
///         output: &mut dyn Write,
///     ) -> io::Result<()> {
///         for event in events {
///             if let HighlightEvent::Source { start, end } = event {
///                 output.write_all(&source.as_bytes()[*start..*end])?;
///             }
///         }
///         Ok(())
///     }
/// }
///
/// let source = "let answer = 42;";
/// let events = [HighlightEvent::Source { start: 0, end: source.len() }];
///
/// let mut output = Vec::new();
/// SourceFormatter.render(source, &events, &mut output)?;
/// assert_eq!(output, source.as_bytes());
/// # Ok::<(), std::io::Error>(())
/// ```
pub trait Formatter<T = ()>: Send + Sync {
    /// Returns the source language this formatter renders.
    ///
    /// Callers that compute their own events already know the language and can
    /// ignore this; the `lumis` entry points read it to pick the grammar they
    /// parse `source` with.
    fn language(&self) -> Language;

    /// Format source code using pre-computed highlight events.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to format
    /// * `events` - Pre-computed highlight events from tree-sitter or any other source
    /// * `output` - Writer to send formatted output to
    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()>;

    /// Format source code that ran out of budget.
    ///
    /// Lumis calls this instead of [`render`](Self::render) when a limit bound
    /// the work, and hands it whatever events survived: nothing but the source
    /// text for [`BudgetExhausted::Time`], because a render stopped part way
    /// has no tree to salvage, and the ordinary stream for
    /// [`BudgetExhausted::Matches`], because that limit costs scopes rather
    /// than the whole highlight.
    ///
    /// The default renders them and says nothing, which is what a formatter
    /// with nowhere to put the fact should do. The HTML formatters override it
    /// to mark the `<pre>`, so a caller can tell a plain document apart from a
    /// file that genuinely has no syntax.
    fn render_budgeted(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
        exhausted: BudgetExhausted,
    ) -> io::Result<()> {
        let _ = exhausted;
        self.render(source, events, output)
    }

    /// [`render`](Self::render), or [`render_budgeted`](Self::render_budgeted)
    /// when a limit bound the work.
    ///
    /// Every host reaching a formatter has the same `Option` to unwrap, so the
    /// unwrapping lives here rather than four times over.
    fn render_budgeted_or(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
        exhausted: Option<BudgetExhausted>,
    ) -> io::Result<()> {
        match exhausted {
            Some(exhausted) => self.render_budgeted(source, events, output, exhausted),
            None => self.render(source, events, output),
        }
    }
}

/// Which limit bound a render.
///
/// The two degrade differently, and a caller reading the output needs to know
/// which happened: `Time` means the text is there and none of it is
/// highlighted, `Matches` means it is highlighted and some scopes are missing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BudgetExhausted {
    /// The time limit ran out, so the document is plain text.
    Time,
    /// The match limit bound the query, so some scopes may be missing.
    Matches,
}

impl BudgetExhausted {
    /// The value this carries in `data-lumis-budget`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Matches => "matches",
        }
    }
}

// Both forwarding impls have to forward `render_budgeted` as well. The default
// would otherwise call *this* `render`, which reaches the inner formatter's
// `render` and never its override, so a boxed or borrowed HTML formatter would
// drop the marker while a bare one kept it. The Elixir NIF holds a
// `Box<dyn Formatter<T>>` and did exactly that.

impl<T> Formatter<T> for Box<dyn Formatter<T>> {
    fn language(&self) -> Language {
        (**self).language()
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        (**self).render(source, events, output)
    }

    fn render_budgeted(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
        exhausted: BudgetExhausted,
    ) -> io::Result<()> {
        (**self).render_budgeted(source, events, output, exhausted)
    }
}

impl<T, F> Formatter<T> for &F
where
    F: Formatter<T> + ?Sized,
{
    fn language(&self) -> Language {
        (**self).language()
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        (**self).render(source, events, output)
    }

    fn render_budgeted(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
        exhausted: BudgetExhausted,
    ) -> io::Result<()> {
        (**self).render_budgeted(source, events, output, exhausted)
    }
}
