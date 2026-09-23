use anyhow::{bail, ensure, Context, Result};
use clap::{Parser, Subcommand};
use lumis::events::HighlightEvent;
use lumis::highlight::{highlight_events_with_options, HighlightOptions};
use lumis::languages::Language;
use lumis_wasm_runtime::{parser_filename, LanguagePackage, PackagedLanguage, ParserMetadata};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::io::{Read as _, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use xz2::write::XzEncoder;

mod stress;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenCss,
    FilterThemeUpdates {
        #[arg(default_value = "")]
        name: String,
    },
    SyncThemes,
    SyncCss,
    ListThemes,
    GenThemesMd,
    LangsList,
    UpgradeParsers {
        #[arg(default_value = "")]
        name: String,
    },
    FetchParsers {
        #[arg(default_value = "")]
        name: String,
    },
    CompressParsers {
        #[arg(default_value = "")]
        name: String,
    },
    UpgradeQueries {
        #[arg(default_value = "")]
        name: String,
    },
    FetchQueries {
        #[arg(default_value = "")]
        name: String,
    },
    CargoUpdateDep {
        #[arg(default_value = "")]
        name: String,
    },
    CargoUpdateFeatures,
    CheckCrateDeps {
        #[arg(long)]
        fix: bool,
    },
    PreprocessQueries {
        #[arg(default_value = "")]
        name: String,
    },
    GenHighlights,
    GenLanguagesMd,
    GenLanguageCatalog {
        #[arg(long)]
        check: bool,
    },
    BuildWasm {
        #[arg(default_value = "")]
        name: String,
    },
    StageWasm {
        name: String,
        /// The version to stage. Defaults to the next free patch on npm, which
        /// is what a one-off local staging wants; the release pipeline passes
        /// the version its plan resolved, so every job agrees on one.
        #[arg(long)]
        version: Option<String>,
    },
    /// Write the Hex package for a parser, from what `stage-wasm` already built.
    StageHexWasm {
        name: String,
    },
    /// Plan a release: the version each parser resolves to, and which
    /// registries are missing it. Emits JSON for the workflow matrix.
    WasmReleasePlan {
        #[arg(default_value = "")]
        filter: String,
    },
    /// Write the Hex meta-package for a bundle, from the npm bundle beside it.
    StageHexBundle {
        name: String,
    },
    /// Write the committed test parsers as a Lumis store directory.
    StageTestParsers {
        #[arg(default_value = "target/test-parsers")]
        out: String,
    },
    /// List parsers whose published package no longer matches languages.toml.
    WasmNeeded {
        #[arg(default_value = "")]
        filter: String,
        #[arg(default_value = "false")]
        force: String,
    },
    WasmMeta {
        name: String,
    },
    /// List the npm package of every parser `wasm-release.yml` publishes.
    WasmPackages,
    RenderConformance {
        source: String,
        #[arg(short = 'l', long)]
        language: String,
        #[arg(short = 'f', long)]
        formatter: String,
        #[arg(short = 't', long)]
        theme: Option<String>,
        #[arg(long)]
        themes: Vec<String>,
        #[arg(long)]
        default_theme: Option<String>,
        #[arg(long)]
        rainbow_brackets: bool,
    },
    DumpEvents {
        source: String,
        #[arg(short = 'l', long)]
        language: String,
    },
    VerifyConformance {
        #[arg(default_value = "")]
        name: String,
    },
    RegenConformance {
        #[arg(default_value = "")]
        name: String,
    },
    /// Run the generated stress-test corpus through the native Rust API.
    Stress {
        #[arg(long, default_value = "target/stress-test/corpus/manifest.json")]
        manifest: String,
        #[arg(long, default_value = "target/stress-test/rust.json")]
        output: String,
        #[arg(long, default_value_t = 1)]
        iterations: usize,
        #[arg(long, default_value_t = 30_000)]
        max_case_ms: u128,
        #[arg(long, default_value_t = 32.0)]
        max_output_amplification: f64,
        #[arg(long)]
        characterize: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenCss => gen_css(),
        Commands::FilterThemeUpdates { name } => filter_theme_updates(&name),
        Commands::SyncThemes => sync_themes(),
        Commands::SyncCss => sync_css(),
        Commands::ListThemes => list_themes(),
        Commands::GenThemesMd => gen_themes_md(),
        Commands::LangsList => langs_list(),
        Commands::UpgradeParsers { name } => upgrade_parsers(&name),
        Commands::FetchParsers { name } => fetch_parsers(&name),
        Commands::CompressParsers { name } => compress_parsers(&name),
        Commands::UpgradeQueries { name } => upgrade_queries(&name),
        Commands::FetchQueries { name } => fetch_queries(&name),
        Commands::CargoUpdateDep { name } => cargo_update_dep(&name),
        Commands::CargoUpdateFeatures => cargo_update_features(),
        Commands::CheckCrateDeps { fix } => check_crate_deps(fix),
        Commands::PreprocessQueries { name } => preprocess_queries(&name),
        Commands::GenHighlights => gen_highlights(),
        Commands::GenLanguagesMd => gen_languages_md(),
        Commands::GenLanguageCatalog { check } => gen_language_catalog(check),
        Commands::BuildWasm { name } => build_wasm(&name),
        Commands::StageWasm { name, version } => stage_wasm(&name, version.as_deref()),
        Commands::StageHexWasm { name } => stage_hex_wasm(&name),
        Commands::WasmReleasePlan { filter } => wasm_release_plan(&filter),
        Commands::StageHexBundle { name } => stage_hex_bundle(&name),
        Commands::StageTestParsers { out } => stage_test_parsers(Path::new(&out)),
        Commands::WasmNeeded { filter, force } => wasm_needed(&filter, &force),
        Commands::WasmMeta { name } => wasm_meta(&name),
        Commands::WasmPackages => wasm_packages(),
        Commands::RenderConformance {
            source,
            language,
            formatter,
            theme,
            themes,
            default_theme,
            rainbow_brackets,
        } => render_conformance(
            &source,
            &language,
            &formatter,
            theme,
            themes,
            default_theme,
            rainbow_brackets,
        ),
        Commands::DumpEvents { source, language } => dump_events(&source, &language),
        Commands::VerifyConformance { name } => verify_conformance(&name),
        Commands::RegenConformance { name } => regen_conformance(&name),
        Commands::Stress {
            manifest,
            output,
            iterations,
            max_case_ms,
            max_output_amplification,
            characterize,
        } => stress::run(stress::Options {
            manifest: PathBuf::from(manifest),
            output: PathBuf::from(output),
            iterations,
            max_case_ms,
            max_output_amplification,
            characterize,
        }),
    }
}

#[allow(clippy::too_many_arguments)]
fn render_conformance(
    source: &str,
    language: &str,
    formatter: &str,
    theme: Option<String>,
    themes: Vec<String>,
    default_theme: Option<String>,
    rainbow_brackets: bool,
) -> Result<()> {
    let language = parse_language(language)?;
    print!(
        "{}",
        render_formatter_output(
            source,
            language,
            formatter,
            theme,
            themes,
            default_theme,
            rainbow_brackets,
            vec![],
        )?
    );
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
enum SerializableHighlightEvent {
    Start { scope: String, language: String },
    Source { start: usize, end: usize },
    End,
    DecorationStart { decoration: SerializableDecoration },
    DecorationEnd,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "camelCase")]
enum SerializableDecoration {
    RainbowBracket { depth: usize },
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct FixtureMetadata {
    name: String,
    language: String,
    theme: String,
    rainbow_brackets: bool,
    html_multi_themes: Option<HtmlMultiThemesFixture>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct HtmlMultiThemesFixture {
    themes: BTreeMap<String, String>,
    /// Absent means CSS-variables-only mode, which is its own rendering branch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default_theme: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    highlight_lines: Vec<usize>,
}

// `serde`'s `skip_serializing_if` hands the predicate a reference.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct FixtureFile {
    name: String,
    language: String,
    theme: String,
    // Omitted from the JSON when false so non-rainbow fixtures stay unchanged.
    #[serde(default, skip_serializing_if = "is_false")]
    rainbow_brackets: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    html_multi_themes: Option<HtmlMultiThemesFixture>,
    #[serde(default)]
    events: Vec<SerializableHighlightEvent>,
}

struct FixtureOutputs {
    metadata: FixtureMetadata,
    events: Vec<SerializableHighlightEvent>,
    html_inline: String,
    html_linked: String,
    html_multi_themes: String,
    terminal: String,
    bbcode: String,
}

fn serialize_events(events: Vec<HighlightEvent<'_>>) -> Vec<SerializableHighlightEvent> {
    events
        .into_iter()
        .map(|event| match event {
            HighlightEvent::Start {
                scope_index,
                language,
            } => SerializableHighlightEvent::Start {
                scope: lumis::highlights::HIGHLIGHT_NAMES[scope_index].to_string(),
                language,
            },
            HighlightEvent::Source { start, end } => {
                SerializableHighlightEvent::Source { start, end }
            }
            HighlightEvent::End => SerializableHighlightEvent::End,
            HighlightEvent::DecorationStart {
                decoration: lumis::decorations::Decoration::RainbowBracket { depth },
            } => SerializableHighlightEvent::DecorationStart {
                decoration: SerializableDecoration::RainbowBracket { depth },
            },
            HighlightEvent::DecorationEnd => SerializableHighlightEvent::DecorationEnd,
            _ => unreachable!("syntax highlighting emits only scope and source events"),
        })
        .collect()
}

fn parse_language(language: &str) -> Result<Language> {
    language
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid language '{language}'"))
}

fn fixture_root() -> PathBuf {
    PathBuf::from("fixtures/conformance")
}

fn fixture_theme(name: &str) -> Result<lumis::themes::Theme> {
    let path = PathBuf::from("fixtures/conformance-themes").join(format!("{name}.json"));
    if path.is_file() {
        return Ok(lumis::themes::from_file(path)?);
    }
    Ok(lumis::themes::get(name)?)
}

fn selected_fixture_dirs(name: &str) -> Result<Vec<PathBuf>> {
    let root = fixture_root();
    if name.is_empty() {
        let mut dirs = fs::read_dir(&root)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if !entry.file_type().ok()?.is_dir() {
                    return None;
                }
                Some(entry.path())
            })
            .collect::<Vec<_>>();
        dirs.sort();
        return Ok(dirs);
    }

    let dir = root.join(name);
    if !dir.is_dir() {
        bail!("fixture '{name}' not found");
    }
    Ok(vec![dir])
}

// Keep the fixture fields explicit so this extraction cannot introduce a
// second formatter configuration model.
#[allow(clippy::too_many_arguments)]
fn render_html_multi_themes_fixture(
    source: &str,
    language: Language,
    themes: Vec<String>,
    default_theme: Option<String>,
    rainbow_brackets: bool,
    highlight_lines: Vec<usize>,
    output: &mut Vec<u8>,
) -> Result<()> {
    if themes.is_empty() {
        bail!("html-multi-themes requires at least one --themes entry");
    }

    let mut theme_map = std::collections::HashMap::new();
    for spec in themes {
        let (name, theme_id) = spec
            .split_once(':')
            .ok_or_else(|| anyhow::anyhow!("invalid theme spec '{spec}'"))?;
        theme_map.insert(name.to_string(), fixture_theme(theme_id)?);
    }

    let mut builder = lumis::HtmlMultiThemesBuilder::new();
    builder.language(language).themes(theme_map);

    if let Some(default_theme) = default_theme {
        builder.default_theme(default_theme);
    }
    if !highlight_lines.is_empty() {
        builder.highlight_lines(Some(lumis::formatters::html_inline::HighlightLines {
            lines: highlight_lines
                .into_iter()
                .map(|line| line..=line)
                .collect(),
            style: Some(lumis::formatters::html_inline::HighlightLinesStyle::Theme),
            class: None,
        }));
    }

    let formatter = builder.build().map_err(|e| anyhow::anyhow!("{e}"))?;
    lumis::write_highlight_with_options(
        output,
        source,
        formatter,
        lumis::HighlightOptions::new().rainbow_brackets(rainbow_brackets),
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn render_formatter_output(
    source: &str,
    language: Language,
    formatter: &str,
    theme: Option<String>,
    themes: Vec<String>,
    default_theme: Option<String>,
    rainbow_brackets: bool,
    highlight_lines: Vec<usize>,
) -> Result<String> {
    let mut output = Vec::new();

    match formatter {
        "html-inline" => {
            let theme_name = theme.unwrap_or_else(|| "dracula".to_string());
            let theme = fixture_theme(&theme_name)?;
            let formatter = lumis::HtmlInlineBuilder::new()
                .language(language)
                .theme(Some(theme))
                .build()
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            lumis::write_highlight_with_options(
                &mut output,
                source,
                formatter,
                lumis::HighlightOptions::new().rainbow_brackets(rainbow_brackets),
            )?;
        }
        "html-linked" => {
            let formatter = lumis::HtmlLinkedBuilder::new()
                .language(language)
                .build()
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            lumis::write_highlight_with_options(
                &mut output,
                source,
                formatter,
                lumis::HighlightOptions::new().rainbow_brackets(rainbow_brackets),
            )?;
        }
        "html-multi-themes" => {
            render_html_multi_themes_fixture(
                source,
                language,
                themes,
                default_theme,
                rainbow_brackets,
                highlight_lines,
                &mut output,
            )?;
        }
        "terminal" => {
            let theme_name = theme.unwrap_or_else(|| "dracula".to_string());
            let theme = fixture_theme(&theme_name)?;
            let formatter = lumis::TerminalBuilder::new()
                .language(language)
                .theme(Some(theme))
                .build()
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            lumis::write_highlight_with_options(
                &mut output,
                source,
                formatter,
                lumis::HighlightOptions::new().rainbow_brackets(rainbow_brackets),
            )?;
        }
        "bbcode-scoped" => {
            let formatter = lumis::BBCodeScopedBuilder::new()
                .language(language)
                .build()
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            lumis::write_highlight_with_options(
                &mut output,
                source,
                formatter,
                lumis::HighlightOptions::new().rainbow_brackets(rainbow_brackets),
            )?;
        }
        other => bail!("unsupported formatter '{other}'"),
    }

    String::from_utf8(output).map_err(Into::into)
}

fn fixture_outputs(
    source: &str,
    language: Language,
    theme: &str,
    name: &str,
    rainbow_brackets: bool,
    html_multi_themes: Option<HtmlMultiThemesFixture>,
) -> Result<FixtureOutputs> {
    let events = highlight_events_with_options(
        source,
        language,
        HighlightOptions::new().rainbow_brackets(rainbow_brackets),
    )?;
    let (multi_themes, multi_default_theme, multi_highlight_lines) =
        html_multi_themes.as_ref().map_or_else(
            || {
                (
                    vec![format!("main:{theme}")],
                    Some("main".to_string()),
                    vec![],
                )
            },
            |config| {
                (
                    config
                        .themes
                        .iter()
                        .map(|(name, theme)| format!("{name}:{theme}"))
                        .collect(),
                    config.default_theme.clone(),
                    config.highlight_lines.clone(),
                )
            },
        );
    let metadata = FixtureMetadata {
        name: name.to_string(),
        language: language.id_name().to_string(),
        theme: theme.to_string(),
        rainbow_brackets,
        html_multi_themes,
    };

    Ok(FixtureOutputs {
        metadata,
        events: serialize_events(events),
        html_inline: render_formatter_output(
            source,
            language,
            "html-inline",
            Some(theme.to_string()),
            vec![],
            None,
            rainbow_brackets,
            vec![],
        )?,
        html_linked: render_formatter_output(
            source,
            language,
            "html-linked",
            None,
            vec![],
            None,
            rainbow_brackets,
            vec![],
        )?,
        html_multi_themes: render_formatter_output(
            source,
            language,
            "html-multi-themes",
            None,
            multi_themes,
            multi_default_theme,
            rainbow_brackets,
            multi_highlight_lines,
        )?,
        terminal: render_formatter_output(
            source,
            language,
            "terminal",
            Some(theme.to_string()),
            vec![],
            None,
            rainbow_brackets,
            vec![],
        )?,
        bbcode: render_formatter_output(
            source,
            language,
            "bbcode-scoped",
            None,
            vec![],
            None,
            rainbow_brackets,
            vec![],
        )?,
    })
}

fn load_fixture_file(dir: &Path) -> Result<FixtureFile> {
    let json = fs::read_to_string(dir.join("fixture.json"))?;
    Ok(serde_json::from_str(&json)?)
}

fn dump_events(source: &str, language: &str) -> Result<()> {
    let language = parse_language(language)?;
    let events = serialize_events(highlight_events_with_options(
        source,
        language,
        HighlightOptions::default(),
    )?);

    println!("{}", serde_json::to_string_pretty(&events)?);
    Ok(())
}

fn verify_conformance(name: &str) -> Result<()> {
    let mut checked = 0usize;

    for dir in selected_fixture_dirs(name)? {
        let source = fs::read_to_string(dir.join("source.txt"))?;
        let stored = load_fixture_file(&dir)?;
        let language = parse_language(&stored.language)?;
        let generated = fixture_outputs(
            &source,
            language,
            &stored.theme,
            &stored.name,
            stored.rainbow_brackets,
            stored.html_multi_themes.clone(),
        )?;

        ensure_fixture_file_match(
            &dir.join("fixture.json"),
            &stored,
            &FixtureFile {
                name: generated.metadata.name.clone(),
                language: generated.metadata.language.clone(),
                theme: generated.metadata.theme.clone(),
                rainbow_brackets: generated.metadata.rainbow_brackets,
                html_multi_themes: generated.metadata.html_multi_themes.clone(),
                events: generated.events.clone(),
            },
        )?;
        ensure_fixture_match(
            &dir.join("html-inline.html"),
            &fs::read_to_string(dir.join("html-inline.html"))?,
            &generated.html_inline,
        )?;
        ensure_fixture_match(
            &dir.join("html-linked.html"),
            &fs::read_to_string(dir.join("html-linked.html"))?,
            &generated.html_linked,
        )?;
        ensure_fixture_match(
            &dir.join("html-multi-themes.html"),
            &fs::read_to_string(dir.join("html-multi-themes.html"))?,
            &generated.html_multi_themes,
        )?;
        ensure_fixture_match(
            &dir.join("terminal.txt"),
            &fs::read_to_string(dir.join("terminal.txt"))?,
            &generated.terminal,
        )?;
        ensure_fixture_match(
            &dir.join("bbcode.txt"),
            &fs::read_to_string(dir.join("bbcode.txt"))?,
            &generated.bbcode,
        )?;

        checked += 1;
        println!("ok {}", dir.display());
    }

    println!("verified {checked} fixture(s)");
    Ok(())
}

fn regen_conformance(name: &str) -> Result<()> {
    let mut regenerated = 0usize;

    for dir in selected_fixture_dirs(name)? {
        let source = fs::read_to_string(dir.join("source.txt"))?;
        let stored = load_fixture_file(&dir)?;
        let language = parse_language(&stored.language)?;
        let generated = fixture_outputs(
            &source,
            language,
            &stored.theme,
            &stored.name,
            stored.rainbow_brackets,
            stored.html_multi_themes.clone(),
        )?;

        fs::write(
            dir.join("fixture.json"),
            serde_json::to_string_pretty(&FixtureFile {
                name: generated.metadata.name,
                language: generated.metadata.language,
                theme: generated.metadata.theme,
                rainbow_brackets: generated.metadata.rainbow_brackets,
                html_multi_themes: generated.metadata.html_multi_themes,
                events: generated.events,
            })? + "\n",
        )?;
        fs::write(dir.join("html-inline.html"), generated.html_inline)?;
        fs::write(dir.join("html-linked.html"), generated.html_linked)?;
        fs::write(
            dir.join("html-multi-themes.html"),
            generated.html_multi_themes,
        )?;
        fs::write(dir.join("terminal.txt"), generated.terminal)?;
        fs::write(dir.join("bbcode.txt"), generated.bbcode)?;

        regenerated += 1;
        println!("regenerated {}", dir.display());
    }

    println!("regenerated {regenerated} fixture(s)");
    Ok(())
}

fn ensure_fixture_match(path: &Path, expected: &str, actual: &str) -> Result<()> {
    if expected == actual {
        return Ok(());
    }

    bail!("fixture mismatch: {}", path.display())
}

fn ensure_fixture_file_match(
    path: &Path,
    expected: &FixtureFile,
    actual: &FixtureFile,
) -> Result<()> {
    if expected == actual {
        return Ok(());
    }

    bail!("fixture mismatch: {}", path.display())
}

fn gen_css() -> Result<()> {
    let css_dir = Path::new("css");
    fs::create_dir_all(css_dir)?;

    let mut themes: Vec<_> = lumis::themes::ALL_THEMES.iter().collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));

    for theme in themes {
        let css = lumis::themes::CssBuilder::new(theme)
            .enable_italic(true)
            .build();
        let css_path = css_dir.join(format!("{}.css", theme.name));
        fs::write(&css_path, css)?;
        println!("{}", css_path.display());
    }

    Ok(())
}

fn without_revision(mut theme: Value) -> Value {
    if let Value::Object(fields) = &mut theme {
        fields.remove("revision");
    }
    theme
}

fn is_revision_only_theme_update(previous: &Value, current: &Value) -> bool {
    previous != current && without_revision(previous.clone()) == without_revision(current.clone())
}

fn filter_theme_updates(name: &str) -> Result<()> {
    let mut theme_paths = if name.is_empty() {
        glob::glob("themes/*.json")?.collect::<Result<Vec<_>, _>>()?
    } else {
        vec![PathBuf::from(format!("themes/{name}.json"))]
    };
    theme_paths.sort();

    for path in theme_paths {
        if !path.exists() {
            continue;
        }

        let previous = Command::new("git")
            .args(["show", &format!("HEAD:{}", path.display())])
            .output()
            .with_context(|| {
                format!("failed to read the previous version of {}", path.display())
            })?;

        // A new theme has no version at HEAD and should always be kept.
        if !previous.status.success() {
            continue;
        }

        let previous_source = String::from_utf8(previous.stdout)
            .with_context(|| format!("previous theme is not UTF-8: {}", path.display()))?;
        let current_source = fs::read_to_string(&path)
            .with_context(|| format!("failed to read generated theme {}", path.display()))?;
        let previous_theme: Value = serde_json::from_str(&previous_source)
            .with_context(|| format!("invalid previous theme JSON: {}", path.display()))?;
        let current_theme: Value = serde_json::from_str(&current_source)
            .with_context(|| format!("invalid generated theme JSON: {}", path.display()))?;

        if is_revision_only_theme_update(&previous_theme, &current_theme) {
            fs::write(&path, previous_source)?;
            println!(
                "Ignored revision-only theme update: {}",
                path.file_stem().unwrap().to_string_lossy()
            );
        }
    }

    Ok(())
}

fn sync_themes() -> Result<()> {
    let dest = Path::new("crates/lumis-core/themes");
    fs::create_dir_all(dest)?;

    for entry in fs::read_dir(dest)? {
        let entry = entry?;
        if entry.path().extension().is_some_and(|e| e == "json") {
            fs::remove_file(entry.path())?;
        }
    }

    for entry in glob::glob("themes/*.json")? {
        let src = entry?;
        let file_name = src.file_name().unwrap();
        fs::copy(&src, dest.join(file_name))?;
    }

    println!("Synced theme JSON files to crates/lumis-core/themes/");
    Ok(())
}

