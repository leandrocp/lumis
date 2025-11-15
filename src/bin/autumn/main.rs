mod gen_theme;

use anyhow::Result;
use autumnus::formatter::Formatter as FormatterTrait;
use autumnus::languages::Language;
use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::Display;
use std::fs;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

/// CLI for the Autumnus syntax highlighter
///
/// This binary provides command-line access to Autumnus's syntax highlighting capabilities.
/// While the package is named 'autumnus', this binary is named 'autumn' for easier typing.
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Commands available in the autumn CLI
#[derive(Subcommand)]
enum Commands {
    /// List all supported programming languages and their file patterns
    ListLanguages,

    /// List all available syntax highlighting themes
    ListThemes,

    /// Dump the Tree-sitter AST for a file (useful for debugging)
    DumpTreeSitter {
        /// Path to the file to analyze
        path: String,

        /// Enable colored output
        #[arg(long)]
        color: bool,
    },

    /// Highlight a file with syntax highlighting
    Highlight {
        /// Path to the file to highlight
        path: String,

        /// Output format (terminal, html-inline, html-linked)
        #[arg(short = 'f', long)]
        formatter: Option<Formatter>,

        /// Theme name (e.g., "dracula", "github_dark")
        #[arg(short = 't', long)]
        theme: Option<String>,

        /// Highlight lines
        #[arg(short = 'l', long)]
        highlight_lines: Option<String>,
    },

    /// Highlight a string of source code
    HighlightSource {
        /// The source code to highlight
        source: String,

        /// Programming language for the source code
        #[arg(short = 'l', long)]
        language: Option<String>,

        /// Output format (terminal, html-inline, html-linked)
        #[arg(short = 'f', long)]
        formatter: Option<Formatter>,

        /// Theme name (e.g., "dracula", "github_dark")
        #[arg(short = 't', long)]
        theme: Option<String>,

        /// Highlight lines
        #[arg(long)]
        highlight_lines: Option<String>,
    },

    /// Generate a theme JSON from a Git repository containing a Neovim theme
    GenTheme {
        /// Git repository URL (e.g., <https://github.com/catppuccin/nvim>)
        #[arg(short = 'u', long)]
        url: String,

        /// Colorscheme name to activate (e.g., catppuccin-mocha)
        #[arg(short = 'c', long)]
        colorscheme: String,

        /// Custom Lua setup code (optional)
        #[arg(short = 's', long)]
        setup: Option<String>,

        /// Output file path (prints to stdout if not specified)
        #[arg(short = 'o', long)]
        output: Option<String>,

        /// Theme appearance: light or dark (defaults to dark)
        #[arg(short = 'a', long)]
        appearance: Option<String>,
    },
}

/// Output format options for syntax highlighting
#[derive(Clone, Default, ValueEnum)]
enum Formatter {
    /// HTML output with inline styles
    HtmlInline,
    /// HTML output with linked stylesheet
    HtmlLinked,
    /// ANSI colored output for terminal (default)
    #[default]
    Terminal,
}

/// Entry point for the autumn CLI
///
/// Parses command line arguments and dispatches to the appropriate handler function.
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListLanguages => list_languages(),
        Commands::ListThemes => list_themes(),
        Commands::DumpTreeSitter { path, color } => dump_tree_sitter(&path, color),
        Commands::Highlight {
            path,
            formatter,
            theme,
            highlight_lines,
        } => highlight(&path, formatter, theme, highlight_lines),
        Commands::HighlightSource {
            source,
            language,
            formatter,
            theme,
            highlight_lines,
        } => highlight_source(
            &source,
            language.as_deref(),
            formatter,
            theme,
            highlight_lines,
        ),
        Commands::GenTheme {
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
    }
}

/// Lists all available themes in alphabetical order
fn list_themes() -> Result<()> {
    let mut themes: Vec<_> = autumnus::themes::ALL_THEMES.iter().collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));

    for theme in themes {
        println!("{}", theme.name);
    }

    Ok(())
}

/// Lists all supported programming languages and their associated file patterns
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

