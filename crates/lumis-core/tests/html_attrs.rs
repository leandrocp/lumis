use lumis_core::events::HighlightEvent;
use lumis_core::formatter::html::AttrValue;
use lumis_core::formatter::{
    Formatter, HtmlInlineBuilder, HtmlLinkedBuilder, HtmlMultiThemesBuilder,
};
use lumis_core::languages::Language;
use std::collections::HashMap;

fn pre_attrs() -> Vec<(String, AttrValue)> {
    vec![
        ("class".to_string(), "shorthand authored".into()),
        ("style".to_string(), "outline: 1px solid red".into()),
        ("id".to_string(), "sample-pre".into()),
        ("inert".to_string(), true.into()),
    ]
}

fn code_attrs() -> Vec<(String, AttrValue)> {
    vec![
        ("class".to_string(), "copyable language-plaintext".into()),
        ("translate".to_string(), "yes".into()),
        ("tabindex".to_string(), false.into()),
        ("id".to_string(), "sample-code".into()),
    ]
}

fn render(formatter: &dyn Formatter<()>) -> String {
    let events: [HighlightEvent<'_, ()>; 0] = [];
    let mut output = Vec::new();
    formatter.render("", &events, &mut output).unwrap();
    String::from_utf8(output).unwrap()
}

fn render_result(formatter: &dyn Formatter<()>) -> std::io::Result<Vec<u8>> {
    let events: [HighlightEvent<'_, ()>; 0] = [];
    let mut output = Vec::new();
    formatter.render("", &events, &mut output)?;
    Ok(output)
}

fn assert_common_attrs(html: &str) {
    assert!(
        html.starts_with(concat!(
            r#"<pre class="lumis shorthand authored" style="outline: 1px solid red" id="sample-pre" inert>"#,
            r#"<code class="language-plaintext copyable" translate="yes" id="sample-code">"#,
        )),
        "{html}"
    );
}

#[test]
fn inline_and_linked_formatters_accept_pre_and_code_attrs() {
    let inline = HtmlInlineBuilder::new()
        .language(Language::PlainText)
        .pre_class(Some("shorthand".to_string()))
        .pre_attrs(pre_attrs())
        .code_attrs(code_attrs())
        .build()
        .unwrap();
    let linked = HtmlLinkedBuilder::new()
        .language(Language::PlainText)
        .pre_class(Some("shorthand".to_string()))
        .pre_attrs(pre_attrs())
        .code_attrs(code_attrs())
        .build()
        .unwrap();

    assert_common_attrs(&render(&inline));
    assert_common_attrs(&render(&linked));
}

#[test]
fn multi_themes_formatter_accepts_pre_and_code_attrs() {
    let themes = HashMap::from([(
        "dark".to_string(),
        lumis_core::themes::get("dracula").unwrap(),
    )]);
    let formatter = HtmlMultiThemesBuilder::new()
        .language(Language::PlainText)
        .themes(themes)
        .pre_class(Some("shorthand".to_string()))
        .pre_attrs(pre_attrs())
        .code_attrs(code_attrs())
        .build()
        .unwrap();

    let html = render(&formatter);
    assert!(
        html.starts_with(concat!(
            r#"<pre class="lumis lumis-themes shorthand dark authored" "#,
            r#"style="--lumis-dark:#f8f8f2; --lumis-dark-bg:#282a36; outline: 1px solid red" "#,
            r#"id="sample-pre" inert>"#,
            r#"<code class="language-plaintext copyable" translate="yes" id="sample-code">"#,
        )),
        "{html}"
    );
}

#[test]
fn rendering_fails_on_an_attribute_name_that_would_break_out_of_the_tag() {
    let formatter = HtmlLinkedBuilder::new()
        .language(Language::PlainText)
        .pre_attrs(vec![("x onclick=alert(1)".to_string(), "y".into())])
        .build()
        .unwrap();

    let error = render_result(&formatter).expect_err("the name is rejected");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(error.to_string().contains("invalid HTML attribute name"));
}
