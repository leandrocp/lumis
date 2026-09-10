mod config;
mod formatter_options;
mod gen_theme;
mod registry;

use anyhow::Result;
use clap::{ArgAction, CommandFactory, Parser, Subcommand, ValueEnum};
use formatter_options::{
    OPTSET_GLOBAL, OPTSET_HTML, OPTSET_MULTI_THEME, OPTSET_STYLED, OPTSET_TERMINAL,
};
use lumis_core::events::HighlightEvent as CoreHighlightEvent;
use lumis_core::formatter::ansi::hex_to_rgb;
use lumis_core::formatter::Formatter as CoreFormatter;
use lumis_core::formatter::TerminalBackground;
use lumis_core::languages::Language;
use lumis_wasm_runtime::tree_sitter_highlight::ParsedLayer;
use lumis_wasm_runtime::{HighlightOptions, HighlightOutput};
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Write as _;
use std::fs;
use std::io::{IsTerminal, Read as _};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use terminal_size::{terminal_size, Width};
use tree_sitter::Node;

type HighlightEvent = CoreHighlightEvent<'static>;

#[derive(Parser)]
#[command(
    version,
    propagate_version = true,
    about = "Syntax Highlighter powered by Tree-sitter and Neovim themes",
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Where to look for WASM parsers and theme JSON files
    #[arg(short = 'd', long, env("LUMIS_DATA_DIR"), global = true, help_heading = OPTSET_GLOBAL)]
    data_dir: Option<PathBuf>,

    /// Path to the configuration file
    #[arg(long, env("LUMIS_CONFIG"), global = true, help_heading = OPTSET_GLOBAL)]
    config: Option<PathBuf>,

    /// Show extra output (downloads, cache hits, etc.)
    #[arg(short = 'v', long, global = true, help_heading = OPTSET_GLOBAL)]
    verbose: bool,

    /// Print help
    #[arg(short = 'h', long, global = true, action = ArgAction::Help, help = "Print help", help_heading = OPTSET_GLOBAL)]
    help: Option<bool>,

    /// Print version
    #[arg(short = 'V', long, global = true, action = ArgAction::Version, help = "Print version", help_heading = OPTSET_GLOBAL)]
    version: Option<bool>,
}

#[derive(Subcommand)]
enum Commands {
    /// Highlight source code from a file or stdin
    #[command(
        after_help = "Examples:\n  lumis highlight main.rs\n  lumis highlight -l javascript main.txt\n  lumis highlight -f html-inline -t dracula main.rs\n  lumis highlight -f html-inline -H 1,3-5 --pre-class code lib.rs\n  lumis highlight -f html-multi-themes --themes light:github_light --themes dark:github_dark main.rs\n  lumis highlight -b theme -w 120 main.rs\n  lumis highlight --background '#282a36' --width 120 main.rs\n  cat main.rs | lumis highlight -l rust\n  echo 'fn main() {}' | lumis highlight -l rust\n\nEach formatter accepts a different set of options; run `lumis formatters show <name>` to see one."
    )]
    Highlight(Box<HighlightArgs>),

    /// Dump Tree-sitter parsing and highlighting output
    Dump {
        #[command(subcommand)]
        command: DumpCommands,
    },

    /// Manage languages
    Languages {
        #[command(subcommand)]
        command: LanguagesCommands,
    },

    /// Manage output formatters
    Formatters {
        #[command(subcommand)]
        command: FormattersCommands,
    },

    /// Manage themes
    Themes {
        #[command(subcommand)]
        command: ThemesCommands,
    },
}

#[derive(clap::Args)]
struct HighlightArgs {
    /// File to highlight (reads from stdin if omitted)
    path: Option<String>,

    /// Language id (e.g. rust, javascript, elixir)
    #[arg(short = 'l', long)]
    language: Option<String>,

    /// Output format [default: terminal]
    #[arg(short = 'f', long)]
    formatter: Option<Formatter>,

    /// Theme name, e.g. dracula, `github_dark`, or auto
    #[arg(short = 't', long)]
    theme: Option<String>,

    /// Render nested brackets using rainbow bracket scopes
    #[arg(long)]
    rainbow_brackets: bool,

    #[command(flatten)]
    terminal: TerminalArgs,

    #[command(flatten)]
    html: HtmlArgs,

    #[command(flatten)]
    styled: StyledArgs,

    #[command(flatten)]
    multi_theme: MultiThemeArgs,
}

#[derive(clap::Args)]
#[command(next_help_heading = OPTSET_TERMINAL)]
struct TerminalArgs {
    /// Fallback background: use `theme`, a hex color, or omit it to inherit the output background
    #[arg(short = 'b', long = "background")]
    background: Option<String>,

    /// Render width for background padding. Use a number or 'auto'.
    #[arg(short = 'w', long)]
    width: Option<String>,
}

#[derive(clap::Args)]
#[command(next_help_heading = OPTSET_HTML)]
struct HtmlArgs {
    /// CSS class appended to the wrapping <pre> tag
    #[arg(long)]
    pre_class: Option<String>,

    /// Opening tag wrapped around the output, e.g. '<figure>'
    #[arg(long, requires = "header_close")]
    header_open: Option<String>,

    /// Closing tag wrapped around the output, e.g. '</figure>'
    #[arg(long, requires = "header_open")]
    header_close: Option<String>,

    /// Lines to highlight, e.g. "1,3-5,10"
    #[arg(short = 'H', long)]
    highlight_lines: Option<String>,

    /// CSS class added to highlighted lines
    #[arg(long, requires = "highlight_lines")]
    highlight_lines_class: Option<String>,
}

#[derive(clap::Args)]
#[command(next_help_heading = OPTSET_STYLED)]
struct StyledArgs {
    /// Apply italic styles from the theme
    #[arg(long)]
    italic: bool,

    /// Add data-highlight attributes naming each scope
    #[arg(long)]
    include_highlights: bool,

    /// Style for highlighted lines: `theme`, `none`, or raw CSS [default: theme]
    #[arg(long, requires = "highlight_lines")]
    highlight_lines_style: Option<String>,
}

#[derive(clap::Args)]
#[command(next_help_heading = OPTSET_MULTI_THEME)]
struct MultiThemeArgs {
    /// Theme pair as `name:theme_id`, can be repeated
    #[arg(long)]
    themes: Vec<String>,

    /// Which --themes entry gets inline styles
    #[arg(long)]
    default_theme: Option<String>,

    /// Prefix for CSS custom properties [default: --lumis]
    ///
    /// A value beginning with `-` needs the `=` form, as in
    /// `--css-variable-prefix=--shiki`.
    #[arg(long, allow_hyphen_values = true)]
    css_variable_prefix: Option<String>,
}

const DEFAULT_CSS_VARIABLE_PREFIX: &str = "--lumis";

impl HighlightArgs {
    fn formatter(&self) -> Formatter {
        self.formatter.unwrap_or_default()
    }

