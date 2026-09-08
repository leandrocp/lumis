//! Every shape an annotation can take, over one small source.
//!
//! Lumis does not compute the ranges. A diff library, a search index or a
//! compiler supplies them; Lumis places them correctly relative to the syntax
//! and hands them to this formatter.
//!
//! Run with `cargo run -p lumis --example annotations`.

use lumis::annotations::{Annotation, Position};
use lumis::events::HighlightEvent;
use lumis::formatters::Formatter;
use lumis::languages::Language;
use lumis::{highlight_with_options, html, HighlightOptions};
use std::io::{self, Write};

const SOURCE: &str = "let total = price + tax;\nlet label = \"☕ café\";\n\nlet net = total - fee;";

/// What each annotation means. Lumis never inspects this.
#[derive(Clone, Copy, Debug)]
enum Mark {
    Line(&'static str),
    Span(&'static str),
    Note(&'static str),
}

struct MarkFormatter;

impl Formatter<Mark> for MarkFormatter {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, Mark>],
        out: &mut dyn Write,
    ) -> io::Result<()> {
        // `AnnotationEnd` carries no payload, so keep a stack of what was opened.
        let mut open: Vec<&str> = Vec::new();

        for event in events {
            match event {
                HighlightEvent::Start { scope_index, .. } => {
                    let scope = lumis::highlights::HIGHLIGHT_NAMES[*scope_index];
                    write!(out, "<span {}>", html::span_linked_attrs(scope))?;
                }
                HighlightEvent::End => out.write_all(b"</span>")?,
                HighlightEvent::AnnotationStart { annotation } => match annotation.data() {
                    Mark::Line(kind) => {
                        write!(out, "<span class=\"line-{kind}\">")?;
                        open.push("span");
                    }
                    Mark::Span(name) => {
                        write!(out, "<mark class=\"{name}\">")?;
                        open.push("mark");
                    }
                    // A point opens and closes with nothing between it.
                    Mark::Note(label) => {
                        write!(out, "<i data-note=\"{}\">", html::escape(label))?;
                        open.push("i");
                    }
                },
                HighlightEvent::AnnotationEnd => {
                    let tag = open.pop().expect("balanced stream");
                    write!(out, "</{tag}>")?;
                }
                HighlightEvent::Source { start, end } => {
                    write!(out, "{}", html::escape(&source[*start..*end]))?;
                }
                // Lumis adds event kinds over time. A formatter renders the
                // ones it knows and skips the rest, rather than failing to
                // compile against a newer Lumis.
                _ => {}
            }
        }

        Ok(())
    }
}

fn render_example() -> Result<String, lumis::AnnotationError> {
    let annotations = [
        // Zero-based line and UTF-8 byte column. This one crosses a line.
        Annotation::new(
            Position::new(0, 0)..Position::new(1, 24),
            Mark::Line("changed"),
        )?,
        // `rice`, inside `price`. Starting mid-token makes Lumis close and
        // reopen the `variable` scope, so it renders as `p` + `rice`.
        Annotation::new(13..17, Mark::Span("edit"))?,
        // `"☕ café"`. Offsets are UTF-8 bytes, so `☕` costs 3 and `é` costs 2.
        Annotation::new(37..48, Mark::Span("text"))?,
        // An empty range is a point. Line 2 is blank, so there is nothing to
        // cover, and a review comment still has somewhere to land.
        Annotation::new(
            Position::new(2, 0)..Position::new(2, 0),
            Mark::Note("why the gap?"),
        )?,
        Annotation::new(
            Position::new(3, 0)..Position::new(3, 22),
            Mark::Line("added"),
        )?,
        // `total - ` and `- fee` overlap without either containing the other,
        // so Lumis closes `left` and reopens `right` after it.
        Annotation::new(61..69, Mark::Span("left"))?,
        Annotation::new(67..72, Mark::Span("right"))?,
    ];

    Ok(highlight_with_options(
        SOURCE,
        MarkFormatter,
        HighlightOptions::new().annotations(&annotations),
    ))
}

fn main() -> Result<(), lumis::AnnotationError> {
    println!("{}", render_example()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_every_annotation_shape() {
        let output = render_example().unwrap();

        // A position range crossing a line boundary.
        assert!(output.contains("<span class=\"line-changed\">"));
        // An offset range starting mid-token splits the `variable` scope.
        assert!(output.contains(
            "<span class=\"l-variable\">p</span><mark class=\"edit\"><span class=\"l-variable\">rice</span>"
        ));
        // Byte offsets over a multibyte literal.
        assert!(output.contains("&quot;☕ café&quot;"));
        // A point renders as an empty element.
        assert!(output.contains("<i data-note=\"why the gap?\"></i>"));
        // The overlapping annotation is closed and reopened, so it starts twice.
        assert_eq!(output.matches("<mark class=\"right\">").count(), 2);
    }
}
