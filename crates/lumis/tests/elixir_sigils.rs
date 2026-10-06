//! Elixir highlights a sigil in time linear in how many escapes its body holds.
//!
//! A lowercase sigil's body parses as a `quoted_content` per run of text and an
//! `escape_sequence` per escape, all of them siblings, so a regex with a few
//! hundred escapes is a few hundred chunks. Published sources reach that: an
//! RFC 822 address regex in one line. Each shape here renders in milliseconds
//! and ran out of its default budget, growing faster than the square of the
//! chunk count, while the string sigil and documentation patterns held a match
//! open for every chunk until the closing delimiter.
#![cfg(feature = "lang-elixir")]

use lumis::{languages::Language, HighlightOptions, HtmlLinkedBuilder};

const CHUNKS: usize = 800;

fn render(source: &str) -> String {
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::Elixir)
        .build()
        .unwrap();

    lumis::highlight_with_options(source, formatter, HighlightOptions::new())
}

fn assert_highlighted_within_budget(html: &str) {
    assert!(
        !html.contains("data-lumis-budget"),
        "the render ran out of its default budget, got {}",
        &html[..html.len().min(200)]
    );
    assert_eq!(
        html.matches(r#"<span class="l-string-escape">\n</span>"#)
            .count(),
        CHUNKS,
        "the sigil came back unhighlighted"
    );
}

#[test]
fn an_escaped_string_sigil_highlights_within_the_default_budget() {
    let source = format!(
        "defmodule M do\n  def f, do: ~s({})\nend\n",
        r"a\n".repeat(CHUNKS)
    );

    assert_highlighted_within_budget(&render(&source));
}

#[test]
fn an_escaped_doc_sigil_highlights_within_the_default_budget() {
    let source = format!(
        "defmodule M do\n  @moduledoc ~s\"\"\"\n  {}\n  \"\"\"\nend\n",
        r"a\n".repeat(CHUNKS)
    );

    assert_highlighted_within_budget(&render(&source));
}
