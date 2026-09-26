//! Canonical HTML for `mise run qa-html-lines`.

use lumis::{
    formatters::{
        html_inline::{HighlightLines, HighlightLinesStyle},
        Formatter,
    },
    languages::Language,
    themes, HtmlInlineBuilder, HtmlLinkedBuilder, HtmlMultiThemesBuilder,
};
use serde::Deserialize;
use std::{collections::BTreeMap, env, fs, path::Path};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    language: String,
    theme: String,
    highlight_lines: Vec<usize>,
    line_numbers: Vec<bool>,
    formatters: Vec<String>,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    source: String,
}

fn formatter(manifest: &Manifest, name: &str, numbered: bool) -> Box<dyn Formatter> {
    let theme = themes::get(&manifest.theme).expect("QA theme exists");
    let lines = manifest.highlight_lines.iter().map(|&n| n..=n).collect();
    let highlights = HighlightLines {
        lines,
        style: Some(HighlightLinesStyle::Theme),
        class: Some("l-highlighted".into()),
    };
    match name {
        "html-inline" => Box::new(
            HtmlInlineBuilder::new()
                .language(Language::Rust)
                .theme(Some(theme))
                .highlight_lines(Some(highlights))
                .line_numbers(numbered)
                .italic(true)
                .build()
                .expect("inline formatter"),
        ),
        "html-linked" => Box::new(
            HtmlLinkedBuilder::new()
                .language(Language::Rust)
                .highlight_lines(Some(lumis::formatters::html_linked::HighlightLines {
                    lines: highlights.lines,
                    class: "l-highlighted".into(),
                }))
                .line_numbers(numbered)
                .build()
                .expect("linked formatter"),
        ),
        "html-multi-themes" => Box::new(
            HtmlMultiThemesBuilder::new()
                .language(Language::Rust)
                .themes([("dark".into(), theme)].into())
                .default_theme("dark")
                .highlight_lines(Some(highlights))
                .line_numbers(numbered)
                .italic(true)
                .build()
                .expect("multi-theme formatter"),
        ),
        other => panic!("unknown QA formatter: {other}"),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert_eq!(args.len(), 2, "pass the manifest and output paths");
    let manifest: Manifest = serde_json::from_slice(&fs::read(&args[0]).expect("read manifest"))
        .expect("parse manifest");
    assert_eq!(manifest.language, "rust");
    let mut output = BTreeMap::new();
    for case in &manifest.cases {
        for name in &manifest.formatters {
            for &numbered in &manifest.line_numbers {
                let gutter = if numbered { "numbered" } else { "plain" };
                output.insert(
                    format!("{}/{name}/{gutter}", case.id),
                    lumis::highlight(&case.source, formatter(&manifest, name, numbered)),
                );
            }
        }
    }
    fs::create_dir_all(Path::new(&args[1]).parent().expect("output directory"))
        .expect("create output directory");
    fs::write(&args[1], serde_json::to_string_pretty(&output).unwrap()).expect("write QA HTML");
}
