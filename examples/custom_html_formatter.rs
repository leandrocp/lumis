//! Custom HTML formatter using public helper functions.
//!
//! This example demonstrates how to create a custom HTML formatter using only
//! the public APIs from the `html` module, without needing to interact with
//! tree-sitter internals directly.

use autumnus::{formatter::Formatter, html, languages::Language, themes, write_highlight, Options};
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
        writeln!(output, "<pre><code>")?;

        for (style, text, _range, scope) in
            html::highlight_iter_with_scopes(source, self.language, self.theme.clone())
                .map_err(io::Error::other)?
        {
            let span = html::span_inline(text, &style, Some(scope));
            write!(output, "{}", span)?;
        }

        writeln!(output, "</code></pre>")?;
        Ok(())
    }
}

fn main() {
    let code = r#"const greeting = "Hello, World!";
console.log(greeting);"#;

    let theme = themes::get("dracula").ok();
    let lang = Language::guess(Some("javascript"), code);

    let formatter = CustomHtmlFormatter::new(lang, theme);

    let options = Options {
        language: Some("javascript"),
        formatter: Box::new(formatter),
    };

    write_highlight(&mut io::stdout(), code, options).expect("Failed to write output");
}
