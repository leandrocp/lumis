//! `BBCode` formatter for syntax highlighting.
//!
//! This module provides the [`BBCodeScoped`] formatter that generates `BBCode` output with
//! highlight scope names as tags (e.g., `[keyword-function-rust]text[/keyword-function-rust]`).
//!
//! It does not emit standard forum-style `BBCode` such as `[b]`, `[color]`, or `[code]`.
//!
//! Works with pre-computed highlight events from any source.

use super::{check_source_ranges, source_text, Formatter};
use crate::decorations::{compose_line_decorations, Decoration, LineSelection, SteppedLineRange};
use crate::events::HighlightEvent;
use crate::languages::Language;
use derive_builder::Builder;
use std::io::{self, Write};
use std::ops::RangeInclusive;

/// The tag a highlighted line is wrapped in.
///
/// Every other tag this formatter emits is a highlight scope with its dots
/// turned into hyphens, and `highlighted` is the scope a theme styles a
/// highlighted line with, so this one is derived the same way rather than
/// configured.
const HIGHLIGHTED_TAG: &str = "highlighted";

/// Configuration for highlighting specific lines in `BBCode` output.
///
/// A highlighted line is wrapped in `[highlighted]...[/highlighted]`, newline
/// included, the way an HTML line sits inside its `<div>`. The consumer defines
/// what that tag looks like, as it already must for every scope tag.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HighlightLines {
    /// List of line ranges to highlight (1-based, inclusive).
    pub lines: Vec<RangeInclusive<usize>>,
}

/// `BBCode` formatter for syntax highlighting using highlight scope names as tags.
///
/// Generates `BBCode` output using scope-based tags derived from tree-sitter scope names.
/// A tag is the scope and the language it was matched in, with the dots turned into
/// hyphens: `keyword.function` in Rust becomes
/// `[keyword-function-rust]...[/keyword-function-rust]`.
/// It does not emit standard forum-style `BBCode` tags.
///
/// Use [`BBCodeScopedBuilder`] to create instances.
///
/// # Example Output
///
/// For the Rust code `fn main() {}`, the formatter generates:
///
/// ```text
/// [keyword-function-rust]fn[/keyword-function-rust] [function-rust]main[/function-rust][punctuation-bracket-rust]([/punctuation-bracket-rust][punctuation-bracket-rust])[/punctuation-bracket-rust] [punctuation-bracket-rust]{[/punctuation-bracket-rust][punctuation-bracket-rust]}[/punctuation-bracket-rust]
/// ```
#[derive(Builder, Clone, Debug)]
#[builder(default)]
pub struct BBCodeScoped {
    #[builder(setter(custom))]
    language: Language,
    highlight_lines: Option<HighlightLines>,
    #[builder(setter(skip), default)]
    stepped_highlight_lines: Vec<SteppedLineRange>,
}

impl BBCodeScopedBuilder {
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

impl BBCodeScoped {
    pub fn new(language: Language, highlight_lines: Option<HighlightLines>) -> Self {
        Self {
            language,
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
}

impl Default for BBCodeScoped {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            highlight_lines: None,
            stepped_highlight_lines: Vec::new(),
        }
    }
}

/// Convert scope name to `BBCode` tag name.
///
/// Converts tree-sitter scope names (dot-separated) to `BBCode` tag names
/// (hyphen-separated).
fn scope_to_tag_name(scope: &str) -> String {
    scope.replace('.', "-")
}

fn write_escaped_text(output: &mut dyn Write, text: &str) -> io::Result<()> {
    for ch in text.chars() {
        match ch {
            '[' => output.write_all(b"&#91;")?,
            ']' => output.write_all(b"&#93;")?,
            _ => write!(output, "{ch}")?,
        }
    }

    Ok(())
}

fn tag_name(scope_index: usize, language: &str) -> String {
    let scope = crate::highlights::HIGHLIGHT_NAMES
        .get(scope_index)
        .copied()
        .unwrap_or("");
    let specialized = format!("{scope}.{language}");
    scope_to_tag_name(&specialized)
}

impl<T> Formatter<T> for BBCodeScoped {
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
        let mut scope_stack: Vec<String> = Vec::new();

        // `BBCode` carries no line structure of its own, so the line
        // decorations are only worth composing when there is a line to mark.
        // Without them the stream, and the output, are exactly what they were.
        let selection = self.line_selection();
        let composed;
        let events: &[HighlightEvent<'_, T>] = if selection.is_empty() {
            events
        } else {
            check_source_ranges(source_bytes, events)?;
            composed = compose_line_decorations(source, events, &selection);
            &composed
        };
        let mut line_highlighted = false;

