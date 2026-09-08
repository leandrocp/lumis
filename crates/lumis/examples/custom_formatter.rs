//! Creating a custom formatter
//!
//! This example demonstrates how to implement a custom formatter by implementing
//! the Formatter trait. Here we create a token metadata formatter that explicitly
//! shows what data is available from the unified event stream.

use lumis::{
    events::HighlightEvent, formatters::Formatter, languages::Language, themes, write_highlight,
};
use std::io::{self, Write};

/// A custom formatter that outputs token metadata to show available data
struct TokenMetadataFormatter {
    language: Language,
    theme: Option<lumis::themes::Theme>,
}

impl TokenMetadataFormatter {
    fn new(language: Language, theme: Option<lumis::themes::Theme>) -> Self {
        Self { language, theme }
    }
}

impl Formatter for TokenMetadataFormatter {
    fn language(&self) -> Language {
        self.language
    }

    fn render(
        &self,
        source: &str,
        events: &[HighlightEvent<'_>],
        output: &mut dyn Write,
    ) -> io::Result<()> {
        let mut scopes = Vec::new();

        for event in events {
            match event {
                HighlightEvent::Start {
                    scope_index,
                    language,
                } => scopes.push((*scope_index, language.as_str())),
                HighlightEvent::End => {
                    scopes.pop();
                }
                HighlightEvent::Source { start, end } => {
                    let text = &source[*start..*end];
                    let (scope, language) = scopes.last().map_or(
                        ("", self.language.id_name()),
                        |(scope_index, language)| {
                            (lumis::highlights::HIGHLIGHT_NAMES[*scope_index], *language)
                        },
                    );
                    let specialized_scope = format!("{scope}.{language}");
                    let style = self
                        .theme
                        .as_ref()
                        .and_then(|theme| {
                            theme
                                .get_style(&specialized_scope)
                                .or_else(|| theme.get_style(scope))
                        })
                        .cloned()
                        .unwrap_or_default();

                    writeln!(
                        output,
                        "{} (lang:{} pos:{}..{} scope:{} fg:{} bg:{})",
                        text.escape_debug(),
                        language,
                        start,
                        end,
                        scope,
                        style.fg.as_deref().unwrap_or("none"),
                        style.bg.as_deref().unwrap_or("none"),
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

fn main() {
    let code = r#"const greeting = "Hello, World!";
console.log(greeting);"#;

    let theme = themes::get("dracula").ok();
    let lang = Language::guess(Some("javascript"), code);

    let formatter = TokenMetadataFormatter::new(lang, theme);

    write_highlight(&mut io::stdout(), code, formatter).expect("Failed to write output");
}
