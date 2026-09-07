//! A side-by-side diff viewer built with Lumis annotations.
//!
//! The example intentionally does not compute a diff. An upstream diff library
//! would supply the line states, changed spans, and annotation ranges used
//! below. Lumis composes those ranges with syntax highlighting, and this custom
//! formatter decides how each annotation should look in HTML.

use lumis::events::HighlightEvent;
use lumis::formatters::Formatter;
use lumis::html;
use lumis::languages::Language;
use lumis::{Annotation, HighlightOptions};
use std::io::{self, Write};
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
enum LineKind {
    Context,
    Added,
    Removed,
    Changed,
}

impl LineKind {
    fn css_class(self) -> &'static str {
        match self {
            Self::Context => "diff-line-context",
            Self::Added => "diff-line-added",
            Self::Removed => "diff-line-removed",
            Self::Changed => "diff-line-changed",
        }
    }

    fn marker(self) -> &'static str {
        match self {
            Self::Context => " ",
            Self::Added => "+",
            Self::Removed => "-",
            Self::Changed => "~",
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum SpanKind {
    Added,
    Removed,
}

impl SpanKind {
    fn css_class(self) -> &'static str {
        match self {
            Self::Added => "diff-span-added",
            Self::Removed => "diff-span-removed",
        }
    }
}

#[derive(Clone, Debug)]
enum DiffAnnotation {
    Line { number: usize, kind: LineKind },
    Span { kind: SpanKind },
    Annotation { label: String },
}

struct DiffHtmlFormatter {
    language: Language,
}

impl Formatter<DiffAnnotation> for DiffHtmlFormatter {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, DiffAnnotation>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        html::open_pre_tag(output, Some("diff-code"), None)?;
        html::open_code_tag(output, &self.language)?;
        let mut annotation_closings = Vec::new();

        for event in events {
            match event {
                HighlightEvent::Start { scope_index, .. } => {
                    let scope = lumis::highlights::HIGHLIGHT_NAMES[*scope_index];
                    write!(output, "<span {}>", html::span_linked_attrs(scope))?;
                }
                HighlightEvent::End => output.write_all(b"</span>")?,
                HighlightEvent::AnnotationStart { annotation } => match annotation.data() {
                    DiffAnnotation::Line { number, kind } => {
                        write!(
                            output,
                            "<span class=\"diff-line {}\" data-line=\"{}\" data-marker=\"{}\">",
                            kind.css_class(),
                            number,
                            kind.marker(),
                        )?;
                        annotation_closings.push("</span>");
                    }
                    DiffAnnotation::Span { kind } => {
                        write!(output, "<mark class=\"diff-span {}\">", kind.css_class())?;
                        annotation_closings.push("</mark>");
                    }
                    DiffAnnotation::Annotation { label } => {
                        write!(
                            output,
                            "<span class=\"diff-annotation\" data-label=\"{}\">",
                            html::escape(label),
                        )?;
                        annotation_closings.push("</span>");
                    }
                },
                HighlightEvent::AnnotationEnd => {
                    let closing = annotation_closings.pop().ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "annotation event stream contains an unmatched end event",
                        )
                    })?;
                    output.write_all(closing.as_bytes())?;
                }
                HighlightEvent::Source { start, end } => {
                    write!(output, "{}", html::escape(&source[*start..*end]))?;
                }
            }
        }

        if !annotation_closings.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "annotation event stream contains an unclosed annotation",
            ));
        }

        html::closing_tags(output)
    }
}

fn render_pane(source: &str, annotations: &[Annotation<DiffAnnotation>]) -> String {
    let formatter = DiffHtmlFormatter {
        language: Language::Rust,
    };
    let options = HighlightOptions::new().annotations(annotations);

    lumis::highlight_with_options(source, formatter, options)
}

fn range_of(source: &str, text: &str) -> Range<usize> {
    let start = source
        .find(text)
        .unwrap_or_else(|| panic!("expected {text:?} in the example source"));
    start..start + text.len()
}

fn last_range_of(source: &str, text: &str) -> Range<usize> {
    let start = source
        .rfind(text)
        .unwrap_or_else(|| panic!("expected {text:?} in the example source"));
    start..start + text.len()
}

