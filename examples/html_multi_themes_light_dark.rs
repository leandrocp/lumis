//! HTML output with light-dark() function for automatic theme switching

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

    // Generate with light-dark() function
    let formatter = HtmlMultiThemesBuilder::new()
        .lang(Language::Rust)
        .themes(themes)
        .default_theme("light-dark()")
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
    <title>Autumnus light-dark() Demo</title>
    <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
    <style>
        html {{
            color-scheme: light dark;
        }}
        body {{
            background: light-dark(#fff, #0d1117);
            color: light-dark(#000, #e6edf3);
        }}
    </style>
</head>
<body class="max-w-4xl mx-auto p-8 transition-colors duration-300">
    <h1 class="text-3xl font-bold mb-6">Autumnus light-dark() Demo</h1>

    <p class="mb-4">
        This demo uses the CSS <code>light-dark()</code> function which automatically
        switches between light and dark themes based on your system preference.
    </p>

    <p class="mb-8 text-sm opacity-75">
        Change your system theme to see the code highlighting update automatically.
    </p>

    {}

</body>
</html>"#,
        highlighted
    );

    std::fs::write("examples/html_multi_themes_light_dark.html", html)?;

    Ok(())
}
