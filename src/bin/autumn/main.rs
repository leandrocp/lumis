use anyhow::Result;
use autumnus::languages::Language;
use autumnus::FormatterOption;
use clap::{Parser, Subcommand, ValueEnum};
use std::fmt::Display;
use std::fs;
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
        Commands::DumpTreeSitter { path } => dump_tree_sitter(&path),
        Commands::Highlight {
            path,
            formatter,
            theme,
        } => highlight(&path, formatter, theme),
        Commands::HighlightSource {
            source,
            language,
            formatter,
            theme,
        } => highlight_source(&source, language.as_deref(), formatter, theme),
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
        println!("{}", name);

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
fn dump_tree_sitter(path: &str) -> Result<()> {
    let bytes = read_or_die(Path::new(&path));
    let source = String::from_utf8_lossy(&bytes).to_string();
    let language = autumnus::languages::Language::guess(path, &source);
    let config = language.config();
    let tree = to_tree(&source, &config.language);
    print_tree(&source, &tree);
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
fn print_tree(src: &str, tree: &tree_sitter::Tree) {
    let mut cursor = tree.walk();
    print_cursor(src, &mut cursor, 0);
}

/// Prints a node in the Tree-sitter AST and recursively visits its children
///
/// # Arguments
/// * `src` - Original source code
/// * `cursor` - Current position in the tree
/// * `depth` - Current depth in the tree (for indentation)
fn print_cursor(src: &str, cursor: &mut tree_sitter::TreeCursor, depth: usize) {
    loop {
        let node = cursor.node();

        let formatted_node = format!(
            "{} {} - {}",
            node.kind().replace('\n', "\\n"),
            node.start_position(),
            node.end_position()
        );

        if node.child_count() == 0 {
            let node_src = &src[node.start_byte()..node.end_byte()];
            println!("{}{} {:?}", "  ".repeat(depth), formatted_node, node_src);
        } else {
            println!("{}{}", "  ".repeat(depth), formatted_node,);
        }

        if cursor.goto_first_child() {
            print_cursor(src, cursor, depth + 1);
            cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

/// Highlights a file with syntax highlighting
///
/// # Arguments
/// * `path` - Path to the file to highlight
/// * `formatter` - Output format (terminal, html-inline, html-linked)
/// * `theme` - Theme name to use for highlighting
fn highlight(path: &str, formatter: Option<Formatter>, theme: Option<String>) -> Result<()> {
    let theme = theme.unwrap_or("catppuccin_frappe".to_string());
    let theme = autumnus::themes::get(&theme).ok();

    let bytes = read_or_die(Path::new(&path));
    let source = std::str::from_utf8(&bytes)
        .map_err(|e| anyhow::anyhow!("Failed to decode file '{}' as UTF-8: {}", path, e))?;

    match formatter.unwrap_or_default() {
        Formatter::HtmlInline => {
            let highlighted = autumnus::highlight(
                source,
                autumnus::Options {
                    lang_or_file: Some(path),
                    formatter: FormatterOption::HtmlInline {
                        pre_class: None,
                        italic: false,
                        include_highlights: false,
                        theme,
                        highlight_lines: None,
                    },
                },
            );

            println!("{}", highlighted);
        }

        Formatter::HtmlLinked => {
            let highlighted = autumnus::highlight(
                source,
                autumnus::Options {
                    lang_or_file: Some(path),
                    formatter: FormatterOption::HtmlLinked {
                        pre_class: None,
                        highlight_lines: None,
                    },
                },
            );

            println!("{}", highlighted);
        }

        Formatter::Terminal => {
            let highlighted = autumnus::highlight(
                source,
                autumnus::Options {
                    lang_or_file: Some(path),
                    formatter: FormatterOption::Terminal { theme },
                },
            );

            println!("{}", highlighted);
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
            eprintln!("No such file: {}", file_arg);
        }
        std::io::ErrorKind::PermissionDenied => {
            eprintln!("Permission denied when reading file: {}", file_arg);
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
fn highlight_source(
    source: &str,
    language: Option<&str>,
    formatter: Option<Formatter>,
    theme: Option<String>,
) -> Result<()> {
    let theme = theme.unwrap_or("catppuccin_frappe".to_string());
    let theme = autumnus::themes::get(&theme).ok();

    match formatter.unwrap_or_default() {
        Formatter::HtmlInline => {
            let highlighted = autumnus::highlight(
                source,
                autumnus::Options {
                    lang_or_file: language,
                    formatter: FormatterOption::HtmlInline {
                        pre_class: None,
                        italic: false,
                        include_highlights: false,
                        theme,
                        highlight_lines: None,
                    },
                },
            );

            println!("{}", highlighted);
        }

        Formatter::HtmlLinked => {
            let highlighted = autumnus::highlight(
                source,
                autumnus::Options {
                    lang_or_file: language,
                    formatter: FormatterOption::HtmlLinked {
                        pre_class: None,
                        highlight_lines: None,
                    },
                },
            );

            println!("{}", highlighted);
        }

        Formatter::Terminal => {
            let highlighted = autumnus::highlight(
                source,
                autumnus::Options {
                    lang_or_file: language,
                    formatter: FormatterOption::Terminal { theme },
                },
            );

            println!("{}", highlighted);
        }
    }

    Ok(())
}