fn sync_css() -> Result<()> {
    let destinations = [
        "crates/lumis/css",
        "packages/elixir/lumis/priv/static/css",
        "packages/javascript/themes/dist/css",
    ];

    for dest in &destinations {
        let dest = Path::new(dest);
        fs::create_dir_all(dest)?;

        for entry in fs::read_dir(dest)? {
            let entry = entry?;
            if entry.path().extension().is_some_and(|e| e == "css") {
                fs::remove_file(entry.path())?;
            }
        }

        for entry in glob::glob("css/*.css")? {
            let src = entry?;
            let file_name = src.file_name().unwrap();
            fs::copy(&src, dest.join(file_name))?;
        }
    }

    println!(
        "Synced CSS files to crates/lumis/css/, packages/elixir/lumis/priv/static/css/, and packages/javascript/themes/dist/css/"
    );
    Ok(())
}

/// Extract (name, url) pairs from themes/themes.lua using `full_moon` AST parsing.
fn parse_themes_lua() -> Result<Vec<(String, String)>> {
    use full_moon::ast::{Expression, Field, LastStmt};
    use full_moon::tokenizer::TokenType;

    let source = fs::read_to_string("themes/themes.lua")?;
    let ast = full_moon::parse(&source).map_err(|e| anyhow::anyhow!("Lua parse error: {e:?}"))?;

    let mut entries = Vec::new();

    // The file is `return { {name=..., url=...}, ... }`
    // Get the return statement's table constructor
    let Some(LastStmt::Return(ret)) = ast.nodes().last_stmt() else {
        bail!("themes.lua: expected a return statement");
    };
    let Some(first_return) = ret.returns().first() else {
        bail!("themes.lua: expected return to contain a value");
    };
    let Expression::TableConstructor(outer) = first_return.value() else {
        bail!("themes.lua: expected return to contain a table");
    };

    // Each field in the outer table is a theme entry (NoKey variant)
    for field in outer.fields() {
        let Field::NoKey(expr) = field else {
            continue;
        };
        let Expression::TableConstructor(inner) = expr else {
            continue;
        };

        let mut name = None;
        let mut url = None;

        for inner_field in inner.fields() {
            let Field::NameKey { key, value, .. } = inner_field else {
                continue;
            };
            let key_str = key.token().to_string();

            if let Expression::String(token_ref) = value {
                if let TokenType::StringLiteral { literal, .. } = token_ref.token().token_type() {
                    match key_str.as_str() {
                        "name" => name = Some(literal.to_string()),
                        "url" => url = Some(literal.to_string()),
                        _ => {}
                    }
                }
            }
        }

        if let (Some(n), Some(u)) = (name, url) {
            entries.push((n, u));
        }
    }

    Ok(entries)
}

fn list_themes() -> Result<()> {
    let mut entries = parse_themes_lua()?;
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    for (name, _) in &entries {
        println!("{name}");
    }
    Ok(())
}

