use lumis::annotations::{Annotation, Position};
use lumis::events::HighlightEvent;
use lumis::formatters::Formatter;
use lumis::languages::Language;
use lumis::HighlightOptions;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Change {
    id: u64,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Observation {
    saw_syntax: bool,
    saw_rainbow: bool,
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
                HighlightEvent::Start { scope_index, .. } => {
                    observation.saw_syntax = true;
                    observation.saw_rainbow |= lumis::highlights::HIGHLIGHT_NAMES[*scope_index]
                        .starts_with("punctuation.bracket.rainbow.");
                }
                HighlightEvent::End => {}
                HighlightEvent::AnnotationStart { annotation } => {
                    observation.annotation_starts += 1;
                    observation
                        .annotations
                        .push((annotation.range().clone(), annotation.data().id));
                }
                HighlightEvent::AnnotationEnd => observation.annotation_ends += 1,
                HighlightEvent::Source { start, end } => {
                    output.write_all(&source.as_bytes()[*start..*end])?;
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
