use autumnus::{highlight, languages::Language, themes, HtmlInlineBuilder};

fn main() {
    let source = r#"fn main() {
    println!("Hello from autumnus!");
}"#;

    let theme = themes::get("dracula").unwrap();
    let formatter = HtmlInlineBuilder::new()
        .lang(Language::Rust)
        .theme(Some(theme))
        .build()
        .unwrap();

    let html = highlight(source, formatter);
    println!("{}", html);
}