    /// Every group holding a flag the user passed that the chosen formatter
    /// would ignore, so one run reports the whole command rather than the first
    /// thing wrong with it.
    fn rejected(&self) -> Vec<(&'static formatter_options::OptionGroup, Vec<&'static str>)> {
        let chosen = self.formatter();
        formatter_options::OPTION_GROUPS
            .iter()
            .filter(|group| !group.accepts(chosen))
            .filter_map(|group| {
                let used: Vec<&'static str> = group
                    .flags
                    .iter()
                    .copied()
                    .filter(|flag| self.is_set(flag))
                    .collect();
                (!used.is_empty()).then_some((group, used))
            })
            .collect()
    }

    fn is_set(&self, flag: &str) -> bool {
        match flag {
            "--theme" => self.theme.is_some(),
            "--background" => self.terminal.background.is_some(),
            "--width" => self.terminal.width.is_some(),
            "--pre-class" => self.html.pre_class.is_some(),
            "--header-open" => self.html.header_open.is_some(),
            "--header-close" => self.html.header_close.is_some(),
            "--highlight-lines" => self.html.highlight_lines.is_some(),
            "--highlight-lines-class" => self.html.highlight_lines_class.is_some(),
            "--italic" => self.styled.italic,
            "--include-highlights" => self.styled.include_highlights,
            "--highlight-lines-style" => self.styled.highlight_lines_style.is_some(),
            "--themes" => !self.multi_theme.themes.is_empty(),
            "--default-theme" => self.multi_theme.default_theme.is_some(),
            "--css-variable-prefix" => self.multi_theme.css_variable_prefix.is_some(),
            other => unreachable!("OPTION_GROUPS names an unknown flag `{other}`"),
        }
    }
}

#[derive(Subcommand)]
enum DumpCommands {
    /// Print Tree-sitter syntax trees
    Tree {
        /// File to parse (reads from stdin if omitted)
        path: Option<String>,

        /// Language id (e.g. rust, javascript, elixir)
        #[arg(short = 'l', long)]
        language: Option<String>,

        /// Output format
        #[arg(long, value_enum, default_value_t)]
        format: TreeFormat,

        /// Include source text, truncated to 80 characters unless a limit or "full" is provided
        #[arg(
            long,
            num_args = 0..=1,
            require_equals = true,
            default_missing_value = "80",
            value_name = "LIMIT"
        )]
        text: Option<TreeText>,

        /// Include resolved highlight-query results
        #[arg(long)]
        highlights: bool,

        /// Include injected language trees
        #[arg(long)]
        injections: bool,
    },

    /// Print raw highlight events as JSON
    Events {
        /// File to highlight (reads from stdin if omitted)
        path: Option<String>,

        /// Language id (e.g. rust, javascript, elixir)
        #[arg(short = 'l', long)]
        language: Option<String>,
    },
}

#[derive(Subcommand)]
enum LanguagesCommands {
    /// Print supported languages and their file patterns
    List,

    /// Print what the catalog knows about one language
    #[command(after_help = "Examples:\n  lumis languages show elixir\n  lumis languages show js")]
    Show {
        /// Language id or alias, e.g. elixir, js
        language: String,
    },

    /// Download and compile parsers so later runs skip both
    #[command(
        after_help = "Examples:\n  lumis languages cache rust javascript\n  lumis languages cache bundle-web\n  lumis languages cache --all\n  lumis languages cache rust --force\n  lumis --data-dir /app/lumis languages cache rust"
    )]
    Cache {
        /// Language names, or a bundle such as bundle-web (e.g. rust javascript elixir)
        languages: Vec<String>,

        /// Cache every language in the catalog
        #[arg(long)]
        all: bool,

        /// Resolve compatible packages again and replace valid cached parsers
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
enum FormattersCommands {
    /// Print available formatters
    List,

    /// Print the options a formatter accepts
    #[command(
        after_help = "Examples:\n  lumis formatters show terminal\n  lumis formatters show html-multi-themes"
    )]
    Show {
        /// Formatter name, e.g. terminal, html-inline
        formatter: Formatter,
    },
}

#[derive(Subcommand)]
enum ThemesCommands {
    /// Print available themes (built-in and from --data-dir)
    List,

    /// Print one theme's appearance and colors
    #[command(
        after_help = "Examples:\n  lumis themes show dracula\n  lumis themes show github_light"
    )]
    Show {
        /// Theme name, e.g. dracula
        theme: String,
    },

    /// Extract a theme from a Neovim colorscheme Git repo
    #[command(
        after_help = "Examples:\n  lumis themes generate -u https://github.com/catppuccin/nvim -c catppuccin-mocha\n  lumis themes generate -u https://github.com/folke/tokyonight.nvim -c tokyonight-night -o tokyonight.json"
    )]
    Generate {
        /// Git repository URL
        #[arg(short = 'u', long)]
        url: String,

        /// Colorscheme name to activate, e.g. catppuccin-mocha
        #[arg(short = 'c', long)]
        colorscheme: String,

        /// Lua code to run before loading the colorscheme
        #[arg(short = 's', long)]
        setup: Option<String>,

        /// Write JSON to this path instead of stdout
        #[arg(short = 'o', long)]
        output: Option<String>,

        /// light or dark [default: dark]
        #[arg(short = 'a', long)]
        appearance: Option<String>,
    },
}

#[derive(Clone, Copy, Default, ValueEnum)]
enum TreeFormat {
    /// Branch lines with language and range metadata
    #[default]
    Lines,
    /// Canonical Tree-sitter S-expressions
    Sexp,
}

#[derive(Clone, Copy)]
enum TreeText {
    Preview(usize),
    Full,
}

impl std::str::FromStr for TreeText {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        if value.eq_ignore_ascii_case("full") {
            return Ok(Self::Full);
        }

        let limit = value
            .parse::<usize>()
            .map_err(|_| "text limit must be a positive integer or 'full'".to_string())?;
        if limit == 0 {
            return Err("text limit must be greater than zero".to_string());
        }
        Ok(Self::Preview(limit))
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
enum Formatter {
    /// HTML with inline style attributes
    HtmlInline,
    /// HTML with CSS class names (pair with a theme stylesheet)
    HtmlLinked,
    /// HTML with CSS custom properties, one set per theme
    HtmlMultiThemes,
    /// ANSI escape codes
    #[default]
    Terminal,
    /// `BBCode` using highlight scope names as tags
    BbcodeScoped,
}

impl Formatter {
    const ALL: [Self; 5] = [
        Self::HtmlInline,
        Self::HtmlLinked,
        Self::HtmlMultiThemes,
        Self::Terminal,
        Self::BbcodeScoped,
    ];

