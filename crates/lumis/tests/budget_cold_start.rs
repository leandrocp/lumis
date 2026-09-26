//! Compiling a language's queries is not charged to the document.
//!
//! This is its own test binary because the claim only holds once per process:
//! every `Language`'s queries live behind a `LazyLock`, and the first render
//! that needs one pays for it. Measured here, an HTML document with a `<style>`
//! and a `<script>` costs about 325 ms the first time and about 0.25 ms every
//! time after — three orders of magnitude, all of it compilation.
//!
//! Charging that would make a budget mean something different for the first
//! render of a language than for the rest, which is the opposite of what a
//! budget is for: the first request after a deploy would come back plain, and
//! nothing about the document would explain why.

use lumis::{languages::Language, Budget, HighlightOptions, HtmlLinkedBuilder};

/// HTML injects css and javascript, so this forces three sets of queries: the
/// root before the clock starts, and two more during the walk.
const SOURCE: &str = "<html><style>a{color:red}</style><script>var x=1;</script></html>\n";

#[test]
fn compiling_queries_is_not_charged_to_the_budget() {
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::HTML)
        .build()
        .unwrap();

    // Well under what compilation costs, and far above what the document does.
    let html = lumis::highlight_with_options(
        SOURCE,
        formatter,
        HighlightOptions::new().budget(Budget::new().time_limit(Some(100))),
    );

    assert!(
        !html.contains("data-lumis-budget"),
        "the first render of a language exhausted its budget compiling queries, got {html}"
    );
    assert!(
        html.matches("<span").count() > SOURCE.lines().count(),
        "the document came back unhighlighted, got {html}"
    );
}
