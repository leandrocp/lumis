//! A render is bounded, and says so when it hits a bound.
//!
//! The two dimensions — query matches and wall clock — are one feature: either
//! one running out has to leave the caller with the whole document and a way to
//! find out it happened. These tests pin both halves of that, because a
//! regression in either is silent by construction. Losing the degradation turns
//! a slow render into an error; losing the marker turns a plain or
//! scope-starved render into something indistinguishable from a correct one.
//!
//! The lever is a 1 ms limit on [`exhausting`] source, which takes seconds to
//! render without a limit, so the render runs out on any machine. `0` cannot be
//! the lever: it removes the limit, as it does in every runtime.

use lumis::{
    languages::Language, Budget, HighlightOptions, HtmlInlineBuilder, HtmlLinkedBuilder,
    HtmlMultiThemesBuilder, TerminalBuilder,
};
use std::collections::HashMap;

/// Rust that nests enough for a query pattern to stay open across a subtree.
const SOURCE: &str = "fn main() { let value = (1 + (2 * (3 - 4))); }\n";

/// Source whose plain rendering is wrong unless it is still HTML-escaped.
const UNSAFE_SOURCE: &str = "fn main() { let _ = a < b && c > d; }\n";

fn html_linked(source: &str, options: HighlightOptions<'_, '_, ()>) -> String {
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::Rust)
        .build()
        .unwrap();

    lumis::highlight_with_options(source, formatter, options)
}

/// `source` with 20,000 unclosed parentheses before its newline. The parser's
/// error recovery takes seconds on them, against the millisecond of [`spent`].
/// They stay on the source's one line, so a plain render is still one line span.
fn exhausting(source: &str) -> String {
    format!("{}{}\n", source.trim_end(), "(".repeat(20_000))
}

/// A budget that [`exhausting`] source runs out of.
fn spent() -> HighlightOptions<'static, 'static, ()> {
    HighlightOptions::new().budget(Budget::new().time_limit(Some(1)))
}

#[test]
fn an_exhausted_time_budget_returns_the_whole_document_as_plain_text() {
    let source = exhausting(SOURCE);
    let html = html_linked(&source, spent());

    assert!(
        html.matches("<span").count() == 1,
        "an exhausted budget renders only the line span, got {html}"
    );
    assert!(
        html.contains(source.trim_end()),
        "the whole source survives the degradation, got {html}"
    );
}

#[test]
fn an_exhausted_time_budget_marks_the_pre() {
    let html = html_linked(&exhausting(SOURCE), spent());

    assert!(
        html.contains(r#"data-lumis-budget="time""#),
        "a time-exhausted render says so on the pre, got {html}"
    );
}

#[test]
fn plain_degradation_still_escapes_html() {
    let html = html_linked(&exhausting(UNSAFE_SOURCE), spent());

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
        html.matches("<span").count() > 1,
        "the default 5 s budget is not reachable by a one-line document, got {html}"
    );
    assert!(
        !html.contains("data-lumis-budget"),
        "a render that stayed inside its budget carries no marker, got {html}"
    );
}

#[test]
fn a_removed_time_limit_highlights() {
    let html = html_linked(
        SOURCE,
        HighlightOptions::new().budget(Budget::new().time_limit(None)),
    );

    assert!(html.matches("<span").count() > 1, "got {html}");
    assert!(!html.contains("data-lumis-budget"), "got {html}");
}

/// `0` removes the limit, as it does in the JavaScript, Elixir and CLI
/// bindings, so one number means the same thing in every runtime.
#[test]
fn a_zero_time_limit_highlights() {
    let html = html_linked(
        SOURCE,
        HighlightOptions::new().budget(Budget::new().time_limit(Some(0))),
    );

    assert!(html.matches("<span").count() > 1, "got {html}");
    assert!(!html.contains("data-lumis-budget"), "got {html}");
}

#[test]
fn every_html_formatter_marks_the_pre() {
    let source = exhausting(SOURCE);
    let inline = lumis::highlight_with_options(
        &source,
        HtmlInlineBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap(),
        spent(),
    );
    let mut themes = HashMap::new();
    themes.insert("main".to_string(), lumis::themes::get("dracula").unwrap());
    let multi = lumis::highlight_with_options(
        &source,
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
            html.matches("<span").count() == 1,
            "{name} rendered scopes, got {html}"
        );
    }
}

#[test]
fn a_formatter_without_an_attribute_channel_still_degrades() {
    // The terminal has nowhere to put a marker. It is still required to hand
    // back the document rather than fail, which is the part that matters.
    let source = exhausting(SOURCE);
    let output = lumis::highlight_with_options(
        &source,
        TerminalBuilder::new()
            .language(Language::Rust)
            .build()
            .unwrap(),
        spent(),
    );

    assert_eq!(output, source, "the terminal returns the source unchanged");
}

#[test]
fn an_exhausted_match_budget_marks_the_pre_and_keeps_highlighting() {
    // Matches are the one dimension that does not degrade to plain: tree-sitter
    // drops in-progress matches and carries on, so the output is highlighted
    // with scopes missing. The marker is the only way to learn that.
    let html = html_linked(
        SOURCE,
        HighlightOptions::new().budget(Budget::new().match_limit(1)),
    );

    assert!(
        html.contains(r#"data-lumis-budget="matches""#),
        "dropped matches are reported, got {html}"
    );
    assert!(
        html.matches("<span").count() > SOURCE.lines().count(),
        "matches is not plain-text degradation; the document stays highlighted, got {html}"
    );
    assert!(
        html.contains("value"),
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
    let source = exhausting(SOURCE);
    let through_box = lumis::highlight_with_options(&source, &boxed, spent());
    let through_reference = lumis::highlight_with_options(
        &source,
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
    let _ = html_linked(&exhausting(SOURCE), spent());
    let after = html_linked(OTHER, HighlightOptions::new());

    assert_eq!(
        after, isolated,
        "the render after an exhausted one is not the render of its own document"
    );
    assert!(after.matches("<span").count() > 1, "got {after}");
    assert!(!after.contains("data-lumis-budget"), "got {after}");
}

/// Rainbow brackets are inside the budget, not beside it.
///
/// They parse and query the document a second time. A render that bounded its
/// highlight and then ran that pass unbounded is not bounded, and decorating an
/// exhausted render would put rainbow spans on a document that is meant to be
/// plain.
#[test]
fn rainbow_brackets_do_not_escape_the_budget() {
    let html = html_linked(&exhausting(SOURCE), spent().rainbow_brackets(true));

    assert!(html.contains(r#"data-lumis-budget="time""#), "got {html}");
    assert!(
        html.matches("<span").count() == 1,
        "an exhausted render came back with rainbow spans, got {html}"
    );

    let ordinary = html_linked(SOURCE, HighlightOptions::new().rainbow_brackets(true));
    assert!(
        ordinary.contains("l-punctuation-bracket-rainbow-"),
        "a render inside its budget still gets rainbow brackets, got {ordinary}"
    );
}

#[test]
fn exhaustion_is_not_an_error() {
    let mut output = Vec::new();
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::Rust)
        .build()
        .unwrap();

    lumis::write_highlight_with_options(&mut output, &exhausting(SOURCE), formatter, spent())
        .expect("an exhausted budget is a rendering outcome, not a failure");
}