    fn slug(self) -> &'static str {
        match self {
            Self::HtmlInline => "html-inline",
            Self::HtmlMultiThemes => "html-multi-themes",
            Self::HtmlLinked => "html-linked",
            Self::Terminal => "terminal",
            Self::BbcodeScoped => "bbcode-scoped",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::HtmlInline => "HTML with inline style attributes",
            Self::HtmlMultiThemes => "HTML with CSS custom properties, one set per theme",
            Self::HtmlLinked => "HTML with CSS class names (pair with a theme stylesheet)",
            Self::Terminal => "ANSI escape codes (default)",
            Self::BbcodeScoped => "BBCode using highlight scope names as tags",
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let data_dir = lumis_wasm_runtime::store::resolve_data_dir(cli.data_dir);
    lumis_wasm_runtime::set_compile_cache_dir(data_dir.clone());
    let config_path = match cli.config {
        Some(path) => path,
        None => config::default_path()?,
    };
    let verbose = cli.verbose;

    match cli.command {
        Commands::Highlight(mut args) => {
            let config = config::Config::load(&config_path)?;
            // The config theme is applied after the check, so a config file
            // cannot make `lumis highlight -f html-linked` fail.
            reject_unaccepted_options(&args);
            let reg = registry::Registry::new(data_dir)?;
            args.theme = args.theme.or(config.highlight.theme);
            do_highlight(&reg, *args, verbose)
        }
        Commands::Formatters { command } => match command {
            FormattersCommands::List => list_formatters(),
            FormattersCommands::Show { formatter } => show_formatter(formatter),
        },
        Commands::Dump { command } => {
            let reg = registry::Registry::new(data_dir)?;
            match command {
                DumpCommands::Tree {
                    path,
                    language,
                    format,
                    text,
                    highlights,
                    injections,
                } => dump_tree(&reg, path, language, format, text, highlights, injections),
                DumpCommands::Events { path, language } => dump_events(&reg, path, language),
            }
        }
        Commands::Languages { command } => match command {
            LanguagesCommands::List => list_languages(),
            LanguagesCommands::Show { language } => show_language(&language),
            LanguagesCommands::Cache {
                languages,
                all,
                force,
            } => {
                let reg = registry::Registry::new(data_dir)?;
                cache_languages(&reg, &languages, all, force, verbose)
            }
        },
        Commands::Themes { command } => match command {
            ThemesCommands::List => list_themes(&data_dir),
            ThemesCommands::Show { theme } => show_theme(&theme, &data_dir),
            ThemesCommands::Generate {
                url,
                colorscheme,
                setup,
                output,
                appearance,
            } => gen_theme::generate_theme(
                &url,
                &colorscheme,
                setup.as_deref(),
                output.as_deref(),
                appearance.as_deref(),
            ),
        },
    }
}

/// Exit with a clap-styled error when the chosen formatter would ignore a flag.
///
/// clap cannot express this: `conflicts_with` keys on another argument being
/// present, and the formatter is a *value* of `--formatter`.
fn reject_unaccepted_options(args: &HighlightArgs) {
    let rejected = args.rejected();
    if rejected.is_empty() {
        return;
    }

    let chosen = args.formatter().slug();
    let all_flags: Vec<String> = rejected
        .iter()
        .flat_map(|(_, used)| used.iter().map(|flag| format!("`{flag}`")))
        .collect();

    // Groups differ in which formatters accept them, so the applicability line
    // cannot be shared.
    let mut message = format!(
        "{} {} not accepted by the `{chosen}` formatter\n",
        all_flags.join(", "),
        if all_flags.len() == 1 { "is" } else { "are" },
    );
    for (group, _) in &rejected {
        let accepts: Vec<&str> = group.accepted_by().iter().map(|f| f.slug()).collect();
        let _ = write!(
            message,
            "\n  {} {} to: {}",
            group.label,
            if group.flags.len() == 1 {
                "applies"
            } else {
                "apply"
            },
            accepts.join(", "),
        );
    }
    let _ = write!(
        message,
        "\n  run `lumis formatters show {chosen}` to see what it accepts"
    );

    Cli::command()
        .find_subcommand_mut("highlight")
        .expect("the highlight subcommand exists")
        .clone()
        .bin_name("lumis highlight")
        .error(clap::error::ErrorKind::ArgumentConflict, message)
        .exit()
}

// Returns `Result` so every `Commands` arm has one signature to dispatch to.
#[allow(clippy::unnecessary_wraps)]
fn list_formatters() -> Result<()> {
    let width = Formatter::ALL
        .iter()
        .map(|f| f.slug().len())
        .max()
        .unwrap_or(0);
    for formatter in Formatter::ALL {
        println!(
            "{:width$}  {}",
            formatter.slug(),
            formatter.description(),
            width = width
        );
    }
    Ok(())
}

// Returns `Result` so every `Commands` arm has one signature to dispatch to.
#[allow(clippy::unnecessary_wraps)]
fn show_formatter(formatter: Formatter) -> Result<()> {
    println!("{}: {}\n", formatter.slug(), formatter.description());
    println!("Accepted options:");
    for flag in formatter_options::accepted_flags(formatter) {
        println!("  {flag}");
    }
    println!("\nRun `lumis highlight --help` for descriptions.");
    Ok(())
}

fn print_field(label: &str, values: &[&str]) {
    if !values.is_empty() {
        println!("  {label}: {}", values.join(", "));
    }
}

fn show_language(name: &str) -> Result<()> {
    let language: Language = name
        .parse()
        .map_err(|_| anyhow::anyhow!("unknown language: {name}"))?;
    let info = language.info();

    println!("{}: {}\n", info.id, info.name);
    print_field("aliases", info.aliases);
    print_field("extensions", &info.extensions);
    print_field("globs", info.globs);
    print_field("emacs modes", info.emacs_modes);
    print_field("shebangs", info.shebangs);
    Ok(())
}

fn show_theme(name: &str, data_dir: &Path) -> Result<()> {
    let theme = resolve_theme(Some(name.to_string()), Some(data_dir), false)
        .ok_or_else(|| anyhow::anyhow!("unknown theme: {name}"))?;

    println!("{}: {}\n", theme.name, theme.appearance);
    if let Some(fg) = theme.fg() {
        println!("  foreground: {fg}");
    }
    if let Some(bg) = theme.bg() {
        println!("  background: {bg}");
    }
    println!("  highlights: {}", theme.highlights.len());
    Ok(())
}

fn cache_languages(
    reg: &registry::Registry,
    languages: &[String],
    all: bool,
    force: bool,
    verbose: bool,
) -> Result<()> {
    if all && !languages.is_empty() {
        return Err(anyhow::anyhow!("pass language names or --all, not both"));
    }

    let expanded: Vec<String> = if all {
        registry::all_language_ids().map(str::to_string).collect()
    } else {
        if languages.is_empty() {
            return Err(anyhow::anyhow!(
                "specify language names, a bundle such as bundle-web, or --all"
            ));
        }
        lumis_wasm_runtime::catalog::expand_bundles(languages.iter().map(String::as_str))?
    };
    let mut seen = std::collections::HashSet::new();
    let names: Vec<String> = expanded
        .iter()
        .filter(|name| {
            let language_id = resolve_language_id(name);
            language_id != "plaintext" && seen.insert(language_id)
        })
        .map(|name| resolve_language_id(name).to_string())
        .collect();

    let mut errors = Vec::new();
    for (name, result) in names.iter().zip(reg.cache_parsers_detailed(&names, force)) {
        match result {
            Ok(outcome) if verbose => {
                eprintln!("--> {name}");
                if let Some(url) = outcome.download_url {
                    eprintln!("downloading from {url}");
                }
                eprintln!(
                    "downloaded to {}",
                    relative_to_current(&outcome.path).display()
                );
                eprintln!("cached in {:.3}s", outcome.elapsed.as_secs_f64());
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("{name}: failed");
                errors.push((name, error));
            }
        }
    }

    if !errors.is_empty() {
        return Err(anyhow::anyhow!(
            "failed to cache {} parser(s)",
            errors.len()
        ));
    }

    // Downloading is the smaller half of a cold parser; the Wasmtime compile is
    // the larger. Compiling each one here writes it into `compiled/`, so a
    // prepared directory carries both. Host cache APIs write the same layout.
    let mut compile_errors = Vec::new();
    for (name, result) in names.iter().zip(reg.precompile_parsers_detailed(&names)) {
        match result {
            Ok(elapsed) if verbose => eprintln!("compiled in {:.3}s", elapsed.as_secs_f64()),
            Ok(_) => {}
            Err(error) => {
                eprintln!("{name}: failed to compile");
                compile_errors.push((name, error));
            }
        }
    }
    if !compile_errors.is_empty() {
        return Err(anyhow::anyhow!(
            "failed to compile {} parser(s)",
            compile_errors.len()
        ));
    }

    Ok(())
}

/// Resolve a user-provided language name to its stable package language ID.
fn resolve_language_id(name: &str) -> &str {
    name.parse::<Language>()
        .map_or(name, |language| language.id_name())
}

// Returns `Result` so every `Commands` arm has one signature to dispatch to.
#[allow(clippy::unnecessary_wraps)]
fn list_themes(data_dir: &Path) -> Result<()> {
    let mut themes: Vec<_> = lumis_core::themes::available_themes()
        .map(|t| t.name.clone())
        .collect();

    // Add file themes from data dir
    let themes_dir = data_dir.join("themes");
    if let Ok(entries) = std::fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                if let Some(stem) = path.file_stem() {
                    let name = format!("{} (file)", stem.to_string_lossy());
                    themes.push(name);
                }
            }
        }
    }

    themes.sort();
    for theme in themes {
        println!("{theme}");
    }

    Ok(())
}