/// Dumps the Tree-sitter AST for a given file
///
/// This is useful for debugging and understanding how Tree-sitter parses your code.
/// The output shows the AST structure with node types, positions, and source text.
///
/// # Arguments
/// * `path` - Path to the file to analyze
/// * `color` - Whether to enable colored output
fn dump_tree_sitter(path: &str, color: bool) -> Result<()> {
    let bytes = read_or_die(Path::new(&path));
    let source = String::from_utf8_lossy(&bytes).to_string();
    let language = autumnus::languages::Language::guess(Some(path), &source);
    let config = language.config();
    let tree = to_tree(&source, &config.language);
    print_tree(&source, &tree, color);
    Ok(())
}

/// Creates a Tree-sitter tree from source code and language
///
/// # Arguments
/// * `src` - Source code to parse
/// * `language` - Tree-sitter language for parsing
fn to_tree(src: &str, language: &tree_sitter::Language) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();

    parser
        .set_language(language)
        .expect("Incompatible tree-sitter version");

    parser.parse(src, None).unwrap()
}

/// Recursively prints the Tree-sitter AST
///
/// # Arguments
/// * `src` - Original source code
/// * `tree` - Tree-sitter tree to print
/// * `color` - Whether to enable colored output
fn print_tree(src: &str, tree: &tree_sitter::Tree, color: bool) {
    let mut cursor = tree.walk();
    print_cursor(src, &mut cursor, 0, color);
    println!();
}

