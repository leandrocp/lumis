use lumis::{formatters::HtmlInline, languages::Language, themes, HtmlInlineBuilder};
use serde::Deserialize;
use std::{env, fs, path::PathBuf};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Document {
    id: String,
    language: String,
    file: String,
}

// The fields mirror the JSON keys the showcase manifest uses.
#[allow(clippy::struct_field_names)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Theme {
    id: String,
    lumis: String,
    tm_theme: String,
}

fn main() {
    let generated_dir =
        PathBuf::from(env::var_os("BENCH_SHOWCASE_DIR").expect("BENCH_SHOWCASE_DIR"));
    let assets_dir = generated_dir.join("assets");
    let documents: Vec<Document> = serde_json::from_slice(
        &fs::read(assets_dir.join("documents.json")).expect("read showcase documents"),
    )
    .expect("parse showcase documents");
    let showcase_themes: Vec<Theme> = serde_json::from_slice(
        &fs::read(assets_dir.join("themes.json")).expect("read showcase themes"),
    )
    .expect("parse showcase themes");

    // bat does not use syntect's default syntax set either, it bundles a larger
    // one; `two-face` is that set packaged for syntect. Comparing against the
    // defaults would credit Lumis for languages syntect users do have.
    let syntaxes = two_face::syntax::extra_newlines();

    for showcase_theme in &showcase_themes {
        let theme = themes::get(&showcase_theme.lumis)
            .unwrap_or_else(|error| panic!("built-in {} theme: {error}", showcase_theme.lumis));
        let syntect_theme = ThemeSet::get_theme(assets_dir.join(&showcase_theme.tm_theme))
            .unwrap_or_else(|error| panic!("load {}: {error}", showcase_theme.tm_theme));

        for document in &documents {
            let source =
                fs::read_to_string(assets_dir.join(&document.file)).expect("read showcase fixture");
            let fragments_dir = generated_dir
                .join("fragments")
                .join(&document.id)
                .join(&showcase_theme.id);
            fs::create_dir_all(&fragments_dir).expect("create fragments directory");

            let formatter: HtmlInline = HtmlInlineBuilder::new()
                .language(language(&document.language))
                .theme(Some(theme.clone()))
                .build()
                .expect("build Lumis formatter");
            let mut lumis_output = Vec::with_capacity(source.len().saturating_mul(3));
            lumis::write_highlight(&mut lumis_output, &source, &formatter)
                .expect("highlight showcase fixture with Lumis");
            validate(&lumis_output, source.len(), "Lumis Rust");
            fs::write(fragments_dir.join("lumis-rust.html"), lumis_output)
                .expect("write Lumis Rust fragment");

            // A syntax can still be missing, and `finish-showcase.mjs` holds the
            // declared list and fails if that set ever changes, so silence is safe.
            if let Some(syntax) = syntaxes.find_syntax_by_extension(extension(&document.file)) {
                let syntect_output =
                    highlighted_html_for_string(&source, &syntaxes, syntax, &syntect_theme)
                        .expect("highlight showcase fixture with syntect");
                validate(syntect_output.as_bytes(), source.len(), "syntect");
                fs::write(fragments_dir.join("syntect.html"), syntect_output)
                    .expect("write syntect fragment");
            }
        }
    }
}

fn language(name: &str) -> Language {
    match name {
        "html" => Language::HTML,
        "rust" => Language::Rust,
        "elixir" => Language::Elixir,
        "go" => Language::Go,
        "markdown" => Language::Markdown,
        "tsx" => Language::Tsx,
        other => panic!("showcase document language {other} is not built into this binary"),
    }
}

fn extension(file: &str) -> &str {
    file.rsplit_once('.').expect("fixture has an extension").1
}

fn validate(output: &[u8], input_bytes: usize, implementation: &str) {
    let html = std::str::from_utf8(output).expect("highlight output is UTF-8");
    assert!(
        output.len() > input_bytes && html.contains("<pre") && html.contains("<span"),
        "{implementation} did not produce highlighted HTML"
    );
}