// Returns `Result` so every `Commands` arm has one signature to dispatch to.
#[allow(clippy::unnecessary_wraps)]
fn list_languages() -> Result<()> {
    for language in Language::iter() {
        let name = Language::id_name(&language);
        println!("{name}");

        for glob in Language::language_globs(language) {
            print!("  {}", glob.as_str());
        }

        println!();
    }

    Ok(())
}

fn resolve_theme(
    name: Option<String>,
    data_dir: Option<&Path>,
    verbose: bool,
) -> Option<lumis_core::themes::Theme> {
    let name = match name.as_deref() {
        None | Some("auto") => {
            if let Some(name) = guess_terminal_theme() {
                name
            } else {
                if verbose {
                    eprintln!("theme: auto unavailable");
                }
                return None;
            }
        }
        Some(name) => name.to_string(),
    };

    // Try built-in theme first
    if let Ok(theme) = lumis_core::themes::get(&name) {
        if verbose {
            eprintln!("theme: {}", theme.name);
        }
        return Some(theme);
    }

    // Try file theme from data dir
    if let Some(dir) = data_dir {
        let path = dir.join("themes").join(format!("{name}.json"));
        if let Ok(theme) = lumis_core::themes::from_file(&path) {
            if verbose {
                eprintln!("theme: {}", theme.name);
            }
            return Some(theme);
        }
    }

    if verbose {
        eprintln!("theme: {name} not found");
    }

    None
}

fn guess_terminal_theme() -> Option<String> {
    let background =
        terminal_colorsaurus::background_color(terminal_colorsaurus::QueryOptions::default())
            .ok()?
            .scale_to_8bit();

    closest_builtin_theme(background)
}

fn closest_builtin_theme(background: (u8, u8, u8)) -> Option<String> {
    lumis_core::themes::available_themes()
        .filter_map(|theme| {
            let theme_background = theme.bg().and_then(hex_to_rgb)?;
            Some((color_distance(background, theme_background), &theme.name))
        })
        .min_by_key(|(distance, _)| *distance)
        .map(|(_, name)| name.clone())
}

// Redmean color distance weights RGB channels based on human perception.
fn color_distance(left: (u8, u8, u8), right: (u8, u8, u8)) -> u64 {
    let red_mean = u64::midpoint(u64::from(left.0), u64::from(right.0));
    let red = i64::from(left.0) - i64::from(right.0);
    let green = i64::from(left.1) - i64::from(right.1);
    let blue = i64::from(left.2) - i64::from(right.2);

    (((512 + red_mean) * red.unsigned_abs().pow(2)) >> 8)
        + 4 * green.unsigned_abs().pow(2)
        + (((767 - red_mean) * blue.unsigned_abs().pow(2)) >> 8)
}

fn read_source(path: Option<String>, language: Option<String>) -> Result<(String, Language)> {
    if let Some(path) = path {
        let bytes = read_or_die(Path::new(&path));
        let source = std::str::from_utf8(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode file '{path}' as UTF-8: {e}"))?
            .to_string();
        let lang = if language.is_some() {
            Language::guess(language.as_deref(), &source)
        } else {
            Language::guess(Some(path.as_str()), &source)
        };
        Ok((source, lang))
    } else if !std::io::stdin().is_terminal() {
        let mut source = String::new();
        std::io::stdin().read_to_string(&mut source)?;
        let lang = Language::guess(language.as_deref(), &source);
        Ok((source, lang))
    } else {
        Err(anyhow::anyhow!(
            "provide a file path or pipe input via stdin"
        ))
    }
}

fn dump_language(lang: Language) -> Result<&'static str> {
    if lang == Language::PlainText {
        return Err(anyhow::anyhow!(
            "could not detect a language; pass --language <language>"
        ));
    }

    Ok(lang.id_name())
}

#[allow(clippy::too_many_arguments)]
fn dump_tree(
    reg: &registry::Registry,
    path: Option<String>,
    language: Option<String>,
    format: TreeFormat,
    text: Option<TreeText>,
    highlights: bool,
    injections: bool,
) -> Result<()> {
    if matches!(format, TreeFormat::Sexp) && (text.is_some() || highlights) {
        return Err(anyhow::anyhow!(
            "--format sexp cannot be combined with --text or --highlights"
        ));
    }

    let (source, lang) = read_source(path, language)?;
    let lang_name = dump_language(lang)?;
    match format {
        TreeFormat::Lines => dump_tree_lines(reg, &source, lang_name, text, highlights, injections),
        TreeFormat::Sexp => dump_tree_sexp(reg, &source, lang_name, injections),
    }
}

