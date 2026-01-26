//! HTML output with manual theme switching via JavaScript
//!
//! This example demonstrates how to implement theme switching with UI buttons.
//! It uses CSS variables from `HtmlMultiThemesBuilder` and JavaScript to toggle
//! between themes by adding/removing a CSS class on the `<html>` element.

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
    <title>Lumis Manual Theme Switching</title>
    <style>
        body {{
            font-family: system-ui, -apple-system, sans-serif;
            max-width: 800px;
            margin: 40px auto;
            padding: 0 20px;
            transition: background-color 0.3s, color 0.3s;
        }}

        html.dark body {{
            background: #0d1117;
            color: #e6edf3;
        }}

        .theme-buttons {{
            margin-bottom: 20px;
            display: flex;
            gap: 10px;
        }}

        .theme-button {{
            padding: 8px 16px;
            border: 2px solid #ccc;
            background: white;
            color: black;
            border-radius: 6px;
            cursor: pointer;
            font-size: 14px;
        }}

        .theme-button:hover {{
            background: #f5f5f5;
        }}

        html.dark .theme-button {{
            background: #21262d;
            color: #e6edf3;
            border-color: #444;
        }}

        html.dark .theme-button:hover {{
            background: #30363d;
        }}

        .theme-button.active {{
            background: #0969da;
            color: white;
            border-color: #0969da;
        }}

        html.dark .theme-button.active {{
            background: #58a6ff;
            color: #0d1117;
            border-color: #58a6ff;
        }}

        html.dark .athl,
        html.dark .athl span {{
            color: var(--athl-dark) !important;
            background-color: var(--athl-dark-bg) !important;
            font-style: var(--athl-dark-font-style) !important;
            font-weight: var(--athl-dark-font-weight) !important;
            text-decoration: var(--athl-dark-text-decoration) !important;
        }}
    </style>
</head>
<body>
    <h1>Lumis Manual Theme Switching</h1>

    <div class="theme-buttons">
        <button onclick="setTheme('light')" id="btn-light" class="theme-button active">
            Light
        </button>
        <button onclick="setTheme('dark')" id="btn-dark" class="theme-button">
            Dark
        </button>
    </div>

    <p>Click the buttons above to manually switch between light and dark themes.</p>

    {}

    <script>
        function setTheme(theme) {{
            const html = document.documentElement;
            const lightBtn = document.getElementById('btn-light');
            const darkBtn = document.getElementById('btn-dark');

            if (theme === 'light') {{
                html.classList.remove('dark');
                lightBtn.classList.add('active');
                darkBtn.classList.remove('active');
            }} else {{
                html.classList.add('dark');
                darkBtn.classList.add('active');
                lightBtn.classList.remove('active');
            }}

            localStorage.setItem('theme', theme);
        }}

        const savedTheme = localStorage.getItem('theme');
        if (savedTheme) {{
            setTheme(savedTheme);
        }}
    </script>
</body>
</html>"#,
        highlighted
    );

    std::fs::write("examples/html_multi_themes_manual.html", html)?;

    Ok(())
}