fn gen_themes_md() -> Result<()> {
    let mut entries = parse_themes_lua()?;
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut table_lines = vec![
        "| Theme | Repository |".to_string(),
        "|-------|------------|".to_string(),
    ];

    for (name, url) in &entries {
        let repo_name = url.strip_prefix("https://github.com/").unwrap_or(url);
        table_lines.push(format!("| `{name}` | [{repo_name}]({url}) |"));
    }

    let mut themes_md = vec![
        "# Supported Themes".to_string(),
        String::new(),
        "> This file is auto-generated by `mise run docs-gen-themes-md`. Do not edit.".to_string(),
        String::new(),
    ];
    themes_md.extend(table_lines.clone());
    themes_md.push(String::new());

    fs::write("THEMES.md", themes_md.join("\n"))?;

    let mut docs_md = vec![
        "---".to_string(),
        "sidebar_position: 2".to_string(),
        "slug: /reference/themes".to_string(),
        "title: Themes".to_string(),
        "description: Built-in Lumis themes and their upstream repositories.".to_string(),
        "keywords:".to_string(),
        "  - lumis themes".to_string(),
        "  - supported themes".to_string(),
        "  - neovim themes".to_string(),
        "---".to_string(),
        String::new(),
        "# Themes".to_string(),
        String::new(),
        "Source: [`THEMES.md`](https://github.com/leandrocp/lumis/blob/main/THEMES.md)."
            .to_string(),
        String::new(),
        "Theme names in this list are the IDs you use across Lumis:".to_string(),
        String::new(),
        "- JavaScript modules: `@lumis-sh/themes/<theme>`".to_string(),
        "- CSS files: `@lumis-sh/themes/css/<theme>.css`".to_string(),
        "- JSON on CDN: `https://unpkg.com/@lumis-sh/themes/dist/json/<theme>.json`".to_string(),
        String::new(),
        "For usage examples, see [Themes](/themes) and [CSS Theme Files](/themes/css-files)."
            .to_string(),
        String::new(),
    ];
    docs_md.extend(table_lines);
    docs_md.push(String::new());

    fs::write("docs/content/reference/themes.md", docs_md.join("\n"))?;
    println!("Generated THEMES.md with {} themes", entries.len());
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct LanguagesToml {
    queries: BTreeMap<String, QueryInfo>,
    parsers: BTreeMap<String, ParserInfo>,
    #[serde(default)]
    bundles: BTreeMap<String, BundleInfo>,
}

#[derive(Debug, serde::Deserialize)]
struct BundleInfo {
    parsers: BundleParsers,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum BundleParsers {
    List(Vec<String>),
    All(String),
}

#[derive(Debug, serde::Deserialize)]
struct QueryInfo {
    git: String,
    rev: String,
    #[allow(dead_code)]
    path: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
struct ParserInfo {
    git: Option<String>,
    rev: Option<String>,
    version: Option<String>,
    #[serde(rename = "crate")]
    crate_field: Option<String>,
    location: Option<String>,
    query_name: Option<String>,
    generate: Option<bool>,
    wasm_name: Option<String>,
    feature: Option<String>,
    #[allow(dead_code)]
    #[serde(default)]
    globs: Vec<String>,
    #[allow(dead_code)]
    #[serde(default)]
    aliases: Vec<String>,
    #[allow(dead_code)]
    variant: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct HighlightsToml {
    scopes: Vec<String>,
}

fn read_languages_toml() -> Result<LanguagesToml> {
    let text = fs::read_to_string("languages.toml")?;
    Ok(toml::from_str(&text)?)
}

fn read_languages_toml_edit() -> Result<toml_edit::DocumentMut> {
    let text = fs::read_to_string("languages.toml")?;
    Ok(text.parse()?)
}

fn write_languages_toml_edit(doc: &toml_edit::DocumentMut) -> Result<()> {
    fs::write("languages.toml", doc.to_string())?;
    Ok(())
}

fn read_lumis_cargo_toml_edit() -> Result<toml_edit::DocumentMut> {
    let text = fs::read_to_string("crates/lumis/Cargo.toml")?;
    Ok(text.parse()?)
}

fn write_lumis_cargo_toml_edit(doc: &toml_edit::DocumentMut) -> Result<()> {
    fs::write("crates/lumis/Cargo.toml", doc.to_string())?;
    Ok(())
}

fn read_lumis_core_cargo_toml_edit() -> Result<toml_edit::DocumentMut> {
    let text = fs::read_to_string("crates/lumis-core/Cargo.toml")?;
    Ok(text.parse()?)
}

fn write_lumis_core_cargo_toml_edit(doc: &toml_edit::DocumentMut) -> Result<()> {
    fs::write("crates/lumis-core/Cargo.toml", doc.to_string())?;
    Ok(())
}

fn parser_feature_name(parser_name: &str, info: &ParserInfo) -> String {
    info.feature
        .clone()
        .unwrap_or_else(|| format!("lang-{}", parser_name.replace('_', "-")))
}

fn sync_bundle_features(
    toml: &LanguagesToml,
    lumis: &mut toml_edit::DocumentMut,
    lumis_core: &mut toml_edit::DocumentMut,
) -> Result<()> {
    let lumis_features = lumis["features"]
        .as_table_like_mut()
        .context("missing [features] in crates/lumis/Cargo.toml")?;
    let lumis_core_features = lumis_core["features"]
        .as_table_like_mut()
        .context("missing [features] in crates/lumis-core/Cargo.toml")?;

    for (bundle_name, bundle) in &toml.bundles {
        let bundle_feature = format!("lang-bundle-{bundle_name}");
        let parser_features = match &bundle.parsers {
            BundleParsers::List(parsers) => parsers
                .iter()
                .map(|parser_name| {
                    let info = toml.parsers.get(parser_name).with_context(|| {
                        format!("bundle '{bundle_name}' references unknown parser '{parser_name}'")
                    })?;
                    Ok(parser_feature_name(parser_name, info))
                })
                .collect::<Result<Vec<_>>>()?,
            BundleParsers::All(value) => {
                if value != "all" {
                    bail!("unsupported bundle parsers value for '{bundle_name}': {value}");
                }
                vec!["all-languages".to_string()]
            }
        };

        let lumis_core_array = toml_edit::Array::from_iter(parser_features.iter().cloned());
        let old_core_key_decor = lumis_core_features
            .key(&bundle_feature)
            .map(|k| k.leaf_decor().clone());
        lumis_core_features.insert(&bundle_feature, toml_edit::value(lumis_core_array));
        if let Some(decor) = old_core_key_decor {
            if let Some(mut key) = lumis_core_features.key_mut(&bundle_feature) {
                *key.leaf_decor_mut() = decor;
            }
        }

        let mut lumis_feature_values = parser_features;
        lumis_feature_values.push(format!("lumis-core/{bundle_feature}"));
        let lumis_array = toml_edit::Array::from_iter(lumis_feature_values);
        let old_lumis_key_decor = lumis_features
            .key(&bundle_feature)
            .map(|k| k.leaf_decor().clone());
        lumis_features.insert(&bundle_feature, toml_edit::value(lumis_array));
        if let Some(decor) = old_lumis_key_decor {
            if let Some(mut key) = lumis_features.key_mut(&bundle_feature) {
                *key.leaf_decor_mut() = decor;
            }
        }

        println!("  synced {bundle_feature}");
    }

    Ok(())
}

fn run_cmd(cmd: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .with_context(|| format!("failed to run: {cmd}"))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn run_cmd_ok(cmd: &str) -> Result<()> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .status()
        .with_context(|| format!("failed to run: {cmd}"))?;
    if !status.success() {
        bail!("command failed: {cmd}");
    }
    Ok(())
}

fn git_ls_remote(url: &str) -> Result<String> {
    run_cmd(&format!("git ls-remote {url} HEAD | cut -f1"))
}

fn is_full_git_sha(rev: &str) -> bool {
    rev.len() == 40 && rev.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn git_resolve_rev(url: &str, rev: &str) -> Result<String> {
    if is_full_git_sha(rev) {
        return Ok(rev.to_string());
    }

    let output = Command::new("git")
        .args(["ls-remote", url, rev, &format!("{rev}^{{}}")])
        .output()
        .with_context(|| format!("failed to resolve git revision {rev} from {url}"))?;
    if !output.status.success() {
        bail!("failed to resolve git revision {rev} from {url}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .find(|line| line.ends_with("^{}"))
        .or_else(|| stdout.lines().next())
        .and_then(|line| line.split_whitespace().next())
        .map(str::to_string)
        .filter(|resolved| !resolved.is_empty())
        .with_context(|| format!("could not resolve git revision {rev} from {url}"))
}

fn release_version_from_tag(tag: &str) -> Option<semver::Version> {
    let version = lenient_semver::parse(tag).ok()?;
    version.pre.is_empty().then_some(version)
}

fn git_release_tags(url: &str) -> Result<BTreeMap<semver::Version, String>> {
    let output = Command::new("git")
        .args(["ls-remote", "--tags", url])
        .output()
        .with_context(|| format!("failed to list git tags from {url}"))?;
    if !output.status.success() {
        bail!("failed to list git tags from {url}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut tags = BTreeMap::<semver::Version, String>::new();
    for line in stdout.lines() {
        let Some((_, reference)) = line.split_once('\t') else {
            continue;
        };
        let Some(tag) = reference.strip_prefix("refs/tags/") else {
            continue;
        };
        if tag.ends_with("^{}") {
            continue;
        }
        let Some(version) = release_version_from_tag(tag) else {
            continue;
        };
        tags.insert(version, tag.to_string());
    }

    Ok(tags)
}

fn git_resolve_version_tag(url: &str, version: &str) -> Option<String> {
    let version = semver::Version::parse(version).ok()?;
    let tag = git_release_tags(url).ok()?.remove(&version)?;
    git_resolve_rev(url, &tag).ok()
}

fn git_latest_release_rev(url: &str) -> Result<Option<(String, String)>> {
    let tags = git_release_tags(url)?;
    let Some((version, tag)) = tags.into_iter().next_back() else {
        return Ok(None);
    };
    let rev = git_resolve_rev(url, &tag)?;
    Ok(Some((version.to_string(), rev)))
}

fn git_is_ancestor(url: &str, ancestor: &str, descendant: &str) -> Result<bool> {
    if ancestor == descendant {
        return Ok(true);
    }

    let repo = tmpdir()?;
    let result = (|| {
        let init = Command::new("git")
            .args(["init", "--bare", &repo])
            .output()
            .with_context(|| format!("failed to initialize temporary git repository at {repo}"))?;
        if !init.status.success() {
            bail!("failed to initialize temporary git repository at {repo}");
        }

        let ancestor_ref = "refs/lumis/ancestor";
        let descendant_ref = "refs/lumis/descendant";
        let fetch = Command::new("git")
            .args([
                "-C",
                &repo,
                "fetch",
                "--quiet",
                "--filter=blob:none",
                "--no-tags",
                url,
                &format!("{ancestor}:{ancestor_ref}"),
                &format!("{descendant}:{descendant_ref}"),
            ])
            .output()
            .with_context(|| {
                format!("failed to fetch parser revisions {ancestor} and {descendant} from {url}")
            })?;
        if !fetch.status.success() {
            bail!(
                "failed to fetch parser revisions {ancestor} and {descendant} from {url}: {}",
                String::from_utf8_lossy(&fetch.stderr).trim()
            );
        }

        let status = Command::new("git")
            .args([
                "-C",
                &repo,
                "merge-base",
                "--is-ancestor",
                ancestor_ref,
                descendant_ref,
            ])
            .status()
            .with_context(|| {
                format!("failed to compare parser revisions {ancestor} and {descendant} from {url}")
            })?;

        match status.code() {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => bail!(
                "git merge-base failed while comparing parser revisions {ancestor} and {descendant} from {url}"
            ),
        }
    })();

    let _ = fs::remove_dir_all(&repo);
    result
}

fn latest_crate_version(crate_name: &str) -> Result<Option<String>> {
    let url = format!("https://crates.io/api/v1/crates/{crate_name}");
    let Ok(mut resp) = ureq::get(&url).call() else {
        return Ok(None);
    };
    let body = resp.body_mut().read_to_string()?;
    let value: Value = serde_json::from_str(&body)?;
    let Some(versions) = value
        .get("versions")
        .and_then(|versions| versions.as_array())
    else {
        return Ok(None);
    };

    let mut latest = None;
    for version in versions {
        if version
            .get("yanked")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(true)
        {
            continue;
        }
        let Some(num) = version.get("num").and_then(|num| num.as_str()) else {
            continue;
        };
        let Ok(parsed) = semver::Version::parse(num) else {
            continue;
        };
        if !parsed.pre.is_empty() {
            continue;
        }
        latest = Some(latest.map_or(parsed.clone(), |current: semver::Version| {
            current.max(parsed)
        }));
    }

    Ok(latest.map(|version| version.to_string()))
}

fn tmpdir() -> Result<String> {
    run_cmd("mktemp -d")
}

fn query_names() -> Result<Vec<String>> {
    let mut names = std::collections::BTreeSet::new();

    for dir in [
        "queries/upstream",
        "queries/brackets",
        "queries/override",
        "queries/append",
    ] {
        let path = Path::new(dir);
        if !path.exists() {
            continue;
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name != "README.md" && name != "default" && entry.file_type()?.is_dir() {
                names.insert(name);
            }
        }
    }

    Ok(names.into_iter().collect())
}

fn resolve_query_source<'a>(
    queries: &'a BTreeMap<String, QueryInfo>,
    query_name: &str,
) -> &'a QueryInfo {
    if query_name != "default" {
        if let Some(info) = queries.get(query_name) {
            return info;
        }
    }
    &queries["default"]
}

fn has_local_override_query(lang: &str) -> bool {
    ["highlights", "injections", "locals"]
        .iter()
        .any(|query_type| {
            Path::new("queries/override")
                .join(lang)
                .join(format!("{query_type}.scm"))
                .exists()
        })
}

fn query_source_name<'a>(parser_name: &'a str, info: &'a ParserInfo) -> &'a str {
    info.query_name.as_deref().unwrap_or(parser_name)
}

fn langs_list() -> Result<()> {
    let toml = read_languages_toml()?;
    for name in toml.parsers.keys() {
        println!("{name}");
    }
    Ok(())
}

fn parser_upgrade_candidate(
    info: &ParserInfo,
    git: &str,
    current_version: &str,
) -> Result<(String, String)> {
    if let Some(crate_name) = info.crate_field.as_deref() {
        let version =
            latest_crate_version(crate_name)?.unwrap_or_else(|| current_version.to_string());
        let revision = git_resolve_version_tag(git, &version)
            .or_else(|| {
                git_latest_release_rev(git)
                    .ok()
                    .flatten()
                    .map(|(_, revision)| revision)
            })
            .unwrap_or_else(|| git_ls_remote(git).unwrap_or_default());
        return Ok((version, revision));
    }

    if let Some((version, revision)) = git_latest_release_rev(git)? {
        return Ok((version, revision));
    }
    Ok((current_version.to_string(), git_ls_remote(git)?))
}

fn upgrade_parsers(name: &str) -> Result<()> {
    let mut doc = read_languages_toml_edit()?;
    let toml = read_languages_toml()?;

    for (parser_name, info) in &toml.parsers {
        let Some(ref git) = info.git else { continue };
        if !name.is_empty() && parser_name != name {
            continue;
        }

        let current_ver = info.version.as_deref().unwrap_or("");
        let current_rev = info.rev.as_deref().unwrap_or("");
        let (ver, new_rev) = parser_upgrade_candidate(info, git, current_ver)?;

        if new_rev.is_empty() {
            println!("Warning: could not resolve revision for {parser_name}, skipping");
            continue;
        }

        // Forks inherit upstream tags. Never replace a recorded fork revision
        // with an older tagged commit that it already contains.
        if !current_rev.is_empty()
            && current_rev != new_rev
            && git_is_ancestor(git, &new_rev, current_rev)?
        {
            println!("  {parser_name}: keeping {current_rev}; candidate {new_rev} is an ancestor");
            continue;
        }

        if current_rev == new_rev && ver == current_ver {
            println!("  {parser_name}: already up to date ({current_rev}, {current_ver})");
        } else {
            if current_rev != new_rev {
                println!("  {parser_name}: {current_rev} -> {new_rev}");
                doc["parsers"][parser_name.as_str()]["rev"] = toml_edit::value(&new_rev);
            }
            if ver != current_ver {
                println!("    version: {current_ver} -> {ver}");
                doc["parsers"][parser_name.as_str()]["version"] = toml_edit::value(&ver);
            }
        }
    }

    write_languages_toml_edit(&doc)?;
    Ok(())
}

fn fetch_parser(tmp: &str, parser_name: &str, info: &ParserInfo, git: &str) -> Result<()> {
    let rev = info.rev.as_deref().unwrap_or("HEAD");
    let parser_dir = vendored_parser_dir_name(parser_name, info);
    let clone_dir = format!("{tmp}/{parser_dir}");

    println!("Fetching {parser_name} from {git} at {rev}");
    run_cmd_ok(&format!("git clone {git} {clone_dir} 2>/dev/null"))?;
    run_cmd_ok(&format!("cd {clone_dir} && git checkout {rev} 2>/dev/null"))?;

    let dest = format!("crates/lumis/vendored_parsers/{parser_dir}");
    if info.generate.unwrap_or(false) {
        run_cmd_ok(&format!("cd {clone_dir} && tree-sitter generate"))?;
        let src = format!("{clone_dir}/src");
        if !Path::new(&src).is_dir() {
            bail!("no generated src directory found for {parser_name} at {src}");
        }

        let _ = run_cmd_ok(&format!("rm -rf {dest}"));
        fs::create_dir_all(&dest)?;
        run_cmd_ok(&format!("cp -r {src} {dest}/"))?;
        println!("  Updated {parser_name} (generated)");
    } else if let Some(ref location) = info.location {
        let src = format!("{clone_dir}/{location}/src");
        if !Path::new(&src).is_dir() {
            bail!("no src directory found for {parser_name} in location {location}");
        }

        let _ = run_cmd_ok(&format!("rm -rf {dest}"));
        let location_dest = format!("{dest}/{location}");
        fs::create_dir_all(&location_dest)?;
        run_cmd_ok(&format!("cp -r {src} {location_dest}/"))?;

        // Multi-grammar repositories can share support files from a root-level
        // common directory (for example XML). Preserve the complete directory:
        // scanners may include or otherwise depend on non-header files too.
        let common = format!("{clone_dir}/common");
        if Path::new(&common).is_dir() {
            run_cmd_ok(&format!("cp -r {common} {dest}/"))?;
        }
        println!("  Updated {parser_name} (location: {location})");
    } else {
        let src = format!("{clone_dir}/src");
        if !Path::new(&src).is_dir() {
            bail!("no src directory found for {parser_name} at {src}");
        }

        fs::create_dir_all(&dest)?;
        let _ = run_cmd_ok(&format!("rm -rf {dest}/src"));
        run_cmd_ok(&format!("cp -r {src} {dest}/"))?;
        println!("  Updated {parser_name}");
    }

    let _ = run_cmd_ok(&format!("rm -rf {clone_dir}"));
    Ok(())
}

fn fetch_parsers(name: &str) -> Result<()> {
    let toml = read_languages_toml()?;
    let tmp = tmpdir()?;

    for (parser_name, info) in &toml.parsers {
        if let Some(crate_name) = &info.crate_field {
            if name.is_empty() || parser_name == name {
                println!("Skipping {parser_name}: parser is provided by crate {crate_name}");
            }
            continue;
        }

        let Some(ref git) = info.git else { continue };
        if !name.is_empty() && parser_name != name {
            continue;
        }
        fetch_parser(&tmp, parser_name, info, git)?;
    }

    let _ = run_cmd_ok(&format!("rm -rf {tmp}"));
    Ok(())
}

fn vendored_parser_dir_name(parser_name: &str, info: &ParserInfo) -> String {
    if let Some(query_name) = &info.query_name {
        return format!("tree-sitter-{query_name}");
    }

    format!("tree-sitter-{parser_name}")
}

fn compress_source_file(parser_name: &str, src_dir: &Path, file_name: &str) -> Result<()> {
    let source = src_dir.join(file_name);
    let compressed = src_dir.join(format!("{file_name}.xz"));
    let bytes = fs::read(&source)
        .with_context(|| format!("failed to read parser source at {}", source.display()))?;
    let output = fs::File::create(&compressed).with_context(|| {
        format!(
            "failed to create compressed parser source at {}",
            compressed.display()
        )
    })?;
    let mut encoder = XzEncoder::new(output, 9);
    encoder.write_all(&bytes).with_context(|| {
        format!(
            "failed to write compressed parser source at {}",
            compressed.display()
        )
    })?;
    encoder.finish().with_context(|| {
        format!(
            "failed to finalize compressed parser source at {}",
            compressed.display()
        )
    })?;
    fs::remove_file(&source).with_context(|| {
        format!(
            "failed to remove uncompressed parser source at {}",
            source.display()
        )
    })?;
    println!("  Compressed {parser_name} {file_name} -> {file_name}.xz");

    Ok(())
}

fn compress_parsers(name: &str) -> Result<()> {
    let toml = read_languages_toml()?;

    for (parser_name, info) in &toml.parsers {
        if info.git.is_none() || info.crate_field.is_some() {
            continue;
        }
        if !name.is_empty() && parser_name != name {
            continue;
        }

        let parser_dir = format!(
            "crates/lumis/vendored_parsers/{}",
            vendored_parser_dir_name(parser_name, info)
        );
        if !Path::new(&parser_dir).exists() {
            continue;
        }

        let src_dir = if let Some(location) = &info.location {
            let nested = Path::new(&parser_dir).join(location).join("src");
            if nested.join("parser.c").exists() || nested.join("parser.c.xz").exists() {
                nested
            } else {
                Path::new(&parser_dir).join("src")
            }
        } else {
            Path::new(&parser_dir).join("src")
        };

        if !src_dir.exists() {
            println!(
                "  Warning: no src directory found for {parser_name} at {}",
                src_dir.display()
            );
            continue;
        }

        if !src_dir.join("parser.c").exists() && !src_dir.join("parser.c.xz").exists() {
            println!(
                "  Warning: no parser source found for {parser_name} at {}",
                src_dir.display()
            );
            continue;
        }

        for file_name in ["parser.c", "scanner.c", "scanner.cc"] {
            if src_dir.join(file_name).exists() {
                compress_source_file(parser_name, &src_dir, file_name)?;
            }
        }
    }

    Ok(())
}

/// Query sources a run named `name` may rewrite.
///
/// An empty name upgrades every source. A named one upgrades only its own
/// entry: `queries.default` backs most of the corpus, so including it here
/// would make a per-language run rewrite a revision every other language
/// shares.
fn queries_to_upgrade<'a>(
    queries: &'a BTreeMap<String, QueryInfo>,
    name: &str,
) -> Vec<(&'a String, &'a QueryInfo)> {
    queries
        .iter()
        .filter(|(query_name, _)| name.is_empty() || query_name.as_str() == name)
        .collect()
}

fn upgrade_queries(name: &str) -> Result<()> {
    let mut doc = read_languages_toml_edit()?;
    let toml = read_languages_toml()?;

    let mut url_revs: BTreeMap<String, String> = BTreeMap::new();
    for (query_name, info) in queries_to_upgrade(&toml.queries, name) {
        let new_rev = if let Some(rev) = url_revs.get(&info.git) {
            rev.clone()
        } else {
            let rev = git_ls_remote(&info.git)?;
            println!("  {} -> {}", info.git, &rev[..12.min(rev.len())]);
            url_revs.insert(info.git.clone(), rev.clone());
            rev
        };

        if info.rev != new_rev {
            println!(
                "  {query_name}: {} -> {}",
                &info.rev[..12.min(info.rev.len())],
                &new_rev[..12.min(new_rev.len())]
            );
            doc["queries"][query_name.as_str()]["rev"] = toml_edit::value(new_rev.as_str());
        }
    }

    write_languages_toml_edit(&doc)?;
    println!("Done. Review changes with: git diff languages.toml");
    Ok(())
}

fn cargo_update_dep(name: &str) -> Result<()> {
    let toml = read_languages_toml()?;
    let mut cargo = read_lumis_cargo_toml_edit()?;
    let deps = cargo["dependencies"]
        .as_table_like_mut()
        .context("missing [dependencies] in crates/lumis/Cargo.toml")?;

    let mut crate_versions = BTreeMap::<String, String>::new();

    for (parser_name, info) in &toml.parsers {
        let Some(crate_name) = info.crate_field.as_ref() else {
            continue;
        };
        let Some(version) = info.version.as_ref() else {
            continue;
        };
        if !name.is_empty() && parser_name != name {
            continue;
        }

        if let Some(existing) = crate_versions.get(crate_name) {
            if existing != version {
                bail!("conflicting versions for crate {crate_name}: {existing} vs {version}");
            }
        } else {
            crate_versions.insert(crate_name.clone(), version.clone());
        }
    }

    for (crate_name, version) in crate_versions {
        let Some(dep_item) = deps.get_mut(&crate_name) else {
            bail!("missing dependency {crate_name} in crates/lumis/Cargo.toml");
        };

        match dep_item {
            toml_edit::Item::Value(toml_edit::Value::String(_)) => {
                *dep_item = toml_edit::value(version.clone());
            }
            toml_edit::Item::Value(toml_edit::Value::InlineTable(table)) => {
                table.insert("version", toml_edit::Value::from(version.clone()));
            }
            toml_edit::Item::Table(table) => {
                table["version"] = toml_edit::value(version.clone());
            }
            _ => bail!("unsupported dependency format for {crate_name}"),
        }

        println!("  synced {crate_name} -> {version}");
    }

    write_lumis_cargo_toml_edit(&cargo)
}

fn cargo_update_features() -> Result<()> {
    let toml = read_languages_toml()?;
    let mut cargo = read_lumis_cargo_toml_edit()?;
    let mut cargo_core = read_lumis_core_cargo_toml_edit()?;

    sync_bundle_features(&toml, &mut cargo, &mut cargo_core)?;

    write_lumis_cargo_toml_edit(&cargo)?;
    write_lumis_core_cargo_toml_edit(&cargo_core)
}

/// Crates published from this repository that something else here depends on.
/// Named rather than counted, so a crate that disappears from the workspace
/// fails here instead of silently shrinking what gets checked.
const RELEASED_CRATES: &[&str] = &[
    "lumis",
    "lumis-build",
    "lumis-cli",
    "lumis-core",
    "lumis-wasm-runtime",
];

/// Requirements deliberately left behind the workspace, and why. A waiver that
/// stops being needed has to fail too, so this list can only shrink.
const CRATE_DEP_WAIVERS: &[(&str, &str, &str)] = &[(
    "crates/autumnus/Cargo.toml",
    "lumis",
    "deprecated rename shim, frozen at the lumis it was published against",
)];

const DEPENDENCY_TABLES: &[&str] = &["dependencies", "dev-dependencies", "build-dependencies"];

const MIN_CHECKED_CRATE_DEPS: usize = 10;

struct CrateDependency {
    manifest: String,
    name: String,
    requirement: Option<String>,
    publishable: bool,
}

fn repository_root() -> Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to locate the repository root")?;
    if !output.status.success() {
        bail!("git rev-parse --show-toplevel failed; not inside a git repository");
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}

/// Paths come back relative to the repository root rather than the working
/// directory, so `CRATE_DEP_WAIVERS` matches wherever this is run from.
fn tracked_cargo_manifests(root: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["ls-files", "--full-name", "-z", "*Cargo.toml"])
        .output()
        .context("failed to list tracked Cargo manifests")?;
    if !output.status.success() {
        bail!("git ls-files failed while listing Cargo manifests");
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_string)
        .collect())
}

/// Visits every dependency entry in a manifest, including the ones nested under
/// `[target.'cfg(...)'.dependencies]`, with the name of the crate it resolves to.
fn for_each_dependency(
    table: &mut toml_edit::Table,
    visit: &mut impl FnMut(&str, &mut toml_edit::Item),
) {
    for (key, item) in table.iter_mut() {
        if DEPENDENCY_TABLES.contains(&key.get()) {
            let Some(dependencies) = item.as_table_like_mut() else {
                continue;
            };
            for (dep_key, spec) in dependencies.iter_mut() {
                // `package = "..."` renames the dependency, so the key is not
                // necessarily the crate being depended on.
                let name = spec
                    .get("package")
                    .and_then(|package| package.as_str())
                    .unwrap_or(dep_key.get())
                    .to_string();
                visit(&name, spec);
            }
        } else if let Some(nested) = item.as_table_mut() {
            for_each_dependency(nested, visit);
        }
    }
}

fn requirement_of(spec: &toml_edit::Item) -> Option<String> {
    match spec {
        toml_edit::Item::Value(toml_edit::Value::String(requirement)) => {
            Some(requirement.value().clone())
        }
        _ => spec
            .get("version")
            .and_then(|version| version.as_str())
            .map(str::to_string),
    }
}

/// `{ workspace = true }` carries no requirement of its own, and cargo rejects a
/// `version` beside it. The `[workspace.dependencies]` entry it inherits from is
/// itself visited, so the requirement is still checked once.
fn inherits_from_workspace(spec: &toml_edit::Item) -> bool {
    spec.get("workspace")
        .and_then(toml_edit::Item::as_bool)
        .unwrap_or(false)
}

struct Manifest {
    path: String,
    file: PathBuf,
    document: toml_edit::DocumentMut,
    publishable: bool,
}

fn is_publishable(document: &toml_edit::DocumentMut) -> bool {
    // A virtual manifest has no package to publish, so a requirement in one can
    // never reach a registry.
    let Some(package) = document.get("package") else {
        return false;
    };
    let Some(publish) = package.get("publish") else {
        return true;
    };
    if let Some(allowed) = publish.as_bool() {
        return allowed;
    }
    // cargo: "`package.publish` must be set to `true` or a non-empty list in
    // Cargo.toml to publish", so `publish = []` is another spelling of `false`.
    publish
        .as_array()
        .is_none_or(|registries| !registries.is_empty())
}

fn read_manifests() -> Result<Vec<Manifest>> {
    let root = repository_root()?;
    tracked_cargo_manifests(&root)?
        .into_iter()
        .map(|path| {
            let file = root.join(&path);
            let text =
                fs::read_to_string(&file).with_context(|| format!("failed to read {path}"))?;
            let document: toml_edit::DocumentMut = text
                .parse()
                .with_context(|| format!("failed to parse {path}"))?;
            let publishable = is_publishable(&document);
            Ok(Manifest {
                path,
                file,
                document,
                publishable,
            })
        })
        .collect()
}

/// Every build in this repository resolves the lumis crates through `path`
/// entries and the root `[patch.crates-io]`, so the `version` requirement
/// beside them is dead weight locally and only takes effect once the crate is
/// published. Nothing here can notice it going stale, which is how lumis 0.12.1
/// shipped requiring `lumis-core = "2"` while calling an API added in 2.2.0
/// (#1118): consumers resolving from an existing Cargo.lock got 2.0.0 and
/// failed to compile.
fn check_crate_deps(fix: bool) -> Result<()> {
    let mut manifests = read_manifests()?;

    let versions: BTreeMap<String, String> = manifests
        .iter()
        .filter_map(|manifest| {
            let package = manifest.document.get("package")?;
            Some((
                package.get("name")?.as_str()?.to_string(),
                package.get("version")?.as_str()?.to_string(),
            ))
        })
        .collect();

    if fix {
        return fix_crate_deps(&mut manifests, &versions);
    }

    let mut dependencies = Vec::new();
    for manifest in &mut manifests {
        let path = manifest.path.clone();
        let publishable = manifest.publishable;
        for_each_dependency(manifest.document.as_table_mut(), &mut |name, spec| {
            if inherits_from_workspace(spec) {
                return;
            }
            dependencies.push(CrateDependency {
                manifest: path.clone(),
                name: name.to_string(),
                requirement: requirement_of(spec),
                publishable,
            });
        });
    }

    let (problems, checked) = crate_dep_problems(&versions, &dependencies, manifests.len());

    if !problems.is_empty() {
        for problem in &problems {
            eprintln!("  {problem}");
        }
        bail!(
            "version requirements on lumis crates must equal the version in this repository.\n\
             Run `mise run check-crate-deps --fix` to update them."
        );
    }

    println!("{checked} lumis dependencies match the workspace versions");
    Ok(())
}

fn is_waived(manifest: &str, name: &str) -> bool {
    CRATE_DEP_WAIVERS
        .iter()
        .any(|(waived_manifest, waived_name, _)| {
            *waived_manifest == manifest && *waived_name == name
        })
}

/// `cargo set-version` rewrites dependents it can reach through `path`. The
/// Elixir NIF depends on `lumis-core` through the registry, so it is invisible
/// to that pass and drifts on every release unless something else moves it.
fn fix_manifest(
    path: &str,
    publishable: bool,
    document: &mut toml_edit::DocumentMut,
    versions: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut changes = Vec::new();

    for_each_dependency(document.as_table_mut(), &mut |name, spec| {
        let Some(version) = versions.get(name) else {
            return;
        };
        if is_waived(path, name) || inherits_from_workspace(spec) {
            return;
        }
        let previous = requirement_of(spec);
        // A path dependency in a crate that is never published has nothing to
        // resolve a requirement against, so do not invent one.
        if previous.is_none() && !publishable {
            return;
        }
        if previous.as_deref() == Some(version.as_str()) {
            return;
        }

        changes.push(format!(
            "{path}: {name} {} -> {version}",
            previous.as_deref().unwrap_or("no version")
        ));

        // Only the requirement moves. Replacing the whole entry would drop the
        // `path`, `features` and `default-features` beside it.
        match spec {
            toml_edit::Item::Value(toml_edit::Value::InlineTable(entry)) => {
                entry.insert("version", toml_edit::Value::from(version.clone()));
                entry.fmt();
            }
            toml_edit::Item::Table(entry) => {
                entry["version"] = toml_edit::value(version.clone());
            }
            _ => *spec = toml_edit::value(version.clone()),
        }
    });

    changes
}

fn fix_crate_deps(manifests: &mut [Manifest], versions: &BTreeMap<String, String>) -> Result<()> {
    let mut changes = Vec::new();

    for manifest in manifests {
        let manifest_changes = fix_manifest(
            &manifest.path,
            manifest.publishable,
            &mut manifest.document,
            versions,
        );
        if !manifest_changes.is_empty() {
            fs::write(&manifest.file, manifest.document.to_string())
                .with_context(|| format!("failed to write {}", manifest.path))?;
            changes.extend(manifest_changes);
        }
    }

    if changes.is_empty() {
        println!("no crate dependency requirements needed updating");
    }
    for change in &changes {
        println!("  {change}");
    }

    check_crate_deps(false)
}

fn crate_dep_problems(
    versions: &BTreeMap<String, String>,
    dependencies: &[CrateDependency],
    manifest_count: usize,
) -> (Vec<String>, usize) {
    let mut problems = Vec::new();

    for name in RELEASED_CRATES {
        if !versions.contains_key(*name) {
            problems.push(format!("{name}: no manifest in this repository defines it"));
        }
    }

    let mut waivers_used = HashSet::new();
    let mut checked = 0;

    for dependency in dependencies {
        let Some(version) = versions.get(&dependency.name) else {
            continue;
        };
        checked += 1;

        let waiver = CRATE_DEP_WAIVERS.iter().find(|(manifest, name, _)| {
            *manifest == dependency.manifest && *name == dependency.name
        });

        if let Some((manifest, name, reason)) = waiver {
            waivers_used.insert((*manifest, *name));
            if dependency.requirement.as_deref() == Some(version.as_str()) {
                problems.push(format!(
                    "{manifest}: {name} = \"{version}\" now matches the workspace, \
                     so the waiver ({reason}) is obsolete and must be removed"
                ));
            }
            continue;
        }

        match &dependency.requirement {
            Some(requirement) if requirement == version => {}
            Some(requirement) => problems.push(format!(
                "{}: {} = \"{requirement}\", expected \"{version}\"",
                dependency.manifest, dependency.name
            )),
            // A path dependency with no version is fine until the crate holding
            // it is published, at which point cargo strips the path and has
            // nothing left to resolve.
            None if dependency.publishable => problems.push(format!(
                "{}: {} declares no version, so publishing it would drop the dependency",
                dependency.manifest, dependency.name
            )),
            None => {}
        }
    }

    for (manifest, name, _) in CRATE_DEP_WAIVERS {
        if !waivers_used.contains(&(*manifest, *name)) {
            problems.push(format!(
                "{manifest}: waived dependency {name} is gone, remove its waiver"
            ));
        }
    }

    // A glob or a `git ls-files` that stops matching would otherwise report the
    // same success as a repository with no drift.
    if checked < MIN_CHECKED_CRATE_DEPS {
        problems.push(format!(
            "only {checked} lumis dependencies found across {manifest_count} manifests, \
             expected at least {MIN_CHECKED_CRATE_DEPS}"
        ));
    }

    (problems, checked)
}

fn fetch_queries(name: &str) -> Result<()> {
    let toml = read_languages_toml()?;
    let tmp = tmpdir()?;
    let mut repo_clones: BTreeMap<String, String> = BTreeMap::new();

    for query_name in query_names()? {
        if !name.is_empty() && query_name != name {
            continue;
        }

        let info = resolve_query_source(&toml.queries, &query_name);
        let clone_key = format!("{}@{}", info.git, info.rev);

        if !repo_clones.contains_key(&clone_key) {
            use md5::{Digest, Md5};
            let hash =
                Md5::digest(clone_key.as_bytes())
                    .iter()
                    .fold(String::new(), |mut hash, byte| {
                        let _ = write!(hash, "{byte:02x}");
                        hash
                    });
            let clone_dir = format!("{tmp}/repo-{hash}");
            println!(
                "Cloning {} at {}",
                info.git,
                &info.rev[..12.min(info.rev.len())]
            );
            let _ = run_cmd_ok(&format!("git clone {} {clone_dir} 2>/dev/null", info.git));
            let _ = run_cmd_ok(&format!(
                "cd {clone_dir} && git checkout {} 2>/dev/null",
                info.rev
            ));
            repo_clones.insert(clone_key.clone(), clone_dir);
        }

        let clone_dir = &repo_clones[&clone_key];
        let dest = format!("queries/upstream/{query_name}");

        let src_dir = if let Some(ref path) = info.path {
            if toml.queries.contains_key(&query_name) && query_name != "default" {
                format!("{clone_dir}/{path}")
            } else {
                format!("{clone_dir}/{path}/{query_name}")
            }
        } else {
            let base = format!("{clone_dir}/queries");
            let sub = format!("{base}/{query_name}");
            if Path::new(&sub).is_dir() {
                sub
            } else {
                base
            }
        };

        if Path::new(&src_dir).is_dir() {
            fs::create_dir_all(&dest)?;
            let _ = run_cmd_ok(&format!("cp -r {src_dir}/* {dest}/ 2>/dev/null"));
            println!("  Updated {query_name} queries");
        } else {
            println!("  Warning: no queries found for {query_name}");
        }
    }

    let _ = run_cmd_ok(&format!("rm -rf {tmp}"));
    Ok(())
}

fn apply_text_replacements(content: &str, lang: &str) -> String {
    let replacements = [
        ("@nospell", ""),
        ("@spell", ""),
        ("; inherits html_tags", "; inherits: html_tags"),
        (
            "#set! @string.special.url url @string.special.url",
            "#set! @string.special.url url \"string.special.url\"",
        ),
        (
            "#set! @_label url @_url",
            "#set! @_label url \"markup.link.url\"",
        ),
        (
            "#set! @_url url @_url",
            "#set! @_url highlight \"markup.link.url\"",
        ),
        (
            "#set! @_hyperlink url @markup.link.url",
            "#set! @_hyperlink highlight \"markup.link.url\"",
        ),
        (r"\c", "(?i)"),
        (r"\(?i)", "(?i)"),
        ("^{[-]|[^|]", r"^\{[-]|^\{[^|]"),
        (r"^\\if", r"^if"),
        (
            "[
  \"\\\\.and\\\\.\"
  \"\\\\.or\\\\.\"
  \"\\\\.eqv\\\\.\"
  \"\\\\.neqv\\\\.\"
  \"\\\\.lt\\\\.\"
  \"\\\\.gt\\\\.\"
  \"\\\\.le\\\\.\"
  \"\\\\.ge\\\\.\"
  \"\\\\.eq\\\\.\"
  \"\\\\.ne\\\\.\"
  \"\\\\.not\\\\.\"
] @keyword.operator",
            "",
        ),
    ];

    let mut s = content.to_string();
    for (old, new) in &replacements {
        s = s.replace(old, new);
    }
    if lang == "swift" {
        s = s.replace(
            "(nil_literal) @constant.builtin",
            "\"nil\" @constant.builtin",
        );
    }
    s
}

fn strip_set_capture_patterns(content: &str) -> String {
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut result = Vec::new();
    let mut i = 0;
    let mut last_end = 0;

    while i < len {
        if bytes[i] == b'(' {
            let start = i;
            let mut depth = 1;
            let mut j = i + 1;
            let mut in_string = false;

            while j < len && depth > 0 {
                let ch = bytes[j];
                if ch == b'"' {
                    let preceding_backslashes = bytes[..j]
                        .iter()
                        .rev()
                        .take_while(|&&byte| byte == b'\\')
                        .count();
                    if preceding_backslashes % 2 == 0 {
                        in_string = !in_string;
                    }
                } else if !in_string {
                    if ch == b'(' {
                        depth += 1;
                    } else if ch == b')' {
                        depth -= 1;
                    }
                }
                j += 1;
            }

            let pattern = &content[start..j];
            if pattern.contains("(#set! @") {
                result.push(&content[last_end..start]);
                last_end = j;
            }
            i = j;
        } else {
            i += 1;
        }
    }

    result.push(&content[last_end..]);
    result.join("")
}

fn read_query_raw(src_dir: &str, lang: &str, query_type: &str) -> String {
    let path = format!("{src_dir}/{lang}/{query_type}.scm");
    fs::read_to_string(path).unwrap_or_default()
}

fn resolve_and_preprocess(
    src_dir: &str,
    override_dir: &str,
    append_dir: &str,
    lang: &str,
    query_type: &str,
    seen: &mut HashSet<String>,
) -> String {
    let key = format!("{lang}/{query_type}");
    if seen.contains(&key) {
        return String::new();
    }
    seen.insert(key);

    let override_path = format!("{override_dir}/{lang}/{query_type}.scm");
    if let Ok(override_content) = fs::read_to_string(&override_path) {
        let mut parts = vec![override_content];

        let append_path = format!("{append_dir}/{lang}/{query_type}.scm");
        if let Ok(append_content) = fs::read_to_string(&append_path) {
            parts.push(append_content);
        }

        return parts.join("\n");
    }

    let raw = if query_type == "brackets" {
        fs::read_to_string(format!("queries/brackets/{lang}/brackets.scm")).unwrap_or_default()
    } else {
        read_query_raw(src_dir, lang, query_type)
    };
    let append_path = format!("{append_dir}/{lang}/{query_type}.scm");
    let append_content = fs::read_to_string(&append_path).ok();

    if raw.is_empty() && append_content.is_none() {
        return String::new();
    }

    let mut parts = Vec::new();
    if !raw.is_empty() {
        let content = apply_text_replacements(&raw, lang);
        let content = strip_set_capture_patterns(&content);

        let mut stripped_lines = Vec::new();

        for line in content.lines() {
            if line.starts_with("; inherits: ") {
                let inherits_str = line.trim_start_matches("; inherits: ").trim();
                for parent in inherits_str.split([',', ' ']).filter(|s| !s.is_empty()) {
                    let parent_content = resolve_and_preprocess(
                        src_dir,
                        override_dir,
                        append_dir,
                        parent,
                        query_type,
                        seen,
                    );
                    if !parent_content.is_empty() {
                        parts.push(format!("; inherits: {parent}"));
                        parts.push(parent_content);
                    }
                }
            } else {
                stripped_lines.push(line);
            }
        }

        parts.push(stripped_lines.join("\n"));
    }

    if let Some(append_content) = append_content {
        parts.push(append_content);
    }

    parts.join("\n")
}

fn preprocess_queries(name: &str) -> Result<()> {
    let src = "queries/upstream";
    let override_dir = "queries/override";
    let append_dir = "queries/append";
    let dest = "queries/processed";

    if name.is_empty() {
        let _ = fs::remove_dir_all(dest);
        if let Ok(default_brackets) = fs::read_to_string("queries/brackets/default/brackets.scm") {
            fs::create_dir_all(format!("{dest}/default"))?;
            fs::write(
                format!("{dest}/default/brackets.scm"),
                format!("; This file is auto-generated. Do not edit.\n{default_brackets}"),
            )?;
        }
    }

    for lang in query_names()? {
        if !name.is_empty() && lang != name {
            continue;
        }
        let _ = fs::remove_dir_all(format!("{dest}/{lang}"));

        fs::create_dir_all(format!("{dest}/{lang}"))?;
        let mut wrote = false;
        for query_type in &["highlights", "injections", "locals", "brackets"] {
            let mut seen = HashSet::new();
            let content =
                resolve_and_preprocess(src, override_dir, append_dir, &lang, query_type, &mut seen);
            if !content.is_empty() {
                // Reject rather than approximate: a Lua pattern that cannot be
                // translated faithfully would ship a regex that means something
                // else in every runtime.
                let content =
                    lumis_build::try_convert_lua_matches(&content).with_context(|| {
                        format!("failed to convert Lua patterns in {lang}/{query_type}.scm")
                    })?;
                let full = format!("; This file is auto-generated. Do not edit.\n{content}");
                fs::write(format!("{dest}/{lang}/{query_type}.scm"), &full)?;
                wrote = true;
            }
        }
        if wrote {
            println!("  {lang}");
        }
    }

    Ok(())
}

fn gen_highlights() -> Result<()> {
    let text = fs::read_to_string("highlights.toml")?;
    let hl: HighlightsToml = toml::from_str(&text)?;
    let mut scopes = hl.scopes;
    scopes.sort();
    let n = scopes.len();

    // Generate highlights.rs
    let mut rs = Vec::new();
    rs.push("// This file is auto-generated from highlights.toml".to_string());
    rs.push("// Run: mise run langs-gen-highlights".to_string());
    rs.push("// Do not edit.".to_string());
    rs.push(String::new());
    rs.push("/// Tree-sitter highlight scope names.".to_string());
    rs.push(format!("pub const HIGHLIGHT_NAMES: [&str; {n}] = ["));
    for scope in &scopes {
        rs.push(format!("    \"{scope}\","));
    }
    rs.push("];".to_string());
    rs.push(String::new());
    rs.push("/// CSS class names for syntax highlighting.".to_string());
    rs.push("///".to_string());
    rs.push(
        "/// Each class name corresponds to a scope name in [`HIGHLIGHT_NAMES`] at the same index."
            .to_string(),
    );
    rs.push(format!("pub const CLASSES: [&str; {n}] = ["));
    for scope in &scopes {
        rs.push(format!("    \"{}\",", scope.replace('.', "-")));
    }
    rs.push("];".to_string());
    rs.push(String::new());

    let rs_path = "crates/lumis-core/src/highlights.rs";
    fs::write(rs_path, rs.join("\n"))?;
    println!("Generated {rs_path} ({n} scopes)");

    // Generate highlights.ts
    let mut ts = vec![
        "// This file is auto-generated from highlights.toml".to_string(),
        "// Run: mise run langs-gen-highlights".to_string(),
        "// Do not edit.".to_string(),
        String::new(),
        "export const HIGHLIGHT_NAMES: string[] = [".to_string(),
    ];
    for scope in &scopes {
        ts.push(format!("  \"{scope}\","));
    }
    ts.push("]".to_string());
    ts.push(String::new());

    let ts_path = "packages/javascript/lumis/src/highlights.ts";
    fs::write(ts_path, ts.join("\n"))?;
    println!("Generated {ts_path} ({n} scopes)");

    Ok(())
}

fn gen_languages_md() -> Result<()> {
    let toml = read_languages_toml()?;

    let mut table_lines = vec![
        "| Language | Parser | Vendored | Version / Rev | Queries | npm | Hex |".to_string(),
        "|----------|--------|----------|---------------|---------|-----|-----|".to_string(),
    ];

    for (lang, info) in &toml.parsers {
        let git = info.git.as_deref().unwrap_or("");
        let rev = info.rev.as_deref().unwrap_or("");
        let short_rev = &rev[..7.min(rev.len())];
        let repo_path = git
            .trim_start_matches("https://github.com/")
            .trim_end_matches(".git");

        let (vendored, version_col, parser_link) = if let Some(ref crate_name) = info.crate_field {
            let v = info.version.as_deref().unwrap_or("?");
            (
                "no",
                format!("`{v}`"),
                format!("[{crate_name}](https://crates.io/crates/{crate_name}/{v})"),
            )
        } else {
            (
                "yes",
                format!("`{short_rev}`"),
                format!("[tree-sitter-{lang}](https://github.com/{repo_path}/tree/{rev})"),
            )
        };

        // One build, two registries: the npm package and the Hex package below
        // carry the same parser, so a row naming only one hides half of where
        // it can be installed from.
        let default_wasm_name = format!("tree-sitter-{lang}");
        let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
        let wasm_suffix = wasm_package_suffix(wasm_name);
        let npm_col = format!(
            "[@lumis-sh/wasm-{wasm_suffix}](https://www.npmjs.com/package/@lumis-sh/wasm-{wasm_suffix})"
        );
        let hex_app = hex_app_name(wasm_name);
        let hex_col = format!("[{hex_app}](https://hex.pm/packages/{hex_app})");

        let query_source = query_source_name(lang, info);
        let query_col = if has_local_override_query(query_source) {
            "local override".to_string()
        } else {
            let qi = resolve_query_source(&toml.queries, query_source);
            let query_repo_path = qi
                .git
                .trim_start_matches("https://github.com/")
                .trim_end_matches(".git");
            let query_repo_name = query_repo_path
                .rsplit('/')
                .next()
                .unwrap_or(query_repo_path);
            let query_short_rev = &qi.rev[..7.min(qi.rev.len())];
            format!(
                "[{query_repo_name}](https://github.com/{query_repo_path}/tree/{}) `{query_short_rev}`",
                qi.rev
            )
        };

        table_lines.push(format!(
            "| {lang} | {parser_link} | {vendored} | {version_col} | {query_col} | {npm_col} | {hex_col} |"
        ));
    }

    if !toml.bundles.is_empty() {
        table_lines.push(String::new());
        table_lines.push("## Bundles".to_string());
        table_lines.push(String::new());
        table_lines.push("| Bundle | Languages |".to_string());
        table_lines.push("|--------|-----------|".to_string());

        let preferred_order = ["web", "web-extra", "system", "backend", "full"];
        let mut bundle_names: Vec<_> = preferred_order
            .iter()
            .filter(|name| toml.bundles.contains_key(**name))
            .copied()
            .collect();
        bundle_names.extend(
            toml.bundles
                .keys()
                .map(String::as_str)
                .filter(|name| !preferred_order.contains(name)),
        );

        for bundle_name in bundle_names {
            let bundle = toml
                .bundles
                .get(bundle_name)
                .with_context(|| format!("missing bundle '{bundle_name}'"))?;
            let parser_names = match &bundle.parsers {
                BundleParsers::List(parsers) => parsers.clone(),
                BundleParsers::All(value) => {
                    if value != "all" {
                        bail!("unsupported bundle parsers value for '{bundle_name}': {value}");
                    }
                    toml.parsers.keys().cloned().collect()
                }
            };
            let languages_col = parser_names
                .iter()
                .map(|parser| format!("`{parser}`"))
                .collect::<Vec<_>>()
                .join(", ");
            table_lines.push(format!("| `{bundle_name}` | {languages_col} |"));
        }
    }

    let mut languages_md = vec![
        "# Supported Languages".to_string(),
        String::new(),
        "> This file is auto-generated by `mise run docs-gen-languages-md`. Do not edit."
            .to_string(),
        String::new(),
    ];
    languages_md.extend(table_lines.clone());

    fs::write("LANGUAGES.md", languages_md.join("\n") + "\n")?;

    // The docs page was a hand-kept copy of this table and had drifted: eight
    // languages short, on query revisions two updates old. Generated from the
    // same rows, it cannot fall behind again.
    let mut docs_md = vec![
        "---".to_string(),
        "title: Language catalog".to_string(),
        "description: Every language Lumis supports, with its parser source, query source, and parser packages on npm and Hex.".to_string(),
        "keywords:".to_string(),
        "  - lumis languages".to_string(),
        "  - tree-sitter parsers".to_string(),
        "  - wasm packages".to_string(),
        "---".to_string(),
        String::new(),
        "Every name in this table is a name you can highlight with, or hand to a preload"
            .to_string(),
        "call — see [Languages](/languages) for when to do which.".to_string(),
        String::new(),
        "Source: [`LANGUAGES.md`](https://github.com/leandrocp/lumis/blob/main/LANGUAGES.md)."
            .to_string(),
        String::new(),
    ];
    docs_md.extend(table_lines);

    fs::write(
        "docs/content/reference/languages.md",
        docs_md.join("\n") + "\n",
    )?;
    println!("Generated LANGUAGES.md and docs/content/reference/languages.md");
    Ok(())
}

fn gen_language_catalog(check: bool) -> Result<()> {
    let toml = read_languages_toml()?;
    let document = read_languages_toml_edit()?;
    let parser_order = document["parsers"]
        .as_table()
        .context("languages.toml must contain a parsers table")?
        .iter()
        .map(|(id, _)| id.to_string())
        .collect::<Vec<_>>();
    let path = "crates/lumis-wasm-runtime/src/catalog.rs";
    let package_version_range = supported_tree_sitter_series()?;
    let output = render_language_catalog(
        &toml.parsers,
        &parser_order,
        &toml.bundles,
        &package_version_range,
    )?;

    if check {
        let current = fs::read_to_string(path)
            .with_context(|| format!("failed to read generated catalog at {path}"))?;
        if current != output {
            bail!("language catalog is stale; run `mise run langs-gen-catalog`");
        }
        println!("Verified {path}");
    } else {
        fs::write(path, output)?;
        println!("Generated {path}");
    }
    Ok(())
}

fn render_language_catalog(
    parsers: &BTreeMap<String, ParserInfo>,
    parser_order: &[String],
    bundles: &BTreeMap<String, BundleInfo>,
    package_version_range: &str,
) -> Result<String> {
    let mut lines = vec![
        "// Auto-generated from languages.toml by `mise run langs-gen-catalog`.".to_string(),
        "// Do not edit manually.".to_string(),
        String::new(),
        "define_catalog! {".to_string(),
        format!("    package_version_range: {package_version_range:?},"),
        "    languages: {".to_string(),
    ];

    for id in parser_order {
        let info = parsers
            .get(id)
            .with_context(|| format!("missing parser metadata for '{id}'"))?;
        let default_wasm_name = format!("tree-sitter-{id}");
        let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
        let package_name = format!("@lumis-sh/wasm-{}", wasm_package_suffix(wasm_name));
        let aliases = info
            .aliases
            .iter()
            .map(|alias| format!("{alias:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        lines.extend([
            format!("        {id:?} => {{"),
            format!("            aliases: [{aliases}],"),
            format!("            package_name: {package_name:?}"),
            "        },".to_string(),
        ]);
    }

    lines.push("    },".to_string());
    lines.push("    bundles: {".to_string());

    for (bundle_name, bundle) in bundles {
        let members = match &bundle.parsers {
            BundleParsers::List(names) => names.clone(),
            BundleParsers::All(value) => {
                if value != "all" {
                    bail!("unsupported bundle parsers value for '{bundle_name}': {value}");
                }
                parser_order.to_vec()
            }
        };
        let rendered = members
            .iter()
            .map(|name| format!("{name:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!("        {bundle_name:?} => [{rendered}],"));
    }

    lines.push("    },".to_string());
    lines.push("}".to_string());
    lines.push(String::new());
    Ok(lines.join("\n"))
}

struct WasmBuildContext<'a> {
    tmp: &'a str,
    out_dir: &'a Path,
    log_dir: &'a Path,
    toolchain: &'a str,
    rebuild: bool,
}

fn build_parser_wasm(
    context: &WasmBuildContext<'_>,
    parser_name: &str,
    wasm_name: &str,
    info: &ParserInfo,
    git: &str,
) -> Result<bool> {
    let rev = info.rev.as_deref().unwrap_or("HEAD");
    let wasm_file = context.out_dir.join(format!("{wasm_name}.wasm"));
    let build_id_file = context.out_dir.join(format!("{wasm_name}.build-id"));
    let build_id = wasm_build_id(
        git,
        rev,
        info.location.as_deref(),
        info.generate.unwrap_or(false),
        context.toolchain,
    );

    if !context.rebuild && cached_parser_is_current(&wasm_file, &build_id_file, &build_id) {
        println!("-> Reusing {wasm_name} built from {rev}");
        return Ok(true);
    }

    // A restored `.wasm` that this revision no longer describes has to go
    // before the rebuild, not after it. `stage-wasm` and `test:queries`
    // both take whatever file is present, so leaving the old one behind
    // would let a parser that failed to build be staged, or have its
    // queries checked against the grammar it replaced.
    let _ = fs::remove_file(&build_id_file);
    let _ = fs::remove_file(&wasm_file);

    let clone_dir = format!("{}/tree-sitter-{parser_name}", context.tmp);
    println!("-> Building WASM for {parser_name} ...");
    println!("* cloning {git}");
    let _ = run_cmd_ok(&format!("git clone --depth 1 {git} {clone_dir}"));
    println!("* checking out {rev}");
    let _ = run_cmd_ok(&format!(
        "cd {clone_dir} && git fetch --depth 1 origin {rev} && git checkout {rev}"
    ));

    let repo_dir = if let Some(ref location) = info.location {
        format!("{clone_dir}/{location}")
    } else {
        clone_dir.clone()
    };
    let metadata_dir = clone_dir.clone();
    let tree_sitter_json = Path::new(&repo_dir).join("tree-sitter.json");
    let has_grammar_source = Path::new(&repo_dir).join("grammar.js").exists()
        || Path::new(&repo_dir).join("grammar.json").exists();
    let has_package_json = Path::new(&repo_dir).join("package.json").exists()
        || Path::new(&metadata_dir).join("package.json").exists();
    let has_package_lock = Path::new(&repo_dir).join("package-lock.json").exists()
        || Path::new(&metadata_dir).join("package-lock.json").exists();

    if has_package_json || tree_sitter_json.exists() {
        println!("* writing tree-sitter.json metadata");
        write_tree_sitter_json(parser_name, &repo_dir, &metadata_dir, info)?;
    }
    if has_grammar_source || info.generate.unwrap_or(false) {
        if has_package_json {
            let install_dir = if Path::new(&repo_dir).join("package.json").exists() {
                &repo_dir
            } else {
                &metadata_dir
            };
            println!("* installing npm dependencies in {install_dir}");
            // `npm ci` refuses to run when a grammar repository ships a
            // `package-lock.json` that has drifted from its `package.json`, which
            // several upstream grammars do. Fall back to `npm install` so the parser
            // stays buildable instead of silently generating without dependencies.
            let ci_failed = has_package_lock
                && run_cmd_ok(&format!("cd {install_dir} && npm ci --ignore-scripts")).is_err();
            if ci_failed {
                println!("  npm ci failed, retrying with npm install");
            }
            if ci_failed || !has_package_lock {
                let _ = run_cmd_ok(&format!("cd {install_dir} && npm install --ignore-scripts"));
            }
        }
        println!("* generating parser sources in {repo_dir}");
        let _ = run_cmd_ok(&format!("cd {repo_dir} && tree-sitter generate"));
    }

    let wasm_path = wasm_file.display();
    let build_log = context.log_dir.join(format!("{wasm_name}.log"));
    println!("* building wasm in {repo_dir}");
    println!("* build log: {}", build_log.display());
    let succeeded = if let Ok(()) = build_repo_wasm(&repo_dir, &wasm_file, &build_log) {
        fs::write(&build_id_file, &build_id)?;
        println!("{wasm_path}");
        true
    } else {
        println!("  ERROR: failed to build {parser_name}");
        false
    };

    let _ = run_cmd_ok(&format!("rm -rf {clone_dir}"));
    Ok(succeeded)
}

fn build_wasm(name: &str) -> Result<()> {
    let toml = read_languages_toml()?;
    let tmp = tmpdir()?;
    let cwd = std::env::current_dir()?;
    let out_dir = cwd.join("tmp/wasm/build");
    let log_dir = cwd.join("tmp/wasm/logs");
    fs::create_dir_all(&out_dir)?;
    fs::create_dir_all(&log_dir)?;
    let mut built_wasm_names = HashSet::new();
    let mut failed = Vec::new();
    let toolchain = wasm_toolchain_id();
    let context = WasmBuildContext {
        tmp: &tmp,
        out_dir: &out_dir,
        log_dir: &log_dir,
        toolchain: &toolchain,
        rebuild: std::env::var("LUMIS_WASM_REBUILD").ok().as_deref() == Some("1"),
    };

    for (parser_name, info) in &toml.parsers {
        let Some(ref git) = info.git else { continue };
        let default_wasm_name = format!("tree-sitter-{parser_name}");
        let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);

        if !name.is_empty() && parser_name != name && wasm_name != name {
            continue;
        }
        if !built_wasm_names.insert(wasm_name.to_string()) {
            continue;
        }
        if !build_parser_wasm(&context, parser_name, wasm_name, info, git)? {
            failed.push(parser_name.clone());
        }
    }

    let _ = run_cmd_ok(&format!("rm -rf {tmp}"));

    // Reported once at the end rather than by returning early, so building the
    // whole corpus still attempts every parser and names all the failures
    // instead of stopping at the first.
    if !failed.is_empty() {
        bail!("failed to build: {}", failed.join(", "));
    }

    Ok(())
}

/// Identifies the toolchain a `.wasm` was produced by, which is the Tree-sitter
/// CLI and nothing else. It compiles through a WASI SDK it downloads itself and
/// pins per release, so its own version covers the compiler too. This used to
/// name Emscripten as well, as a hedge against an input that might matter;
/// nothing invokes `emcc` any more, so the hedge only recorded an empty string.
fn wasm_toolchain_id() -> String {
    run_cmd("tree-sitter --version").unwrap_or_default()
}

fn wasm_build_id(
    git: &str,
    rev: &str,
    location: Option<&str>,
    generate: bool,
    toolchain: &str,
) -> String {
    format!(
        "{git}\n{rev}\n{}\n{generate}\n{toolchain}\n",
        location.unwrap_or("")
    )
}

/// A recorded build id says the inputs still match; the magic number says the
/// bytes survived being cached. Neither alone is enough to skip a build.
fn cached_parser_is_current(wasm_file: &Path, build_id_file: &Path, build_id: &str) -> bool {
    fs::read(wasm_file).is_ok_and(|bytes| bytes.starts_with(b"\0asm"))
        && fs::read_to_string(build_id_file).is_ok_and(|cached| cached == build_id)
}

fn build_repo_wasm(repo_dir: &str, wasm_file: &Path, build_log: &Path) -> Result<()> {
    build_repo_wasm_with("tree-sitter", repo_dir, wasm_file, build_log)
}

/// Takes the compiler as an argument so a test can supply one whose exit status
/// it chooses. Asserting on a missing `tree-sitter` instead would pass for the
/// wrong reason on any machine without the CLI installed.
fn build_repo_wasm_with(
    tree_sitter: &str,
    repo_dir: &str,
    wasm_file: &Path,
    build_log: &Path,
) -> Result<()> {
    // Printed before the build rather than logged after it: a grammar large
    // enough to exhaust the runner takes the process down with it, and this
    // line is then the only evidence the build ever started.
    println!("[cmd] tree-sitter build --wasm -o {}", wasm_file.display());

    let started = Instant::now();
    let output = Command::new(tree_sitter)
        .current_dir(repo_dir)
        .args(["build", "--wasm", "-o"])
        .arg(wasm_file)
        .output()
        .with_context(|| format!("failed to run tree-sitter build in {repo_dir}"))?;

    let mut tail = String::from_utf8_lossy(&output.stdout).into_owned();
    tail.push_str(&String::from_utf8_lossy(&output.stderr));
    let _ = writeln!(tail, "[end] {:.1}s", started.elapsed().as_secs_f64());

    fs::write(
        build_log,
        format!(
            "[cmd] tree-sitter build --wasm -o {}\n{tail}",
            wasm_file.display()
        ),
    )
    .with_context(|| format!("failed to write {}", build_log.display()))?;
    print!("{tail}");

    if !output.status.success() {
        bail!("tree-sitter build failed in {repo_dir}");
    }

    Ok(())
}

fn write_tree_sitter_json(
    parser_name: &str,
    build_dir: &str,
    metadata_dir: &str,
    info: &ParserInfo,
) -> Result<()> {
    let package_path = if Path::new(build_dir).join("package.json").exists() {
        Path::new(build_dir).join("package.json")
    } else {
        Path::new(metadata_dir).join("package.json")
    };
    let package = if package_path.exists() {
        Some(
            serde_json::from_str::<Value>(
                &fs::read_to_string(&package_path)
                    .with_context(|| format!("failed to read {}", package_path.display()))?,
            )
            .with_context(|| format!("failed to parse {}", package_path.display()))?,
        )
    } else {
        None
    };

    let existing_path = Path::new(build_dir).join("tree-sitter.json");
    let existing = if existing_path.exists() {
        Some(
            serde_json::from_str::<Value>(
                &fs::read_to_string(&existing_path)
                    .with_context(|| format!("failed to read {}", existing_path.display()))?,
            )
            .with_context(|| format!("failed to parse {}", existing_path.display()))?,
        )
    } else {
        None
    };

    let selected_grammar = existing
        .as_ref()
        .and_then(|value| value.get("grammars"))
        .and_then(Value::as_array)
        .and_then(|grammars| {
            if let Some(location) = info.location.as_deref() {
                grammars.iter().find(|grammar| {
                    grammar.get("path").and_then(Value::as_str).unwrap_or(".") == location
                })
            } else {
                grammars.first()
            }
        });
    let selected_package_grammar = package
        .as_ref()
        .and_then(|value| value.get("tree-sitter"))
        .and_then(Value::as_array)
        .and_then(|grammars| {
            if let Some(location) = info.location.as_deref() {
                grammars.iter().find(|grammar| {
                    grammar.get("path").and_then(Value::as_str).unwrap_or(".") == location
                })
            } else {
                grammars.first()
            }
        });

    let package_name = package
        .as_ref()
        .and_then(|value| value.get("name"))
        .and_then(Value::as_str)
        .unwrap_or(parser_name);
    let grammar_name = selected_grammar
        .and_then(|grammar| grammar.get("name"))
        .and_then(Value::as_str)
        .or_else(|| {
            selected_package_grammar
                .and_then(|grammar| grammar.get("name"))
                .and_then(Value::as_str)
        })
        .unwrap_or_else(|| {
            if info.location.is_some() {
                parser_name
            } else {
                package_name
                    .strip_prefix("tree-sitter-")
                    .unwrap_or(package_name)
            }
        })
        .replace('_', "-");
    let grammar_path = info
        .location
        .clone()
        .or_else(|| {
            selected_grammar
                .and_then(|grammar| grammar.get("path"))
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
        })
        .unwrap_or_else(|| ".".to_string());
    let scope = selected_grammar
        .and_then(|grammar| grammar.get("scope"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .or_else(|| {
            selected_package_grammar
                .and_then(|entry| entry.get("scope"))
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
        })
        .unwrap_or_else(|| format!("source.{grammar_name}"));
    let file_types = selected_grammar
        .and_then(|grammar| grammar.get("file-types"))
        .and_then(Value::as_array)
        .cloned()
        .or_else(|| {
            selected_package_grammar
                .and_then(|entry| entry.get("file-types"))
                .and_then(Value::as_array)
                .cloned()
        })
        .unwrap_or_else(|| {
            info.globs
                .iter()
                .filter_map(|glob| {
                    glob.strip_prefix("*.")
                        .map(|ext| Value::String(ext.to_string()))
                })
                .collect()
        });
    let repository = existing
        .as_ref()
        .and_then(|value| value.get("metadata"))
        .and_then(|value| value.get("links"))
        .and_then(|value| value.get("repository"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .or_else(|| {
            package.as_ref().and_then(|value| {
                value.get("repository").and_then(|repo| match repo {
                    Value::String(url) => Some(url.clone()),
                    Value::Object(obj) => obj
                        .get("url")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    _ => None,
                })
            })
        })
        .unwrap_or_else(|| info.git.clone().unwrap_or_default());
    let version = existing
        .as_ref()
        .and_then(|value| value.get("metadata"))
        .and_then(|value| value.get("version"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .or_else(|| {
            package
                .as_ref()
                .and_then(|value| value.get("version"))
                .and_then(Value::as_str)
                .map(ToOwned::to_owned)
        })
        .unwrap_or_else(|| info.version.clone().unwrap_or_default());
    let license = existing
        .as_ref()
        .and_then(|value| value.get("metadata"))
        .and_then(|value| value.get("license"))
        .and_then(Value::as_str)
        .or_else(|| {
            package
                .as_ref()
                .and_then(|value| value.get("license"))
                .and_then(Value::as_str)
        })
        .unwrap_or("UNKNOWN");
    let description = existing
        .as_ref()
        .and_then(|value| value.get("metadata"))
        .and_then(|value| value.get("description"))
        .and_then(Value::as_str)
        .or_else(|| {
            package
                .as_ref()
                .and_then(|value| value.get("description"))
                .and_then(Value::as_str)
        })
        .unwrap_or("");
    let normalized_repository = if let Some(repo) = repository.strip_prefix("git@github.com:") {
        format!("https://github.com/{}", repo.trim_end_matches(".git"))
    } else if repository.contains('/') && !repository.contains("://") {
        format!("https://github.com/{}", repository.trim_end_matches(".git"))
    } else {
        repository
    };
    let grammar_dir = Path::new(build_dir).join(&grammar_path);
    let external_files: Vec<Value> = [
        "src/scanner.c",
        "src/scanner.cc",
        "src/scanner.cpp",
        "src/scanner.h",
    ]
    .into_iter()
    .filter(|scanner| grammar_dir.join(scanner).exists())
    .map(|scanner| {
        if grammar_path == "." {
            Value::String(scanner.to_string())
        } else {
            Value::String(format!("{grammar_path}/{scanner}"))
        }
    })
    .collect();
    let bindings = existing
        .as_ref()
        .and_then(|value| value.get("bindings"))
        .cloned()
        .unwrap_or_else(|| json!({ "c": true }));

    let tree_sitter_json = json!({
        "grammars": [
            {
                "name": grammar_name,
                "camelcase": selected_grammar
                    .and_then(|grammar| grammar.get("camelcase"))
                    .and_then(Value::as_str).map_or_else(|| to_camel_case(parser_name), ToOwned::to_owned),
                "scope": scope,
                "path": grammar_path,
                "file-types": file_types,
                "external-files": external_files,
            }
        ],
        "metadata": {
            "version": version,
            "license": license,
            "description": description,
            "authors": [],
            "links": {
                "repository": normalized_repository,
            }
        },
        "bindings": bindings,
    });

    let output_path = Path::new(build_dir).join("tree-sitter.json");
    fs::write(
        &output_path,
        serde_json::to_string_pretty(&tree_sitter_json)
            .context("failed to serialize synthesized tree-sitter.json")?,
    )
    .with_context(|| format!("failed to write {}", output_path.display()))?;

    Ok(())
}

fn to_camel_case(value: &str) -> String {
    value
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let mut out = String::new();
            if let Some(first) = chars.next() {
                out.push(first.to_ascii_uppercase());
                out.extend(chars);
            }
            out
        })
        .collect()
}

fn tree_sitter_cli_minor(version: &str) -> Result<String> {
    let mut parts = version.trim().split('.');
    let major = parts
        .next()
        .filter(|part| !part.is_empty())
        .context("tree-sitter version is missing major component")?;
    let minor = parts
        .next()
        .filter(|part| !part.is_empty())
        .context("tree-sitter version is missing minor component")?;
    Ok(format!("{major}.{minor}"))
}

fn next_wasm_npm_version(pkg_name: &str, ts_cli: &str) -> Result<String> {
    let output = Command::new("npm")
        .args(["view", pkg_name, "versions", "--json"])
        .output()
        .with_context(|| format!("failed to inspect published versions for {pkg_name}"))?;

    let versions = if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_npm_versions_json(&stdout)?
    } else {
        Vec::new()
    };

    let prefix = format!("{ts_cli}.");
    let next_patch = versions
        .iter()
        .filter_map(|version| version.strip_prefix(&prefix))
        .filter_map(|patch| patch.parse::<u64>().ok())
        .max()
        .map_or(0, |patch| patch + 1);

    Ok(format!("{ts_cli}.{next_patch}"))
}

fn parse_npm_versions_json(input: &str) -> Result<Vec<String>> {
    let value: Value = serde_json::from_str(input.trim()).context("invalid npm versions JSON")?;
    match value {
        Value::Array(items) => Ok(items
            .into_iter()
            .filter_map(|item| item.as_str().map(ToOwned::to_owned))
            .collect()),
        Value::String(version) => Ok(vec![version]),
        _ => Ok(Vec::new()),
    }
}

fn stage_wasm(name: &str, version: Option<&str>) -> Result<()> {
    let toml = read_languages_toml()?;
    let (parser_name, info) = toml
        .parsers
        .iter()
        .find(|(parser_name, info)| {
            let default_wasm_name = format!("tree-sitter-{parser_name}");
            let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
            parser_name.as_str() == name || wasm_name == name
        })
        .with_context(|| format!("Unknown parser or wasm artifact: {name}"))?;

    let default_wasm_name = format!("tree-sitter-{parser_name}");
    let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
    let pkg_name = format!("@lumis-sh/wasm-{}", wasm_package_suffix(wasm_name));

    let wasm_file = format!("tmp/wasm/build/{wasm_name}.wasm");
    if !Path::new(&wasm_file).exists() {
        bail!("ERROR: {wasm_file} not found. Run 'mise run wasm-build {wasm_name}' first.");
    }

    let out = format!("tmp/wasm/publish/{wasm_name}");
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out)?;

    fs::copy("templates/wasm/LICENSE", format!("{out}/LICENSE"))?;

    let git_url = info.git.as_deref().unwrap_or("");
    let rev = info.rev.as_deref().unwrap_or("");
    let ts_cli_version = run_cmd("tree-sitter --version")
        .unwrap_or_default()
        .replace("tree-sitter ", "");
    let ts_cli_minor = tree_sitter_cli_minor(&ts_cli_version)?;
    let npm_version = match version {
        Some(version) => version.to_string(),
        None => next_wasm_npm_version(&pkg_name, &ts_cli_minor)?,
    };
    let version = info.version.as_deref().unwrap_or("0.1.0");
    fs::copy(&wasm_file, format!("{out}/{wasm_name}.wasm"))?;

    let wasm_bytes = fs::read(&wasm_file)?;
    let wasm_sha256 = sha256_hex(&wasm_bytes);
    let grammar_name = wasm_grammar_name(&wasm_bytes)?;
    let languages = packaged_languages(&toml, wasm_name)?;
    let definition_hash = language_definition_hash(&toml, wasm_name, &languages)?;
    let language_ids = languages.keys().cloned().collect::<Vec<_>>();
    let languages_text = language_ids.join(", ");
    let language_package = LanguagePackage {
        package_name: pkg_name.clone(),
        version: npm_version.clone(),
        definition_hash: definition_hash.clone(),
        parser: ParserMetadata {
            name: wasm_name.to_string(),
            grammar_name,
            upstream_version: info.version.clone(),
            revision: info.rev.clone(),
            sha256: wasm_sha256.clone(),
            size: u64::try_from(wasm_bytes.len()).expect("parser size fits in u64"),
        },
        languages,
    };
    language_package.validate()?;
    fs::write(
        format!("{out}/lumis.json"),
        format!("{}\n", serde_json::to_string_pretty(&language_package)?),
    )?;

    let browser_entry = render_template(
        "templates/wasm/index.js.template",
        &[("wasm_name", wasm_name)],
    )?;
    fs::write(format!("{out}/index.js"), browser_entry)?;

    fs::copy(
        "templates/wasm/index.d.ts.template",
        format!("{out}/index.d.ts"),
    )?;

    let readme = render_template(
        "templates/wasm/README.md.template",
        &[
            ("wasm_name", wasm_name),
            ("pkg_name", &pkg_name),
            ("hex_package", &hex_app_name(wasm_name)),
            ("languages_text", &languages_text),
            ("git_url", git_url),
            ("rev", rev),
            ("upstream_version", version),
            ("npm_version", &npm_version),
            ("tree_sitter_cli_version", &ts_cli_version),
            ("tree_sitter_cli", &ts_cli_minor),
        ],
    )?;
    fs::write(format!("{out}/README.md"), readme)?;

    let pkg = render_template(
        "templates/wasm/package.json.template",
        &[
            ("pkg_name", &pkg_name),
            ("npm_version", &npm_version),
            ("languages_text", &languages_text),
            ("tree_sitter_cli", &ts_cli_minor),
            ("wasm_name", wasm_name),
            ("definition_hash", &definition_hash),
        ],
    )?;
    fs::write(format!("{out}/package.json"), pkg)?;

    let store = "tmp/wasm/local";
    let local = format!("{store}/parsers");
    fs::create_dir_all(&local)?;
    let suffix = wasm_package_suffix(wasm_name);
    fs::write(
        format!("{local}/{suffix}.lumis.json"),
        serde_json::to_vec(&language_package)?,
    )?;
    fs::copy(
        &wasm_file,
        format!("{local}/{wasm_name}-{npm_version}-{wasm_sha256}.wasm"),
    )?;

    println!("Staged in {out}");
    println!("Runtime-ready copy in {local} as {npm_version}");
    println!("Use it with: export LUMIS_DATA_DIR=$PWD/{store}");
    Ok(())
}

/// Lay the committed parser fixtures out the way a published package would be,
/// so every runtime's tests exercise the real resolve, verify and load path
/// without a network.
fn stage_test_parsers(out: &Path) -> Result<()> {
    const FIXTURES: &str = "fixtures/test-parsers";

    let toml = read_languages_toml()?;
    let fixture_version = lumis_wasm_runtime::lowest_compatible_package_version();
    let parsers = out.join("parsers");
    fs::create_dir_all(&parsers)?;

    let mut staged = 0usize;
    for entry in fs::read_dir(FIXTURES)? {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("wasm") {
            continue;
        }
        let wasm_name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .with_context(|| format!("unreadable fixture name: {}", path.display()))?
            .to_string();

        let wasm = fs::read(&path)?;
        let languages = packaged_languages(&toml, &wasm_name)?;
        let package = LanguagePackage {
            package_name: format!("@lumis-sh/wasm-{}", wasm_package_suffix(&wasm_name)),
            version: fixture_version.clone(),
            definition_hash: language_definition_hash(&toml, &wasm_name, &languages)?,
            parser: ParserMetadata {
                name: wasm_name.clone(),
                grammar_name: wasm_grammar_name(&wasm)?,
                upstream_version: None,
                revision: None,
                sha256: sha256_hex(&wasm),
                size: u64::try_from(wasm.len()).expect("parser size fits in u64"),
            },
            languages,
        };
        package.validate()?;

        fs::write(
            parsers.join(format!("{}.lumis.json", wasm_package_suffix(&wasm_name))),
            serde_json::to_vec(&package)?,
        )?;
        fs::write(parsers.join(parser_filename(&package)), &wasm)?;
        staged += 1;
    }

    if staged == 0 {
        bail!("no parser fixtures found in {FIXTURES}");
    }
    println!("{}", out.display());
    Ok(())
}

fn packaged_languages(
    toml: &LanguagesToml,
    wasm_name: &str,
) -> Result<BTreeMap<String, PackagedLanguage>> {
    let default_brackets =
        fs::read_to_string("queries/processed/default/brackets.scm").unwrap_or_default();
    let mut languages = BTreeMap::new();

    for (id, info) in &toml.parsers {
        let default_name = format!("tree-sitter-{id}");
        if info.wasm_name.as_deref().unwrap_or(&default_name) != wasm_name {
            continue;
        }

        let query_name = info.query_name.as_deref().unwrap_or(id);
        let query = |kind: &str| {
            fs::read_to_string(format!("queries/processed/{query_name}/{kind}.scm"))
                .unwrap_or_default()
        };
        let brackets = {
            let language_brackets = query("brackets");
            if language_brackets.is_empty() {
                default_brackets.clone()
            } else {
                language_brackets
            }
        };

        languages.insert(
            id.clone(),
            PackagedLanguage {
                aliases: info.aliases.clone(),
                highlights: query("highlights"),
                injections: query("injections"),
                locals: query("locals"),
                brackets,
            },
        );
    }

    if languages.is_empty() {
        bail!("no languages use WASM parser '{wasm_name}'");
    }
    Ok(languages)
}

fn language_definition_hash(
    toml: &LanguagesToml,
    wasm_name: &str,
    languages: &BTreeMap<String, PackagedLanguage>,
) -> Result<String> {
    use sha2::{Digest, Sha256};

    let revisions = toml
        .parsers
        .iter()
        .filter_map(|(id, info)| {
            let default_name = format!("tree-sitter-{id}");
            (info.wasm_name.as_deref().unwrap_or(&default_name) == wasm_name)
                .then_some(info.rev.as_deref().unwrap_or(""))
        })
        .collect::<HashSet<_>>();
    if revisions.len() != 1 {
        bail!("WASM parser '{wasm_name}' must have one shared parser revision");
    }

    let mut digest = Sha256::new();
    hash_definition_field(&mut digest, b"lumis-language-package-v3");
    hash_definition_field(&mut digest, wasm_name.as_bytes());
    hash_definition_field(
        &mut digest,
        revisions.into_iter().next().unwrap_or_default().as_bytes(),
    );
    for (id, language) in languages {
        hash_definition_field(&mut digest, id.as_bytes());
        let mut aliases = language.aliases.clone();
        aliases.sort();
        for alias in aliases {
            hash_definition_field(&mut digest, alias.as_bytes());
        }
        hash_definition_field(&mut digest, language.highlights.as_bytes());
        hash_definition_field(&mut digest, language.injections.as_bytes());
        hash_definition_field(&mut digest, language.locals.as_bytes());
        hash_definition_field(&mut digest, language.brackets.as_bytes());
    }
    Ok(lower_hex(&digest.finalize()))
}

fn hash_definition_field(digest: &mut sha2::Sha256, value: &[u8]) {
    use sha2::Digest;

    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
}

fn wasm_grammar_name(wasm: &[u8]) -> Result<String> {
    lumis_wasm_runtime::grammar_name(wasm)
        .context("WASM parser must export exactly one tree_sitter_* language symbol")
}

fn sha256_hex(bytes: &[u8]) -> String {
    lumis_wasm_runtime::sha256_hex(bytes)
}

fn lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn wasm_meta(name: &str) -> Result<()> {
    let toml = read_languages_toml()?;
    let (parser_name, info) = toml
        .parsers
        .iter()
        .find(|(parser_name, info)| {
            let default_wasm_name = format!("tree-sitter-{parser_name}");
            let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
            parser_name.as_str() == name || wasm_name == name
        })
        .with_context(|| format!("Unknown parser or wasm artifact: {name}"))?;

    let default_wasm_name = format!("tree-sitter-{parser_name}");
    let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);

    println!("wasm_name={wasm_name}");
    println!("hex_app={}", hex_app_name(wasm_name));
    Ok(())
}

fn wasm_packages() -> Result<()> {
    let toml = read_languages_toml()?;
    let mut packages = BTreeSet::new();
    for (id, info) in &toml.parsers {
        let default_wasm_name = format!("tree-sitter-{id}");
        let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
        packages.insert(format!("@lumis-sh/wasm-{}", wasm_package_suffix(wasm_name)));
    }
    for package in packages {
        println!("{package}");
    }
    Ok(())
}

const PACKAGE_FORMAT_VERSION: u32 = 3;

const REGISTRY_CONCURRENCY: usize = 16;

const REGISTRY_ATTEMPTS: u32 = 3;

const REGISTRY_TIMEOUT: Duration = Duration::from_secs(30);

/// The `major.minor` series published packages are versioned within.
///
/// Read from the `mise.toml` pin rather than the installed CLI, so this answers
/// the same question on a machine that has no tree-sitter installed. A pin that
/// is not a version — `latest`, which `mise use tree-sitter@latest` writes —
/// is rejected, because it silently produced package versions like `latest.1`.
fn supported_tree_sitter_series() -> Result<String> {
    let mise: toml::Value =
        toml::from_str(&fs::read_to_string("mise.toml").context("could not read mise.toml")?)?;
    let pin = mise
        .get("tools")
        .and_then(|tools| tools.get("tree-sitter"))
        .context("mise.toml does not pin tree-sitter")?;
    let pin = pin
        .as_str()
        .context("mise.toml's tree-sitter pin must be a version string such as \"0.26\"")?;
    tree_sitter_series(pin)
}

fn tree_sitter_series(pin: &str) -> Result<String> {
    let version = lenient_semver::parse(pin).map_err(|error| {
        anyhow::anyhow!(
            "mise.toml pins tree-sitter = {pin:?}; it must be a version such as \"0.26\" so the \
             published package series can be derived from it: {error}"
        )
    })?;
    Ok(format!("{}.{}", version.major, version.minor))
}

/// The full packument carries every version's `package.json`, so one request
/// answers what `npm view` needs one call per version to answer.
///
/// Unlike `npm view`, this reads npmjs.org rather than whatever registry npm is
/// configured for. Both release workflows pin that host explicitly and every
/// `@lumis-sh/wasm-*` package is public, so a mirror or private registry is out
/// of scope here.
fn fetch_packument(agent: &ureq::Agent, pkg: &str) -> Result<Option<Value>> {
    let url = format!("https://registry.npmjs.org/{pkg}");
    let mut last_error = None;

    for attempt in 0..REGISTRY_ATTEMPTS {
        if attempt > 0 {
            thread::sleep(Duration::from_millis(250 * u64::from(attempt)));
        }
        // `call` returns once the headers arrive, so a reset or truncated body
        // surfaces from the read and is just as transient as a failed connect.
        let read = agent
            .get(&url)
            .call()
            .and_then(|mut response| response.body_mut().read_to_string());
        match read {
            Ok(body) => {
                let packument = serde_json::from_str(&body)
                    .with_context(|| format!("invalid packument for {pkg}"))?;
                return Ok(Some(packument));
            }
            Err(ureq::Error::StatusCode(404)) => return Ok(None),
            Err(err) => last_error = Some(err),
        }
    }

    Err(last_error.expect("a failed attempt records its error"))
        .with_context(|| format!("failed to query {pkg} on the npm registry"))
}

/// Each request is almost entirely round trip, so the whole catalog is worth
/// fetching at once. Results come back in the order the packages were given.
fn fetch_packuments(packages: &[String]) -> Vec<Result<Option<Value>>> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(REGISTRY_TIMEOUT))
        .build()
        .into();
    let next = AtomicUsize::new(0);
    let mut fetched: Vec<(usize, Result<Option<Value>>)> = thread::scope(|scope| {
        let workers: Vec<_> = (0..REGISTRY_CONCURRENCY.min(packages.len()))
            .map(|_| {
                scope.spawn(|| {
                    let mut done = Vec::new();
                    loop {
                        let index = next.fetch_add(1, Ordering::Relaxed);
                        let Some(pkg) = packages.get(index) else {
                            return done;
                        };
                        done.push((index, fetch_packument(&agent, pkg)));
                    }
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().expect("npm registry worker panicked"))
            .collect()
    });
    fetched.sort_by_key(|(index, _)| *index);
    fetched.into_iter().map(|(_, result)| result).collect()
}

/// Whether some published version in the current series already carries this
/// exact language definition.
fn published_for_definition(packument: &Value, expected: &str, series: &str) -> bool {
    let Some(versions) = packument.get("versions").and_then(Value::as_object) else {
        return false;
    };
    let prefix = format!("{series}.");
    versions.iter().any(|(version, manifest)| {
        version.starts_with(&prefix)
            && manifest
                .get("lumis")
                .is_some_and(|meta| definition_matches(meta, expected, series))
    })
}

/// Packages published before the v3 format carry no `definitionHash`, so they
/// never match and are always rebuilt.
fn definition_matches(meta: &Value, expected: &str, series: &str) -> bool {
    meta.get("definitionHash").and_then(Value::as_str) == Some(expected)
        && meta.get("treeSitter").and_then(Value::as_str) == Some(series)
        && meta.get("formatVersion").and_then(Value::as_u64) == Some(PACKAGE_FORMAT_VERSION.into())
}

/// A bundle missing from Hex.
#[derive(Debug, serde::Serialize)]
struct BundlePlanEntry {
    bundle: String,
    app: String,
    version: String,
}

/// What a parser resolves to, and where it is missing.
#[derive(Debug, serde::Serialize)]
struct ReleasePlanEntry {
    wasm_name: String,
    npm_package: String,
    hex_package: String,
    version: String,
    npm: bool,
    hex: bool,
}

/// Plan a release for every parser matching `filter`.
///
/// The version comes from the definition, not from a counter. A definition
/// already published keeps the version it was published under, so a registry
/// that is behind receives that exact version rather than a fresh one — which
/// is what stops "publish the missing half" from turning into "release a new
/// version of something that did not change". Only a definition nothing has
/// seen takes the next patch, and that is computed from both registries so
/// neither can hand out a version the other already used.
///
/// # Errors
/// Fails when the catalog cannot be read or a registry cannot be reached.
fn wasm_release_plan(filter: &str) -> Result<()> {
    let series = supported_tree_sitter_series()?;
    let toml = read_languages_toml()?;
    let candidates = release_candidates(&toml, filter)?;

    let npm_packages: Vec<String> = candidates.iter().map(|(_, pkg, _)| pkg.clone()).collect();
    let packuments = fetch_packuments(&npm_packages);
    let registry = hex_registry()?;

    let parsers = plan_parsers(candidates, packuments, &registry, &series)?;
    let bundles = plan_bundles(&registry, filter, &bundle_versions()?);

    println!(
        "{}",
        serde_json::to_string(&json!({ "parsers": parsers, "bundles": bundles }))?
    );
    Ok(())
}

/// The parsers a filter names, empty when it names none.
///
/// One reader, because "is this run filtered?" is asked twice and a second
/// spelling of it would let the parsers and the bundles disagree about the
/// answer for the same input.
fn filter_parsers(filter: &str) -> HashSet<&str> {
    filter
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect()
}

/// Every parser matching `filter`, with the npm package and definition hash
/// that decide where it stands.
fn release_candidates(toml: &LanguagesToml, filter: &str) -> Result<Vec<(String, String, String)>> {
    let wanted = filter_parsers(filter);

    let mut seen = HashSet::new();
    let mut candidates = Vec::new();
    for (id, info) in &toml.parsers {
        let default_name = format!("tree-sitter-{id}");
        let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_name);
        if !wanted.is_empty() && !wanted.contains(id.as_str()) && !wanted.contains(wasm_name) {
            continue;
        }
        if !seen.insert(wasm_name.to_string()) {
            continue;
        }

        let languages = packaged_languages(toml, wasm_name)?;
        let expected = language_definition_hash(toml, wasm_name, &languages)?;
        let suffix = wasm_package_suffix(wasm_name);
        candidates.push((
            wasm_name.to_string(),
            format!("@lumis-sh/wasm-{suffix}"),
            expected,
        ));
    }
    Ok(candidates)
}

/// The parsers a registry is missing, and the version each resolves to.
fn plan_parsers(
    candidates: Vec<(String, String, String)>,
    packuments: Vec<Result<Option<Value>>>,
    registry: &HashMap<String, Vec<String>>,
    series: &str,
) -> Result<Vec<ReleasePlanEntry>> {
    let mut parsers = Vec::new();
    for ((wasm_name, npm_package, expected), packument) in candidates.into_iter().zip(packuments) {
        let packument = packument?;
        let hex_package = hex_app_name(&wasm_name);
        let hex = registry.get(&hex_package).cloned().unwrap_or_default();

        let version = packument
            .as_ref()
            .and_then(|p| version_for_definition(p, &expected, series))
            // Published already: the registry that has it fixes the version,
            // and the other gets the same one.
            .unwrap_or_else(|| next_patch(series, packument.as_ref(), &hex));

        let on_npm = packument
            .as_ref()
            .and_then(|p| p.get("versions"))
            .and_then(Value::as_object)
            .is_some_and(|versions| versions.contains_key(&version));
        let on_hex = hex.contains(&version);

        if !on_npm || !on_hex {
            parsers.push(ReleasePlanEntry {
                wasm_name,
                npm_package,
                hex_package,
                version,
                npm: !on_npm,
                hex: !on_hex,
            });
        }
    }
    Ok(parsers)
}

/// The bundles Hex is missing.
///
/// No build and no npm: bundles carry no bytes, and npm's are released by tag
/// alongside the other JavaScript packages.
///
/// Nothing on a filtered run. A bundle depends on every parser it groups, so
/// one published after a run that covered a subset has Hex dependencies that
/// cannot resolve, and the parsers it is short of are exactly the ones the
/// filter left out.
fn plan_bundles(
    registry: &HashMap<String, Vec<String>>,
    filter: &str,
    candidates: &[(String, String)],
) -> Vec<BundlePlanEntry> {
    if !filter_parsers(filter).is_empty() {
        return Vec::new();
    }

    let mut bundles = Vec::new();
    for (name, version) in candidates {
        let app = format!("lumis_wasm_bundle_{}", name.replace('-', "_"));
        if !registry
            .get(&app)
            .is_some_and(|published| published.contains(version))
        {
            bundles.push(BundlePlanEntry {
                bundle: name.clone(),
                app,
                version: version.clone(),
            });
        }
    }
    bundles
}

/// Each bundle and the version its npm package is at.
///
/// The npm package is the source for both members and version, so the two
/// registries cannot disagree about what a bundle is.
fn bundle_versions() -> Result<Vec<(String, String)>> {
    let mut bundles = Vec::new();
    for entry in glob::glob("packages/javascript/wasm-bundle-*/package.json")? {
        let path = entry?;
        let manifest: Value = serde_json::from_slice(&fs::read(&path)?)
            .with_context(|| format!("invalid package.json at {}", path.display()))?;
        let name = path
            .parent()
            .and_then(|dir| dir.file_name())
            .and_then(|dir| dir.to_str())
            .and_then(|dir| dir.strip_prefix("wasm-bundle-"))
            .with_context(|| format!("could not name the bundle at {}", path.display()))?;
        let version = manifest
            .get("version")
            .and_then(Value::as_str)
            .with_context(|| format!("{} has no version", path.display()))?;
        bundles.push((name.to_string(), version.to_string()));
    }
    bundles.sort();
    Ok(bundles)
}

/// The version a definition was published under on npm, if it was.
///
/// npm alone, because it is the only registry that can answer: a packument
/// carries each version's `lumis` metadata, while Hex's registry file carries
/// versions and nothing else. That makes the answer only as good as the
/// invariant **Hex never holds a definition npm does not**, which
/// `wasm-release.yml` keeps by publishing Hex after npm and holding it back
/// when npm fails. Break that and a Hex-only definition reads as unpublished,
/// takes a fresh patch from `next_patch`, and is released to Hex again
/// unchanged on every run.
fn version_for_definition(packument: &Value, expected: &str, series: &str) -> Option<String> {
    let versions = packument.get("versions")?.as_object()?;
    let prefix = format!("{series}.");
    versions
        .iter()
        .filter(|(version, manifest)| {
            version.starts_with(&prefix)
                && manifest
                    .get("lumis")
                    .is_some_and(|meta| definition_matches(meta, expected, series))
        })
        .map(|(version, _)| version.clone())
        .max_by_key(|version| patch_of(version, series))
}

/// The next free patch in `series`, above whatever either registry has used.
///
/// Both, so a version cannot be handed out that one registry already holds —
/// which would make the two disagree about what that version contains.
fn next_patch(series: &str, packument: Option<&Value>, hex: &[String]) -> String {
    let highest = |versions: Box<dyn Iterator<Item = String>>| {
        versions.filter_map(|v| patch_of(&v, series)).max()
    };

    let npm_max = packument
        .and_then(|p| p.get("versions"))
        .and_then(Value::as_object)
        .and_then(|versions| highest(Box::new(versions.keys().cloned())));
    let hex_max = highest(Box::new(hex.iter().cloned()));

    let next = npm_max.max(hex_max).map_or(0, |patch| patch + 1);
    format!("{series}.{next}")
}

fn patch_of(version: &str, series: &str) -> Option<u64> {
    version
        .strip_prefix(&format!("{series}."))
        .and_then(|patch| patch.parse().ok())
}

/// Every package on Hex and the versions it has published.
///
/// Fetched once per plan, however many parsers it covers: this is the whole
/// registry, so asking about ten parsers and asking about all of them are the
/// same single request. The per-package API cannot be used for this — it
/// rate-limits at a hundred a minute, so a hundred and thirteen lookups fail
/// partway through, and somewhere different each run.
///
/// The response is a gzipped, signed protobuf. Only two fields are read — the
/// payload out of the envelope, and name plus versions out of each package —
/// so this decodes them directly instead of taking a protobuf dependency for
/// four field numbers. The signature is not checked: this decides whether to
/// attempt a publish, and Hex rejects a duplicate regardless of what we
/// believed here.
fn hex_registry() -> Result<HashMap<String, Vec<String>>> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(REGISTRY_TIMEOUT))
        .build()
        .into();

    let mut last_error = None;
    for attempt in 0..REGISTRY_ATTEMPTS {
        if attempt > 0 {
            thread::sleep(Duration::from_millis(250 * u64::from(attempt)));
        }
        let read = agent
            .get("https://repo.hex.pm/versions")
            .call()
            .and_then(|mut response| response.body_mut().read_to_vec());
        match read {
            Ok(body) => return decode_hex_registry(&body),
            Err(err) => last_error = Some(err),
        }
    }

    bail!(
        "could not read the Hex registry: {}",
        last_error.expect("a failed attempt records its error")
    )
}

fn decode_hex_registry(gzipped: &[u8]) -> Result<HashMap<String, Vec<String>>> {
    let mut payload = Vec::new();
    flate2::read::GzDecoder::new(gzipped)
        .read_to_end(&mut payload)
        .context("the Hex registry was not gzip")?;

    // Signed { payload = 1, signature = 2 }
    let signed = protobuf_field(&payload, 1).context("the Hex registry carried no payload")?;
    // Versions { packages = 1 }
    let mut registry = HashMap::new();
    for package in protobuf_fields(signed, 1) {
        // Package { name = 1, versions = 2 }
        let Some(name) = protobuf_field(package, 1) else {
            continue;
        };
        let name = String::from_utf8_lossy(name).into_owned();
        let versions = protobuf_fields(package, 2)
            .map(|v| String::from_utf8_lossy(v).into_owned())
            .collect();
        registry.insert(name, versions);
    }
    Ok(registry)
}

/// Every length-delimited value of `field` in a protobuf message.
fn protobuf_fields(message: &[u8], field: u64) -> impl Iterator<Item = &[u8]> {
    let mut at = 0;
    std::iter::from_fn(move || {
        while at < message.len() {
            let (key, next) = protobuf_varint(message, at)?;
            at = next;
            let (number, wire) = (key >> 3, key & 7);
            match wire {
                2 => {
                    let (len, next) = protobuf_varint(message, at)?;
                    let len = usize::try_from(len).ok()?;
                    at = next;
                    let value = message.get(at..at + len)?;
                    at += len;
                    if number == field {
                        return Some(value);
                    }
                }
                0 => {
                    let (_, next) = protobuf_varint(message, at)?;
                    at = next;
                }
                _ => return None,
            }
        }
        None
    })
}

fn protobuf_field(message: &[u8], field: u64) -> Option<&[u8]> {
    protobuf_fields(message, field).next()
}

fn protobuf_varint(bytes: &[u8], mut at: usize) -> Option<(u64, usize)> {
    let (mut value, mut shift) = (0u64, 0u32);
    loop {
        let byte = *bytes.get(at)?;
        at += 1;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Some((value, at));
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
}

/// Parsers whose published package no longer matches `languages.toml`.
fn wasm_needed(filter: &str, force: &str) -> Result<()> {
    let series = supported_tree_sitter_series()?;
    let toml = read_languages_toml()?;
    let force = matches!(
        force.to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    );
    let wanted: HashSet<&str> = filter
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();

    let mut revisions: BTreeMap<String, &str> = BTreeMap::new();
    for (id, info) in &toml.parsers {
        let default_name = format!("tree-sitter-{id}");
        let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_name);
        let revision = info.rev.as_deref().unwrap_or("");
        if let Some(previous) = revisions.insert(wasm_name.to_string(), revision) {
            if previous != revision {
                bail!("{wasm_name} is shared by different revisions: {previous} and {revision}");
            }
        }
    }

    let mut candidates = Vec::new();
    let mut seen = HashSet::new();
    for (id, info) in &toml.parsers {
        let default_name = format!("tree-sitter-{id}");
        let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_name);
        if !wanted.is_empty() && !wanted.contains(id.as_str()) && !wanted.contains(wasm_name) {
            continue;
        }
        if seen.insert(wasm_name.to_string()) {
            candidates.push(wasm_name.to_string());
        }
    }

    if force {
        println!("{}", candidates.join(" "));
        return Ok(());
    }

    let mut checks = Vec::with_capacity(candidates.len());
    for wasm_name in candidates {
        let languages = packaged_languages(&toml, &wasm_name)?;
        let expected = language_definition_hash(&toml, &wasm_name, &languages)?;
        let pkg = format!("@lumis-sh/wasm-{}", wasm_package_suffix(&wasm_name));
        checks.push((wasm_name, pkg, expected));
    }

    let packages: Vec<String> = checks.iter().map(|(_, pkg, _)| pkg.clone()).collect();

    let mut needed = Vec::new();
    for ((wasm_name, pkg, expected), packument) in checks.iter().zip(fetch_packuments(&packages)) {
        let published = packument?
            .is_some_and(|packument| published_for_definition(&packument, expected, &series));
        if !published {
            eprintln!("Need to publish {pkg} for {expected}");
            needed.push(wasm_name.clone());
        }
    }

    println!("{}", needed.join(" "));
    Ok(())
}

/// Write the Hex package for `name` from the npm package `stage_wasm` staged.
///
/// Deliberately reads `tmp/wasm/publish/<wasm>/`, the directory npm publishes,
/// rather than rebuilding or re-deriving anything. One build produces one
/// parser, and both registries receive that exact file — so "the Hex package
/// and the npm package agree" is a property of the pipeline rather than
/// something to verify after the fact. The digest check below is belt and
/// braces: it fails if the two ever stop being the same bytes.
///
/// # Errors
/// Fails when the npm package has not been staged, or when the parser does not
/// match the digest its own manifest declares.
fn stage_hex_wasm(name: &str) -> Result<()> {
    let toml = read_languages_toml()?;
    let (parser_name, info) = toml
        .parsers
        .iter()
        .find(|(parser_name, info)| {
            let default_wasm_name = format!("tree-sitter-{parser_name}");
            let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
            parser_name.as_str() == name || wasm_name == name
        })
        .with_context(|| format!("Unknown parser or wasm artifact: {name}"))?;

    let default_wasm_name = format!("tree-sitter-{parser_name}");
    let wasm_name = info.wasm_name.as_deref().unwrap_or(&default_wasm_name);
    let suffix = wasm_package_suffix(wasm_name);

    let npm = format!("tmp/wasm/publish/{wasm_name}");
    let manifest_path = format!("{npm}/lumis.json");
    if !Path::new(&manifest_path).exists() {
        bail!(
            "ERROR: {manifest_path} not found. Run 'mise run wasm-publish-prepare {wasm_name}' first."
        );
    }

    let manifest_bytes = fs::read(&manifest_path)?;
    let package: LanguagePackage = serde_json::from_slice(&manifest_bytes)?;
    let wasm_bytes = fs::read(format!("{npm}/{wasm_name}.wasm"))?;

    verify_staged_parser(wasm_name, &wasm_bytes, &package)?;

    let app_name = hex_app_name(wasm_name);
    let module_name = elixir_module_name(&app_name);

    let out = format!("tmp/wasm/hex/{app_name}");
    let _ = fs::remove_dir_all(&out);
    let parsers = format!("{out}/priv/parsers");
    fs::create_dir_all(&parsers)?;

    // The layout `Lumis.Packages` reads: the manifest under the package suffix,
    // and the parser under the content-addressed name the store resolves.
    fs::write(format!("{parsers}/{suffix}.lumis.json"), &manifest_bytes)?;
    fs::write(
        format!("{parsers}/{}", parser_filename(&package)),
        &wasm_bytes,
    )?;

    fs::copy(format!("{npm}/LICENSE"), format!("{out}/LICENSE"))?;

    let languages_text = package
        .languages
        .keys()
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    let mix = render_template(
        "templates/wasm/mix.exs.template",
        &[
            ("module_name", &module_name),
            ("app_name", &app_name),
            ("hex_version", &package.version),
            ("languages_text", &languages_text),
            ("git_url", info.git.as_deref().unwrap_or("")),
        ],
    )?;
    fs::write(format!("{out}/mix.exs"), mix)?;

    // Its own README rather than npm's: that one is titled with the npm package
    // name and sends the reader to a CDN, neither of which helps someone
    // looking at this on hex.pm.
    let readme = render_template(
        "templates/wasm/hex.README.md.template",
        &[
            ("app_name", &app_name),
            (
                "hex_requirement",
                &hex_series_requirement(&package.version)?,
            ),
            ("lumis_requirement", &elixir_lumis_requirement()?),
            ("languages_text", &languages_text),
            ("git_url", info.git.as_deref().unwrap_or("")),
            ("upstream_version", info.version.as_deref().unwrap_or("")),
            ("rev", info.rev.as_deref().unwrap_or("")),
            (
                "tree_sitter_cli_version",
                &run_cmd("tree-sitter --version")
                    .unwrap_or_default()
                    .replace("tree-sitter ", ""),
            ),
        ],
    )?;
    fs::write(format!("{out}/README.md"), readme)?;

    println!("{app_name} {} -> {out}", package.version);
    Ok(())
}

/// Write the Hex meta-package for the `name` bundle.
///
/// Members and version both come from `packages/javascript/wasm-bundle-<name>`,
/// the npm package generated from `languages.toml`, rather than being read from
/// the TOML again. One source means the two registries cannot disagree about
/// what a bundle contains or what version it is — which is the whole point of a
/// bundle, since a project adds one instead of naming languages itself.
///
/// No bytes. `bundle_full` would be on the order of 100 MB as a single package
/// and is a few hundred bytes as a list of dependencies.
///
/// # Errors
/// Fails when the npm bundle has not been generated, or names a package no
/// parser in `languages.toml` produces.
fn stage_hex_bundle(name: &str) -> Result<()> {
    let npm_dir = format!("packages/javascript/wasm-bundle-{name}");
    let manifest = format!("{npm_dir}/package.json");
    if !Path::new(&manifest).exists() {
        bail!("ERROR: {manifest} not found. Run 'mise run build-wasm-bundles' first.");
    }

    let npm: serde_json::Value = serde_json::from_slice(&fs::read(&manifest)?)?;
    let version = npm
        .get("version")
        .and_then(serde_json::Value::as_str)
        .with_context(|| format!("{manifest} has no version"))?;
    let dependencies = npm
        .get("dependencies")
        .and_then(serde_json::Value::as_object)
        .with_context(|| format!("{manifest} has no dependencies"))?;
    let description = npm
        .get("description")
        .and_then(serde_json::Value::as_str)
        .with_context(|| format!("{manifest} has no description"))?;
    check_hex_description(&manifest, description)?;

    let app_name = format!("lumis_wasm_bundle_{}", name.replace('-', "_"));
    let module_name = elixir_module_name(&app_name);

    let members = hex_bundle_members(dependencies)?;

    let deps = members
        .iter()
        .map(|(app, requirement)| format!("      {{:{app}, \"{requirement}\"}}"))
        .collect::<Vec<_>>()
        .join(",\n");
    let languages = members
        .iter()
        .map(|(app, _)| app.trim_start_matches("lumis_wasm_"))
        .collect::<Vec<_>>();
    let languages_text = languages.join(", ");

    let out = format!("tmp/wasm/hex/{app_name}");
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out)?;

    fs::copy("templates/wasm/LICENSE", format!("{out}/LICENSE"))?;

    let readme = render_template(
        "templates/wasm/hex.bundle.README.md.template",
        &[
            ("app_name", &app_name),
            ("hex_requirement", &hex_series_requirement(version)?),
            ("lumis_requirement", &elixir_lumis_requirement()?),
            ("languages_text", &languages_text),
        ],
    )?;
    fs::write(format!("{out}/README.md"), readme)?;

    let mix = render_template(
        "templates/wasm/bundle.mix.exs.template",
        &[
            ("module_name", &module_name),
            ("app_name", &app_name),
            ("hex_version", version),
            ("description", description),
            ("deps", &deps),
        ],
    )?;
    fs::write(format!("{out}/mix.exs"), mix)?;

    println!("{app_name} {version} -> {out} ({} members)", members.len());
    Ok(())
}

