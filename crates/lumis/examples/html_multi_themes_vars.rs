//! HTML output with multiple themes using CSS variables
//!
//! This example demonstrates how to support light/dark themes that switch
//! automatically based on the user's OS preference using CSS variables.
//!
//! The `HtmlMultiThemesBuilder` generates HTML with CSS custom properties
//! (variables) for each theme. A `@media (prefers-color-scheme: dark)` rule
//! then selects which set of variables to apply.

use lumis::{formatter::Formatter, languages::Language, themes, HtmlMultiThemesBuilder};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
fn main() {
    println!("Hello, world!");
}
"#;
    let mut themes = HashMap::new();
    themes.insert("light".to_string(), themes::get("github_light")?);
    themes.insert("dark".to_string(), themes::get("github_dark")?);

    // Generate with CSS variables
    let formatter = HtmlMultiThemesBuilder::new()
        .lang(Language::Rust)
        .themes(themes)
        .default_theme("light")
        .build()
        .map_err(|e| format!("Build error: {}", e))?;

    let mut output = Vec::new();
    formatter.format(source, &mut output)?;
    let highlighted = String::from_utf8(output)?;

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Lumis Multiple Themes Demo</title>
    <style>
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            max-width: 800px;
            margin: 40px auto;
            padding: 0 20px;
            background: light-dark(#fff, #0d1117);
            color: light-dark(#000, #e6edf3);
        }}

        @media (prefers-color-scheme: dark) {{
            .lumis,
            .lumis span {{
                color: var(--lumis-dark) !important;
                background-color: var(--lumis-dark-bg) !important;
                /* Optional, if you also want font styles */
                font-style: var(--lumis-dark-font-style) !important;
                font-weight: var(--lumis-dark-font-weight) !important;
                text-decoration: var(--lumis-dark-text-decoration) !important;
            }}
        }}
    </style>
</head>
<body>
    <h1>Lumis Multiple Themes Demo</h1>
    <p>Change your system theme preference to see the syntax highlighting update automatically.</p>

    {}

</body>
</html>"#,
        highlighted
    );

    std::fs::write("examples/html_multi_themes_vars.html", html)?;

    Ok(())
}
