//! `LanguageConfig::config` with `lumis::vendor::tree_sitter_highlight`, used
//! from outside the crate ([#2086](https://github.com/leandrocp/lumis/issues/2086)).
//! Narrowing either back to `pub(crate)` fails to compile here, which the
//! doctest cannot catch because CI runs the suite through nextest.
#![cfg(all(feature = "lang-rust", feature = "lang-markdown"))]

use lumis::highlights::HIGHLIGHT_NAMES;
use lumis::languages::{Language, LanguageConfig};
use lumis::vendor::tree_sitter_highlight::{HighlightEvent, Highlighter, Interrupt};

/// Each highlighted slice of `source` as `(text, language, scope)`, innermost scope only.
fn highlighted(language: Language, source: &str) -> Vec<(String, String, &'static str)> {
    let mut highlighter = Highlighter::new();
    let events = highlighter
        .highlight(
            language.config(),
            source.as_bytes(),
            Interrupt::none(),
            |name| {
                name.parse::<Language>()
                    .ok()
                    .map(|language| language.config())
            },
        )
        .expect("highlight starts");

    let mut open = Vec::new();
    let mut slices = Vec::new();
    for event in events {
        match event.expect("highlight event") {
            HighlightEvent::HighlightStart {
                highlight,
                language,
            } => open.push((language, HIGHLIGHT_NAMES[highlight.0])),
            HighlightEvent::HighlightEnd => {
                open.pop().expect("balanced highlight events");
            }
            HighlightEvent::Source { start, end } => {
                if let Some((language, scope)) = open.last() {
                    slices.push((source[start..end].to_string(), language.clone(), *scope));
                }
            }
        }
    }
    assert!(open.is_empty(), "unclosed highlights: {open:?}");
    slices
}

fn slice(text: &str, language: &str, scope: &'static str) -> (String, String, &'static str) {
    (text.to_string(), language.to_string(), scope)
}

#[test]
fn config_highlights_its_language_with_lumis_scope_names() {
    let slices = highlighted(Language::Rust, "fn main() {}\n");

    assert!(
        slices.contains(&slice("fn", "rust", "keyword.function")),
        "{slices:?}"
    );
    assert!(
        slices.contains(&slice("main", "rust", "function")),
        "{slices:?}"
    );
}

#[test]
fn injection_callback_resolves_injected_languages_through_config() {
    let slices = highlighted(
        Language::Markdown,
        "# Title\n\n```rust\nfn main() {}\n```\n",
    );

    assert!(
        slices.contains(&slice("fn", "rust", "keyword.function")),
        "{slices:?}"
    );
    assert!(
        slices.iter().any(|(_, language, _)| language == "markdown"),
        "{slices:?}"
    );
}