/// What Hex refuses to build a package over.
const HEX_DESCRIPTION_LIMIT: usize = 300;

/// Refuse a description Hex will refuse, while the manifest is still in reach.
///
/// Hex enforces this at `mix hex.publish`, which in a release runs on a runner
/// after every parser has already been published — `lumis_wasm_bundle_full` hit
/// it at 865 characters and stopped there, with the registry half moved. Read
/// here it is a staging error naming the file to edit.
///
/// # Errors
/// Fails on a description over [`HEX_DESCRIPTION_LIMIT`] characters.
fn check_hex_description(manifest: &str, description: &str) -> Result<()> {
    let length = description.chars().count();
    ensure!(
        length <= HEX_DESCRIPTION_LIMIT,
        "{manifest} has a {length}-character description, and Hex stops the \
         package build over {HEX_DESCRIPTION_LIMIT}"
    );
    Ok(())
}

/// The `lumis_wasm_*` dependencies meaning what the npm bundle's mean.
///
/// Sorted, because the generated `mix.exs` is read by people and a JSON object's
/// order is not something to inherit.
///
/// # Errors
/// Fails on a dependency that no parser package produces, or on a requirement
/// [`hex_requirement`] cannot translate.
fn hex_bundle_members(
    dependencies: &serde_json::Map<String, serde_json::Value>,
) -> Result<Vec<(String, String)>> {
    let mut members = Vec::new();
    for (package_name, requirement) in dependencies {
        let suffix = package_name
            .strip_prefix("@lumis-sh/wasm-")
            .with_context(|| format!("{package_name} is not a parser package"))?;
        let requirement = requirement
            .as_str()
            .with_context(|| format!("{package_name} has no version requirement"))?;
        members.push((
            format!("lumis_wasm_{}", suffix.replace('-', "_")),
            hex_requirement(requirement)?,
        ));
    }
    members.sort();
    Ok(members)
}

