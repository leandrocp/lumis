use lumis::annotations::{Annotation, AnnotationError, Position};
use lumis::events::HighlightEvent;
use lumis::formatters::Formatter;
use lumis::highlight::{highlight_events_with_options, HighlightError};
use lumis::languages::Language;
use lumis::HighlightOptions;
use std::io::{self, Write};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Eq)]
struct Change {
    id: u64,
}

#[test]
fn event_annotations_borrow_data_but_not_source_or_cancellation() {
    let annotations =
        [Annotation::new(Position::new(1, 4)..Position::new(1, 9), Change { id: 8 }).unwrap()];

    for cancellation_first in [false, true] {
        let events = {
            let source = String::from("let π = (3);\nlet café = 4;");
            let flag = AtomicUsize::new(0);
            let options = if cancellation_first {
                HighlightOptions::new()
                    .cancellation(&flag)
                    .annotations(&annotations)
            } else {
                HighlightOptions::new()
                    .annotations(&annotations)
                    .cancellation(&flag)
            };
            highlight_events_with_options(&source, Language::Rust, options.rainbow_brackets(true))
                .unwrap()
        };

        let starts: Vec<_> = events
            .iter()
            .filter_map(|event| match event {
                HighlightEvent::AnnotationStart { range, data } => Some((range.clone(), *data)),
                _ => None,
            })
            .collect();
        assert_eq!(starts.len(), 1);
        assert_eq!(starts[0].0, 18..23);
        assert!(std::ptr::eq(starts[0].1, annotations[0].data()));
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, HighlightEvent::AnnotationEnd))
                .count(),
            1
        );
        assert!(events
            .iter()
            .any(|event| matches!(event, HighlightEvent::Start { .. })));
        assert!(events
            .iter()
            .any(|event| matches!(event, HighlightEvent::DecorationStart { .. })));
    }
}

