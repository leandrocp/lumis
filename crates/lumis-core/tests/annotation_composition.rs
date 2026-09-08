//! Rust's half of the annotation composition parity check.
//!
//! `fixtures/annotation-composition.json` holds one expected event stream per
//! case. This asserts Rust produces it;
//! `packages/javascript/lumis/test/annotation-composition.test.ts` asserts the
//! TypeScript port produces the same. Rust is the reference, so a difference is
//! a bug in the port rather than something to record.

use lumis_core::annotations::{compose_annotations, Annotation, AnnotationRange, Position};
use lumis_core::events::HighlightEvent;
use lumis_core::highlights::HIGHLIGHT_NAMES;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Manifest {
    cases: Vec<Case>,
}

#[derive(Debug, Deserialize)]
struct Case {
    name: String,
    source: String,
    events: Vec<SyntaxEvent>,
    annotations: Vec<CaseAnnotation>,
    expected: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum SyntaxEvent {
    Start {
        scope: String,
        language: String,
    },
    Source {
        #[serde(rename = "startByte")]
        start_byte: usize,
        #[serde(rename = "endByte")]
        end_byte: usize,
    },
    End,
}

#[derive(Debug, Deserialize)]
struct CaseAnnotation {
    range: CaseRange,
    data: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum CaseRange {
    Offset {
        start: usize,
        end: usize,
    },
    Position {
        start: CasePosition,
        end: CasePosition,
    },
}

#[derive(Debug, Deserialize)]
struct CasePosition {
    line: usize,
    column: usize,
}

fn manifest() -> Manifest {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/annotation-composition.json");
    serde_json::from_str(&fs::read_to_string(&path).expect("read annotation-composition.json"))
        .expect("parse annotation-composition.json")
}

fn scope_index(name: &str) -> usize {
    HIGHLIGHT_NAMES
        .iter()
        .position(|candidate| *candidate == name)
        .unwrap_or_else(|| panic!("`{name}` is not a highlight scope"))
}

/// One line per case, so a failure diff points at the event that moved.
fn render(source: &str, case: &Case) -> String {
    let events: Vec<HighlightEvent<'_, ()>> = case
        .events
        .iter()
        .map(|event| match event {
            SyntaxEvent::Start { scope, language } => HighlightEvent::Start {
                scope_index: scope_index(scope),
                language: language.clone(),
            },
            SyntaxEvent::Source {
                start_byte,
                end_byte,
            } => HighlightEvent::Source {
                start: *start_byte,
                end: *end_byte,
            },
            SyntaxEvent::End => HighlightEvent::End,
        })
        .collect();

    let annotations: Vec<Annotation<String>> = case
        .annotations
        .iter()
        .map(|annotation| {
            let range = match &annotation.range {
                CaseRange::Offset { start, end } => AnnotationRange::Offset(*start..*end),
                CaseRange::Position { start, end } => AnnotationRange::Position(
                    Position::new(start.line, start.column)..Position::new(end.line, end.column),
                ),
            };
            Annotation::new(range, annotation.data.clone())
                .expect("fixture annotations are well-formed")
        })
        .collect();

    match compose_annotations(source, &events, &annotations) {
        Err(error) => format!("ERROR: {error}"),
        Ok(events) => events
            .iter()
            .map(|event| match event {
                HighlightEvent::Start { scope_index, .. } => {
                    format!("S:{}", HIGHLIGHT_NAMES[*scope_index])
                }
                HighlightEvent::Source { start, end } => format!("T:{start}-{end}"),
                HighlightEvent::End => "E".to_string(),
                HighlightEvent::AnnotationStart { annotation } => format!(
                    "A+{}@{}-{}",
                    annotation.data(),
                    annotation.range().start,
                    annotation.range().end
                ),
                HighlightEvent::AnnotationEnd => "A-".to_string(),
                event => panic!("fixture corpus has no notation for {event:?}"),
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}

#[test]
fn the_corpus_covers_the_shapes_composition_has_to_get_right() {
    let manifest = manifest();

    // A discovery bug that found nothing would otherwise pass silently.
    assert!(
        manifest.cases.len() >= 24,
        "corpus shrank to {} cases",
        manifest.cases.len()
    );

    for required in [
        "scope/annotation-inside-scope",
        "multi/overlapping",
        "multi/nested",
        "position/across-lines",
        "utf8/mid-character-offset",
        "point/blank-line",
        "point/end-of-source",
        "point/nested-in-span-start",
        "point/at-span-end",
    ] {
        assert!(
            manifest.cases.iter().any(|case| case.name == required),
            "the corpus lost its `{required}` case"
        );
    }
}

#[test]
fn rust_produces_the_expected_stream() {
    for case in &manifest().cases {
        assert_eq!(
            render(&case.source, case),
            case.expected,
            "{}: composition changed",
            case.name
        );
    }
}
