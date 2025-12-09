//! HTML output with multiple theme support

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
    themes.insert("dim".to_string(), themes::get("catppuccin_frappe")?);

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
    <title>Autumnus Multiple Themes - Interactive Demo</title>
    <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
    <style>
        html[data-theme="dark"] {{
            --bg: #0d1117;
            --fg: #e6edf3;
        }}
        html[data-theme="dim"] {{
            --bg: #303446;
            --fg: #c6d0f5;
        }}
        body {{
            background: var(--bg, #fff);
            color: var(--fg, #000);
        }}

        html:not([data-theme]) #btn-system,
        html[data-theme="light"] #btn-light,
        html[data-theme="dark"] #btn-dark,
        html[data-theme="dim"] #btn-dim {{
            background: #111827;
            color: white;
            border-color: #111827;
        }}

        html[data-theme="dark"] .athl-themes,
        html[data-theme="dark"] .athl-themes span {{
            color: var(--athl-dark) !important;
            background-color: var(--athl-dark-bg) !important;
        }}

        html[data-theme="dim"] .athl-themes,
        html[data-theme="dim"] .athl-themes span {{
            color: var(--athl-dim) !important;
            background-color: var(--athl-dim-bg) !important;
        }}

        @media (prefers-color-scheme: dark) {{
            html:not([data-theme]) .athl-themes,
            html:not([data-theme]) .athl-themes span {{
                color: var(--athl-dark) !important;
                background-color: var(--athl-dark-bg) !important;
            }}
        }}
    </style>
</head>
<body class="max-w-4xl mx-auto p-8 transition-colors duration-300">
    <h1 class="text-3xl font-bold mb-6">Autumnus Multiple Themes Demo</h1>

    <div class="flex gap-2 flex-wrap mb-8">
        <button onclick="setTheme('system')" id="btn-system"
                class="px-5 py-2.5 border-2 border-gray-600 bg-white text-gray-900 rounded-lg cursor-pointer text-sm transition-colors hover:bg-gray-100">
            System Preference
        </button>
        <button onclick="setTheme('light')" id="btn-light"
                class="px-5 py-2.5 border-2 border-gray-600 bg-white text-gray-900 rounded-lg cursor-pointer text-sm transition-colors hover:bg-gray-100">
            Light Theme
        </button>
        <button onclick="setTheme('dark')" id="btn-dark"
                class="px-5 py-2.5 border-2 border-gray-600 bg-white text-gray-900 rounded-lg cursor-pointer text-sm transition-colors hover:bg-gray-100">
            Dark Theme
        </button>
        <button onclick="setTheme('dim')" id="btn-dim"
                class="px-5 py-2.5 border-2 border-gray-600 bg-white text-gray-900 rounded-lg cursor-pointer text-sm transition-colors hover:bg-gray-100">
            Dim Theme
        </button>
    </div>

    {}

    <script>
        function setTheme(theme) {{
            const html = document.documentElement;

            if (theme === 'system') {{
                html.removeAttribute('data-theme');
            }} else {{
                html.setAttribute('data-theme', theme);
            }}
        }}
    </script>
</body>
</html>"#,
        highlighted
    );

    std::fs::write("examples/html_multi_themes.html", html)?;

    Ok(())
}