/// The Hex requirement meaning what an npm one means.
///
/// Not a rewrite of the prefix: the two spell overlapping but different things,
/// and `~> 1.2.3` silently narrows `^1.2.3` from "below 2.0.0" to "below
/// 1.3.0". They coincide only below 1.0, which is where every parser is today
/// and is exactly why this would have gone unnoticed until it mattered.
///
/// # Errors
/// Fails on a requirement with no equivalent, rather than publishing a bundle
/// that means something the npm one does not.
fn hex_requirement(npm: &str) -> Result<String> {
    let exact = |version: &str| -> Result<(u64, u64, u64)> {
        let mut parts = version.split('.');
        let mut next = || -> Result<u64> {
            parts
                .next()
                .and_then(|part| part.parse().ok())
                .with_context(|| format!("{npm} is not a version this can translate"))
        };
        let parsed = (next()?, next()?, next()?);
        if parts.next().is_some() {
            bail!("{npm} is not a version this can translate");
        }
        Ok(parsed)
    };

    if let Some(version) = npm.strip_prefix('^') {
        let (major, minor, patch) = exact(version)?;

        // npm moves the breaking digit left as the version approaches zero, and
        // `~>` does not follow it, so each case needs its own bounds.
        if major == 0 && minor == 0 {
            // `^0.0.3` is patch-locked; `~> 0.0.3` would run to 0.1.0.
            return Ok(format!(">= {version} and < 0.0.{}", patch + 1));
        }
        if major == 0 {
            // `^0.26.0` and `~> 0.26.0` both stop below 0.27.0.
            return Ok(format!("~> {version}"));
        }
        // `~>` with three parts stops below X.(Y+1).0, so adding an upper bound
        // to it narrows rather than widens: the two intersect at the lower one.
        Ok(format!(">= {version} and < {}.0.0", major + 1))
    } else if let Some(version) = npm.strip_prefix('~') {
        exact(version)?;
        Ok(format!("~> {version}"))
    } else {
        // Bare in npm is exact, and `~>` is not.
        exact(npm)?;
        Ok(format!("== {npm}"))
    }
}

