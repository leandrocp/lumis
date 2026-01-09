//! Custom HTML formatter using public helper functions.
//!
//! This example demonstrates how to build a custom HTML formatter using
//! the public APIs from the `html` module:

use autumnus::{
    formatter::Formatter, highlight::highlight_iter, html, languages::Language, themes,
    write_highlight,
};
use std::io::{self, Write};

/// A custom HTML formatter built using only public helper functions
struct CustomHtmlFormatter {
    language: Language,
    theme: Option<autumnus::themes::Theme>,
}

impl CustomHtmlFormatter {
    fn new(language: Language, theme: Option<autumnus::themes::Theme>) -> Self {
        Self { language, theme }
    }
}

impl Formatter for CustomHtmlFormatter {
    fn format(&self, source: &str, output: &mut dyn Write) -> io::Result<()> {
        html::open_pre_tag(output, None, self.theme.as_ref())?;
        html::open_code_tag(output, &self.language)?;

        highlight_iter(
            source,
            self.language,
            self.theme.clone(),
            |text, _range, scope, _style| {
                let span = html::span_inline(
                    text,
                    scope,
                    Some(self.language),
                    self.theme.as_ref(),
                    false,
                    true,
                );
                write!(output, "{}", span)
            },
        )
        .map_err(io::Error::other)?;

        html::closing_tags(output)?;
        Ok(())
    }
}

fn main() {
    let code = r#"const greeting = "Hello, World!";
console.log(greeting);"#;

    let theme = themes::get("dracula").ok();
    let lang = Language::guess(Some("javascript"), code);

    let formatter = CustomHtmlFormatter::new(lang, theme);

    write_highlight(&mut io::stdout(), code, formatter).expect("Failed to write output");
}