fn dump_tree_sexp(
    reg: &registry::Registry,
    source: &str,
    lang_name: &str,
    injections: bool,
) -> Result<()> {
    if !injections {
        let tree = reg.parse_tree(lang_name, source)?;
        println!("{:#}", tree.root_node());
        return Ok(());
    }

    let output = highlight_output(reg, source, lang_name, true, true)?;
    let multiple_layers = output.layers.len() > 1;
    for (index, layer) in output.layers.iter().enumerate() {
        if index > 0 {
            println!();
        }
        if multiple_layers {
            let root = layer.tree.root_node();
            let start = root.start_position();
            let end = root.end_position();
            println!(
                "language: {}, depth: {}, range: {}:{}-{}:{}",
                layer.language, layer.depth, start.row, start.column, end.row, end.column
            );
        }
        println!("{:#}", layer.tree.root_node());
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum SerializableHighlightEvent {
    Start { scope: String, language: String },
    Source { start: usize, end: usize },
    End,
}

fn dump_events(
    reg: &registry::Registry,
    path: Option<String>,
    language: Option<String>,
) -> Result<()> {
    let (source, lang) = read_source(path, language)?;
    let events = highlight_to_events(reg, &source, dump_language(lang)?, false)?
        .into_iter()
        .map(|event| match event {
            HighlightEvent::Start {
                scope_index,
                language,
            } => SerializableHighlightEvent::Start {
                scope: lumis_core::highlights::HIGHLIGHT_NAMES[scope_index].to_string(),
                language,
            },
            HighlightEvent::Source { start, end } => {
                SerializableHighlightEvent::Source { start, end }
            }
            HighlightEvent::End => SerializableHighlightEvent::End,
            _ => unreachable!("syntax highlighting emits only scope and source events"),
        })
        .collect::<Vec<_>>();

    println!("{}", serde_json::to_string_pretty(&events)?);
    Ok(())
}

struct TreeNode<'tree> {
    node: Node<'tree>,
    depth: usize,
    field: Option<&'static str>,
    language: &'tree str,
}

struct OpenHighlight {
    scope_index: usize,
    language: String,
    start: Option<usize>,
    end: usize,
    order: usize,
}

struct TreeHighlight {
    scope_index: usize,
    language: String,
    start: usize,
    end: usize,
    order: usize,
}

fn tree_injections(
    layers: &[ParsedLayer],
) -> std::collections::HashMap<(usize, usize), Vec<usize>> {
    let mut injections = std::collections::HashMap::<(usize, usize), Vec<usize>>::new();

    for (child_index, child) in layers
        .iter()
        .enumerate()
        .filter(|(_, layer)| layer.depth > 0)
    {
        let child_root = child.tree.root_node();
        let parent = layers
            .iter()
            .enumerate()
            .filter(|(_, layer)| {
                layer.depth + 1 == child.depth
                    && layer.tree.root_node().start_byte() <= child_root.start_byte()
                    && layer.tree.root_node().end_byte() >= child_root.end_byte()
            })
            .min_by_key(|(_, layer)| {
                layer.tree.root_node().end_byte() - layer.tree.root_node().start_byte()
            });

        if let Some((parent_index, parent)) = parent {
            let parent_root = parent.tree.root_node();
            let owner = parent_root
                .named_descendant_for_byte_range(child_root.start_byte(), child_root.end_byte())
                .unwrap_or(parent_root);
            injections
                .entry((parent_index, owner.id()))
                .or_default()
                .push(child_index);
        }
    }

    for child_layers in injections.values_mut() {
        child_layers.sort_by_key(|index| {
            let root = layers[*index].tree.root_node();
            std::cmp::Reverse(root.end_byte() - root.start_byte())
        });
    }

    injections
}

#[allow(clippy::too_many_arguments)]
fn collect_tree_nodes<'tree>(
    layer_index: usize,
    node: Node<'tree>,
    depth: usize,
    field: Option<&'static str>,
    layers: &'tree [ParsedLayer],
    injections: &std::collections::HashMap<(usize, usize), Vec<usize>>,
    nodes: &mut Vec<TreeNode<'tree>>,
) {
    nodes.push(TreeNode {
        node,
        depth,
        field,
        language: &layers[layer_index].language,
    });

    if let Some(child_layers) = injections.get(&(layer_index, node.id())) {
        for child_index in child_layers {
            collect_tree_nodes(
                *child_index,
                layers[*child_index].tree.root_node(),
                depth + 1,
                None,
                layers,
                injections,
                nodes,
            );
        }
    }

    let mut cursor = node.walk();
    if !cursor.goto_first_child() {
        return;
    }

    loop {
        let child = cursor.node();
        if child.is_named() {
            collect_tree_nodes(
                layer_index,
                child,
                depth + 1,
                cursor.field_name(),
                layers,
                injections,
                nodes,
            );
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn tree_highlights(events: Vec<HighlightEvent>) -> Result<Vec<TreeHighlight>> {
    let mut open = Vec::new();
    let mut highlights = Vec::new();
    let mut order = 0usize;

    for event in events {
        match event {
            HighlightEvent::Start {
                scope_index,
                language,
            } => {
                open.push(OpenHighlight {
                    scope_index,
                    language,
                    start: None,
                    end: 0,
                    order,
                });
                order += 1;
            }
            HighlightEvent::Source { start, end } => {
                for highlight in &mut open {
                    highlight.start.get_or_insert(start);
                    highlight.end = end;
                }
            }
            HighlightEvent::End => {
                let highlight = open.pop().ok_or_else(|| {
                    anyhow::anyhow!("highlight event stream contains an unmatched end event")
                })?;
                if let Some(start) = highlight.start {
                    highlights.push(TreeHighlight {
                        scope_index: highlight.scope_index,
                        language: highlight.language,
                        start,
                        end: highlight.end,
                        order: highlight.order,
                    });
                }
            }
            _ => unreachable!("syntax highlighting emits only scope and source events"),
        }
    }

    if !open.is_empty() {
        return Err(anyhow::anyhow!(
            "highlight event stream contains unclosed start events"
        ));
    }

    highlights.sort_by_key(|highlight| highlight.order);
    highlights.dedup_by(|left, right| {
        left.scope_index == right.scope_index
            && left.language == right.language
            && left.start == right.start
            && left.end == right.end
    });
    Ok(highlights)
}

fn source_line_starts(source: &str) -> Vec<usize> {
    let mut starts = vec![0];
    starts.extend(
        source
            .bytes()
            .enumerate()
            .filter_map(|(index, byte)| (byte == b'\n').then_some(index + 1)),
    );
    starts
}

fn source_point(line_starts: &[usize], byte: usize) -> (usize, usize) {
    let row = line_starts.partition_point(|start| *start <= byte) - 1;
    (row, byte - line_starts[row])
}

enum TreeRenderEntry {
    Capture(usize),
    Child(usize),
}

fn truncate_tree_text(text: &str, limit: usize) -> String {
    const MARKER: &str = "...";

    let char_count = text.chars().count();
    if char_count <= limit {
        return text.to_string();
    }

    if limit <= MARKER.len() {
        return text.chars().take(limit).collect();
    }

    let visible_chars = limit - MARKER.len();
    let head_chars = visible_chars.div_ceil(2);
    let tail_chars = visible_chars / 2;
    let head_end = text
        .char_indices()
        .nth(head_chars)
        .map_or(text.len(), |(index, _)| index);
    let tail_start = if tail_chars == 0 {
        text.len()
    } else {
        text.char_indices()
            .nth(char_count - tail_chars)
            .map_or(text.len(), |(index, _)| index)
    };

    format!("{}{}{}", &text[..head_end], MARKER, &text[tail_start..])
}

fn tree_text_metadata(text: &str, mode: TreeText) -> Result<String> {
    let text = match mode {
        TreeText::Preview(limit) => truncate_tree_text(text, limit),
        TreeText::Full => text.to_string(),
    };
    Ok(format!(", text: {}", serde_json::to_string(&text)?))
}

#[allow(clippy::too_many_arguments)]
fn render_tree_node(
    index: usize,
    nodes: &[TreeNode<'_>],
    highlights_by_node: &[Vec<&TreeHighlight>],
    source: &str,
    line_starts: &[usize],
    text_mode: Option<TreeText>,
    prefix: &str,
    is_last: Option<bool>,
) -> Result<usize> {
    let item = &nodes[index];
    let connector = match is_last {
        None => "",
        Some(true) => "└── ",
        Some(false) => "├── ",
    };
    let field = item
        .field
        .map_or_else(String::new, |field| format!("field: {field}, "));
    let missing = if item.node.is_missing() {
        "MISSING "
    } else {
        ""
    };
    let start = item.node.start_position();
    let end = item.node.end_position();
    let text = if let Some(mode) = text_mode {
        let text = source
            .get(item.node.byte_range())
            .ok_or_else(|| anyhow::anyhow!("syntax node has an invalid byte range"))?;
        tree_text_metadata(text, mode)?
    } else {
        String::new()
    };

    println!(
        "{prefix}{connector}[{missing}{}] {field}language: {}, range: {}:{}-{}:{}{text}",
        item.node.kind(),
        item.language,
        start.row,
        start.column,
        end.row,
        end.column,
    );

    let subtree_end = nodes[index + 1..]
        .iter()
        .position(|node| node.depth <= item.depth)
        .map_or(nodes.len(), |offset| index + 1 + offset);
    let mut entries = highlights_by_node[index]
        .iter()
        .enumerate()
        .map(|(capture_index, _)| TreeRenderEntry::Capture(capture_index))
        .chain(
            (index + 1..subtree_end)
                .filter(|child_index| nodes[*child_index].depth == item.depth + 1)
                .map(TreeRenderEntry::Child),
        )
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| match entry {
        TreeRenderEntry::Capture(capture_index) => {
            (highlights_by_node[index][*capture_index].start, 0usize)
        }
        TreeRenderEntry::Child(child_index) => (nodes[*child_index].node.start_byte(), 1usize),
    });

    let child_prefix = match is_last {
        None => prefix.to_string(),
        Some(true) => format!("{prefix}    "),
        Some(false) => format!("{prefix}│   "),
    };
    let entry_count = entries.len();
    for (entry_index, entry) in entries.into_iter().enumerate() {
        let entry_is_last = entry_index + 1 == entry_count;
        let entry_connector = if entry_is_last {
            "└── "
        } else {
            "├── "
        };
        match entry {
            TreeRenderEntry::Capture(capture_index) => {
                let highlight = highlights_by_node[index][capture_index];
                let text = if let Some(mode) = text_mode {
                    let capture_text =
                        source.get(highlight.start..highlight.end).ok_or_else(|| {
                            anyhow::anyhow!(
                                "highlight capture has an invalid byte range {}..{}",
                                highlight.start,
                                highlight.end
                            )
                        })?;
                    tree_text_metadata(capture_text, mode)?
                } else {
                    String::new()
                };
                let capture_start = source_point(line_starts, highlight.start);
                let capture_end = source_point(line_starts, highlight.end);
                let scope = lumis_core::highlights::HIGHLIGHT_NAMES[highlight.scope_index];
                println!(
                    "{child_prefix}{entry_connector}@{scope} language: {}, range: {}:{}-{}:{}{text}",
                    highlight.language,
                    capture_start.0,
                    capture_start.1,
                    capture_end.0,
                    capture_end.1,
                );
            }
            TreeRenderEntry::Child(child_index) => {
                render_tree_node(
                    child_index,
                    nodes,
                    highlights_by_node,
                    source,
                    line_starts,
                    text_mode,
                    &child_prefix,
                    Some(entry_is_last),
                )?;
            }
        }
    }

    Ok(subtree_end)
}

fn dump_tree_lines(
    reg: &registry::Registry,
    source: &str,
    lang_name: &str,
    text: Option<TreeText>,
    highlights: bool,
    injections: bool,
) -> Result<()> {
    let output = if highlights || injections {
        highlight_output(reg, source, lang_name, true, injections)?
    } else {
        let tree = reg.parse_tree(lang_name, source)?;
        let range = tree.root_node().range();
        HighlightOutput {
            events: Vec::new(),
            layers: vec![ParsedLayer {
                tree,
                language: lang_name.to_string(),
                ranges: vec![range],
                depth: 0,
            }],
            unresolved: Vec::new(),
        }
    };
    let resolved_highlights = if highlights {
        tree_highlights(output.events)?
    } else {
        Vec::new()
    };
    let layers = output.layers;
    let injection_map = tree_injections(&layers);
    let mut nodes = Vec::new();
    for (layer_index, layer) in layers
        .iter()
        .enumerate()
        .filter(|(_, layer)| layer.depth == 0)
    {
        collect_tree_nodes(
            layer_index,
            layer.tree.root_node(),
            0,
            None,
            &layers,
            &injection_map,
            &mut nodes,
        );
    }
    let line_starts = source_line_starts(source);

    let mut highlights_by_node = vec![Vec::new(); nodes.len()];
    for highlight in &resolved_highlights {
        let owner = nodes
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                item.language == highlight.language
                    && item.node.start_byte() <= highlight.start
                    && item.node.end_byte() >= highlight.end
            })
            .min_by_key(|(_, item)| {
                (
                    item.node.end_byte() - item.node.start_byte(),
                    std::cmp::Reverse(item.depth),
                )
            })
            .map(|(index, _)| index)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "no {} syntax node contains highlight capture {}..{}",
                    highlight.language,
                    highlight.start,
                    highlight.end
                )
            })?;

        highlights_by_node[owner].push(highlight);
    }

    let mut index = 0;
    while index < nodes.len() {
        index = render_tree_node(
            index,
            &nodes,
            &highlights_by_node,
            source,
            &line_starts,
            text,
            "",
            None,
        )?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn do_highlight(reg: &registry::Registry, args: HighlightArgs, verbose: bool) -> Result<()> {
    let (source, lang) = read_source(args.path.clone(), args.language.clone())?;

    if verbose {
        eprintln!("--");
        eprintln!("language: {}", lang.id_name());
    }

    if lang == Language::PlainText {
        if verbose {
            eprintln!("--\n");
        }
        print!("{source}");
        return Ok(());
    }

    let lang_name = lang.id_name();
    let events = highlight_to_events(reg, &source, lang_name, args.rainbow_brackets)?;

    render_output(reg, &source, &events, lang, args, verbose)
}

/// Line ranges plus the class and style that apply to them, or `None` when no
/// lines were named.
fn inline_highlight_lines(
    args: &HighlightArgs,
) -> Result<Option<lumis_core::formatter::html_inline::HighlightLines>> {
    use lumis_core::formatter::html_inline::{HighlightLines, HighlightLinesStyle};

    let Some(lines) = args.html.highlight_lines.as_deref() else {
        return Ok(None);
    };

    let style = match args.styled.highlight_lines_style.as_deref() {
        None | Some("theme") => Some(HighlightLinesStyle::Theme),
        Some("none") => None,
        Some(css) => Some(HighlightLinesStyle::Style(css.to_string())),
    };

    Ok(Some(HighlightLines {
        lines: parse_highlight_lines(lines)?,
        style,
        class: args.html.highlight_lines_class.clone(),
    }))
}

fn linked_highlight_lines(
    args: &HighlightArgs,
) -> Result<Option<lumis_core::formatter::html_linked::HighlightLines>> {
    use lumis_core::formatter::html_linked::HighlightLines;

    let Some(lines) = args.html.highlight_lines.as_deref() else {
        return Ok(None);
    };

    Ok(Some(HighlightLines {
        lines: parse_highlight_lines(lines)?,
        class: args
            .html
            .highlight_lines_class
            .clone()
            .unwrap_or_else(|| "l-highlighted".to_string()),
    }))
}

fn header_element(args: &HighlightArgs) -> Option<lumis_core::formatter::HtmlElement> {
    // clap's `requires` keeps these two either both set or both absent.
    let open_tag = args.html.header_open.clone()?;
    let close_tag = args.html.header_close.clone()?;
    Some(lumis_core::formatter::HtmlElement {
        open_tag,
        close_tag,
    })
}

fn print_verbose_separator(verbose: bool) {
    if verbose {
        eprintln!("--\n");
    }
}

// These are the already-validated CLI option groups; bundling them again here
// would create a second configuration model for one formatter.
#[allow(clippy::too_many_arguments)]
fn render_html_multi_themes(
    reg: &registry::Registry,
    source: &str,
    events: &[HighlightEvent],
    lang: Language,
    themes: &[String],
    default_theme: Option<&str>,
    css_variable_prefix: Option<&str>,
    pre_class: Option<String>,
    italic: bool,
    include_highlights: bool,
    highlight_lines: Option<lumis_core::formatter::html_inline::HighlightLines>,
    header: Option<lumis_core::formatter::HtmlElement>,
    verbose: bool,
) -> Result<Vec<u8>> {
    if themes.is_empty() {
        return Err(anyhow::anyhow!(
            "--formatter html-multi-themes requires --themes"
        ));
    }

    let mut theme_map = std::collections::HashMap::new();
    for theme_spec in themes {
        let parts: Vec<&str> = theme_spec.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid theme format '{theme_spec}', expected 'name:theme_id'"
            ));
        }
        let theme_name = parts[0];
        let theme_id = parts[1];
        let theme = resolve_theme(Some(theme_id.to_string()), Some(reg.data_dir()), verbose)
            .ok_or_else(|| anyhow::anyhow!("Theme '{theme_id}' not found"))?;
        theme_map.insert(theme_name.to_string(), theme);
    }

    print_verbose_separator(verbose);
    let mut builder = lumis_core::formatter::HtmlMultiThemesBuilder::new();
    builder
        .language(lang)
        .themes(theme_map)
        .css_variable_prefix(
            css_variable_prefix
                .unwrap_or(DEFAULT_CSS_VARIABLE_PREFIX)
                .to_string(),
        )
        .pre_class(pre_class)
        .italic(italic)
        .include_highlights(include_highlights)
        .highlight_lines(highlight_lines)
        .header(header);

    if let Some(default) = default_theme {
        builder.default_theme(default.to_string());
    }

    let fmt = builder.build().map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut output = Vec::new();
    fmt.render(source, events, &mut output)?;
    Ok(output)
}

