//! HTML formatter with linked CSS classes.
//!
//! This module provides the [`HtmlLinked`] formatter that generates HTML output with
//! CSS classes for syntax highlighting, working from pre-computed highlight events.

use super::{Formatter, HtmlElement};
use crate::decorations::{LineSelection, SteppedLineRange};
use crate::events::HighlightEvent;
use crate::languages::Language;
use derive_builder::Builder;
use std::{
    io::{self, Write},
    ops::RangeInclusive,
};

/// Configuration for highlighting specific lines in HTML linked output.
#[derive(Clone, Debug)]
pub struct HighlightLines {
    /// List of line ranges to highlight (1-based, inclusive).
    pub lines: Vec<RangeInclusive<usize>>,
    /// The CSS class name to add to highlighted line elements.
    pub class: String,
}

impl Default for HighlightLines {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            class: "l-highlighted".to_string(),
        }
    }
}

/// HTML formatter with CSS classes.
///
/// Generates HTML with CSS classes instead of inline styles.
/// Works with pre-computed highlight events from any source.
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct HtmlLinked {
    #[builder(setter(custom))]
    language: Language,
    pre_class: Option<String>,
    highlight_lines: Option<HighlightLines>,
    #[builder(setter(skip), default)]
    stepped_highlight_lines: Vec<SteppedLineRange>,
    header: Option<HtmlElement>,
}

impl HtmlLinkedBuilder {
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

impl HtmlLinked {
    pub fn new(
        language: Language,
        pre_class: Option<String>,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
            language,
            pre_class,
            highlight_lines,
            stepped_highlight_lines: Vec::new(),
            header,
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

    fn get_line_class_suffix(&self, is_highlighted: bool) -> Option<String> {
        if !is_highlighted {
            return None;
        }

        self.highlight_lines
            .as_ref()
            .map(|highlight| format!(" {}", highlight.class))
    }

    fn span_attrs_from_index(scope_index: usize) -> String {
        let scope = crate::highlights::HIGHLIGHT_NAMES
            .get(scope_index)
            .copied()
            .unwrap_or("");
        crate::formatter::html::span_linked_attrs(scope)
    }
}

impl Default for HtmlLinked {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            pre_class: None,
            highlight_lines: None,
            stepped_highlight_lines: Vec::new(),
            header: None,
        }
    }
}

impl<T> Formatter<T> for HtmlLinked {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, T>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let mut buffer = Vec::new();

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.open_tag)?;
        }

        crate::formatter::html::open_pre_tag(&mut buffer, self.pre_class.as_deref(), None)?;
        crate::formatter::html::open_code_tag(&mut buffer, &self.language)?;

        let class_suffix = self.get_line_class_suffix(true);
        crate::formatter::html::write_html_lines(
            &mut buffer,
            source,
            events,
            &self.line_selection(),
            &|scope_index, _language| Self::span_attrs_from_index(scope_index),
            (class_suffix.as_deref(), None),
        )?;

        crate::formatter::html::closing_tags(&mut buffer)?;

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.close_tag)?;
        }

        output.write_all(&buffer)?;
        Ok(())
    }
}
