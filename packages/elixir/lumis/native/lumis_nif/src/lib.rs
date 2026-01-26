use std::collections::HashMap;

mod elixir;

use elixir::{ExFormatterOption, ExTheme};
use lumis::{languages, themes};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rustler::{Encoder, Env, Error, NifMap, NifResult, Term};

/// Lazy per-theme cache to eliminate repeated allocations.
/// Themes are converted and cached on first access, amortizing the cost.
static THEME_CACHE: Lazy<RwLock<HashMap<String, ExTheme>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Cached list of theme names to avoid repeated allocations.
/// Built once on first call to available_themes().
static THEME_NAMES: Lazy<Vec<String>> = Lazy::new(|| {
    themes::available_themes()
        .map(|theme| theme.name.to_owned())
        .collect()
});

rustler::atoms! {
    ok,
    error,
}

rustler::init!("Elixir.Lumis.Native");

#[derive(Debug, NifMap)]
pub struct ExOptions<'a> {
    pub language: Option<&'a str>,
    pub formatter: ExFormatterOption,
}

#[rustler::nif(schedule = "DirtyCpu")]
pub fn highlight<'a>(env: Env<'a>, source: &'a str, options: ExOptions) -> NifResult<Term<'a>> {
    let language = lumis::languages::Language::guess(options.language, source);
    let formatter = options
        .formatter
        .into_formatter(language)
        .map_err(|e| Error::Term(Box::new(e)))?;

    let output = lumis::highlight(source, formatter);

    Ok((ok(), output).encode(env))
}

#[rustler::nif]
fn available_languages() -> HashMap<String, (String, Vec<String>)> {
    languages::available_languages()
}

#[rustler::nif]
fn available_themes() -> Vec<String> {
    // Return a clone of the cached theme names list
    // This is cheaper than rebuilding the list every time
    THEME_NAMES.clone()
}

#[rustler::nif]
fn get_theme(name: &str) -> NifResult<ExTheme> {
    // Fast path: check if theme is already cached (read lock)
    {
        let cache = THEME_CACHE.read();
        if let Some(cached_theme) = cache.get(name) {
            return Ok(cached_theme.clone());
        }
    }

    // Slow path: load theme, convert, and cache it (write lock)
    let theme = themes::get(name).map_err(|_e| Error::Atom("error"))?;
    let ex_theme = ExTheme::from(&theme);

    // Cache the converted theme for future calls
    {
        let mut cache = THEME_CACHE.write();
        cache.insert(name.to_string(), ex_theme.clone());
    }

    Ok(ex_theme)
}

#[rustler::nif]
fn build_theme_from_file(path: &str) -> NifResult<ExTheme> {
    themes::from_file(path)
        .map(|theme| ExTheme::from(&theme))
        .map_err(|_e| Error::Atom("error"))
}

#[rustler::nif]
fn build_theme_from_json_string(json_string: &str) -> NifResult<ExTheme> {
    themes::from_json(json_string)
        .map(|theme| ExTheme::from(&theme))
        .map_err(|_e| Error::Atom("error"))
}

#[cfg(test)]
mod tests {
    use lumis::{languages::Language, HtmlInlineBuilder};

    #[test]
    fn test_highlight_works() {
        let source = "@test :test";
        let lang = Language::guess(Some("elixir"), source);
        let formatter = HtmlInlineBuilder::new().lang(lang).build().unwrap();

        let result = lumis::highlight(source, formatter);

        assert!(!result.is_empty(), "Output should not be empty");

        assert!(
            result.contains("<pre"),
            "Output should contain opening <pre> tag"
        );

        assert!(result.contains("<code"), "Output should contain <code> tag");

        assert!(
            result.contains("test"),
            "Output should contain 'test' keyword"
        );
    }
}
