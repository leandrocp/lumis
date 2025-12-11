//! HTML output with multiple theme support using OS preference

use autumnus::{formatter::Formatter, languages::Language, themes, HtmlMultiThemesBuilder};
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
    <title>Autumnus Multiple Themes Demo</title>
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
            .athl,
            .athl span {{
                color: var(--athl-dark) !important;
                background-color: var(--athl-dark-bg) !important;
                /* Optional, if you also want font styles */
                font-style: var(--athl-dark-font-style) !important;
                font-weight: var(--athl-dark-font-weight) !important;
                text-decoration: var(--athl-dark-text-decoration) !important;
            }}
        }}
    </style>
</head>
<body>
    <h1>Autumnus Multiple Themes Demo</h1>
    <p>Change your system theme preference to see the syntax highlighting update automatically.</p>

    {}

</body>
</html>"#,
        highlighted
    );

    std::fs::write("examples/html_multi_themes.html", html)?;

    Ok(())
}
