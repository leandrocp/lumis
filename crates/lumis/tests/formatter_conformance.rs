use lumis::{
    highlight, highlight::highlight_events_with_options, highlight::HighlightOptions,
    languages::Language, themes, BBCodeScopedBuilder, HtmlInlineBuilder, HtmlLinkedBuilder,
    HtmlMultiThemesBuilder, TerminalBuilder,
};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FixtureMetadata {
    #[allow(dead_code)]
    name: String,
    language: String,
    theme: String,
    #[serde(default)]
    rainbow_brackets: bool,
    #[serde(default)]
    html_multi_themes: Option<HtmlMultiThemesFixture>,
    events: Vec<SerializableHighlightEvent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HtmlMultiThemesFixture {
    themes: BTreeMap<String, String>,
    #[serde(default)]
    default_theme: Option<String>,
    #[serde(default)]
    highlight_lines: Vec<usize>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
enum SerializableHighlightEvent {
    Start { scope: String, language: String },
    Source { start: usize, end: usize },
    End,
}

struct Fixture {
    metadata: FixtureMetadata,
    source: String,
    html_inline: String,
    html_linked: String,
    html_multi_themes: String,
    terminal: String,
    bbcode: String,
}

fn conformance_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/conformance")
}

fn fixture_theme(name: &str) -> lumis::themes::Theme {
    let path = conformance_dir()
        .join("../conformance-themes")
        .join(format!("{name}.json"));
    if path.is_file() {
        return themes::from_file(path).unwrap();
    }
    themes::get(name).unwrap()
}

fn load_fixture(name: &str) -> Fixture {
    let dir = conformance_dir().join(name);
    let metadata = serde_json::from_str::<FixtureMetadata>(
        &fs::read_to_string(dir.join("fixture.json")).expect("failed to read fixture metadata"),
    )
    .expect("failed to parse fixture metadata");

    Fixture {
        source: fs::read_to_string(dir.join("source.txt")).expect("failed to read source"),
        html_inline: fs::read_to_string(dir.join("html-inline.html"))
            .expect("failed to read html-inline"),
        html_linked: fs::read_to_string(dir.join("html-linked.html"))
            .expect("failed to read html-linked"),
        html_multi_themes: fs::read_to_string(dir.join("html-multi-themes.html"))
            .expect("failed to read html-multi-themes"),
        terminal: fs::read_to_string(dir.join("terminal.txt")).expect("failed to read terminal"),
        bbcode: fs::read_to_string(dir.join("bbcode.txt")).expect("failed to read bbcode"),
        metadata,
    }
}

fn check_events(fixture: &Fixture) {
    let lang: Language = fixture.metadata.language.parse().expect("invalid language");
    let events = highlight_events_with_options(
        &fixture.source,
        lang,
        HighlightOptions::new().rainbow_brackets(fixture.metadata.rainbow_brackets),
    )
    .expect("events should build");
    let serialized = events
        .into_iter()
        .map(|event| match event {
            lumis_core::events::HighlightEvent::Start {
                scope_index,
                language,
            } => SerializableHighlightEvent::Start {
                scope: lumis_core::highlights::HIGHLIGHT_NAMES[scope_index].to_string(),
                language,
            },
            lumis_core::events::HighlightEvent::Source { start, end } => {
                SerializableHighlightEvent::Source { start, end }
            }
            lumis_core::events::HighlightEvent::End => SerializableHighlightEvent::End,
            _ => unreachable!("syntax highlighting emits only scope and source events"),
        })
        .collect::<Vec<_>>();
    assert_eq!(serialized, fixture.metadata.events);
}

fn check_html_inline(fixture: &Fixture) {
    let lang: Language = fixture.metadata.language.parse().unwrap();
    let theme = fixture_theme(&fixture.metadata.theme);
    let fmt = HtmlInlineBuilder::new()
        .language(lang)
        .theme(Some(theme))
        .build()
        .unwrap();
    assert_eq!(
        normalize_newlines(&lumis::highlight_with_options(
            &fixture.source,
            fmt,
            HighlightOptions::new().rainbow_brackets(fixture.metadata.rainbow_brackets),
        )),
        normalize_newlines(&fixture.html_inline)
    );
}

fn check_html_linked(fixture: &Fixture) {
    let lang: Language = fixture.metadata.language.parse().unwrap();
    let fmt = HtmlLinkedBuilder::new().language(lang).build().unwrap();
    let mut out = Vec::new();
    lumis::write_highlight_with_options(
        &mut out,
        &fixture.source,
        fmt,
        HighlightOptions::new().rainbow_brackets(fixture.metadata.rainbow_brackets),
    )
    .unwrap();
    assert_eq!(
        normalize_newlines(&String::from_utf8(out).unwrap()),
        normalize_newlines(&fixture.html_linked)
    );
}

fn check_html_multi_themes(fixture: &Fixture) {
    let lang: Language = fixture.metadata.language.parse().unwrap();
    let mut map = HashMap::new();
    let mut builder = HtmlMultiThemesBuilder::new();

    if let Some(config) = &fixture.metadata.html_multi_themes {
        for (name, theme) in &config.themes {
            map.insert(name.clone(), fixture_theme(theme));
        }
        if let Some(default_theme) = &config.default_theme {
            builder.default_theme(default_theme.clone());
        }

        if !config.highlight_lines.is_empty() {
            builder.highlight_lines(Some(lumis::formatters::html_inline::HighlightLines {
                lines: config
                    .highlight_lines
                    .iter()
                    .map(|line| *line..=*line)
                    .collect(),
                style: Some(lumis::formatters::html_inline::HighlightLinesStyle::Theme),
                class: None,
            }));
        }
    } else {
        map.insert("main".to_string(), fixture_theme(&fixture.metadata.theme));
        builder.default_theme("main");
    }

    let fmt = builder.language(lang).themes(map).build().unwrap();
    let mut out = Vec::new();
    lumis::write_highlight_with_options(
        &mut out,
        &fixture.source,
        fmt,
        HighlightOptions::new().rainbow_brackets(fixture.metadata.rainbow_brackets),
    )
    .unwrap();
    assert_eq!(
        normalize_newlines(&String::from_utf8(out).unwrap()),
        normalize_newlines(&fixture.html_multi_themes)
    );
}

fn check_terminal(fixture: &Fixture) {
    let lang: Language = fixture.metadata.language.parse().unwrap();
    let theme = fixture_theme(&fixture.metadata.theme);
    let fmt = TerminalBuilder::new()
        .language(lang)
        .theme(Some(theme))
        .build()
        .unwrap();
    let mut out = Vec::new();
    lumis::write_highlight_with_options(
        &mut out,
        &fixture.source,
        fmt,
        HighlightOptions::new().rainbow_brackets(fixture.metadata.rainbow_brackets),
    )
    .unwrap();
    assert_eq!(
        normalize_newlines(&String::from_utf8(out).unwrap()),
        normalize_newlines(&fixture.terminal)
    );
}

fn check_bbcode(fixture: &Fixture) {
    let lang: Language = fixture.metadata.language.parse().unwrap();
    let fmt = BBCodeScopedBuilder::new().language(lang).build().unwrap();
    let mut out = Vec::new();
    lumis::write_highlight_with_options(
        &mut out,
        &fixture.source,
        fmt,
        HighlightOptions::new().rainbow_brackets(fixture.metadata.rainbow_brackets),
    )
    .unwrap();
    assert_eq!(
        normalize_newlines(&String::from_utf8(out).unwrap()),
        normalize_newlines(&fixture.bbcode)
    );
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

#[test]
#[allow(deprecated)]
fn deprecated_lang_builder_alias_still_works() {
    let formatter = HtmlInlineBuilder::new()
        .lang(Language::Rust)
        .build()
        .unwrap();

    let html = highlight("fn main() {}", formatter);

    assert!(html.contains("language-rust"));
}

// Generate one test module per fixture directory found at build time.
include!(concat!(env!("OUT_DIR"), "/conformance_tests.rs"));