fn render_output(
    reg: &registry::Registry,
    source: &str,
    events: &[HighlightEvent],
    lang: Language,
    args: HighlightArgs,
    verbose: bool,
) -> Result<()> {
    let chosen = args.formatter();
    let HighlightArgs {
        ref theme,
        terminal: TerminalArgs {
            ref background,
            ref width,
        },
        html: HtmlArgs { ref pre_class, .. },
        styled:
            StyledArgs {
                italic,
                include_highlights,
                ..
            },
        multi_theme:
            MultiThemeArgs {
                ref themes,
                ref default_theme,
                ref css_variable_prefix,
            },
        ..
    } = args;

    let highlight_lines = inline_highlight_lines(&args)?;
    let header = header_element(&args);

    match chosen {
        Formatter::HtmlInline => {
            let theme_obj = resolve_theme(theme.clone(), Some(reg.data_dir()), verbose);
            print_verbose_separator(verbose);
            let mut builder = lumis_core::formatter::HtmlInlineBuilder::new();
            builder
                .language(lang)
                .theme(theme_obj)
                .pre_class(pre_class.clone())
                .italic(italic)
                .include_highlights(include_highlights)
                .highlight_lines(highlight_lines)
                .header(header);

            let fmt = builder.build().map_err(|e| anyhow::anyhow!("{e}"))?;
            let mut output = Vec::new();
            fmt.render(source, events, &mut output)?;
            print!("{}", String::from_utf8(output)?);
        }

        Formatter::HtmlMultiThemes => {
            let output = render_html_multi_themes(
                reg,
                source,
                events,
                lang,
                themes,
                default_theme.as_deref(),
                css_variable_prefix.as_deref(),
                pre_class.clone(),
                italic,
                include_highlights,
                highlight_lines,
                header,
                verbose,
            )?;
            print!("{}", String::from_utf8(output)?);
        }

        Formatter::HtmlLinked => {
            print_verbose_separator(verbose);
            let mut builder = lumis_core::formatter::HtmlLinkedBuilder::new();
            builder
                .language(lang)
                .pre_class(pre_class.clone())
                .highlight_lines(linked_highlight_lines(&args)?)
                .header(header);

            let fmt = builder.build().map_err(|e| anyhow::anyhow!("{e}"))?;
            let mut output = Vec::new();
            fmt.render(source, events, &mut output)?;
            print!("{}", String::from_utf8(output)?);
        }

        Formatter::Terminal => {
            let theme_obj = resolve_theme(theme.clone(), Some(reg.data_dir()), verbose);
            print_verbose_separator(verbose);
            let mut builder = lumis_core::formatter::TerminalBuilder::new();
            builder
                .language(lang)
                .theme(theme_obj)
                .background(parse_terminal_background(background.as_deref()))
                .width(resolve_terminal_width(width.as_deref())?);

            let fmt = builder.build().map_err(|e| anyhow::anyhow!("{e}"))?;
            let mut output = Vec::new();
            fmt.render(source, events, &mut output)?;
            print!("{}", String::from_utf8(output)?);
        }

        Formatter::BbcodeScoped => {
            print_verbose_separator(verbose);
            let fmt = lumis_core::formatter::BBCodeScoped::new(lang);
            let mut output = Vec::new();
            fmt.render(source, events, &mut output)?;
            print!("{}", String::from_utf8(output)?);
        }
    }

    Ok(())
}

