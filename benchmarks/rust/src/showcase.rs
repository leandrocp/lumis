use lumis::{formatters::HtmlInline, languages::Language, themes, HtmlInlineBuilder};
use serde::Deserialize;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use syntect::{
    highlighting::{Theme as SyntectTheme, ThemeSet},
    html::highlighted_html_for_string,
    parsing::{SyntaxReference, SyntaxSet},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: String,
    pub language: String,
    pub file: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowcaseTheme {
    pub id: String,
    pub lumis: String,
    pub tm_theme: String,
}

pub fn generated_dir() -> PathBuf {
    PathBuf::from(env::var_os("BENCH_SHOWCASE_DIR").expect("BENCH_SHOWCASE_DIR"))
}

pub fn documents(assets_dir: &Path) -> Vec<Document> {
    serde_json::from_slice(
        &fs::read(assets_dir.join("documents.json")).expect("read showcase documents"),
    )
    .expect("parse showcase documents")
}

pub fn themes(assets_dir: &Path) -> Vec<ShowcaseTheme> {
    serde_json::from_slice(&fs::read(assets_dir.join("themes.json")).expect("read showcase themes"))
        .expect("parse showcase themes")
}

pub fn lumis_theme(theme: &ShowcaseTheme) -> themes::Theme {
    themes::get(&theme.lumis)
        .unwrap_or_else(|error| panic!("built-in {} theme: {error}", theme.lumis))
}

pub fn syntect_theme(assets_dir: &Path, theme: &ShowcaseTheme) -> SyntectTheme {
    ThemeSet::get_theme(assets_dir.join(&theme.tm_theme))
        .unwrap_or_else(|error| panic!("load {}: {error}", theme.tm_theme))
}

/// bat does not use syntect's default syntax set either, it bundles a larger
/// one; `two-face` is that set packaged for syntect. Comparing against the
/// defaults would credit Lumis for languages syntect users do have.
pub fn syntect_syntaxes() -> SyntaxSet {
    two_face::syntax::extra_newlines()
}

/// A syntax can still be missing, and `finish-showcase.mjs` holds the declared
/// list and fails if that set ever changes, so `None` is safe to skip.
pub fn syntect_syntax<'a>(
    syntaxes: &'a SyntaxSet,
    document: &Document,
) -> Option<&'a SyntaxReference> {
    let extension = document
        .file
        .rsplit_once('.')
        .expect("fixture has an extension")
        .1;
    syntaxes.find_syntax_by_extension(extension)
}

pub fn lumis_formatter(document: &Document, theme: &themes::Theme) -> HtmlInline {
    let language: Language = document.language.parse().unwrap_or_else(|error| {
        panic!("showcase document language {}: {error}", document.language)
    });
    HtmlInlineBuilder::new()
        .language(language)
        .theme(Some(theme.clone()))
        .build()
        .expect("build Lumis formatter")
}

pub fn highlight_lumis(source: &str, formatter: &HtmlInline) -> Vec<u8> {
    let mut output = Vec::with_capacity(source.len().saturating_mul(3));
    lumis::write_highlight(&mut output, source, formatter)
        .expect("highlight showcase fixture with Lumis");
    output
}

pub fn highlight_syntect(
    source: &str,
    syntaxes: &SyntaxSet,
    syntax: &SyntaxReference,
    theme: &SyntectTheme,
) -> String {
    highlighted_html_for_string(source, syntaxes, syntax, theme)
        .expect("highlight showcase fixture with syntect")
}