/// `lumis_wasm_json` as `LumisWasmJson`.
fn elixir_module_name(app_name: &str) -> String {
    app_name
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Fail unless `bytes` are the parser the manifest beside them describes.
///
/// The guarantee the Hex package rests on: it is staged from what npm ships, so
/// a mismatch means the two came from different builds and one of them is not
/// the artifact that was reviewed.
fn verify_staged_parser(wasm_name: &str, bytes: &[u8], package: &LanguagePackage) -> Result<()> {
    let actual = sha256_hex(bytes);
    if actual != package.parser.sha256 {
        bail!(
            "ERROR: {wasm_name}.wasm does not match the manifest staged beside it\n  \
             manifest {}\n  file     {actual}",
            package.parser.sha256
        );
    }
    Ok(())
}

/// The Hex application name for a parser.
///
/// Hex application names are atoms: lowercase with underscores, so a package
/// like `embedded-template` cannot keep its hyphen.
fn hex_app_name(wasm_name: &str) -> String {
    format!(
        "lumis_wasm_{}",
        wasm_package_suffix(wasm_name).replace('-', "_")
    )
}

fn wasm_package_suffix(wasm_name: &str) -> &str {
    wasm_name.strip_prefix("tree-sitter-").unwrap_or(wasm_name)
}

/// Fill a template's `{placeholder}` tokens, failing on one left unfilled.
///
/// These files are rendered only while publishing, so a typo or a placeholder
/// added to a template and nowhere else reaches hex.pm and npm as literal
/// `{hex_requirement}` in a README nobody reads again.
///
/// # Errors
/// Fails when the template cannot be read, or when the rendered output still
/// holds a `{lower_snake_case}` token.
fn render_template(path: &str, values: &[(&str, &str)]) -> Result<String> {
    let mut rendered = fs::read_to_string(path)?;
    for (key, value) in values {
        rendered = rendered.replace(&format!("{{{key}}}"), value);
    }
    if let Some(name) = leftover_placeholder(&rendered) {
        bail!("{path} has no value for {{{name}}}");
    }
    Ok(rendered)
}

/// The first `{lower_snake_case}` token in `rendered`, if any.
///
/// Every other brace these templates carry — Elixir's `%{}` and `{:dep, "..."}`,
/// JSON objects — holds something outside that alphabet, so it is left alone.
fn leftover_placeholder(rendered: &str) -> Option<&str> {
    let mut rest = rendered;
    while let Some(open) = rest.find('{') {
        rest = &rest[open + 1..];
        let close = rest.find('}')?;
        let name = &rest[..close];
        if !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
            return Some(name);
        }
    }
    None
}

