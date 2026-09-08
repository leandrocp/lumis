use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lumis::{formatters::HtmlInline, languages::Language, themes, HtmlInlineBuilder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};
use std::time::Duration;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    scenarios: Vec<ScenarioSpec>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ScenarioSpec {
    id: String,
    input_bytes: usize,
    files: Vec<FileSpec>,
}

#[derive(Deserialize)]
struct FileSpec {
    path: String,
    language: String,
    syntax: String,
}

struct Scenario {
    id: String,
    input_bytes: usize,
    files: Vec<SourceFile>,
}

struct SourceFile {
    language: String,
    syntax: String,
    source: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScenarioMetadata {
    scenario: String,
    input_bytes: usize,
    file_count: usize,
    language_count: usize,
    output_bytes: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImplementationMetadata {
    implementation: &'static str,
    scenarios: Vec<ScenarioMetadata>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    schema_version: u8,
    runner: &'static str,
    implementations: Vec<ImplementationMetadata>,
}

fn repo_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_scenarios() -> Vec<Scenario> {
    let manifest_path = env::var_os("BENCH_SCENARIO_MANIFEST").map_or_else(
        || repo_dir().join("target/benchmarks/fixtures/scenarios.json"),
        PathBuf::from,
    );
    let manifest: Manifest = serde_json::from_str(
        &fs::read_to_string(manifest_path).expect("read resolved benchmark manifest"),
    )
    .expect("parse resolved benchmark manifest");

    manifest
        .scenarios
        .into_iter()
        .map(|scenario| {
            let files = scenario
                .files
                .into_iter()
                .map(|file| SourceFile {
                    language: file.language,
                    syntax: file.syntax,
                    source: fs::read_to_string(repo_dir().join(file.path))
                        .expect("read benchmark fixture"),
                })
                .collect::<Vec<_>>();
            assert_eq!(
                scenario.input_bytes,
                files.iter().map(|file| file.source.len()).sum::<usize>(),
                "scenario input bytes"
            );
            Scenario {
                id: scenario.id,
                input_bytes: scenario.input_bytes,
                files,
            }
        })
        .collect()
}

fn language(id: &str) -> Language {
    match id {
        "c" => Language::C,
        "css" => Language::CSS,
        "go" => Language::Go,
        "html" => Language::HTML,
        "java" => Language::Java,
        "javascript" => Language::JavaScript,
        "json" => Language::JSON,
        "python" => Language::Python,
        "ruby" => Language::Ruby,
        "rust" => Language::Rust,
        other => panic!("unsupported benchmark language: {other}"),
    }
}

fn initialize_lumis(scenario: &Scenario) -> Vec<(String, HtmlInline)> {
    let theme = themes::get("github_dark").expect("built-in github_dark theme");
    scenario
        .files
        .iter()
        .map(|file| file.language.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|id| {
            let formatter = HtmlInlineBuilder::new()
                .language(language(id))
                .theme(Some(theme.clone()))
                .build()
                .expect("build Lumis formatter");
            (id.to_owned(), formatter)
        })
        .collect()
}

fn render_lumis(runtime: &[(String, HtmlInline)], scenario: &Scenario, validate: bool) -> usize {
    scenario
        .files
        .iter()
        .map(|file| {
            let formatter = &runtime
                .iter()
                .find(|(id, _)| id == &file.language)
                .expect("Lumis formatter for fixture language")
                .1;
            let mut output = Vec::with_capacity(file.source.len().saturating_mul(3));
            lumis::write_highlight(&mut output, black_box(&file.source), formatter)
                .expect("highlight fixture with Lumis");
            if validate {
                validate_html(&output, file.source.len(), "Lumis Rust");
            }
            black_box(output.len())
        })
        .sum()
}

struct SyntectRuntime {
    syntaxes: SyntaxSet,
    themes: ThemeSet,
}

fn initialize_syntect() -> SyntectRuntime {
    SyntectRuntime {
        syntaxes: SyntaxSet::load_defaults_newlines(),
        themes: ThemeSet::load_defaults(),
    }
}

fn render_syntect(runtime: &SyntectRuntime, scenario: &Scenario, validate: bool) -> usize {
    let theme = &runtime.themes.themes["base16-ocean.dark"];
    scenario
        .files
        .iter()
        .map(|file| {
            let syntax = runtime
                .syntaxes
                .find_syntax_by_extension(&file.syntax)
                .unwrap_or_else(|| panic!("syntect syntax for {}", file.syntax));
            let output = highlighted_html_for_string(
                black_box(&file.source),
                &runtime.syntaxes,
                syntax,
                theme,
            )
            .expect("highlight fixture with syntect");
            if validate {
                validate_html(output.as_bytes(), file.source.len(), "syntect");
            }
            black_box(output.len())
        })
        .sum()
}

fn validate_html(output: &[u8], input_bytes: usize, implementation: &str) {
    let html = std::str::from_utf8(output).expect("benchmark output is UTF-8");
    assert!(
        output.len() > input_bytes && html.contains("<pre") && html.contains("<span"),
        "{implementation} did not produce highlighted HTML"
    );
}

fn duration(name: &str, default: f64) -> Duration {
    Duration::from_secs_f64(
        env::var(name)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(default),
    )
}

fn write_metadata(scenarios: &[Scenario]) {
    let Some(directory) = env::var_os("BENCH_METADATA_DIR") else {
        return;
    };
    let lumis = scenarios
        .iter()
        .map(|scenario| {
            let runtime = initialize_lumis(scenario);
            scenario_metadata(scenario, render_lumis(&runtime, scenario, true))
        })
        .collect();
    let syntect = scenarios
        .iter()
        .map(|scenario| {
            let runtime = initialize_syntect();
            scenario_metadata(scenario, render_syntect(&runtime, scenario, true))
        })
        .collect();
    let metadata = Metadata {
        schema_version: 1,
        runner: "criterion",
        implementations: vec![
            ImplementationMetadata {
                implementation: "lumis-rust",
                scenarios: lumis,
            },
            ImplementationMetadata {
                implementation: "syntect",
                scenarios: syntect,
            },
        ],
    };
    let path = Path::new(&directory).join("rust-metadata.json");
    fs::create_dir_all(path.parent().expect("metadata parent")).expect("create metadata directory");
    fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&metadata).expect("serialize Rust metadata")
        ),
    )
    .expect("write Rust metadata");
}