        for event in events {
            match event {
                HighlightEvent::Source { start, end } => {
                    write_escaped_text(output, source_text(source_bytes, *start, *end)?)?;
                }
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => {
                    let tag_name = tag_name(*scope_index, language);
                    write!(output, "[{tag_name}]")?;
                    scope_stack.push(tag_name);
                }
                HighlightEvent::End => {
                    if let Some(tag_name) = scope_stack.pop() {
                        write!(output, "[/{tag_name}]")?;
                    }
                }
                HighlightEvent::DecorationStart {
                    decoration: Decoration::Line { highlighted, .. },
                } => {
                    line_highlighted = *highlighted;
                    if line_highlighted {
                        write!(output, "[{HIGHLIGHTED_TAG}]")?;
                    }
                }
                HighlightEvent::DecorationEnd => {
                    if line_highlighted {
                        write!(output, "[/{HIGHLIGHTED_TAG}]")?;
                        line_highlighted = false;
                    }
                }
                // Caller annotations carry data this formatter has never seen.
                HighlightEvent::AnnotationStart { .. } | HighlightEvent::AnnotationEnd => {}
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scope_index(scope: &str) -> usize {
        crate::highlights::HIGHLIGHT_NAMES
            .iter()
            .position(|candidate| *candidate == scope)
            .unwrap()
    }

    #[test]
    fn escapes_bbcode_delimiters_in_source_text() {
        let formatter = BBCodeScoped::default();
        let mut output = Vec::new();
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source { start: 0, end: 7 }];

        formatter.render("[url=x]", &events, &mut output).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "&#91;url=x&#93;");
    }

    /// Without lines to mark there is nothing to compose, so a scope still
    /// spans the newline it did before.
    #[test]
    fn leaves_the_stream_alone_when_no_line_is_highlighted() {
        let formatter = BBCodeScoped::default();
        let mut output = Vec::new();
        let events: [HighlightEvent<'_, ()>; 3] = [
            HighlightEvent::Start {
                scope_index: scope_index("string"),
                language: "json".to_string(),
            },
            HighlightEvent::Source { start: 0, end: 3 },
            HighlightEvent::End,
        ];

        formatter.render("a\nb", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[string-json]a\nb[/string-json]"
        );
    }

    #[test]
    fn wraps_a_highlighted_line_and_the_newline_that_ends_it() {
        let formatter = BBCodeScoped::new(
            Language::PlainText,
            Some(HighlightLines {
                lines: std::iter::once(1..=1).collect(),
            }),
        );
        let mut output = Vec::new();
        let events: [HighlightEvent<'_, ()>; 1] = [HighlightEvent::Source { start: 0, end: 3 }];

        formatter.render("a\nb", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[highlighted]a\n[/highlighted]b"
        );
    }

    /// Lines are the outer layer, so a scope crossing one is closed and
    /// reopened inside each line's tag rather than straddling both.
    #[test]
    fn a_scope_crossing_a_highlighted_line_boundary_nests_inside_it() {
        let formatter = BBCodeScoped::new(
            Language::PlainText,
            Some(HighlightLines {
                lines: std::iter::once(2..=2).collect(),
            }),
        );
        let mut output = Vec::new();
        let events: [HighlightEvent<'_, ()>; 3] = [
            HighlightEvent::Start {
                scope_index: scope_index("string"),
                language: "json".to_string(),
            },
            HighlightEvent::Source { start: 0, end: 3 },
            HighlightEvent::End,
        ];

        formatter.render("a\nb", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[string-json]a\n[/string-json][highlighted][string-json]b[/string-json][/highlighted]"
        );
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
            }),
        ] {
            let formatter = BBCodeScoped::new(Language::PlainText, highlight_lines);
            let mut output = Vec::new();

            let error = formatter
                .render("a\nb", &events, &mut output)
                .expect_err("an out-of-range source range is invalid data");

            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        }
    }

    #[test]
    fn preserves_nested_scopes() {
        let formatter = BBCodeScoped::default();
        let mut output = Vec::new();
        let string_scope = scope_index("string");
        let tag_scope = scope_index("tag");
        let events: [HighlightEvent<'_, ()>; 7] = [
            HighlightEvent::Start {
                scope_index: string_scope,
                language: "javascript".to_string(),
            },
            HighlightEvent::Source { start: 0, end: 1 },
            HighlightEvent::Start {
                scope_index: tag_scope,
                language: "html".to_string(),
            },
            HighlightEvent::Source { start: 1, end: 4 },
            HighlightEvent::End,
            HighlightEvent::Source { start: 4, end: 5 },
            HighlightEvent::End,
        ];

        formatter.render("`<a>`", &events, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "[string-javascript]`[tag-html]<a>[/tag-html]`[/string-javascript]"
        );
    }
}