/// Prints a node in the Tree-sitter AST and recursively visits its children
///
/// # Arguments
/// * `src` - Original source code
/// * `cursor` - Current position in the tree
/// * `depth` - Current depth in the tree (for indentation)
/// * `color` - Whether to enable colored output
fn print_cursor(src: &str, cursor: &mut tree_sitter::TreeCursor, depth: usize, color: bool) {
    let node = cursor.node();
    let field_name = cursor.field_name();

    let start = node.start_position();
    let end = node.end_position();

    let indent = "  ".repeat(depth);

    let is_anonymous = !node.is_named();

    if !is_anonymous {
        if let Some(field) = field_name {
            if color {
                print!("{indent}\x1b[36m{field}\x1b[0m: ");
            } else {
                print!("{indent}{field}: ");
            }
        } else {
            print!("{indent}");
        }

        let node_kind = node.kind().replace('\n', "\\n");
        if color {
            print!("\x1b[35m(\x1b[0m\x1b[34m{node_kind}\x1b[0m");
        } else {
            print!("({node_kind}");
        }

        let node_kind = node.kind();
        if node_kind != "source" && node_kind != "source_file" {
            let node_text = &src[node.start_byte()..node.end_byte()];
            let truncated = if node_text.len() > 60 {
                format!("{} (truncated)", &node_text[..60])
            } else {
                node_text.to_string()
            };

            if color {
                print!(" \x1b[32m{truncated:?}\x1b[0m");
            } else {
                print!(" {truncated:?}");
            }
        }

        if color {
            print!(
                " \x1b[90m; [{}, {}] - [{}, {}]\x1b[0m",
                start.row, start.column, end.row, end.column
            );
        } else {
            print!(
                " ; [{}, {}] - [{}, {}]",
                start.row, start.column, end.row, end.column
            );
        }
    }

    let has_children = cursor.goto_first_child();

    if has_children {
        loop {
            let child_node = cursor.node();
            let child_is_anonymous = !child_node.is_named();

            if !child_is_anonymous {
                if !is_anonymous {
                    println!();
                }
                print_cursor(
                    src,
                    cursor,
                    if is_anonymous { depth } else { depth + 1 },
                    color,
                );
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }

    if !is_anonymous {
        if color {
            print!("\x1b[35m)\x1b[0m");
        } else {
            print!(")");
        }
    }
}

/// Highlights a file with syntax highlighting
///
/// # Arguments
/// * `path` - Path to the file to highlight
/// * `formatter` - Output format (terminal, html-inline, html-linked)
/// * `theme` - Theme name to use for highlighting
/// * `highlight_lines` - Optional string specifying lines to highlight (e.g., "1,3-5,8")
fn highlight(
    path: &str,
    formatter: Option<Formatter>,
    theme: Option<String>,
    highlight_lines: Option<String>,
) -> Result<()> {
    let theme = theme.unwrap_or("catppuccin_frappe".to_string());
    let theme = autumnus::themes::get(&theme).ok();

    let parsed_highlight_lines = if let Some(lines_str) = highlight_lines {
        Some(parse_highlight_lines(&lines_str)?)
    } else {
        None
    };

    let bytes = read_or_die(Path::new(&path));
    let source = std::str::from_utf8(&bytes)
        .map_err(|e| anyhow::anyhow!("Failed to decode file '{}' as UTF-8: {}", path, e))?;

    let language = autumnus::languages::Language::guess(Some(path), source);

    match formatter.unwrap_or_default() {
        Formatter::HtmlInline => {
            let formatter = if let Some(lines) = parsed_highlight_lines {
                let html_highlight_lines = autumnus::formatter::html_inline::HighlightLines {
                    lines,
                    style: Some(autumnus::formatter::html_inline::HighlightLinesStyle::Theme),
                    class: None,
                };
                autumnus::HtmlInlineBuilder::new()
                    .lang(language)
                    .theme(theme)
                    .italic(false)
                    .include_highlights(false)
                    .highlight_lines(Some(html_highlight_lines))
                    .build()
                    .unwrap()
            } else {
                autumnus::HtmlInlineBuilder::new()
                    .lang(language)
                    .theme(theme)
                    .italic(false)
                    .include_highlights(false)
                    .build()
                    .unwrap()
            };

            let mut output = Vec::new();
            formatter.format(source, &mut output).unwrap();
            let highlighted = String::from_utf8(output).unwrap();

            println!("{highlighted}");
        }

        Formatter::HtmlLinked => {
            let formatter = if let Some(lines) = parsed_highlight_lines {
                let html_highlight_lines = autumnus::formatter::html_linked::HighlightLines {
                    lines,
                    class: "highlighted".to_string(),
                };
                autumnus::HtmlLinkedBuilder::new()
                    .lang(language)
                    .highlight_lines(Some(html_highlight_lines))
                    .build()
                    .unwrap()
            } else {
                autumnus::HtmlLinkedBuilder::new()
                    .lang(language)
                    .build()
                    .unwrap()
            };

            let mut output = Vec::new();
            formatter.format(source, &mut output).unwrap();
            let highlighted = String::from_utf8(output).unwrap();

            println!("{highlighted}");
        }

        Formatter::Terminal => {
            let formatter = autumnus::TerminalBuilder::new()
                .lang(language)
                .theme(theme)
                .build()
                .unwrap();

            let mut output = Vec::new();
            formatter.format(source, &mut output).unwrap();
            let highlighted = String::from_utf8(output).unwrap();

            println!("{highlighted}");
        }
    }

    Ok(())
}

const EXIT_BAD_ARGUMENTS: i32 = 2;

/// Reads a file or exits with an error message
///
/// # Arguments
/// * `path` - Path to the file to read
fn read_or_die(path: &Path) -> Vec<u8> {
    match fs::read(path) {
        Ok(src) => src,
        Err(e) => {
            eprint_read_error(&FileArgument::NamedPath(path.to_path_buf()), &e);
            std::process::exit(EXIT_BAD_ARGUMENTS);
        }
    }
}

/// Prints a user-friendly error message for file read errors
///
/// # Arguments
/// * `file_arg` - File argument that caused the error
/// * `e` - The error that occurred
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
    };
}

/// Represents different types of file arguments that can be passed to the CLI
#[allow(dead_code)]
enum FileArgument {
    /// A path to a file
    NamedPath(std::path::PathBuf),
    /// Standard input
    Stdin,
    /// /dev/null
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

/// Parses a highlight_lines string into a vector of `RangeInclusive<usize>`
///
/// Supports formats like:
/// - "1" (single line)
/// - "1,3,5" (multiple single lines)
/// - "1-3" (range from 1 to 3)
/// - "1,3-5,8" (mix of single lines and ranges)
///
/// # Arguments
/// * `input` - The string to parse
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
                    "Start line ({}) must be less than or equal to end line ({})",
                    start,
                    end
                ));
            }

            ranges.push(start..=end);
        } else {
            let line: usize = part
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid line number: '{}'", part))?;

            if line == 0 {
                return Err(anyhow::anyhow!("Line numbers must be greater than 0"));
            }

            ranges.push(line..=line);
        }
    }

    Ok(ranges)
}