fn scenario_metadata(scenario: &Scenario, output_bytes: usize) -> ScenarioMetadata {
    assert!(output_bytes > scenario.input_bytes);
    ScenarioMetadata {
        scenario: scenario.id.clone(),
        input_bytes: scenario.input_bytes,
        file_count: scenario.files.len(),
        language_count: scenario
            .files
            .iter()
            .map(|file| file.language.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        output_bytes,
    }
}

fn benchmarks(c: &mut Criterion) {
    let scenarios = load_scenarios();
    write_metadata(&scenarios);
    let sample_size = env::var("BENCH_SAMPLES")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(20)
        .max(10);

    for scenario in &scenarios {
        let mut group = c.benchmark_group(&scenario.id);
        group.sample_size(sample_size);
        group.warm_up_time(duration("BENCH_WARMUP_SECONDS", 0.5));
        group.measurement_time(duration("BENCH_TIME_SECONDS", 1.0));

        group.bench_function(BenchmarkId::new("lumis-rust", "total"), |b| {
            b.iter_with_large_drop(|| {
                let runtime = initialize_lumis(scenario);
                let output_bytes = render_lumis(&runtime, scenario, false);
                black_box((runtime, output_bytes))
            });
        });
        group.bench_function(BenchmarkId::new("syntect", "total"), |b| {
            b.iter_with_large_drop(|| {
                let runtime = initialize_syntect();
                let output_bytes = render_syntect(&runtime, scenario, false);
                black_box((runtime, output_bytes))
            });
        });
        group.finish();
    }
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
