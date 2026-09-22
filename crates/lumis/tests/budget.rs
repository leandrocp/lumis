//! A render is bounded, and says so when it hits a bound.
//!
//! The two dimensions — query matches and wall clock — are one feature: either
//! one running out has to leave the caller with the whole document and a way to
//! find out it happened. These tests pin both halves of that, because a
//! regression in either is silent by construction. Losing the degradation turns
//! a slow render into an error; losing the marker turns a plain or
//! scope-starved render into something indistinguishable from a correct one.
//!
//! `time_limit(Some(0))` is the deterministic lever: the deadline is `now`, so
//! it is already past by the first check and no test here depends on how fast
//! the machine running it is.

use lumis::{
    languages::Language, HighlightOptions, HtmlInlineBuilder, HtmlLinkedBuilder,
    HtmlMultiThemesBuilder, TerminalBuilder,
};
use std::collections::HashMap;

/// Rust that nests enough for a query pattern to stay open across a subtree.
const SOURCE: &str = "fn main() { let value = (1 + (2 * (3 - 4))); }\n";

/// Source whose plain rendering is wrong unless it is still HTML-escaped.
const UNSAFE_SOURCE: &str = "fn main() { let _ = a < b && c > d; }\n";

fn html_linked(source: &str, options: HighlightOptions<'_, ()>) -> String {
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::Rust)
        .build()
        .unwrap();

    lumis::highlight_with_options(source, formatter, options)
}

/// The clock is already spent, so every check sees an expired deadline.
fn spent() -> HighlightOptions<'static, ()> {
    HighlightOptions::new().time_limit(Some(0))
}

#[test]
fn an_exhausted_time_budget_returns_the_whole_document_as_plain_text() {
    let html = html_linked(SOURCE, spent());

    assert!(
        !html.contains("<span"),
        "an exhausted budget renders no scopes, got {html}"
    );
    assert!(
        html.contains(SOURCE.trim_end()),
        "the whole source survives the degradation, got {html}"
    );
}