fn parse_terminal_background(bg: Option<&str>) -> TerminalBackground {
    match bg {
        Some("theme") => TerminalBackground::Theme,
        Some(color) => TerminalBackground::Color(color.to_string()),
        None => TerminalBackground::Inherit,
    }
}

fn resolve_terminal_width(width: Option<&str>) -> Result<Option<usize>> {
    match width {
        Some("auto") | None => Ok(auto_terminal_width()),
        Some(raw) => raw.parse::<usize>().map(Some).map_err(|_| {
            anyhow::anyhow!("invalid width '{raw}', expected a positive integer or 'auto'")
        }),
    }
}

fn auto_terminal_width() -> Option<usize> {
    compute_auto_terminal_width(
        std::io::stdout().is_terminal(),
        terminal_size().map(|(Width(width), _)| usize::from(width)),
        std::env::var("COLUMNS")
            .ok()
            .and_then(|value| value.parse().ok()),
    )
}

fn compute_auto_terminal_width(
    stdout_is_terminal: bool,
    detected_width: Option<usize>,
    columns_env: Option<usize>,
) -> Option<usize> {
    if !stdout_is_terminal {
        return None;
    }

    detected_width.or(columns_env)
}

const EXIT_BAD_ARGUMENTS: i32 = 2;

fn read_or_die(path: &Path) -> Vec<u8> {
    match fs::read(path) {
        Ok(src) => src,
        Err(e) => {
            eprint_read_error(&FileArgument::NamedPath(path.to_path_buf()), &e);
            std::process::exit(EXIT_BAD_ARGUMENTS);
        }
    }
}

