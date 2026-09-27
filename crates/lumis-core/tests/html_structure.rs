use lumis_core::events::HighlightEvent;
use lumis_core::formatter::html::{AttrValue, HtmlStructure};
use lumis_core::formatter::html_inline::{HighlightLines, HighlightLinesStyle};
use lumis_core::formatter::{
    BudgetExhausted, Formatter, HtmlElement, HtmlInlineBuilder, HtmlLinkedBuilder,
    HtmlMultiThemesBuilder,
};
use lumis_core::languages::Language;
use lumis_core::themes;
use std::collections::HashMap;

/// A string literal that crosses a line boundary, ending in a newline.
const SOURCE: &str = "\"a\nb\"\n";

fn events() -> Vec<HighlightEvent<'static, ()>> {
    let string = lumis_core::highlights::HIGHLIGHT_NAMES
        .iter()
        .position(|scope| *scope == "string")
        .expect("`string` is a highlight scope");

    vec![
        HighlightEvent::Start {
            scope_index: string,
            language: "javascript".to_string(),
        },
        HighlightEvent::Source { start: 0, end: 5 },
        HighlightEvent::End,
        HighlightEvent::Source { start: 5, end: 6 },
    ]
}

fn render(formatter: &dyn Formatter<()>) -> String {
    let mut output = Vec::new();
    formatter.render(SOURCE, &events(), &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

fn render_budgeted(formatter: &dyn Formatter<()>) -> String {
    let mut output = Vec::new();
    formatter
        .render_budgeted(SOURCE, &events(), &mut output, BudgetExhausted::Matches)
        .unwrap();
    String::from_utf8(output).unwrap()
}

fn light_dark_themes() -> HashMap<String, themes::Theme> {
    HashMap::from([
        ("light".to_string(), themes::get("github_light").unwrap()),
        ("dark".to_string(), themes::get("github_dark").unwrap()),
    ])
}

fn inline_highlight_lines() -> HighlightLines {
    HighlightLines {
        lines: vec![1..=1, 2..=2],
        style: Some(HighlightLinesStyle::Style("background: yellow".to_string())),
        class: Some("active".to_string()),
    }
}

fn pre_attrs() -> Vec<(String, AttrValue)> {
    vec![("id".to_string(), "sample-pre".into())]
}

fn code_attrs() -> Vec<(String, AttrValue)> {
    vec![("id".to_string(), "sample-code".into())]
}

fn header() -> HtmlElement {
    HtmlElement {
        open_tag: "<figure>".to_string(),
        close_tag: "</figure>".to_string(),
    }
}

fn html_inline(block_options: bool) -> impl Formatter<()> {
    let mut builder = HtmlInlineBuilder::new();
    builder
        .language(Language::JavaScript)
        .theme(Some(themes::get("dracula").unwrap()))
        .structure(HtmlStructure::Inline);
    if block_options {
        builder
            .pre_class(Some("code".to_string()))
            .pre_attrs(pre_attrs())
            .code_attrs(code_attrs())
            .highlight_lines(Some(inline_highlight_lines()))
            .line_numbers(true)
            .header(Some(header()));
    }
    builder.build().unwrap()
}

fn html_linked(block_options: bool) -> impl Formatter<()> {
    let mut builder = HtmlLinkedBuilder::new();
    builder
        .language(Language::JavaScript)
        .structure(HtmlStructure::Inline);
    if block_options {
        builder
            .pre_class(Some("code".to_string()))
            .pre_attrs(pre_attrs())
            .code_attrs(code_attrs())
            .highlight_lines(Some(lumis_core::formatter::html_linked::HighlightLines {
                lines: vec![1..=1, 2..=2],
                class: "active".to_string(),
            }))
            .line_numbers(true)
            .header(Some(header()));
    }
    builder.build().unwrap()
}

fn html_multi_themes(block_options: bool) -> impl Formatter<()> {
    let mut builder = HtmlMultiThemesBuilder::new();
    builder
        .language(Language::JavaScript)
        .themes(light_dark_themes())
        .default_theme("light-dark()")
        .structure(HtmlStructure::Inline);
    if block_options {
        builder
            .pre_class(Some("code".to_string()))
            .pre_attrs(pre_attrs())
            .code_attrs(code_attrs())
            .highlight_lines(Some(inline_highlight_lines()))
            .line_numbers(true)
            .header(Some(header()));
    }
    builder.build().unwrap()
}

#[test]
fn inline_structure_writes_only_the_spans() {
    assert_eq!(
        render(&html_inline(false)),
        "<span style=\"color: #f1fa8c;\">&quot;a</span>\n<span style=\"color: #f1fa8c;\">b&quot;</span>"
    );
    assert_eq!(
        render(&html_linked(false)),
        "<span class=\"l-string\">&quot;a</span>\n<span class=\"l-string\">b&quot;</span>"
    );
    assert_eq!(
        render(&html_multi_themes(false)),
        "<span style=\"color: light-dark(#0a3069, #a5d6ff);\">&quot;a</span>\n<span style=\"color: light-dark(#0a3069, #a5d6ff);\">b&quot;</span>"
    );
}

/// An integration highlighting both code blocks and inline code can add
/// `structure` to the options it already has, rather than first removing every
/// option that describes a block.
#[test]
fn inline_structure_ignores_the_options_that_describe_a_block() {
    assert_eq!(render(&html_inline(true)), render(&html_inline(false)));
    assert_eq!(render(&html_linked(true)), render(&html_linked(false)));
    assert_eq!(
        render(&html_multi_themes(true)),
        render(&html_multi_themes(false))
    );
}

#[test]
fn inline_structure_has_nowhere_to_mark_an_exhausted_budget() {
    assert_eq!(
        render_budgeted(&html_inline(false)),
        render(&html_inline(false))
    );
    assert_eq!(
        render_budgeted(&html_linked(false)),
        render(&html_linked(false))
    );
    assert_eq!(
        render_budgeted(&html_multi_themes(false)),
        render(&html_multi_themes(false))
    );
}

#[test]
fn block_is_the_default_structure() {
    assert_eq!(HtmlStructure::default(), HtmlStructure::Block);

    let formatter = HtmlLinkedBuilder::new()
        .language(Language::JavaScript)
        .build()
        .unwrap();
    let explicit = HtmlLinkedBuilder::new()
        .language(Language::JavaScript)
        .structure(HtmlStructure::Block)
        .build()
        .unwrap();

    assert!(render(&formatter).starts_with("<pre class=\"lumis\">"));
    assert_eq!(render(&formatter), render(&explicit));
}
