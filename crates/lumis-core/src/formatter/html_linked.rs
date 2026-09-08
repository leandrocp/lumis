//! HTML formatter with linked CSS classes.
//!
//! This module provides the [`HtmlLinked`] formatter that generates HTML output with
//! CSS classes for syntax highlighting, working from pre-computed highlight events.

use super::{Formatter, HtmlElement};
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
            header,
        }
    }

    fn get_line_class_suffix(&self, line_number: usize) -> Option<String> {
        self.highlight_lines.as_ref().and_then(|hl| {
            if hl.lines.iter().any(|range| range.contains(&line_number)) {
                Some(format!(" {}", hl.class))
            } else {
                None
            }
        })
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
            header: None,
        }
    }
}

impl<T> Formatter<T> for HtmlLinked {
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

        let lines = crate::formatter::html::render_lines_from_events(
            source,
            events,
            |scope_index, _language| Self::span_attrs_from_index(scope_index),
        );

        for (i, line) in lines.iter().enumerate() {
            let line_number = i + 1;
            let class_suffix = self.get_line_class_suffix(line_number);
            let line_with_newline = format!("{line}\n");
            let wrapped = crate::formatter::html::wrap_line(
                line_number,
                &line_with_newline,
                class_suffix.as_deref(),
                None,
            );
            write!(&mut buffer, "{wrapped}")?;
        }

        crate::formatter::html::closing_tags(&mut buffer)?;

        if let Some(ref header) = self.header {
            write!(buffer, "{}", header.close_tag)?;
        }

        output.write_all(&buffer)?;
        Ok(())
    }
}