#[test]
fn events_without_annotations_remain_static_with_a_cancellation_flag() {
    let events: Vec<HighlightEvent<'static>> = {
        let flag = AtomicUsize::new(0);
        highlight_events_with_options(
            "let x = 1;",
            Language::Rust,
            HighlightOptions::new().cancellation(&flag),
        )
        .unwrap()
    };
    assert_ne!(events, [] as [HighlightEvent<'_>; 0]);
}

#[test]
fn event_api_rejects_annotations_that_cannot_be_placed() {
    let cases = [
        (
            "{\"a\": 1}",
            Annotation::new(1..99, ()).unwrap(),
            AnnotationError::OutOfBounds {
                index: 0,
                end: 99,
                source_len: 8,
            },
        ),
        (
            "π",
            Annotation::new(1..2, ()).unwrap(),
            AnnotationError::NotCharBoundary {
                index: 0,
                offset: 1,
            },
        ),
        (
            "π",
            Annotation::new(Position::new(1, 0)..Position::new(1, 1), ()).unwrap(),
            AnnotationError::LineOutOfBounds {
                index: 0,
                line: 1,
                line_count: 1,
            },
        ),
    ];
    for (source, annotation, expected) in cases {
        let annotations = [annotation];
        let error = highlight_events_with_options(
            source,
            Language::Rust,
            HighlightOptions::new().annotations(&annotations),
        )
        .unwrap_err();
        assert_eq!(error, HighlightError::Annotation(expected.clone()));
        assert_eq!(error.to_string(), expected.to_string());
        assert!(std::error::Error::source(&error).is_none());
    }
}

#[test]
fn point_annotation_in_empty_source_is_preserved() {
    let annotations = [Annotation::new(0..0, Change { id: 7 }).unwrap()];
    let events = highlight_events_with_options(
        "",
        Language::Rust,
        HighlightOptions::new().annotations(&annotations),
    )
    .unwrap();
    assert_eq!(
        events,
        vec![
            HighlightEvent::AnnotationStart {
                range: 0..0,
                data: annotations[0].data()
            },
            HighlightEvent::AnnotationEnd,
        ]
    );
}

#[test]
fn event_api_with_explicit_languages_composes_annotations() {
    let annotations = [Annotation::new(4..5, Change { id: 7 }).unwrap()];
    let options = HighlightOptions::new().annotations(&annotations);
    let events = lumis::highlight::highlight_events_with_languages(
        "let x = 1;",
        Language::Rust,
        options,
        &std::collections::HashSet::from([Language::Rust]),
    )
    .unwrap();
    assert_eq!(
        events,
        highlight_events_with_options("let x = 1;", Language::Rust, options).unwrap()
    );
    assert!(events
        .iter()
        .any(|event| matches!(event, HighlightEvent::AnnotationStart { .. })));
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Observation {
    saw_syntax: bool,
    saw_rainbow: bool,
    rainbow_depths: Vec<usize>,
    annotations: Vec<(std::ops::Range<usize>, u64)>,
    annotation_starts: usize,
    annotation_ends: usize,
}

struct TestFormatter {
    observation: Arc<Mutex<Observation>>,
}

impl Formatter<Change> for TestFormatter {
    fn language(&self) -> Language {
        Language::Rust
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_, Change>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let mut observation = self.observation.lock().unwrap();

        for event in events {
            match event {
                HighlightEvent::Start { .. } => {
                    observation.saw_syntax = true;
                }
                HighlightEvent::End | HighlightEvent::DecorationEnd => {}
                HighlightEvent::AnnotationStart { range, data } => {
                    observation.annotation_starts += 1;
                    observation.annotations.push((range.clone(), data.id));
                }
                HighlightEvent::AnnotationEnd => observation.annotation_ends += 1,
                HighlightEvent::Source { start, end } => {
                    output.write_all(&source.as_bytes()[*start..*end])?;
                }
                HighlightEvent::DecorationStart {
                    decoration: lumis::decorations::Decoration::RainbowBracket { depth },
                } => {
                    observation.saw_rainbow = true;
                    observation.rainbow_depths.push(*depth);
                }
                event => panic!("this test observes every event kind, and missed {event:?}"),
            }
        }

        Ok(())
    }
}

#[test]
fn top_level_highlight_composes_typed_annotations_with_syntax_events() {
    let source = "let value = (1);";
    let start = source.find("value").unwrap();
    let annotations = [Annotation::new(start..start + "value".len(), Change { id: 7 }).unwrap()];
    let options = HighlightOptions::new()
        .annotations(&annotations)
        .rainbow_brackets(true);
    let observation = Arc::new(Mutex::new(Observation::default()));
    let formatter: Box<dyn Formatter<Change>> = Box::new(TestFormatter {
        observation: Arc::clone(&observation),
    });
    let output = lumis::highlight_with_options(source, formatter, options);

    assert_eq!(output, source);
    assert_eq!(
        *observation.lock().unwrap(),
        Observation {
            saw_syntax: true,
            saw_rainbow: true,
            rainbow_depths: vec![0, 0],
            annotations: vec![(start..start + 5, 7)],
            annotation_starts: 1,
            annotation_ends: 1,
        }
    );
}

#[test]
fn write_highlight_rejects_annotations_outside_the_source() {
    let source = "let value = 1;";
    let annotations = [Annotation::new(0..source.len() + 1, Change { id: 7 }).unwrap()];
    let observation = Arc::new(Mutex::new(Observation::default()));
    let formatter = TestFormatter {
        observation: Arc::clone(&observation),
    };
    let mut output = Vec::new();

    let error = lumis::write_highlight_with_options(
        &mut output,
        source,
        formatter,
        HighlightOptions::new().annotations(&annotations),
    )
    .unwrap_err();

    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(*observation.lock().unwrap(), Observation::default());
    assert_eq!(output, b"");
}

#[test]
fn write_highlight_resolves_position_ranges_before_rendering() {
    let source = "let π = 3;\nlet café = 4;";
    let annotations =
        [Annotation::new(Position::new(1, 4)..Position::new(1, 9), Change { id: 8 }).unwrap()];
    let observation = Arc::new(Mutex::new(Observation::default()));
    let formatter = TestFormatter {
        observation: Arc::clone(&observation),
    };
    let mut output = Vec::new();

    lumis::write_highlight_with_options(
        &mut output,
        source,
        formatter,
        HighlightOptions::new().annotations(&annotations),
    )
    .unwrap();

    let start = source.find("café").unwrap();
    assert_eq!(
        observation.lock().unwrap().annotations,
        vec![(start..start + "café".len(), 8)]
    );
}
