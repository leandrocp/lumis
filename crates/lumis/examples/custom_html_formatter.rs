//! Custom HTML formatter using public helper functions.
//!
//! This example demonstrates how to build a custom HTML formatter using
//! the public APIs from the `html` module:

use lumis::{
    events::HighlightEvent, formatters::Formatter, html, languages::Language, themes,
    write_highlight,
};
use std::io::{self, Write};

/// A custom HTML formatter built using only public helper functions
struct CustomHtmlFormatter {
    language: Language,
    theme: Option<lumis::themes::Theme>,
}

impl CustomHtmlFormatter {
    fn new(language: Language, theme: Option<lumis::themes::Theme>) -> Self {
        Self { language, theme }
    }
}

impl Formatter for CustomHtmlFormatter {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        html::open_pre_tag(output, None, self.theme.as_ref())?;
        html::open_code_tag(output, &self.language)?;

        for event in events {
            match event {
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => {
                    let scope = lumis::highlights::HIGHLIGHT_NAMES[*scope_index];
                    let language = language.parse().ok();
                    let attrs =
                        html::span_inline_attrs(language, scope, self.theme.as_ref(), false, true);
                    write!(output, "<span {attrs}>")?;
                }
                HighlightEvent::End => output.write_all(b"</span>")?,
                HighlightEvent::Source { start, end } => {
                    write!(output, "{}", html::escape(&source[*start..*end]))?;
                }
                // Anything this formatter does not render, it skips.
                _ => {}
            }
        }

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