fn render_example() -> Result<String, Box<dyn std::error::Error>> {
    let old_source = "\
fn calculate(price: i32, tax: i32) -> i32 {
    price + tax
}
";
    let new_source = "\
fn calculate(price: i64, tax: i64, fee: i64) -> i64 {
    let subtotal = price + tax;
    subtotal + fee
}
";

    // A diff library would determine these ranges. Lumis only consumes them.
    let old_annotations = [
        Annotation::new(
            range_of(old_source, "fn calculate(price: i32, tax: i32) -> i32 {"),
            DiffAnnotation::Line {
                number: 1,
                kind: LineKind::Changed,
            },
        )?,
        Annotation::new(
            range_of(old_source, "    price + tax"),
            DiffAnnotation::Line {
                number: 2,
                kind: LineKind::Removed,
            },
        )?,
        Annotation::new(
            last_range_of(old_source, "}"),
            DiffAnnotation::Line {
                number: 3,
                kind: LineKind::Context,
            },
        )?,
        Annotation::new(
            range_of(old_source, "i32"),
            DiffAnnotation::Span {
                kind: SpanKind::Removed,
            },
        )?,
        Annotation::new(
            range_of(old_source, "price + tax"),
            DiffAnnotation::Span {
                kind: SpanKind::Removed,
            },
        )?,
    ];

    let new_annotations = [
        Annotation::new(
            range_of(
                new_source,
                "fn calculate(price: i64, tax: i64, fee: i64) -> i64 {",
            ),
            DiffAnnotation::Line {
                number: 1,
                kind: LineKind::Changed,
            },
        )?,
        Annotation::new(
            range_of(new_source, "    let subtotal = price + tax;"),
            DiffAnnotation::Line {
                number: 2,
                kind: LineKind::Added,
            },
        )?,
        Annotation::new(
            range_of(new_source, "    subtotal + fee"),
            DiffAnnotation::Line {
                number: 3,
                kind: LineKind::Added,
            },
        )?,
        Annotation::new(
            last_range_of(new_source, "}"),
            DiffAnnotation::Line {
                number: 4,
                kind: LineKind::Context,
            },
        )?,
        Annotation::new(
            range_of(new_source, "i64"),
            DiffAnnotation::Span {
                kind: SpanKind::Added,
            },
        )?,
        Annotation::new(
            range_of(new_source, "    let subtotal = price + tax;"),
            DiffAnnotation::Span {
                kind: SpanKind::Added,
            },
        )?,
        Annotation::new(
            range_of(new_source, "    subtotal + fee"),
            DiffAnnotation::Span {
                kind: SpanKind::Added,
            },
        )?,
        Annotation::new(
            last_range_of(new_source, "fee"),
            DiffAnnotation::Annotation {
                label: "New service fee".to_owned(),
            },
        )?,
    ];

    let old_html = render_pane(old_source, &old_annotations);
    let new_html = render_pane(new_source, &new_annotations);
    let mut page = String::from(PAGE_START);
    page.push_str(&old_html);
    page.push_str(PAGE_MIDDLE);
    page.push_str(&new_html);
    page.push_str(PAGE_END);
    Ok(page)
}

const PAGE_START: &str = r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Lumis annotation diff viewer</title>
<style>
:root { color-scheme: dark; font-family: ui-sans-serif, system-ui, sans-serif; }
body { margin: 0; background: #0d1117; color: #e6edf3; }
main { padding: 2rem; }
h1 { margin: 0 0 .35rem; font-size: 1.35rem; }
.subtitle { margin: 0 0 1.25rem; color: #8b949e; }
.diff-viewer { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); border: 1px solid #30363d; border-radius: 10px; overflow: hidden; }
.diff-pane + .diff-pane { border-left: 1px solid #30363d; }
.diff-pane h2 { margin: 0; padding: .7rem 1rem; background: #161b22; border-bottom: 1px solid #30363d; font-size: .85rem; font-weight: 600; }
.diff-code { margin: 0; padding: 0; overflow-x: auto; background: #0d1117; }
.diff-code code { display: block; min-width: max-content; padding: .5rem 0; }
.diff-line { display: inline-block; min-width: 100%; min-height: 1.45em; }
.diff-line::before { display: inline-block; width: 4.5rem; margin-right: .75rem; padding-right: .7rem; color: #6e7681; text-align: right; content: attr(data-line) " " attr(data-marker); user-select: none; }
.diff-line-added { background: rgba(46, 160, 67, .18); }
.diff-line-removed { background: rgba(248, 81, 73, .18); }
.diff-line-changed { background: rgba(187, 128, 9, .18); }
.diff-line-added::before { color: #56d364; background: rgba(46, 160, 67, .16); }
.diff-line-removed::before { color: #ff7b72; background: rgba(248, 81, 73, .16); }
.diff-line-changed::before { color: #e3b341; background: rgba(187, 128, 9, .16); }
.diff-span { border-radius: 3px; color: inherit; }
.diff-span-added { background: rgba(46, 160, 67, .42); }
.diff-span-removed { background: rgba(248, 81, 73, .42); }
.diff-annotation { position: relative; border-bottom: 2px dotted #d2a8ff; }
.diff-annotation::after { position: absolute; z-index: 1; left: 0; bottom: 1.5rem; width: max-content; max-width: 12rem; padding: .3rem .45rem; border: 1px solid #8957e5; border-radius: 5px; background: #2d1b4e; color: #d2a8ff; font: 11px/1.2 ui-sans-serif, system-ui, sans-serif; content: "● " attr(data-label); }
.l-keyword, .l-keyword-function { color: #ff7b72; }
.l-function { color: #d2a8ff; }
.l-variable, .l-variable-parameter { color: #ffa657; }
.l-type, .l-type-builtin { color: #79c0ff; }
.l-operator, .l-punctuation-bracket, .l-punctuation-delimiter { color: #8b949e; }
@media (max-width: 850px) {
  main { padding: 1rem; }
  .diff-viewer { grid-template-columns: 1fr; }
  .diff-pane + .diff-pane { border-left: 0; border-top: 1px solid #30363d; }
}
</style>
</head>
<body>
<main>
<h1>Annotation API: diff viewer</h1>
<p class="subtitle">Caller-supplied ranges composed with Lumis syntax events</p>
<div class="diff-viewer">
<section class="diff-pane">
<h2>calculator.rs · before</h2>
"#;

const PAGE_MIDDLE: &str = r#"
</section>
<section class="diff-pane">
<h2>calculator.rs · after</h2>
"#;

const PAGE_END: &str = r"
</section>
</div>
</main>
</body>
</html>
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", render_example()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_a_complete_diff_viewer_from_typed_annotations() {
        let output = render_example().unwrap();

        assert!(output.contains("calculator.rs · before"));
        assert!(output.contains("calculator.rs · after"));
        assert!(output.contains("data-marker=\"-\""));
        assert!(output.contains("data-marker=\"+\""));
        assert!(output.contains("data-marker=\"~\""));
        assert!(output.contains("diff-span-removed"));
        assert!(output.contains("diff-span-added"));
        assert!(output.contains("data-label=\"New service fee\""));
        assert!(output.contains("<span class=\"l-"));
    }
}