/// Converts an absolute path to a path relative to the current directory
///
/// # Arguments
/// * `path` - The path to convert
fn relative_to_current(path: &Path) -> PathBuf {
    if let Ok(current_path) = std::env::current_dir() {
        let path = try_canonicalize(path);
        let current_path = try_canonicalize(&current_path);

        if let Ok(rel_path) = path.strip_prefix(current_path) {
            return rel_path.into();
        }
    }

    path.into()
}

/// Attempts to canonicalize a path, falling back to the original path on error
///
/// # Arguments
/// * `path` - The path to canonicalize
fn try_canonicalize(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.into())
}

/// Highlights a string of source code
///
/// # Arguments
/// * `source` - The source code to highlight
/// * `language` - Programming language for the source code
/// * `formatter` - Output format (terminal, html-inline, html-linked)
/// * `theme` - Theme name to use for highlighting
/// * `highlight_lines` - Optional string specifying lines to highlight (e.g., "1,3-5,8")
fn highlight_source(
    source: &str,
    language: Option<&str>,
    formatter: Option<Formatter>,
    theme: Option<String>,
    highlight_lines: Option<String>,
) -> Result<()> {
    let theme = theme.unwrap_or("catppuccin_frappe".to_string());
    let theme = autumnus::themes::get(&theme).ok();

    let parsed_highlight_lines = if let Some(lines_str) = highlight_lines {
        Some(parse_highlight_lines(&lines_str)?)
    } else {
        None
    };

    let lang = autumnus::languages::Language::guess(language, source);

    match formatter.unwrap_or_default() {
        Formatter::HtmlInline => {
            let formatter = if let Some(lines) = parsed_highlight_lines {
                let html_highlight_lines = autumnus::formatter::html_inline::HighlightLines {
                    lines,
                    style: Some(autumnus::formatter::html_inline::HighlightLinesStyle::Theme),
                    class: None,
                };
                autumnus::HtmlInlineBuilder::new()
                    .lang(lang)
                    .theme(theme)
                    .italic(false)
                    .include_highlights(false)
                    .highlight_lines(Some(html_highlight_lines))
                    .build()
                    .unwrap()
            } else {
                autumnus::HtmlInlineBuilder::new()
                    .lang(lang)
                    .theme(theme)
                    .italic(false)
                    .include_highlights(false)
                    .build()
                    .unwrap()
            };

            let mut output = Vec::new();
            formatter.format(source, &mut output).unwrap();
            let highlighted = String::from_utf8(output).unwrap();

            println!("{highlighted}");
        }

        Formatter::HtmlLinked => {
            let formatter = if let Some(lines) = parsed_highlight_lines {
                let html_highlight_lines = autumnus::formatter::html_linked::HighlightLines {
                    lines,
                    class: "highlighted".to_string(),
                };
                autumnus::HtmlLinkedBuilder::new()
                    .lang(lang)
                    .highlight_lines(Some(html_highlight_lines))
                    .build()
                    .unwrap()
            } else {
                autumnus::HtmlLinkedBuilder::new()
                    .lang(lang)
                    .build()
                    .unwrap()
            };

            let mut output = Vec::new();
            formatter.format(source, &mut output).unwrap();
            let highlighted = String::from_utf8(output).unwrap();

            println!("{highlighted}");
        }

        Formatter::Terminal => {
            let formatter = autumnus::TerminalBuilder::new()
                .lang(lang)
                .theme(theme)
                .build()
                .unwrap();

            let mut output = Vec::new();
            formatter.format(source, &mut output).unwrap();
            let highlighted = String::from_utf8(output).unwrap();

            println!("{highlighted}");
        }
    }

    Ok(())
}