/// The Hex requirement for a package of `version`, bounded to its own series.
///
/// Three components rather than the two a reader expects from `~> 0.26`, which
/// in Hex runs to 1.0.0. A parser package's minor is its Tree-sitter series, and
/// `lumis_wasm_runtime` refuses to load a package outside the one series it was
/// built for, so the looser requirement lets Mix resolve a parser that then
/// fails at highlight time. `~> 0.26.0` is the range the runtime accepts,
/// spelled in Hex.
///
/// # Errors
/// Fails on a version whose major and minor are not numbers.
fn hex_series_requirement(version: &str) -> Result<String> {
    let mut parts = version.split('.');
    let mut number = || -> Result<u64> {
        parts
            .next()
            .and_then(|part| part.parse().ok())
            .with_context(|| format!("{version} is not a version a requirement can be built from"))
    };
    let (major, minor) = (number()?, number()?);
    Ok(format!("{major}.{minor}.0"))
}

/// The `lumis` requirement a generated README installs alongside a parser.
///
/// Read from the Elixir package in this repository rather than written into the
/// templates, so the install snippet on ~120 published packages does not have to
/// be remembered when Lumis releases a minor.
///
/// # Errors
/// Fails when `mix.exs` no longer carries a `@version` attribute, rather than
/// publishing a README that names a version nobody has.
fn elixir_lumis_requirement() -> Result<String> {
    let path = "packages/elixir/lumis/mix.exs";
    let mix = fs::read_to_string(path)?;
    let version = mix
        .split_once("@version \"")
        .and_then(|(_, rest)| rest.split_once('"'))
        .map(|(version, _)| version)
        .with_context(|| format!("{path} has no @version attribute"))?;
    let mut parts = version.split('.');
    let mut number = || -> Result<u64> {
        parts
            .next()
            .and_then(|part| part.parse().ok())
            .with_context(|| format!("{path} declares @version \"{version}\""))
    };
    let (major, minor) = (number()?, number()?);
    Ok(format!("{major}.{minor}"))
}

#[cfg(test)]
mod hex_wasm_tests {
    use super::*;

    fn packument(versions: &[(&str, &str)]) -> Value {
        json!({
            "versions": versions.iter().map(|(version, hash)| {
                (version.to_string(), json!({
                    "lumis": {
                        "definitionHash": hash,
                        "treeSitter": "0.26",
                        "formatVersion": PACKAGE_FORMAT_VERSION,
                    }
                }))
            }).collect::<serde_json::Map<_, _>>()
        })
    }

    /// The bug this resolution exists to prevent: asking to publish the
    /// registry that is behind used to compute a *new* version, releasing
    /// something that had not changed to the registry that was already current.
    #[test]
    fn a_published_definition_keeps_the_version_it_went_out_under() {
        let published = packument(&[("0.26.2", "old"), ("0.26.3", "current")]);

        assert_eq!(
            version_for_definition(&published, "current", "0.26"),
            Some("0.26.3".to_string()),
            "the definition is already out as 0.26.3, so that is the version"
        );
    }

    #[test]
    fn a_definition_nobody_has_takes_the_next_patch() {
        let published = packument(&[("0.26.2", "old"), ("0.26.3", "older")]);

        assert_eq!(version_for_definition(&published, "new", "0.26"), None);
        assert_eq!(next_patch("0.26", Some(&published), &[]), "0.26.4");
    }

    /// Either registry having used a patch rules it out, or the two would
    /// disagree about what that version contains.
    #[test]
    fn the_next_patch_clears_both_registries() {
        let published = packument(&[("0.26.3", "a")]);
        let hex = vec!["0.26.1".to_string(), "0.26.9".to_string()];

        assert_eq!(next_patch("0.26", Some(&published), &hex), "0.26.10");
        assert_eq!(next_patch("0.26", None, &hex), "0.26.10");
        assert_eq!(next_patch("0.26", None, &[]), "0.26.0");
    }

    /// A bundle depends on every parser it groups, so one published after a run
    /// that covered a subset has Hex dependencies that cannot resolve.
    #[test]
    fn a_filtered_run_plans_no_bundles() {
        let registry = HashMap::new();
        let candidates = [("web".to_string(), "0.26.3".to_string())];

        assert_eq!(
            plan_bundles(&registry, "", &candidates).len(),
            1,
            "Hex has no bundles, so an unfiltered run plans the one there is"
        );
        assert!(plan_bundles(&registry, "json", &candidates).is_empty());
        assert!(plan_bundles(&registry, "json,elixir", &candidates).is_empty());
    }

    /// Both readers of the filter have to agree on what counts as filtered, or
    /// a run can select every parser while planning no bundles.
    #[test]
    fn separators_alone_do_not_filter() {
        assert!(filter_parsers("").is_empty());
        assert!(filter_parsers(" , ").is_empty());
        assert_eq!(filter_parsers(" json , elixir ").len(), 2);
    }

    /// A version from another series is not a patch of this one.
    #[test]
    fn other_series_are_ignored() {
        let published = packument(&[("0.25.9", "a"), ("0.26.1", "b")]);

        assert_eq!(next_patch("0.26", Some(&published), &[]), "0.26.2");
    }

    #[test]
    fn the_hex_registry_decodes_to_packages_and_versions() {
        // Signed { payload: Versions { packages: [Package{name, versions}] } }
        fn len_delim(field: u64, body: &[u8]) -> Vec<u8> {
            let mut out = vec![u8::try_from((field << 3) | 2).unwrap()];
            let mut len = body.len();
            loop {
                let mut byte = u8::try_from(len & 0x7f).unwrap();
                len >>= 7;
                if len > 0 {
                    byte |= 0x80;
                }
                out.push(byte);
                if len == 0 {
                    break;
                }
            }
            out.extend_from_slice(body);
            out
        }

        let mut package = len_delim(1, b"lumis_wasm_json");
        package.extend(len_delim(2, b"0.26.2"));
        package.extend(len_delim(2, b"0.26.3"));
        let versions = len_delim(1, &package);
        let signed = len_delim(1, &versions);

        let mut gzipped = Vec::new();
        {
            use std::io::Write as _;
            let mut encoder =
                flate2::write::GzEncoder::new(&mut gzipped, flate2::Compression::fast());
            encoder.write_all(&signed).unwrap();
            encoder.finish().unwrap();
        }

        let registry = decode_hex_registry(&gzipped).unwrap();
        assert_eq!(
            registry.get("lumis_wasm_json").map(Vec::as_slice),
            Some(["0.26.2".to_string(), "0.26.3".to_string()].as_slice())
        );
    }

    #[test]
    fn npm_requirements_become_hex_ones_that_mean_the_same() {
        // Below 1.0 the two notations coincide, which is where parsers are.
        assert_eq!(hex_requirement("^0.26.0").unwrap(), "~> 0.26.0");
        assert_eq!(hex_requirement("~0.26.0").unwrap(), "~> 0.26.0");

        // Above it they do not. `~>` with three parts already stops below
        // 1.3.0, so pairing it with an upper bound intersects to the lower one
        // rather than widening to 2.0.0.
        assert_eq!(hex_requirement("^1.2.3").unwrap(), ">= 1.2.3 and < 2.0.0");

        // npm moves the breaking digit left again at 0.0.x: `^0.0.3` is
        // patch-locked where `~> 0.0.3` would run to 0.1.0.
        assert_eq!(hex_requirement("^0.0.3").unwrap(), ">= 0.0.3 and < 0.0.4");

        // Bare is exact in npm, and `~>` is a range.
        assert_eq!(hex_requirement("1.2.3").unwrap(), "== 1.2.3");

        // Anything else is refused rather than guessed at.
        for unsupported in [">=1.2.3", "1.2.x", "*", "1.2", "latest"] {
            assert!(
                hex_requirement(unsupported).is_err(),
                "{unsupported} has no equivalent and must not be translated"
            );
        }
    }

    /// `lumis_wasm_bundle_full` reached Hex's limit at 865 characters, and the
    /// only thing that noticed was `mix hex.publish` on a runner, after the
    /// parsers it groups had been published.
    #[test]
    fn a_description_hex_would_reject_is_refused_while_staging() {
        assert!(check_hex_description("a/package.json", "Lumis WASM full language bundle").is_ok());
        assert!(check_hex_description("a/package.json", &"x".repeat(300)).is_ok());

        let too_long = check_hex_description("a/package.json", &"x".repeat(301))
            .expect_err("Hex stops the package build over 300");
        let message = too_long.to_string();
        assert!(
            message.contains("a/package.json"),
            "{message} names no file"
        );
        assert!(message.contains("301"), "{message} does not say how long");

        // Characters, not bytes: a multi-byte description under the limit is
        // one Hex accepts, and counting bytes would refuse it here.
        assert!(check_hex_description("a/package.json", &"é".repeat(300)).is_ok());
    }

    /// The npm bundle is the same package, and already carries the text. A
    /// second one generated here is a second thing to keep in step.
    #[test]
    fn every_bundle_manifest_carries_a_description_hex_accepts() {
        let manifests = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../packages/javascript")
            .join("wasm-bundle-*/package.json");

        let mut checked = 0;
        for entry in glob::glob(&manifests.to_string_lossy()).unwrap() {
            let path = entry.unwrap();
            let manifest: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            let description = manifest
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or_else(|| panic!("{} has no description", path.display()));

            check_hex_description(&path.to_string_lossy(), description)
                .expect("a bundle manifest Hex would reject cannot be staged");
            checked += 1;
        }
        assert!(checked >= 5, "found {checked} bundle manifests");
    }

    #[test]
    fn bundle_modules_are_named_after_their_application() {
        assert_eq!(
            elixir_module_name("lumis_wasm_bundle_web_extra"),
            "LumisWasmBundleWebExtra"
        );
        assert_eq!(elixir_module_name("lumis_wasm_json"), "LumisWasmJson");
    }

    /// A parser only loads inside the Tree-sitter series it was built for, so
    /// the requirement has to stop at the next one. `~> 0.26` would not.
    #[test]
    fn a_requirement_stops_at_the_end_of_its_series() {
        assert_eq!(hex_series_requirement("0.26.4").unwrap(), "0.26.0");
        assert_eq!(hex_series_requirement("0.1.0").unwrap(), "0.1.0");
        assert_eq!(hex_series_requirement("1.2.3").unwrap(), "1.2.0");
        assert!(hex_series_requirement("nightly").is_err());
        assert!(hex_series_requirement("0.x.4").is_err());
    }

