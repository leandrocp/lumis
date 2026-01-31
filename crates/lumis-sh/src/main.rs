mod samples;

use std::collections::HashMap;
use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::{extract::Query, extract::State, routing::get, Router};
use lumis::themes::Appearance;
use serde::Deserialize;
use tower_http::compression::CompressionLayer;

#[derive(Clone)]
pub struct AppState {
    pub languages: Arc<Vec<LangEntry>>,
    pub themes: Arc<Vec<ThemeEntry>>,
    pub samples: Arc<HashMap<&'static str, &'static str>>,
}

#[derive(Clone)]
pub struct LangEntry {
    pub id: String,
    pub name: String,
}

#[derive(Clone)]
pub struct ThemeEntry {
    pub id: String,
    pub name: String,
    pub appearance: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct IndexTemplate {
    languages_json: String,
    themes_json: String,
    selected_lang: String,
    selected_theme: String,
    highlighted_html: String,
}

#[derive(Deserialize)]
struct IndexParams {
    lang: Option<String>,
    theme: Option<String>,
}

async fn index(
    State(state): State<AppState>,
    Query(params): Query<IndexParams>,
) -> IndexTemplate {
    let selected_lang = params
        .lang
        .filter(|l| state.samples.contains_key(l.as_str()))
        .unwrap_or_else(|| "rust".to_string());

    let selected_theme = params
        .theme
        .filter(|t| lumis::themes::get(t).is_ok())
        .unwrap_or_else(|| "dracula".to_string());

    let source = state
        .samples
        .get(selected_lang.as_str())
        .copied()
        .unwrap_or("// No sample available");

    let theme = lumis::themes::get(&selected_theme).expect("theme validated above");
    let language = lumis::languages::Language::guess(Some(&selected_lang), source);

    let formatter = lumis::HtmlInlineBuilder::new()
        .lang(language)
        .theme(Some(theme))
        .pre_class(Some(
            "w-full overflow-auto rounded-lg p-8 font-mono text-sm antialiased leading-6"
                .to_string(),
        ))
        .italic(false)
        .include_highlights(true)
        .build()
        .expect("failed to build formatter");

    let highlighted_html = lumis::highlight(source, formatter);

    let languages_json = format!(
        "[{}]",
        state
            .languages
            .iter()
            .map(|l| format!(
                "{{\"id\":\"{}\",\"name\":\"{}\"}}",
                l.id.replace('"', "\\\""),
                l.name.replace('"', "\\\"")
            ))
            .collect::<Vec<_>>()
            .join(",")
    );

    let themes_json = format!(
        "[{}]",
        state
            .themes
            .iter()
            .map(|t| format!(
                "{{\"id\":\"{}\",\"name\":\"{}\",\"appearance\":\"{}\"}}",
                t.id.replace('"', "\\\""),
                t.name.replace('"', "\\\""),
                t.appearance
            ))
            .collect::<Vec<_>>()
            .join(",")
    );

    IndexTemplate {
        languages_json,
        themes_json,
        selected_lang,
        selected_theme,
        highlighted_html,
    }
}

#[tokio::main]
async fn main() {
    let samples = Arc::new(samples::samples());

    let available = lumis::languages::available_languages();
    let mut languages: Vec<LangEntry> = available
        .into_iter()
        .filter(|(id, _)| samples.contains_key(id.as_str()))
        .map(|(id, (name, _))| LangEntry { id, name })
        .collect();
    languages.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let mut themes: Vec<ThemeEntry> = lumis::themes::available_themes()
        .map(|t| ThemeEntry {
            id: t.name.to_string(),
            name: t.name.to_string(),
            appearance: match t.appearance {
                Appearance::Light => "light".to_string(),
                Appearance::Dark => "dark".to_string(),
            },
        })
        .collect();
    themes.sort_by(|a, b| a.id.cmp(&b.id));

    let state = AppState {
        languages: Arc::new(languages),
        themes: Arc::new(themes),
        samples,
    };

    let app = Router::new()
        .route("/", get(index))
        .layer(CompressionLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