fn eprint_read_error(file_arg: &FileArgument, e: &std::io::Error) {
    match e.kind() {
        std::io::ErrorKind::NotFound => {
            eprintln!("No such file: {file_arg}");
        }
        std::io::ErrorKind::PermissionDenied => {
            eprintln!("Permission denied when reading file: {file_arg}");
        }
        _ => match file_arg {
            FileArgument::NamedPath(path) if path.is_dir() => {
                eprintln!("Expected a file, got a directory: {}", path.display());
            }
            _ => eprintln!("Could not read file: {} (error {:?})", file_arg, e.kind()),
        },
    }
}

#[allow(dead_code)]
enum FileArgument {
    NamedPath(std::path::PathBuf),
    Stdin,
    DevNull,
}

impl Display for FileArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileArgument::NamedPath(path) => {
                write!(f, "{}", relative_to_current(path).display())
            }
            FileArgument::Stdin => write!(f, "(stdin)"),
            FileArgument::DevNull => write!(f, "/dev/null"),
        }
    }
}

fn parse_highlight_lines(input: &str) -> Result<Vec<RangeInclusive<usize>>> {
    let mut ranges = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        if let Some((start, end)) = part.split_once('-') {
            let start: usize = start
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid line number: '{}'", start.trim()))?;
            let end: usize = end
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid line number: '{}'", end.trim()))?;

            if start == 0 || end == 0 {
                return Err(anyhow::anyhow!("Line numbers must be greater than 0"));
            }
            if start > end {
                return Err(anyhow::anyhow!(
                    "Start line ({start}) must be less than or equal to end line ({end})"
                ));
            }

            ranges.push(start..=end);
        } else {
            let line: usize = part
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid line number: '{part}'"))?;

            if line == 0 {
                return Err(anyhow::anyhow!("Line numbers must be greater than 0"));
            }

            ranges.push(line..=line);
        }
    }

    Ok(ranges)
}

#[allow(dead_code)]
fn relative_to_current(path: &Path) -> PathBuf {
    if let Ok(current_path) = std::env::current_dir() {
        let path = path.canonicalize().unwrap_or_else(|_| path.into());
        let current_path = current_path.canonicalize().unwrap_or(current_path);

        if let Ok(relative_path) = path.strip_prefix(&current_path) {
            return relative_path.into();
        }

        return path;
    }

    path.into()
}

fn highlight_output(
    reg: &registry::Registry,
    source: &str,
    lang_name: &str,
    layers: bool,
    injections: bool,
) -> Result<HighlightOutput> {
    reg.highlight(
        source,
        lang_name,
        &HighlightOptions {
            layers,
            injections,
            ..HighlightOptions::default()
        },
    )
}

fn highlight_to_events(
    reg: &registry::Registry,
    source: &str,
    lang_name: &str,
    rainbow_brackets: bool,
) -> Result<Vec<HighlightEvent>> {
    Ok(reg
        .highlight(
            source,
            lang_name,
            &HighlightOptions {
                rainbow_brackets,
                ..HighlightOptions::default()
            },
        )?
        .events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn truncate_tree_text_respects_small_limits_and_keeps_both_ends() {
        assert_eq!(truncate_tree_text("abcdefghij", 5), "a...j");
        assert_eq!(truncate_tree_text("aé🙂xyzuvw", 7), "aé...vw");
        assert_eq!(truncate_tree_text("abcdefghij", 1), "a");
        assert_eq!(truncate_tree_text("abcdefghij", 2), "ab");
        assert_eq!(truncate_tree_text("abcdefghij", 3), "abc");
        assert_eq!(truncate_tree_text("é🙂xyz", 1), "é");
        assert_eq!(truncate_tree_text("é🙂xyz", 2), "é🙂");
        assert_eq!(truncate_tree_text("é🙂xyz", 3), "é🙂x");
        assert_eq!(truncate_tree_text("short", 80), "short");
    }

    #[test]
    fn highlight_to_events_uses_cached_injection_parsers() {
        let dir = tempdir().unwrap();
        let reg = registry::Registry::new(dir.path().to_path_buf()).unwrap();
        reg.cache_test_language(
            "html",
            "html",
            include_bytes!("../../../fixtures/test-parsers/tree-sitter-html.wasm"),
            include_str!("../../../queries/processed/html/highlights.scm"),
            include_str!("../../../queries/processed/html/injections.scm"),
            "",
        );
        reg.cache_test_language(
            "javascript",
            "javascript",
            include_bytes!("../../../fixtures/test-parsers/tree-sitter-javascript.wasm"),
            include_str!("../../../queries/processed/javascript/highlights.scm"),
            include_str!("../../../queries/processed/javascript/injections.scm"),
            include_str!("../../../queries/processed/javascript/locals.scm"),
        );

        let source = r"
<script>
  const count = 1
</script>
";

        let events = highlight_to_events(&reg, source, "html", false).unwrap();

        assert!(events.iter().any(|event| matches!(
            event,
            HighlightEvent::Start { language, .. } if language == "javascript"
        )));
    }

    #[test]
    fn closest_builtin_theme_matches_exact_background() {
        assert_eq!(
            closest_builtin_theme((0x22, 0x24, 0x36)).as_deref(),
            Some("tokyonight_moon")
        );
    }

    #[test]
    fn compute_auto_terminal_width_prefers_detected_terminal_width() {
        assert_eq!(
            compute_auto_terminal_width(true, Some(120), Some(80)),
            Some(120)
        );
    }

    #[test]
    fn compute_auto_terminal_width_falls_back_to_columns_env() {
        assert_eq!(compute_auto_terminal_width(true, None, Some(80)), Some(80));
    }

    #[test]
    fn compute_auto_terminal_width_returns_none_when_not_a_tty() {
        assert_eq!(
            compute_auto_terminal_width(false, Some(120), Some(80)),
            None
        );
    }
}
