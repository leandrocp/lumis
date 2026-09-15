use lumis::{formatters::HtmlInline, languages::Language, themes, HtmlInlineBuilder};

const SOURCE: &str = r#"
fn main() {
    println!("package-size");
}
"#;

fn main() {
    let theme = themes::get("github_dark").expect("built-in github_dark theme");
    let languages = [
        Language::C,
        Language::CSS,
        Language::Go,
        Language::HTML,
        Language::Java,
        Language::JavaScript,
        Language::JSON,
        Language::Python,
        Language::Ruby,
        Language::Rust,
    ];
    let output_bytes = languages
        .into_iter()
        .map(|language| {
            let formatter: HtmlInline = HtmlInlineBuilder::new()
                .language(language)
                .theme(Some(theme.clone()))
                .build()
                .expect("build Lumis formatter");
            let mut output = Vec::new();
            lumis::write_highlight(&mut output, SOURCE, &formatter).expect("highlight with Lumis");
            output.len()
        })
        .sum::<usize>();
    println!("{output_bytes}");
}
