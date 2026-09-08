//! HTML formatter with inline CSS styles.
//!
//! This module provides the [`HtmlInline`] formatter that generates HTML output with
//! inline CSS styles for syntax highlighting, working from pre-computed highlight events.

use super::{Formatter, HtmlElement};
use crate::events::HighlightEvent;
use crate::languages::Language;
use crate::themes::Theme;
use derive_builder::Builder;
use std::{
    io::{self, Write},
    ops::RangeInclusive,
};

/// Configuration for highlighting specific lines in HTML inline output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HighlightLines {
    /// List of line ranges to highlight (1-based, inclusive).
    pub lines: Vec<RangeInclusive<usize>>,
    /// The styling method to use for highlighted lines.
    pub style: Option<HighlightLinesStyle>,
    /// Optional CSS class to add to highlighted lines.
    pub class: Option<String>,
}

/// Defines how highlighted lines should be styled in HTML inline output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HighlightLinesStyle {
    /// Use the theme's 'highlighted' style if available.
    Theme,
    /// Use a custom CSS style string.
    Style(String),
}

impl Default for HighlightLines {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            style: Some(HighlightLinesStyle::Theme),
            class: None,
        }
    }
}

/// HTML formatter with inline CSS styles.
///
/// Generates self-contained HTML with styles embedded directly in elements.
/// Works with pre-computed highlight events from any source.
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct HtmlInline {
    #[builder(setter(custom))]
    language: Language,
    theme: Option<Theme>,
    pre_class: Option<String>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<HighlightLines>,
    header: Option<HtmlElement>,
}

impl HtmlInlineBuilder {
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

impl HtmlInline {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        language: Language,
        theme: Option<Theme>,
        pre_class: Option<String>,
        italic: bool,
        include_highlights: bool,
        highlight_lines: Option<HighlightLines>,
        header: Option<HtmlElement>,
    ) -> Self {
        Self {
            language,
            theme,
            pre_class,
            italic,
            include_highlights,
            highlight_lines,
            header,
        }
    }

    fn get_line_attrs(&self, line_number: usize) -> (Option<String>, Option<String>) {
        let is_highlighted = self
            .highlight_lines
            .as_ref()
            .is_some_and(|hl| hl.lines.iter().any(|r| r.contains(&line_number)));

        if !is_highlighted {
            return (None, None);
        }

        let class_suffix = self
            .highlight_lines
            .as_ref()
            .and_then(|hl| hl.class.as_ref())
            .map(|c| format!(" {c}"));

        let style = self.get_highlight_style();

        (class_suffix, style)
    }

    fn get_highlight_style(&self) -> Option<String> {
        let highlight_lines = self.highlight_lines.as_ref()?;

        match &highlight_lines.style {
            Some(HighlightLinesStyle::Theme) => {
                let theme = self.theme.as_ref()?;
                let highlighted_style = theme.get_style("highlighted")?;
                Some(highlighted_style.css(self.italic, " "))
            }
            Some(HighlightLinesStyle::Style(style_string)) => Some(style_string.clone()),
            None => None,
        }
    }

    fn span_attrs_from_index(&self, scope_index: usize, language: &str) -> String {
        let scope = crate::highlights::HIGHLIGHT_NAMES
            .get(scope_index)
            .copied()
            .unwrap_or("");
        let lang = language.parse::<Language>().ok();
        crate::formatter::html::span_inline_attrs(
            lang,
            scope,
            self.theme.as_ref(),
            self.italic,
            self.include_highlights,
        )
    }
}

impl Default for HtmlInline {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        }
    }
}

impl<T> Formatter<T> for HtmlInline {
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

        crate::formatter::html::open_pre_tag(
            &mut buffer,
            self.pre_class.as_deref(),
            self.theme.as_ref(),
        )?;
        crate::formatter::html::open_code_tag(&mut buffer, &self.language)?;

        let lines = crate::formatter::html::render_lines_from_events(
            source,
            events,
            |scope_index, language| self.span_attrs_from_index(scope_index, language),
        );

        for (i, line) in lines.iter().enumerate() {
            let line_number = i + 1;
            let line_with_newline = format!("{line}\n");
            let (class_suffix, style) = self.get_line_attrs(line_number);
            let wrapped = crate::formatter::html::wrap_line(
                line_number,
                &line_with_newline,
                class_suffix.as_deref(),
                style.as_deref(),
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