    /// The whole point of the guard: a template gaining a placeholder that the
    /// staging code does not fill has to stop the publish, not ship as text.
    #[test]
    fn an_unfilled_placeholder_is_not_published() {
        let template = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../templates/wasm/hex.README.md.template")
            .to_string_lossy()
            .into_owned();

        let filled = render_template(
            &template,
            &[
                ("app_name", "lumis_wasm_json"),
                ("hex_requirement", "0.26.0"),
                ("lumis_requirement", "0.8"),
                ("languages_text", "json"),
                ("npm_package", "@lumis-sh/wasm-json"),
                ("git_url", "https://example.invalid/tree-sitter-json"),
                ("upstream_version", "0.24.8"),
                ("rev", "0123456"),
                ("tree_sitter_cli_version", "0.26.0"),
            ],
        )
        .expect("every placeholder the template carries has a value");
        assert!(filled.contains(r#"{:lumis_wasm_json, "~> 0.26.0"}"#));
        assert!(filled.contains(r#"{:lumis, "~> 0.8"}"#));

        assert!(
            render_template(&template, &[]).is_err(),
            "an unfilled placeholder has to fail the staging run"
        );
    }

    /// Elixir map and dependency syntax is braces too, and rendering must not
    /// mistake either for a placeholder.
    #[test]
    fn only_placeholders_look_like_placeholders() {
        assert_eq!(leftover_placeholder("{app_name} here"), Some("app_name"));
        assert_eq!(leftover_placeholder("%{\n  \"Lumis\" => \"x\"\n}"), None);
        assert_eq!(leftover_placeholder("{:lumis, \"~> 0.8\"}"), None);
        assert_eq!(leftover_placeholder("{unclosed"), None);
        assert_eq!(leftover_placeholder("{}"), None);
        assert_eq!(leftover_placeholder("%{\n  {inner}\n}"), Some("inner"));
    }

    #[test]
    fn hex_app_names_are_valid_atoms() {
        assert_eq!(hex_app_name("tree-sitter-json"), "lumis_wasm_json");
        // Hyphens cannot survive: an application name is an atom.
        assert_eq!(
            hex_app_name("tree-sitter-embedded-template"),
            "lumis_wasm_embedded_template"
        );
        assert_eq!(
            hex_app_name("tree-sitter-markdown_inline"),
            "lumis_wasm_markdown_inline"
        );

        for name in ["tree-sitter-json", "tree-sitter-embedded-template"] {
            let app = hex_app_name(name);
            assert!(
                app.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "{app} is not usable as an Elixir application name"
            );
        }
    }

    /// The whole point of staging Hex from the npm output: both registries get
    /// the parser one build produced. A mismatch means a rebuild happened
    /// between them, and the two packages would ship different bytes.
    #[test]
    fn staging_hex_refuses_a_parser_that_does_not_match_its_manifest() {
        let staged = b"the parser npm received";
        let package = LanguagePackage {
            package_name: "@lumis-sh/wasm-json".into(),
            version: "0.26.4".into(),
            definition_hash: "hash".into(),
            parser: ParserMetadata {
                name: "tree-sitter-json".into(),
                grammar_name: "json".into(),
                upstream_version: None,
                revision: None,
                sha256: sha256_hex(staged),
                size: staged.len() as u64,
            },
            languages: BTreeMap::new(),
        };

        assert!(
            verify_staged_parser("tree-sitter-json", staged, &package).is_ok(),
            "the parser that was staged has to pass"
        );

        let error = verify_staged_parser("tree-sitter-json", b"a different build", &package)
            .expect_err("a parser from another build must be refused");
        assert!(error.to_string().contains("does not match the manifest"));
    }
}

#[cfg(test)]
mod crate_dep_tests {
    use super::*;

    fn versions() -> BTreeMap<String, String> {
        RELEASED_CRATES
            .iter()
            .map(|name| (name.to_string(), "2.3.0".to_string()))
            .collect()
    }

    fn dependency(manifest: &str, requirement: Option<&str>) -> CrateDependency {
        CrateDependency {
            manifest: manifest.to_string(),
            name: "lumis-core".to_string(),
            requirement: requirement.map(str::to_string),
            publishable: true,
        }
    }

    /// The waived entry keeps every case above the corpus floor, so a real
    /// problem is what fails rather than the sanity check underneath it.
    fn padding() -> Vec<CrateDependency> {
        let (manifest, name, _) = CRATE_DEP_WAIVERS[0];
        let mut dependencies = vec![CrateDependency {
            manifest: manifest.to_string(),
            name: name.to_string(),
            requirement: Some("0.0.1".to_string()),
            publishable: true,
        }];
        dependencies.extend(
            (0..MIN_CHECKED_CRATE_DEPS)
                .map(|index| dependency(&format!("c{index}"), Some("2.3.0"))),
        );
        dependencies
    }

    fn problems(extra: Vec<CrateDependency>) -> Vec<String> {
        let mut dependencies = padding();
        dependencies.extend(extra);
        let count = dependencies.len();
        crate_dep_problems(&versions(), &dependencies, count).0
    }

    #[test]
    fn matching_requirements_are_accepted() {
        assert!(
            problems(Vec::new()).is_empty(),
            "no dependencies, so no problems"
        );
    }

    #[test]
    fn a_requirement_below_the_workspace_version_is_reported() {
        let reported = problems(vec![dependency("crates/lumis/Cargo.toml", Some("2"))]);
        assert_eq!(reported.len(), 1);
        assert!(
            reported[0].contains("lumis-core = \"2\", expected \"2.3.0\""),
            "{reported:?}"
        );
    }

    #[test]
    fn a_newer_requirement_than_the_workspace_is_reported() {
        assert_eq!(
            problems(vec![dependency("crates/lumis/Cargo.toml", Some("2.4.0"))]).len(),
            1
        );
    }

    #[test]
    fn a_publishable_crate_must_declare_a_version() {
        let reported = problems(vec![dependency("crates/lumis/Cargo.toml", None)]);
        assert_eq!(reported.len(), 1);
        assert!(reported[0].contains("declares no version"), "{reported:?}");
    }

    #[test]
    fn an_unpublished_crate_may_omit_the_version() {
        let mut unpublished = dependency("benchmarks/rust/Cargo.toml", None);
        unpublished.publishable = false;
        assert!(
            problems(vec![unpublished]).is_empty(),
            "an unpublished crate needs no version requirement"
        );
    }

    #[test]
    fn a_waiver_that_stopped_drifting_is_reported() {
        let (manifest, name, _) = CRATE_DEP_WAIVERS[0];
        let mut dependencies: Vec<CrateDependency> = padding()
            .into_iter()
            .filter(|dependency| dependency.manifest != manifest)
            .collect();
        dependencies.push(CrateDependency {
            manifest: manifest.to_string(),
            name: name.to_string(),
            requirement: Some("2.3.0".to_string()),
            publishable: true,
        });
        let count = dependencies.len();
        let reported = crate_dep_problems(&versions(), &dependencies, count).0;
        assert_eq!(reported.len(), 1);
        assert!(reported[0].contains("obsolete"), "{reported:?}");
    }

    #[test]
    fn a_waiver_whose_dependency_is_gone_is_reported() {
        let (manifest, _, _) = CRATE_DEP_WAIVERS[0];
        let dependencies: Vec<CrateDependency> = padding()
            .into_iter()
            .filter(|dependency| dependency.manifest != manifest)
            .collect();
        let count = dependencies.len();
        let reported = crate_dep_problems(&versions(), &dependencies, count).0;
        assert_eq!(reported.len(), 1);
        assert!(reported[0].contains("remove its waiver"), "{reported:?}");
    }

    fn fixed(manifest: &str) -> String {
        let mut document: toml_edit::DocumentMut = manifest.parse().expect("valid manifest");
        fix_manifest(
            "crates/example/Cargo.toml",
            true,
            &mut document,
            &versions(),
        );
        document.to_string()
    }

    fn collected(manifest: &str) -> Vec<(String, Option<String>)> {
        let mut document: toml_edit::DocumentMut = manifest.parse().expect("valid manifest");
        let mut found = Vec::new();
        for_each_dependency(document.as_table_mut(), &mut |name, spec| {
            found.push((name.to_string(), requirement_of(spec)));
        });
        found
    }

    #[test]
    fn target_specific_and_renamed_dependencies_are_visited() {
        let mut found = collected(
            r#"
[dependencies]
lumis-core = { version = "2.0.0", features = ["all-languages"] }

[target.'cfg(windows)'.dependencies]
core-alias = { package = "lumis-core", version = "2.1.0" }

[build-dependencies]
lumis-build = "1.0.0"
"#,
        );
        found.sort();

        assert_eq!(
            found,
            vec![
                ("lumis-build".to_string(), Some("1.0.0".to_string())),
                ("lumis-core".to_string(), Some("2.0.0".to_string())),
                // Reached only through `[target.'cfg(windows)'.dependencies]`,
                // and named by `package = "lumis-core"` rather than its key.
                ("lumis-core".to_string(), Some("2.1.0".to_string())),
            ]
        );
    }

    #[test]
    fn a_dependency_table_keeps_its_other_fields() {
        let fixed = fixed(
            r#"
[dependencies.lumis-core]
path = "../lumis-core"
version = "2"
default-features = false
features = ["lang-rust"]
"#,
        );

        assert!(fixed.contains("version = \"2.3.0\""), "{fixed}");
        assert!(fixed.contains("path = \"../lumis-core\""), "{fixed}");
        assert!(fixed.contains("default-features = false"), "{fixed}");
        assert!(fixed.contains("features = [\"lang-rust\"]"), "{fixed}");
    }

    #[test]
    fn an_inline_table_keeps_its_other_fields() {
        let fixed = fixed(
            "[dependencies]\nlumis-core = { path = \"../lumis-core\", version = \"2\", features = [\"lang-rust\"] }\n",
        );

        assert!(fixed.contains("version = \"2.3.0\""), "{fixed}");
        assert!(fixed.contains("path = \"../lumis-core\""), "{fixed}");
        assert!(fixed.contains("features = [\"lang-rust\"]"), "{fixed}");
    }

    #[test]
    fn a_string_requirement_stays_a_string() {
        assert_eq!(
            fixed("[dependencies]\nlumis-core = \"2\"\n"),
            "[dependencies]\nlumis-core = \"2.3.0\"\n"
        );
    }

    #[test]
    fn a_workspace_inherited_dependency_is_left_alone() {
        let manifest = "[dependencies]\nlumis-core = { workspace = true }\n";
        assert_eq!(fixed(manifest), manifest);

        let mut document: toml_edit::DocumentMut = manifest.parse().expect("valid manifest");
        let mut inherited = Vec::new();
        for_each_dependency(document.as_table_mut(), &mut |_, spec| {
            inherited.push(inherits_from_workspace(spec));
        });
        assert_eq!(inherited, vec![true]);
    }

    fn publishable(manifest: &str) -> bool {
        is_publishable(&manifest.parse().expect("valid manifest"))
    }

    #[test]
    fn a_manifest_without_a_package_is_not_publishable() {
        assert!(!publishable("[workspace]\nmembers = []\n"));
    }

    #[test]
    fn publish_decides_whether_a_requirement_can_reach_a_registry() {
        assert!(publishable("[package]\nname = \"example\"\n"));
        assert!(publishable(
            "[package]\nname = \"example\"\npublish = true\n"
        ));
        assert!(!publishable(
            "[package]\nname = \"example\"\npublish = false\n"
        ));
        // cargo rejects `cargo publish` for both of these.
        assert!(!publishable(
            "[package]\nname = \"example\"\npublish = []\n"
        ));
        assert!(publishable(
            "[package]\nname = \"example\"\npublish = [\"crates-io\"]\n"
        ));
    }

    #[test]
    fn discovery_that_finds_nothing_fails() {
        let (reported, checked) = crate_dep_problems(&BTreeMap::new(), &[], 0);
        assert_eq!(checked, 0);
        assert!(
            reported
                .iter()
                .any(|problem| problem.contains("expected at least")),
            "{reported:?}"
        );
        for name in RELEASED_CRATES {
            assert!(
                reported.iter().any(|problem| problem.starts_with(name)),
                "{name} not reported missing: {reported:?}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH: &str = "a4ee9c153c66f995d081895c3e57a0a4eed81ebb91f2320a0810bbf51c3598dc";

    fn packument(versions: Value) -> Value {
        json!({ "versions": versions })
    }

    fn marker(hash: &str, series: &str, format: u32) -> Value {
        json!({
            "lumis": {
                "definitionHash": hash,
                "treeSitter": series,
                "formatVersion": format,
            }
        })
    }

    #[test]
    fn a_published_version_carrying_the_definition_needs_no_republish() {
        let packument = packument(json!({
            "0.25.0": marker(HASH, "0.25", PACKAGE_FORMAT_VERSION),
            "0.26.0": json!({ "lumis": { "rev": "18b0515", "treeSitter": "0.26" } }),
            "0.26.1": marker(HASH, "0.26", PACKAGE_FORMAT_VERSION),
        }));

        assert!(published_for_definition(&packument, HASH, "0.26"));
    }

    #[test]
    fn a_definition_published_only_outside_the_series_still_needs_publishing() {
        let packument = packument(json!({
            "0.25.0": marker(HASH, "0.25", PACKAGE_FORMAT_VERSION),
        }));

        assert!(!published_for_definition(&packument, HASH, "0.26"));
    }

    #[test]
    fn a_neighbouring_series_is_not_a_prefix_match() {
        let packument = packument(json!({
            "0.260.0": marker(HASH, "0.26", PACKAGE_FORMAT_VERSION),
        }));

        assert!(!published_for_definition(&packument, HASH, "0.26"));
    }

    #[test]
    fn every_stale_marker_needs_publishing() {
        for (label, manifest) in [
            ("no lumis key", json!({ "name": "@lumis-sh/wasm-rust" })),
            (
                "pre-v3 marker",
                json!({ "lumis": { "rev": "18b0515", "treeSitter": "0.26" } }),
            ),
            (
                "older format",
                marker(HASH, "0.26", PACKAGE_FORMAT_VERSION - 1),
            ),
            (
                "different definition",
                marker("0000000000000000", "0.26", PACKAGE_FORMAT_VERSION),
            ),
            (
                "series disagrees with the version",
                marker(HASH, "0.25", PACKAGE_FORMAT_VERSION),
            ),
        ] {
            let packument = packument(json!({ "0.26.0": manifest }));
            assert!(
                !published_for_definition(&packument, HASH, "0.26"),
                "{label} must not count as published"
            );
        }
    }

    #[test]
    fn a_packument_without_versions_needs_publishing() {
        assert!(!published_for_definition(
            &json!({ "dist-tags": { "latest": "0.26.1" } }),
            HASH,
            "0.26"
        ));
    }

    #[test]
    fn every_build_input_changes_the_build_id() {
        let baseline = wasm_build_id("git://g", "rev1", Some("sub"), false, "ts\nem");

        for (label, other) in [
            (
                "git",
                wasm_build_id("git://other", "rev1", Some("sub"), false, "ts\nem"),
            ),
            (
                "rev",
                wasm_build_id("git://g", "rev2", Some("sub"), false, "ts\nem"),
            ),
            (
                "location",
                wasm_build_id("git://g", "rev1", None, false, "ts\nem"),
            ),
            (
                "generate",
                wasm_build_id("git://g", "rev1", Some("sub"), true, "ts\nem"),
            ),
            (
                "toolchain",
                wasm_build_id("git://g", "rev1", Some("sub"), false, "ts\nem2"),
            ),
        ] {
            assert_ne!(baseline, other, "{label} must invalidate a cached parser");
        }

        assert_eq!(
            baseline,
            wasm_build_id("git://g", "rev1", Some("sub"), false, "ts\nem"),
            "the same inputs must reuse a cached parser"
        );
    }

    #[test]
    fn a_parser_is_reused_only_when_it_is_intact_and_current() {
        let dir = PathBuf::from(tmpdir().unwrap());
        let wasm = dir.join("tree-sitter-x.wasm");
        let id_file = dir.join("tree-sitter-x.build-id");
        let seed = |bytes: &[u8], id: &str| {
            fs::write(&wasm, bytes).unwrap();
            fs::write(&id_file, id).unwrap();
        };

        seed(b"\0asm\x01\0\0\0", "id-1");
        assert!(
            cached_parser_is_current(&wasm, &id_file, "id-1"),
            "an intact parser recorded against these inputs is reusable"
        );
        assert!(
            !cached_parser_is_current(&wasm, &id_file, "id-2"),
            "a different build id must not be reused"
        );

        seed(b"corrupt", "id-1");
        assert!(
            !cached_parser_is_current(&wasm, &id_file, "id-1"),
            "bytes that are not a wasm module must not be reused"
        );

        seed(b"\0asm\x01\0\0\0", "id-1");
        fs::remove_file(&id_file).unwrap();
        assert!(
            !cached_parser_is_current(&wasm, &id_file, "id-1"),
            "a parser with no recorded build id must not be reused"
        );

        fs::write(&id_file, "id-1").unwrap();
        fs::remove_file(&wasm).unwrap();
        assert!(
            !cached_parser_is_current(&wasm, &id_file, "id-1"),
            "a recorded build id without a parser must not be reused"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    fn query_sources() -> BTreeMap<String, QueryInfo> {
        ["default", "python", "sql"]
            .into_iter()
            .map(|name| {
                (
                    name.to_string(),
                    QueryInfo {
                        git: format!("https://example.invalid/{name}.git"),
                        rev: "0000000".to_string(),
                        path: None,
                    },
                )
            })
            .collect()
    }

    fn selected(name: &str) -> Vec<String> {
        queries_to_upgrade(&query_sources(), name)
            .into_iter()
            .map(|(query_name, _)| query_name.clone())
            .collect()
    }

    #[test]
    fn an_unscoped_upgrade_covers_every_query_source() {
        assert_eq!(selected(""), ["default", "python", "sql"]);
    }

    #[test]
    fn a_language_with_its_own_query_source_upgrades_only_that_source() {
        assert_eq!(selected("python"), ["python"]);
    }

    /// Most languages read `queries.default`, and the weekly language update
    /// runs one job per language. Letting a scoped run reach `default` opened
    /// 115 pull requests carrying the same shared revision bump.
    #[test]
    fn a_language_backed_by_the_default_source_upgrades_nothing() {
        assert!(
            selected("c").is_empty(),
            "`c` reads the default query source"
        );
        assert!(
            selected("rust").is_empty(),
            "`rust` reads the default query source"
        );
    }

    #[test]
    fn the_default_source_is_still_upgradable_by_name() {
        assert_eq!(selected("default"), ["default"]);
    }

    fn published(hash: &str) -> Value {
        serde_json::json!({
            "definitionHash": hash,
            "treeSitter": "0.26",
            "formatVersion": 3,
        })
    }

    #[test]
    fn a_matching_package_needs_no_republish() {
        assert!(definition_matches(&published("abc"), "abc", "0.26"));
    }

    /// A stub compiler that exits with `code`, so the test decides the status
    /// `build_repo_wasm_with` has to react to.
    #[cfg(unix)]
    fn stub_tree_sitter(dir: &Path, code: i32) -> PathBuf {
        use std::os::unix::fs::PermissionsExt as _;

        let path = dir.join(format!("tree-sitter-stub-{code}"));
        fs::write(
            &path,
            format!("#!/bin/sh\necho 'stub compiler' >&2\nexit {code}\n"),
        )
        .unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    /// `execve` reports `ETXTBSY` for a file any process still holds open for
    /// writing. Nothing here keeps a stub open, but `cargo test` runs the rest
    /// of the suite on sibling threads, and a child one of them forks inherits
    /// whatever descriptors are open at that instant — including the one
    /// `stub_tree_sitter` writes through. `O_CLOEXEC` only closes it once that
    /// child reaches its own `execve`, so the stub is unrunnable until then and
    /// retrying is what clears it. Distinct stub paths do not: the race is in
    /// writing a file and then executing it, not in reusing the name.
    #[cfg(unix)]
    fn build_with_stub(stub: &Path, dir: &Path, log: &Path) -> Result<()> {
        for _ in 0..100 {
            let result = build_repo_wasm_with(
                stub.to_str().unwrap(),
                dir.to_str().unwrap(),
                &dir.join("out.wasm"),
                log,
            );
            let busy = result
                .as_ref()
                .err()
                .and_then(|error| error.downcast_ref::<std::io::Error>())
                .is_some_and(|error| error.kind() == std::io::ErrorKind::ExecutableFileBusy);
            if !busy {
                return result;
            }
            thread::sleep(Duration::from_millis(10));
        }
        panic!("{} stayed busy for a second", stub.display());
    }

    // This used to pass through `sh -c ... | tee`, whose exit status is `tee`'s.
    // A failed build reported success and the caller printed the path of a wasm
    // that was never written.
    #[cfg(unix)]
    #[test]
    fn a_build_reports_the_compiler_exit_status() {
        let dir = std::env::temp_dir().join(format!("lumis-build-status-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let log = dir.join("build.log");
        let failed_stub = stub_tree_sitter(&dir, 1);
        let succeeded_stub = stub_tree_sitter(&dir, 0);

        let failed = build_with_stub(&failed_stub, &dir, &log);
        assert!(
            failed.is_err(),
            "a nonzero exit status must be an error: {failed:?}"
        );
        assert!(
            fs::read_to_string(&log).unwrap().contains("stub compiler"),
            "a failed build still has to leave its output in the log"
        );

        let succeeded = build_with_stub(&succeeded_stub, &dir, &log);

        let _ = fs::remove_dir_all(&dir);
        assert!(
            succeeded.is_ok(),
            "a zero exit status must succeed: {succeeded:?}"
        );
    }

    #[test]
    fn every_field_must_agree() {
        assert!(!definition_matches(&published("abc"), "def", "0.26"));
        assert!(!definition_matches(&published("abc"), "abc", "0.27"));
        let mut old_format = published("abc");
        old_format["formatVersion"] = serde_json::json!(2);
        assert!(!definition_matches(&old_format, "abc", "0.26"));
    }

    #[test]
    fn a_package_predating_the_format_never_matches() {
        // What npm actually returns for the published catalog today.
        let legacy = serde_json::json!({
            "language": "json",
            "parser": "tree-sitter-json",
            "treeSitter": "0.26",
        });
        assert!(!definition_matches(&legacy, "abc", "0.26"));
    }
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn run_test_git(repo: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .output()
            .expect("test git command should run");

        assert!(
            output.status.success(),
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn commit_test_revision(repo: &Path, contents: &str) -> String {
        fs::write(repo.join("parser.c"), contents).expect("test revision should be written");
        run_test_git(repo, &["add", "parser.c"]);
        run_test_git(repo, &["commit", "--quiet", "-m", contents.trim()]);
        run_test_git(repo, &["rev-parse", "HEAD"])
    }

    #[test]
    fn release_version_from_tag_accepts_short_stable_tags() {
        let expected = semver::Version::new(0, 20, 0);

        assert_eq!(release_version_from_tag("0.20"), Some(expected.clone()));
        assert_eq!(release_version_from_tag("v0.20"), Some(expected));
        assert_eq!(
            release_version_from_tag("v0.20.1"),
            Some(semver::Version::new(0, 20, 1))
        );
        assert_eq!(release_version_from_tag("0.20-beta"), None);
    }

    fn unique_test_root() -> std::path::PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("lumis-dev-query-test-{unique}-{counter}"))
    }

    #[test]
    fn git_ancestry_detects_backward_parser_updates() {
        let root = unique_test_root();
        let source = root.join("source");
        fs::create_dir_all(&source).expect("test repository should be created");

        run_test_git(&source, &["init", "--quiet"]);
        run_test_git(&source, &["config", "commit.gpgsign", "false"]);
        run_test_git(&source, &["config", "user.name", "Lumis"]);
        run_test_git(&source, &["config", "user.email", "dev@lumis.sh"]);
        let first = commit_test_revision(&source, "first\n");
        let second = commit_test_revision(&source, "second\n");
        let source_url = source.to_string_lossy();

        assert!(git_is_ancestor(&source_url, &first, &second)
            .expect("forward ancestry check should succeed"));
        assert!(!git_is_ancestor(&source_url, &second, &first)
            .expect("backward ancestry check should succeed"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_and_preprocess_uses_append_without_upstream() {
        let root = unique_test_root();
        let upstream = root.join("upstream");
        let override_root = root.join("override");
        let append_root = root.join("append");
        let append_lang_dir = append_root.join("demo");

        fs::create_dir_all(&append_lang_dir).expect("append dir should be created");
        fs::write(
            append_lang_dir.join("highlights.scm"),
            "((comment) @comment)\n",
        )
        .expect("append query should be written");

        let mut seen = HashSet::new();
        let content = resolve_and_preprocess(
            upstream.to_str().expect("upstream path should be valid"),
            override_root
                .to_str()
                .expect("override path should be valid"),
            append_root.to_str().expect("append path should be valid"),
            "demo",
            "highlights",
            &mut seen,
        );

        assert_eq!(content, "((comment) @comment)\n");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_and_preprocess_override_replaces_upstream_query() {
        let root = unique_test_root();
        let upstream = root.join("upstream");
        let upstream_dir = upstream.join("demo");
        let override_root = root.join("override");
        let append_root = root.join("append");
        let override_lang_dir = override_root.join("demo");

        fs::create_dir_all(&upstream_dir).expect("upstream dir should be created");
        fs::create_dir_all(&override_lang_dir).expect("override dir should be created");
        fs::write(
            upstream_dir.join("highlights.scm"),
            "(bad_node_name) @error\n",
        )
        .expect("upstream query should be written");
        fs::write(
            override_lang_dir.join("highlights.scm"),
            "((comment) @comment)\n",
        )
        .expect("override query should be written");

        let mut seen = HashSet::new();
        let content = resolve_and_preprocess(
            upstream.to_str().expect("upstream path should be valid"),
            override_root
                .to_str()
                .expect("override path should be valid"),
            append_root.to_str().expect("append path should be valid"),
            "demo",
            "highlights",
            &mut seen,
        );

        assert_eq!(content, "((comment) @comment)\n");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn resolve_and_preprocess_appends_after_override() {
        let root = unique_test_root();
        let upstream = root.join("upstream");
        let upstream_dir = upstream.join("demo");
        let override_root = root.join("override");
        let override_lang_dir = override_root.join("demo");
        let append_root = root.join("append");
        let append_lang_dir = append_root.join("demo");

        fs::create_dir_all(&upstream_dir).expect("upstream dir should be created");
        fs::create_dir_all(&override_lang_dir).expect("override dir should be created");
        fs::create_dir_all(&append_lang_dir).expect("append dir should be created");
        fs::write(
            upstream_dir.join("highlights.scm"),
            "(bad_node_name) @error\n",
        )
        .expect("upstream query should be written");
        fs::write(
            override_lang_dir.join("highlights.scm"),
            "((comment) @comment)\n",
        )
        .expect("override query should be written");
        fs::write(
            append_lang_dir.join("highlights.scm"),
            "((string) @string)\n",
        )
        .expect("append query should be written");

        let mut seen = HashSet::new();
        let content = resolve_and_preprocess(
            upstream.to_str().expect("upstream path should be valid"),
            override_root
                .to_str()
                .expect("override path should be valid"),
            append_root.to_str().expect("append path should be valid"),
            "demo",
            "highlights",
            &mut seen,
        );

        assert_eq!(content, "((comment) @comment)\n\n((string) @string)\n");

        let _ = fs::remove_dir_all(root);
    }

    /// The working directory is process-wide, so tests that read relative query
    /// paths have to take turns. Without this they race each other and one of
    /// them intermittently sees the other's empty temporary tree.
    static CWD: Mutex<()> = Mutex::new(());

    fn in_directory<T>(root: &Path, body: impl FnOnce() -> T + std::panic::UnwindSafe) -> T {
        let guard = CWD
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let cwd = std::env::current_dir().expect("cwd should be available");
        std::env::set_current_dir(root).expect("should switch to temp dir");

        let result = std::panic::catch_unwind(body);

        std::env::set_current_dir(cwd).expect("should restore cwd");
        drop(guard);
        result.unwrap()
    }

    #[test]
    fn local_override_query_detection_checks_any_query_file() {
        let root = unique_test_root();
        let override_lang_dir = root.join("queries/override/demo");

        fs::create_dir_all(&override_lang_dir).expect("override dir should be created");
        fs::write(
            override_lang_dir.join("locals.scm"),
            "(node) @local.scope\n",
        )
        .expect("override query should be written");

        in_directory(&root, || {
            assert!(has_local_override_query("demo"));
            assert!(!has_local_override_query("missing"));
        });

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn query_source_name_prefers_configured_query_name() {
        let with_query_name: ParserInfo =
            toml::from_str("query_name = \"nu\"").expect("parser info should parse");
        let without_query_name: ParserInfo = toml::from_str("").expect("parser info should parse");

        assert_eq!(query_source_name("nushell", &with_query_name), "nu");
        assert_eq!(query_source_name("nushell", &without_query_name), "nushell");

        let queries = BTreeMap::from([
            (
                "default".to_string(),
                QueryInfo {
                    git: "https://github.com/nvim-treesitter/nvim-treesitter.git".to_string(),
                    rev: "default-rev".to_string(),
                    path: None,
                },
            ),
            (
                "nu".to_string(),
                QueryInfo {
                    git: "https://github.com/nushell/tree-sitter-nu.git".to_string(),
                    rev: "nu-rev".to_string(),
                    path: None,
                },
            ),
        ]);

        assert_eq!(
            resolve_query_source(&queries, query_source_name("nushell", &with_query_name)).rev,
            "nu-rev"
        );
        assert_eq!(
            resolve_query_source(&queries, query_source_name("nushell", &without_query_name)).rev,
            "default-rev"
        );
    }

    #[test]
    fn query_names_includes_override_only_languages() {
        let root = unique_test_root();
        let override_lang_dir = root.join("queries/override/demo");

        fs::create_dir_all(&override_lang_dir).expect("override dir should be created");
        fs::write(
            override_lang_dir.join("highlights.scm"),
            "((comment) @comment)\n",
        )
        .expect("override query should be written");

        in_directory(&root, || {
            assert_eq!(
                query_names().expect("query names should load"),
                vec!["demo"]
            );
        });

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn strip_set_capture_patterns_handles_escaped_backslashes() {
        let content = r#"((hard_line_break
  "\\" @conceal)
  (#set! conceal ""))

((inline_link
  (link_destination) @_url) @_label
  (#set! @_label url @_url))

(comment) @comment
"#;

        let stripped = strip_set_capture_patterns(content);

        assert!(stripped.contains("hard_line_break"));
        assert!(!stripped.contains("inline_link"));
        assert!(stripped.contains("(comment) @comment"));
    }

    #[test]
    fn apply_text_replacements_uses_published_swift_nil_node() {
        let query = "(nil_literal) @constant.builtin";

        assert_eq!(
            apply_text_replacements(query, "swift"),
            "\"nil\" @constant.builtin"
        );
        assert_eq!(apply_text_replacements(query, "nim"), query);
    }

    #[test]
    fn language_catalog_preserves_source_order_and_package_overrides() {
        let parsers = BTreeMap::from([
            (
                "alpha".to_string(),
                ParserInfo {
                    aliases: vec!["a".to_string()],
                    ..ParserInfo::default()
                },
            ),
            (
                "zeta".to_string(),
                ParserInfo {
                    wasm_name: Some("tree-sitter-shared".to_string()),
                    ..ParserInfo::default()
                },
            ),
        ]);
        let order = vec!["zeta".to_string(), "alpha".to_string()];
        let bundles = BTreeMap::from([
            (
                "web".to_string(),
                BundleInfo {
                    parsers: BundleParsers::List(vec!["alpha".to_string()]),
                },
            ),
            (
                "full".to_string(),
                BundleInfo {
                    parsers: BundleParsers::All("all".to_string()),
                },
            ),
        ]);

        let catalog = render_language_catalog(&parsers, &order, &bundles, "0.26")
            .expect("catalog should be generated");

        assert!(catalog.find("\"zeta\"").unwrap() < catalog.find("\"alpha\"").unwrap());
        assert!(catalog.contains("package_name: \"@lumis-sh/wasm-shared\""));
        assert!(catalog.contains("aliases: [\"a\"]"));
        // `parsers = "all"` expands to the catalog, in the same order.
        assert!(catalog.contains("\"full\" => [\"zeta\", \"alpha\"]"));
        assert!(catalog.contains("\"web\" => [\"alpha\"]"));
        assert!(catalog.contains("package_version_range: \"0.26\""));
        assert!(!catalog.contains("version:"));
    }

    #[test]
    fn tree_sitter_series_uses_semver_parsing() {
        assert_eq!(tree_sitter_series("0.26").unwrap(), "0.26");
        assert_eq!(tree_sitter_series("v0.26.11").unwrap(), "0.26");
        assert!(tree_sitter_series("latest").is_err());
        assert!(tree_sitter_series("not-a-version-0.26").is_err());
    }

    // Bundle membership is a cross-runtime promise: `@lumis-sh/wasm-bundle-web` and
    // `Lumis.Languages.load(:bundle_web)` must name the same languages. Every runtime
    // reads it from this one table, so the shipped catalog going stale is what would
    // break that, and `gen-catalog --check` only runs where a Rust toolchain does.
    #[test]
    fn shipped_bundles_match_languages_toml() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../languages.toml");
        let text = fs::read_to_string(&path).expect("languages.toml should be readable");
        let toml: LanguagesToml = toml::from_str(&text).expect("languages.toml should parse");
        let document: toml_edit::DocumentMut = text.parse().expect("languages.toml should parse");
        let order = document["parsers"]
            .as_table()
            .expect("languages.toml must contain a parsers table")
            .iter()
            .map(|(id, _)| id.to_string())
            .collect::<Vec<_>>();

        let expected = toml
            .bundles
            .iter()
            .map(|(name, bundle)| {
                let members = match &bundle.parsers {
                    BundleParsers::All(_) => order.clone(),
                    BundleParsers::List(list) => list.clone(),
                };
                (name.as_str(), members)
            })
            .collect::<BTreeMap<_, _>>();

        let shipped = lumis_wasm_runtime::catalog::BUNDLES
            .iter()
            .map(|(name, members)| {
                (
                    *name,
                    members
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        assert_eq!(shipped, expected, "run `mise run langs-gen-catalog`");
        assert!(
            shipped.contains_key("full"),
            "bundles should include `full`"
        );
        assert!(
            shipped["full"].len() > 100,
            "`full` should be every language"
        );
    }

    #[test]
    fn revision_only_theme_updates_are_ignored() {
        let previous = json!({
            "name": "demo",
            "appearance": "dark",
            "revision": "old",
            "highlights": {
                "normal": { "fg": "#ffffff", "bg": "#000000" }
            }
        });
        let current = json!({
            "name": "demo",
            "appearance": "dark",
            "revision": "new",
            "highlights": {
                "normal": { "fg": "#ffffff", "bg": "#000000" }
            }
        });

        assert!(is_revision_only_theme_update(&previous, &current));
    }

    #[test]
    fn actual_theme_updates_are_kept() {
        let previous = json!({
            "name": "demo",
            "appearance": "dark",
            "revision": "old",
            "highlights": {
                "normal": { "fg": "#ffffff", "bg": "#000000" }
            }
        });
        let current = json!({
            "name": "demo",
            "appearance": "dark",
            "revision": "new",
            "highlights": {
                "normal": { "fg": "#eeeeee", "bg": "#000000" }
            }
        });

        assert!(!is_revision_only_theme_update(&previous, &current));
        assert!(!is_revision_only_theme_update(&previous, &previous));
    }
}