#[test]
fn an_exhausted_time_budget_marks_the_pre() {
    let html = html_linked(SOURCE, spent());

    assert!(
        html.contains(r#"data-lumis-budget="time""#),
        "a time-exhausted render says so on the pre, got {html}"
    );
}

#[test]
fn plain_degradation_still_escapes_html() {
    let html = html_linked(UNSAFE_SOURCE, spent());

    assert!(
        html.contains("a &lt; b &amp;&amp; c &gt; d"),
        "degrading to plain text does not mean degrading to raw text, got {html}"
    );
    assert!(
        !html.contains("< b &&"),
        "unescaped source reached the output, got {html}"
    );
}

#[test]
fn an_unspent_budget_highlights_and_marks_nothing() {
    let html = html_linked(SOURCE, HighlightOptions::new());

    assert!(
        html.contains("<span"),
        "the default 5 s budget is not reachable by a one-line document, got {html}"
    );
    assert!(
        !html.contains("data-lumis-budget"),
        "a render that stayed inside its budget carries no marker, got {html}"
    );
}

#[test]
fn a_removed_time_limit_highlights() {
    let html = html_linked(SOURCE, HighlightOptions::new().time_limit(None));

    assert!(html.contains("<span"), "got {html}");
    assert!(!html.contains("data-lumis-budget"), "got {html}");
}

#[test]
fn every_html_formatter_marks_the_pre() {
    let inline = lumis::highlight_with_options(
        SOURCE,
        HtmlInlineBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap(),
        spent(),
    );
    let mut themes = HashMap::new();
    themes.insert("main".to_string(), lumis::themes::get("dracula").unwrap());
    let multi = lumis::highlight_with_options(
        SOURCE,
        HtmlMultiThemesBuilder::new()
            .language(Language::Rust)
            .themes(themes)
            .default_theme("main")
            .build()
            .unwrap(),
        spent(),
    );

    for (name, html) in [("html_inline", &inline), ("html_multi_themes", &multi)] {
        assert!(
            html.contains(r#"data-lumis-budget="time""#),
            "{name} does not mark the pre, got {html}"
        );
        assert!(
            !html.contains("<span"),
            "{name} rendered scopes, got {html}"
        );
    }
}

#[test]
fn a_formatter_without_an_attribute_channel_still_degrades() {
    // The terminal has nowhere to put a marker. It is still required to hand
    // back the document rather than fail, which is the part that matters.
    let output = lumis::highlight_with_options(
        SOURCE,
        TerminalBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap(),
        spent(),
    );

    assert_eq!(output, SOURCE, "the terminal returns the source unchanged");
}

#[test]
fn an_exhausted_match_budget_marks_the_pre_and_keeps_highlighting() {
    // Matches are the one dimension that does not degrade to plain: tree-sitter
    // drops in-progress matches and carries on, so the output is highlighted
    // with scopes missing. The marker is the only way to learn that.
    let html = html_linked(SOURCE, HighlightOptions::new().match_limit(1));

    assert!(
        html.contains(r#"data-lumis-budget="matches""#),
        "dropped matches are reported, got {html}"
    );
    assert!(
        html.contains(SOURCE.trim_end()) || html.contains("value"),
        "the document is still whole, got {html}"
    );
}

/// A boxed formatter marks the pre, like the bare one it holds.
///
/// `Formatter` is implemented for `Box<dyn Formatter<T>>` and `&F` by
/// forwarding, and a forward that stops at `render` silently loses the marker:
/// the default `render_budgeted` calls the *wrapper's* `render`, which reaches
/// the inner formatter's `render` and never its override. The Elixir NIF holds
/// a `Box<dyn Formatter<T>>`, so this is not a hypothetical shape.
#[test]
// The borrow is the subject, not an accident: passing the formatter by value
// would select the inherent impl and leave `impl Formatter for &F` untested.
#[allow(clippy::needless_borrows_for_generic_args)]
fn the_forwarding_impls_do_not_swallow_the_marker() {
    let boxed: Box<dyn lumis::formatters::Formatter<()>> = Box::new(
        HtmlLinkedBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap(),
    );
    let through_box = lumis::highlight_with_options(SOURCE, &boxed, spent());
    let through_reference = lumis::highlight_with_options(
        SOURCE,
        &HtmlLinkedBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap(),
        spent(),
    );

    for (shape, html) in [("Box", &through_box), ("&F", &through_reference)] {
        assert!(
            html.contains(r#"data-lumis-budget="time""#),
            "the {shape} impl dropped the marker, got {html}"
        );
    }
}

/// The render after an exhausted one is a render of its own document.
///
/// tree-sitter resumes a stopped parse on the next call unless the parser is
/// reset, and highlighters are pooled and reused. Without the reset the next
/// render continues the previous document — under its own fresh clock, so it is
/// not bounded either, and a single pathological input poisons every render
/// that follows it on the same parser.
#[test]
fn an_exhausted_render_does_not_leak_into_the_next_one() {
    // `lumis::highlight` keeps one tree-sitter highlighter per thread, and
    // cargo gives this test a thread of its own, so both calls here share the
    // parser that the first one interrupted.
    const OTHER: &str = "const X: u8 = 7;\n";

    let isolated = html_linked(OTHER, HighlightOptions::new());
    let _ = html_linked(SOURCE, spent());
    let after = html_linked(OTHER, HighlightOptions::new());

    assert_eq!(
        after, isolated,
        "the render after an exhausted one is not the render of its own document"
    );
    assert!(after.contains("<span"), "got {after}");
    assert!(!after.contains("data-lumis-budget"), "got {after}");
}

#[test]
fn exhaustion_is_not_an_error() {
    let mut output = Vec::new();
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::Rust)
        .build()
        .unwrap();

    lumis::write_highlight_with_options(&mut output, SOURCE, formatter, spent())
        .expect("an exhausted budget is a rendering outcome, not a failure");
}
