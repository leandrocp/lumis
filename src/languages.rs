//! Language detection and Tree-sitter configuration.
//!
//! This module provides the [`Language`] enum that represents all supported programming
//! languages and their Tree-sitter configurations. It includes automatic language
//! detection based on file extensions, file names, content analysis, and shebangs.
//!
//! # Language Detection
//!
//! The language detection works in the following priority order:
//! 1. **Exact name match** - `"rust"`, `"elixir"`, `"javascript"`, etc.
//! 2. **File path/name patterns** - `"app.ex"`, `"Dockerfile"`, `"Makefile"`, etc.
//! 3. **File extension** - `".rs"`, `".js"`, `".py"`, etc.
//! 4. **Emacs mode header** - `// -*- mode: rust -*-`
//! 5. **Shebang** - `#!/usr/bin/env python`
//! 6. **Content heuristics** - HTML doctype, XML declaration, etc.
//! 7. **Fallback** - [`Language::PlainText`]
//!
//! # Examples
//!
//! ## Basic language guessing
//!
//! ```rust
//! use autumnus::languages::Language;
//!
//! // By language name
//! let lang = Language::guess(Some("rust"), "");
//! assert_eq!(lang, Language::Rust);
//!
//! // By file extension
//! let lang = Language::guess(Some("rs"), "");
//! assert_eq!(lang, Language::Rust);
//!
//! // By file path
//! let lang = Language::guess(Some("src/main.rs"), "");
//! assert_eq!(lang, Language::Rust);
//! ```
//!
//! ## Content-based detection
//!
//! ```rust
//! use autumnus::languages::Language;
//!
//! // Shebang detection
//! let code = "#!/usr/bin/env python3\nprint('Hello')";
//! let lang = Language::guess(None, code);
//! assert_eq!(lang, Language::Python);
//!
//! // HTML doctype detection
//! let html = "<!DOCTYPE html>\n<html></html>";
//! let lang = Language::guess(None, html);
//! assert_eq!(lang, Language::HTML);
//! ```
//!
//! ## Getting language information
//!
//! ```rust
//! use autumnus::languages::{Language, available_languages};
//!
//! // Get friendly name
//! assert_eq!(Language::Rust.name(), "Rust");
//! assert_eq!(Language::CSharp.name(), "C#");
//!
//! // Get all supported languages
//! let languages = available_languages();
//! assert!(languages.contains_key("rust"));
//! assert!(languages.contains_key("elixir"));
//!
//! let (name, extensions) = &languages["rust"];
//! assert_eq!(name, "Rust");
//! assert!(extensions.contains(&"*.rs".to_string()));
//! ```
//!
//! ## Tree-sitter integration
//!
//! ```rust
//! use autumnus::languages::Language;
//! use autumnus::vendor::tree_sitter_highlight::Highlighter;
//!
//! let lang = Language::Rust;
//! let config = lang.config();
//!
//! let mut highlighter = Highlighter::new();
//! let events = highlighter.highlight(
//!     config,
//!     b"fn main() {}",
//!     None,
//!     |_| None
//! ).unwrap();
//! ```

// Guess Language copied from https://github.com/Wilfred/difftastic/blob/f34a9014760efbaed01b972caba8b73754da16c9/src/parse/guess_language.rs

use crate::constants::HIGHLIGHT_NAMES;
use crate::vendor::tree_sitter_highlight::HighlightConfiguration;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::{path::Path, sync::LazyLock};
use strum::{EnumIter, IntoEnumIterator};

unsafe extern "C" {
    #[cfg(feature = "lang-angular")]
    fn tree_sitter_angular() -> *const ();
    #[cfg(feature = "lang-astro")]
    fn tree_sitter_astro() -> *const ();
    #[cfg(feature = "lang-caddy")]
    fn tree_sitter_caddy() -> *const ();
    #[cfg(feature = "lang-clojure")]
    fn tree_sitter_clojure() -> *const ();
    #[cfg(feature = "lang-commonlisp")]
    fn tree_sitter_commonlisp() -> *const ();
    #[cfg(feature = "lang-csv")]
    fn tree_sitter_csv() -> *const ();
    #[cfg(feature = "lang-dart")]
    fn tree_sitter_dart() -> *const ();
    #[cfg(feature = "lang-dockerfile")]
    fn tree_sitter_dockerfile() -> *const ();
    #[cfg(feature = "lang-eex")]
    fn tree_sitter_eex() -> *const ();
    #[cfg(feature = "lang-fish")]
    fn tree_sitter_fish() -> *const ();
    #[cfg(feature = "lang-glimmer")]
    fn tree_sitter_glimmer() -> *const ();
    #[cfg(feature = "lang-graphql")]
    fn tree_sitter_graphql() -> *const ();
    #[cfg(feature = "lang-iex")]
    fn tree_sitter_iex() -> *const ();
    #[cfg(feature = "lang-kotlin")]
    fn tree_sitter_kotlin() -> *const ();
    #[cfg(feature = "lang-latex")]
    fn tree_sitter_latex() -> *const ();
    #[cfg(feature = "lang-liquid")]
    fn tree_sitter_liquid() -> *const ();
    #[cfg(feature = "lang-llvm")]
    fn tree_sitter_llvm() -> *const ();
    #[cfg(feature = "lang-make")]
    fn tree_sitter_make() -> *const ();
    #[cfg(feature = "lang-markdown")]
    fn tree_sitter_markdown() -> *const ();
    #[cfg(feature = "lang-markdown-inline")]
    fn tree_sitter_markdown_inline() -> *const ();
    #[cfg(feature = "lang-perl")]
    fn tree_sitter_perl() -> *const ();
    #[cfg(feature = "lang-scss")]
    fn tree_sitter_scss() -> *const ();
    #[cfg(feature = "lang-surface")]
    fn tree_sitter_surface() -> *const ();
    #[cfg(feature = "lang-typst")]
    fn tree_sitter_typst() -> *const ();
    #[cfg(feature = "lang-vim")]
    fn tree_sitter_vim() -> *const ();
    #[cfg(feature = "lang-vue")]
    fn tree_sitter_vue() -> *const ();
}

include!(concat!(env!("OUT_DIR"), "/queries_constants.rs"));

#[derive(Clone, Copy, Debug, Default, EnumIter, Eq, PartialEq)]
pub enum Language {
    #[cfg(feature = "lang-angular")]
    Angular,
    #[cfg(feature = "lang-asm")]
    Assembly,
    #[cfg(feature = "lang-astro")]
    Astro,
    #[cfg(feature = "lang-bash")]
    Bash,
    #[cfg(feature = "lang-c")]
    C,
    #[cfg(feature = "lang-caddy")]
    Caddy,
    #[cfg(feature = "lang-cmake")]
    CMake,
    #[cfg(feature = "lang-cpp")]
    CPlusPlus,
    #[cfg(feature = "lang-css")]
    CSS,
    #[cfg(feature = "lang-csv")]
    CSV,
    #[cfg(feature = "lang-csharp")]
    CSharp,
    #[cfg(feature = "lang-clojure")]
    Clojure,
    #[cfg(feature = "lang-comment")]
    Comment,
    #[cfg(feature = "lang-commonlisp")]
    CommonLisp,
    #[cfg(feature = "lang-dart")]
    Dart,
    Diff,
    #[cfg(feature = "lang-dockerfile")]
    Dockerfile,
    #[cfg(feature = "lang-eex")]
    EEx,
    #[cfg(feature = "lang-ejs")]
    EJS,
    #[cfg(feature = "lang-erb")]
    ERB,
    #[cfg(feature = "lang-elixir")]
    Elixir,
    #[cfg(feature = "lang-elm")]
    Elm,
    #[cfg(feature = "lang-erlang")]
    Erlang,
    #[cfg(feature = "lang-fish")]
    Fish,
    #[cfg(feature = "lang-fsharp")]
    FSharp,
    #[cfg(feature = "lang-gleam")]
    Gleam,
    #[cfg(feature = "lang-glimmer")]
    Glimmer,
    #[cfg(feature = "lang-go")]
    Go,
    #[cfg(feature = "lang-graphql")]
    GraphQL,
    #[cfg(feature = "lang-heex")]
    HEEx,
    #[cfg(feature = "lang-html")]
    HTML,
    #[cfg(feature = "lang-haskell")]
    Haskell,
    #[cfg(feature = "lang-hcl")]
    HCL,
    #[cfg(feature = "lang-iex")]
    IEx,
    #[cfg(feature = "lang-json")]
    JSON,
    #[cfg(feature = "lang-java")]
    Java,
    #[cfg(feature = "lang-javascript")]
    JavaScript,
    #[cfg(feature = "lang-kotlin")]
    Kotlin,
    #[cfg(feature = "lang-latex")]
    LaTeX,
    #[cfg(feature = "lang-liquid")]
    Liquid,
    #[cfg(feature = "lang-llvm")]
    Llvm,
    #[cfg(feature = "lang-lua")]
    Lua,
    #[cfg(feature = "lang-make")]
    Make,
    #[cfg(feature = "lang-markdown")]
    Markdown,
    #[cfg(feature = "lang-markdown-inline")]
    MarkdownInline,
    #[cfg(feature = "lang-nix")]
    Nix,
    #[cfg(feature = "lang-ocaml")]
    OCaml,
    #[cfg(feature = "lang-ocaml")]
    OCamlInterface,
    #[cfg(feature = "lang-objc")]
    ObjC,
    #[cfg(feature = "lang-perl")]
    Perl,
    #[cfg(feature = "lang-php")]
    Php,
    #[default]
    PlainText,
    #[cfg(feature = "lang-powershell")]
    PowerShell,
    #[cfg(feature = "lang-protobuf")]
    ProtoBuf,
    #[cfg(feature = "lang-python")]
    Python,
    #[cfg(feature = "lang-r")]
    R,
    #[cfg(feature = "lang-regex")]
    Regex,
    #[cfg(feature = "lang-ruby")]
    Ruby,
    #[cfg(feature = "lang-rust")]
    Rust,
    #[cfg(feature = "lang-scss")]
    SCSS,
    #[cfg(feature = "lang-sql")]
    SQL,
    #[cfg(feature = "lang-scala")]
    Scala,
    #[cfg(feature = "lang-surface")]
    Surface,
    #[cfg(feature = "lang-svelte")]
    Svelte,
    #[cfg(feature = "lang-swift")]
    Swift,
    #[cfg(feature = "lang-toml")]
    Toml,
    #[cfg(feature = "lang-tsx")]
    Tsx,
    #[cfg(feature = "lang-typescript")]
    TypeScript,
    #[cfg(feature = "lang-typst")]
    Typst,
    #[cfg(feature = "lang-vim")]
    Vim,
    #[cfg(feature = "lang-vue")]
    Vue,
    #[cfg(feature = "lang-xml")]
    XML,
    #[cfg(feature = "lang-yaml")]
    YAML,
    #[cfg(feature = "lang-zig")]
    Zig,
}

/// Error returned when a language cannot be determined from input.
///
/// This error occurs when using [`std::str::FromStr`] or the `.parse()` method
/// with an unrecognized language name, file extension, or file path.
///
/// # Example
///
/// ```rust
/// use autumnus::languages::Language;
///
/// let result: Result<Language, _> = "unknown_lang".parse();
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageParseError(String);

impl std::fmt::Display for LanguageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown language or file type: {}", self.0)
    }
}

impl std::error::Error for LanguageParseError {}

impl std::str::FromStr for Language {
    type Err = LanguageParseError;

    /// Parse a language from a string.
    ///
    /// The input can be:
    /// - A language name (e.g., "rust", "python", "javascript")
    /// - A file extension (e.g., "rs", "py", "js")
    /// - A file path (e.g., "src/main.rs", "script.py")
    ///
    /// Returns an error if the language cannot be determined from the input.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use autumnus::languages::Language;
    /// use std::str::FromStr;
    ///
    /// // From language name
    /// let lang = Language::from_str("rust").unwrap();
    /// assert_eq!(lang, Language::Rust);
    ///
    /// // Using .parse()
    /// let lang: Language = "python".parse().unwrap();
    /// assert_eq!(lang, Language::Python);
    ///
    /// // From extension
    /// let lang: Language = "js".parse().unwrap();
    /// assert_eq!(lang, Language::JavaScript);
    ///
    /// // From file path
    /// let lang: Language = "src/main.rs".parse().unwrap();
    /// assert_eq!(lang, Language::Rust);
    ///
    /// // Empty string defaults to PlainText
    /// let lang: Language = "".parse().unwrap();
    /// assert_eq!(lang, Language::PlainText);
    ///
    /// // Unknown language returns error
    /// assert!(Language::from_str("unknown").is_err());
    /// assert!("unknown".parse::<Language>().is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Language::PlainText);
        }

        let s_lower = s.to_ascii_lowercase();

        let exact = match s_lower.as_str() {
            #[cfg(feature = "lang-angular")]
            "angular" => Some(Language::Angular),
            #[cfg(feature = "lang-asm")]
            "asm" | "assembly" => Some(Language::Assembly),
            #[cfg(feature = "lang-astro")]
            "astro" => Some(Language::Astro),
            #[cfg(feature = "lang-bash")]
            "bash" => Some(Language::Bash),
            #[cfg(feature = "lang-c")]
            "c" => Some(Language::C),
            #[cfg(feature = "lang-caddy")]
            "caddy" => Some(Language::Caddy),
            #[cfg(feature = "lang-clojure")]
            "clojure" => Some(Language::Clojure),
            #[cfg(feature = "lang-comment")]
            "comment" => Some(Language::Comment),
            #[cfg(feature = "lang-commonlisp")]
            "commonlisp" => Some(Language::CommonLisp),
            #[cfg(feature = "lang-cpp")]
            "c++" | "cpp" => Some(Language::CPlusPlus),
            #[cfg(feature = "lang-cmake")]
            "cmake" => Some(Language::CMake),
            #[cfg(feature = "lang-csharp")]
            "c#" | "csharp" => Some(Language::CSharp),
            #[cfg(feature = "lang-csv")]
            "csv" => Some(Language::CSV),
            #[cfg(feature = "lang-css")]
            "css" => Some(Language::CSS),
            #[cfg(feature = "lang-dart")]
            "dart" => Some(Language::Dart),
            "diff" => Some(Language::Diff),
            #[cfg(feature = "lang-dockerfile")]
            "dockerfile" | "docker" => Some(Language::Dockerfile),
            #[cfg(feature = "lang-eex")]
            "eex" => Some(Language::EEx),
            #[cfg(feature = "lang-ejs")]
            "ejs" => Some(Language::EJS),
            #[cfg(feature = "lang-erb")]
            "erb" => Some(Language::ERB),
            #[cfg(feature = "lang-elixir")]
            "elixir" => Some(Language::Elixir),
            #[cfg(feature = "lang-elm")]
            "elm" => Some(Language::Elm),
            #[cfg(feature = "lang-erlang")]
            "erlang" => Some(Language::Erlang),
            #[cfg(feature = "lang-fish")]
            "fish" => Some(Language::Fish),
            #[cfg(feature = "lang-fsharp")]
            "f#" | "fsharp" => Some(Language::FSharp),
            #[cfg(feature = "lang-gleam")]
            "gleam" => Some(Language::Gleam),
            #[cfg(feature = "lang-glimmer")]
            "ember" | "glimmer" | "handlebars" => Some(Language::Glimmer),
            #[cfg(feature = "lang-go")]
            "go" => Some(Language::Go),
            #[cfg(feature = "lang-graphql")]
            "graphql" => Some(Language::GraphQL),
            #[cfg(feature = "lang-haskell")]
            "haskell" => Some(Language::Haskell),
            #[cfg(feature = "lang-hcl")]
            "hcl" | "terraform" => Some(Language::HCL),
            #[cfg(feature = "lang-heex")]
            "heex" => Some(Language::HEEx),
            #[cfg(feature = "lang-html")]
            "html" => Some(Language::HTML),
            #[cfg(feature = "lang-iex")]
            "iex" => Some(Language::IEx),
            #[cfg(feature = "lang-java")]
            "java" => Some(Language::Java),
            #[cfg(feature = "lang-javascript")]
            "jsx" | "javascript" => Some(Language::JavaScript),
            #[cfg(feature = "lang-json")]
            "json" => Some(Language::JSON),
            #[cfg(feature = "lang-kotlin")]
            "kotlin" => Some(Language::Kotlin),
            #[cfg(feature = "lang-latex")]
            "latex" => Some(Language::LaTeX),
            #[cfg(feature = "lang-liquid")]
            "liquid" => Some(Language::Liquid),
            #[cfg(feature = "lang-llvm")]
            "llvm" => Some(Language::Llvm),
            #[cfg(feature = "lang-lua")]
            "lua" => Some(Language::Lua),
            #[cfg(feature = "lang-objc")]
            "objc" | "objective-c" => Some(Language::ObjC),
            #[cfg(feature = "lang-ocaml")]
            "ocaml" => Some(Language::OCaml),
            #[cfg(feature = "lang-ocaml")]
            "ocaml_interface" => Some(Language::OCamlInterface),
            #[cfg(feature = "lang-perl")]
            "perl" => Some(Language::Perl),
            #[cfg(feature = "lang-make")]
            "make" => Some(Language::Make),
            #[cfg(feature = "lang-markdown")]
            "markdown" => Some(Language::Markdown),
            #[cfg(feature = "lang-markdown-inline")]
            "markdown_inline" => Some(Language::MarkdownInline),
            #[cfg(feature = "lang-nix")]
            "nix" => Some(Language::Nix),
            #[cfg(feature = "lang-php")]
            "php" => Some(Language::Php),
            #[cfg(feature = "lang-powershell")]
            "powershell" => Some(Language::PowerShell),
            #[cfg(feature = "lang-protobuf")]
            "protobuf" => Some(Language::ProtoBuf),
            #[cfg(feature = "lang-python")]
            "python" => Some(Language::Python),
            #[cfg(feature = "lang-r")]
            "r" => Some(Language::R),
            #[cfg(feature = "lang-regex")]
            "regex" => Some(Language::Regex),
            #[cfg(feature = "lang-ruby")]
            "ruby" => Some(Language::Ruby),
            #[cfg(feature = "lang-rust")]
            "rust" => Some(Language::Rust),
            #[cfg(feature = "lang-scala")]
            "scala" => Some(Language::Scala),
            #[cfg(feature = "lang-scss")]
            "scss" => Some(Language::SCSS),
            #[cfg(feature = "lang-sql")]
            "sql" => Some(Language::SQL),
            #[cfg(feature = "lang-surface")]
            "surface" => Some(Language::Surface),
            #[cfg(feature = "lang-svelte")]
            "svelte" => Some(Language::Svelte),
            #[cfg(feature = "lang-swift")]
            "swift" => Some(Language::Swift),
            #[cfg(feature = "lang-toml")]
            "toml" => Some(Language::Toml),
            #[cfg(feature = "lang-typescript")]
            "typescript" => Some(Language::TypeScript),
            #[cfg(feature = "lang-tsx")]
            "tsx" => Some(Language::Tsx),
            #[cfg(feature = "lang-typst")]
            "typst" => Some(Language::Typst),
            #[cfg(feature = "lang-vim")]
            "vim" | "viml" | "vimscript" => Some(Language::Vim),
            #[cfg(feature = "lang-vue")]
            "vue" => Some(Language::Vue),
            #[cfg(feature = "lang-xml")]
            "xml" => Some(Language::XML),
            #[cfg(feature = "lang-yaml")]
            "yaml" => Some(Language::YAML),
            #[cfg(feature = "lang-zig")]
            "zig" => Some(Language::Zig),
            _ => None,
        };

        if let Some(lang) = exact {
            return Ok(lang);
        }

        let path = Path::new(&s_lower);

        if let Some(lang) = Self::from_glob(path) {
            return Ok(lang);
        }

        if let Some(lang) = Self::from_extension(&s_lower) {
            return Ok(lang);
        }

        Err(LanguageParseError(s.to_string()))
    }
}

impl Language {
    /// Guess the language based on an optional language hint and source content.
    ///
    /// # Arguments
    ///
    /// * `language` - Optional language hint. Can be:
    ///   - `None`: Try to auto-detect language from source content
    ///   - `Some(s)`: Language name, file extension, or file path
    /// * `src` - The source code content to analyze
    ///
    /// # Detection Strategy
    ///
    /// When `language` is `Some(...)`:
    /// 1. Try to parse as language name/extension/path via `FromStr`
    /// 2. If parsing succeeds, return that language
    /// 3. If parsing fails, fall through to content-based detection
    ///
    /// When `language` is `None` or parsing fails:
    /// 1. Check for Emacs mode header (`// -*- mode: rust -*-`)
    /// 2. Check for shebang (`#!/usr/bin/env python`)
    /// 3. Apply content heuristics (HTML doctype, XML declaration, etc.)
    /// 4. Default to `PlainText` if nothing matches
    ///
    /// # Examples
    ///
    /// ```rust
    /// use autumnus::languages::Language;
    ///
    /// // Explicit language
    /// let lang = Language::guess(Some("rust"), "");
    /// assert_eq!(lang, Language::Rust);
    ///
    /// // Auto-detect from shebang
    /// let lang = Language::guess(None, "#!/usr/bin/env python3\nprint('hi')");
    /// assert_eq!(lang, Language::Python);
    ///
    /// // File path hint
    /// let lang = Language::guess(Some("src/main.rs"), "");
    /// assert_eq!(lang, Language::Rust);
    /// ```
    pub fn guess(language: Option<&str>, src: &str) -> Self {
        // If a language hint is provided, try to parse it
        if let Some(input) = language {
            if let Ok(lang) = input.parse() {
                return lang;
            }
            // If parsing fails, continue to content-based detection
        }

        // Auto-detection from content
        if let Some(lang) = Self::from_emacs_mode_header(src) {
            return lang;
        }

        if let Some(lang) = Self::from_shebang(src) {
            return lang;
        }

        #[cfg(feature = "lang-html")]
        if Self::looks_like_html(src) {
            return Language::HTML;
        }

        #[cfg(feature = "lang-xml")]
        if Self::looks_like_xml(src) {
            return Language::XML;
        }

        #[cfg(feature = "lang-objc")]
        if Self::looks_like_objc(Path::new(""), src) {
            return Language::ObjC;
        }

        Language::PlainText
    }

    fn from_glob(path: &Path) -> Option<Self> {
        match path.file_name() {
            Some(name) => {
                let name = name.to_string_lossy().into_owned();
                for language in Language::iter() {
                    for glob in Language::language_globs(language) {
                        if glob.matches(&name) {
                            return Some(language);
                        }
                    }
                }

                None
            }
            None => None,
        }
    }

    fn from_extension(token: &str) -> Option<Self> {
        let token_pattern = format!("*.{token}");

        for language in Language::iter() {
            for glob in Language::language_globs(language) {
                if glob.matches(&token_pattern) {
                    return Some(language);
                }
            }
        }
        None
    }

    // TODO: https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries/embedded_template
    pub fn language_globs(language: Language) -> Vec<glob::Pattern> {
        let glob_strs: &'static [&'static str] = match language {
            #[cfg(feature = "lang-angular")]
            Language::Angular => &["*.angular", "component.html"],
            #[cfg(feature = "lang-asm")]
            Language::Assembly => &["*.s", "*.asm", "*.assembly"],
            #[cfg(feature = "lang-astro")]
            Language::Astro => &["*.astro"],
            #[cfg(feature = "lang-bash")]
            Language::Bash => &[
                "*.bash",
                "*.bats",
                "*.cgi",
                "*.command",
                "*.env",
                "*.fcgi",
                "*.ksh",
                "*.sh",
                "*.sh.in",
                "*.tmux",
                "*.tool",
                "*.zsh",
                ".bash_aliases",
                ".bash_history",
                ".bash_logout",
                ".bash_profile",
                ".bashrc",
                ".cshrc",
                ".env",
                ".env.example",
                ".flaskenv",
                ".kshrc",
                ".login",
                ".profile",
                ".zlogin",
                ".zlogout",
                ".zprofile",
                ".zshenv",
                ".zshrc",
                "9fs",
                "PKGBUILD",
                "bash_aliases",
                "bash_logout",
                "bash_profile",
                "bashrc",
                "cshrc",
                "ebuild",
                "eclass",
                "gradlew",
                "kshrc",
                "login",
                "man",
                "profile",
                "zlogin",
                "zlogout",
                "zprofile",
                "zshenv",
                "zshrc",
            ],
            #[cfg(feature = "lang-c")]
            Language::C => &["*.c"],
            #[cfg(feature = "lang-caddy")]
            Language::Caddy => &["Caddyfile", "caddyfile"],
            #[cfg(feature = "lang-clojure")]
            Language::Clojure => &[
                "*.bb", "*.boot", "*.clj", "*.cljc", "*.clje", "*.cljs", "*.cljx", "*.edn",
                "*.joke", "*.joker",
            ],
            #[cfg(feature = "lang-comment")]
            Language::Comment => &[],
            #[cfg(feature = "lang-commonlisp")]
            Language::CommonLisp => &["*.lisp", "*.lsp", "*.asd"],
            #[cfg(feature = "lang-cmake")]
            Language::CMake => &["*.cmake", "*.cmake.in", "CMakeLists.txt"],
            #[cfg(feature = "lang-csharp")]
            Language::CSharp => &["*.cs"],
            #[cfg(feature = "lang-csv")]
            Language::CSV => &["*.csv"],
            #[cfg(feature = "lang-cpp")]
            Language::CPlusPlus => &[
                "*.cc", "*.cpp", "*.h", "*.hh", "*.hpp", "*.ino", "*.cxx", "*.cu", "*.hxx",
            ],
            #[cfg(feature = "lang-css")]
            Language::CSS => &["*.css"],
            #[cfg(feature = "lang-dart")]
            Language::Dart => &["*.dart"],
            Language::Diff => &["*.diff"],
            #[cfg(feature = "lang-dockerfile")]
            Language::Dockerfile => &[
                "Dockerfile",
                "dockerfile",
                "docker",
                "Containerfile",
                "container",
                "*.dockerfile",
                "*.docker",
                "*.container",
            ],
            #[cfg(feature = "lang-eex")]
            Language::EEx => &["*.eex"],
            #[cfg(feature = "lang-ejs")]
            Language::EJS => &["*.ejs"],
            #[cfg(feature = "lang-erb")]
            Language::ERB => &["*.erb"],
            #[cfg(feature = "lang-elixir")]
            Language::Elixir => &["*.ex", "*.exs"],
            #[cfg(feature = "lang-elm")]
            Language::Elm => &["*.elm"],
            #[cfg(feature = "lang-erlang")]
            Language::Erlang => &[
                "*.erl",
                "*.app",
                "*.app.src",
                "*.es",
                "*.escript",
                "*.hrl",
                "*.xrl",
                "*.yrl",
                "Emakefile",
                "rebar.config",
            ],
            #[cfg(feature = "lang-fish")]
            Language::Fish => &["*.fish"],
            #[cfg(feature = "lang-fsharp")]
            Language::FSharp => &["*.fs", "*.fsx", "*.fsi"],
            #[cfg(feature = "lang-gleam")]
            Language::Gleam => &["*.gleam"],
            #[cfg(feature = "lang-glimmer")]
            Language::Glimmer => &["*.hbs", "*.handlebars", "*.html.handlebars", "*.glimmer"],
            #[cfg(feature = "lang-go")]
            Language::Go => &["*.go"],
            #[cfg(feature = "lang-graphql")]
            Language::GraphQL => &[],
            #[cfg(feature = "lang-haskell")]
            Language::Haskell => &["*.hs", "*.hs-boot"],
            #[cfg(feature = "lang-hcl")]
            Language::HCL => &["*.hcl", "*.nomad", "*.tf", "*.tfvars", "*.workflow"],
            #[cfg(feature = "lang-heex")]
            Language::HEEx => &["*.heex", "*.neex"],
            #[cfg(feature = "lang-html")]
            Language::HTML => &["*.html", "*.htm", "*.xhtml"],
            #[cfg(feature = "lang-iex")]
            Language::IEx => &["*.iex"],
            #[cfg(feature = "lang-java")]
            Language::Java => &["*.java"],
            #[cfg(feature = "lang-javascript")]
            Language::JavaScript => &["*.cjs", "*.js", "*.mjs", "*.snap", "*.jsx"],
            #[cfg(feature = "lang-json")]
            Language::JSON => &[
                "*.json",
                "*.avsc",
                "*.geojson",
                "*.gltf",
                "*.har",
                "*.ice",
                "*.JSON-tmLanguage",
                "*.jsonl",
                "*.mcmeta",
                "*.tfstate",
                "*.tfstate.backup",
                "*.topojson",
                "*.webapp",
                "*.webmanifest",
                ".arcconfig",
                ".auto-changelog",
                ".c8rc",
                ".htmlhintrc",
                ".imgbotconfig",
                ".nycrc",
                ".tern-config",
                ".tern-project",
                ".watchmanconfig",
                "Pipfile.lock",
                "composer.lock",
                "mcmod.info",
                "flake.lock",
            ],
            #[cfg(feature = "lang-kotlin")]
            Language::Kotlin => &["*.kt", "*.ktm", "*.kts"],
            #[cfg(feature = "lang-latex")]
            Language::LaTeX => &["*.aux", "*.cls", "*.sty", "*.tex"],
            #[cfg(feature = "lang-liquid")]
            Language::Liquid => &["*liquid"],
            #[cfg(feature = "lang-llvm")]
            Language::Llvm => &["*.llvm", "*.ll"],
            #[cfg(feature = "lang-lua")]
            Language::Lua => &["*.lua"],
            #[cfg(feature = "lang-make")]
            Language::Make => &[
                "*.mak",
                "*.d",
                "*.make",
                "*.makefile",
                "*.mk",
                "*.mkfile",
                "*.dsp",
                "BSDmakefile",
                "GNUmakefile",
                "Kbuild",
                "Makefile",
                "MAKEFILE",
                "Makefile.am",
                "Makefile.boot",
                "Makefile.frag",
                "Makefile*.in",
                "Makefile.inc",
                "Makefile.wat",
                "makefile",
                "makefile.sco",
                "mkfile",
            ],
            #[cfg(feature = "lang-markdown")]
            Language::Markdown => &["*.md", ".MD", "README", "LICENSE"],
            #[cfg(feature = "lang-markdown-inline")]
            Language::MarkdownInline => &[],
            #[cfg(feature = "lang-nix")]
            Language::Nix => &["*.nix"],
            #[cfg(feature = "lang-objc")]
            Language::ObjC => &["*.m", "*.objc"],
            #[cfg(feature = "lang-ocaml")]
            Language::OCaml => &["*.ml"],
            #[cfg(feature = "lang-ocaml")]
            Language::OCamlInterface => &["*.mli"],
            #[cfg(feature = "lang-perl")]
            Language::Perl => &["*.pm", "*.pl", "*.t"],
            #[cfg(feature = "lang-php")]
            Language::Php => &[
                "*.php", "*.phtml", "*.php3", "*.php4", "*.php5", "*.php7", "*.phps",
            ],
            #[cfg(feature = "lang-powershell")]
            Language::PowerShell => &["*.ps1", "*.psm1"],
            #[cfg(feature = "lang-protobuf")]
            Language::ProtoBuf => &["*.proto", "*.protobuf", "*.proto2", "*.proto3"],
            Language::PlainText => &[],
            #[cfg(feature = "lang-python")]
            Language::Python => &["*.py", "*.py3", "*.pyi", "*.bzl", "TARGETS", "BUCK", "DEPS"],
            #[cfg(feature = "lang-r")]
            Language::R => &["*.R", "*.r", "*.rd", "*.rsx", ".Rprofile", "expr-dist"],
            #[cfg(feature = "lang-regex")]
            Language::Regex => &["*.regex"],
            #[cfg(feature = "lang-ruby")]
            Language::Ruby => &[
                "*.rb",
                "*.builder",
                "*.spec",
                "*.rake",
                "Gemfile",
                "Rakefile",
            ],
            #[cfg(feature = "lang-rust")]
            Language::Rust => &["*.rs"],
            #[cfg(feature = "lang-scala")]
            Language::Scala => &["*.scala", "*.sbt", "*.sc"],
            #[cfg(feature = "lang-scss")]
            Language::SCSS => &["*.scss"],
            #[cfg(feature = "lang-sql")]
            Language::SQL => &["*.sql", "*.pgsql"],
            #[cfg(feature = "lang-surface")]
            Language::Surface => &["*.surface", "*.sface"],
            #[cfg(feature = "lang-svelte")]
            Language::Svelte => &["*.svelte"],
            #[cfg(feature = "lang-swift")]
            Language::Swift => &["*.swift"],
            #[cfg(feature = "lang-toml")]
            Language::Toml => &[
                "*.toml",
                "Cargo.lock",
                "Gopkg.lock",
                "Pipfile",
                "pdm.lock",
                "poetry.lock",
                "uv.lock",
            ],
            #[cfg(feature = "lang-typescript")]
            Language::TypeScript => &["*.ts"],
            #[cfg(feature = "lang-tsx")]
            Language::Tsx => &["*.tsx"],
            #[cfg(feature = "lang-typst")]
            Language::Typst => &["*.typ", "*.typst"],
            #[cfg(feature = "lang-vim")]
            Language::Vim => &["*.vim", "*.viml"],
            #[cfg(feature = "lang-vue")]
            Language::Vue => &["*.vue"],
            #[cfg(feature = "lang-xml")]
            Language::XML => &[
                "*.ant",
                "*.csproj",
                // Following GitHub, treat MJML as XML.
                // https://documentation.mjml.io/
                "*.mjml",
                "*.plist",
                "*.resx",
                "*.svg",
                "*.ui",
                "*.vbproj",
                "*.xaml",
                "*.xml",
                "*.xsd",
                "*.xsl",
                "*.xslt",
                "*.zcml",
                "*.rng",
                "App.config",
                "nuget.config",
                "packages.config",
                ".classpath",
                ".cproject",
                ".project",
            ],
            #[cfg(feature = "lang-yaml")]
            Language::YAML => &["*.yaml", "*.yml"],
            #[cfg(feature = "lang-zig")]
            Language::Zig => &["*.zig"],
        };

        glob_strs
            .iter()
            .map(|name| glob::Pattern::new(name).expect("failed to guess language by path"))
            .collect()
    }

    /// Try to guess the language based on an Emacs mode comment at the
    /// beginning of the file.
    ///
    /// <https://www.gnu.org/software/emacs/manual/html_node/emacs/Choosing-Modes.html>
    /// <https://www.gnu.org/software/emacs/manual/html_node/emacs/Specifying-File-Variables.html>
    fn from_emacs_mode_header(src: &str) -> Option<Language> {
        lazy_static! {
            static ref MODE_RE: Regex = Regex::new(r"-\*-.*mode:([^;]+?);.*-\*-").unwrap();
            static ref SHORTHAND_RE: Regex = Regex::new(r"-\*-(.+)-\*-").unwrap();
        }

        // Emacs allows the mode header to occur on the second line if the
        // first line is a shebang.
        for line in split_on_newlines(src).take(2) {
            let mode_name: String = match (MODE_RE.captures(line), SHORTHAND_RE.captures(line)) {
                (Some(cap), _) | (_, Some(cap)) => cap[1].into(),
                _ => "".into(),
            };
            let lang = match mode_name.to_ascii_lowercase().trim() {
                #[cfg(feature = "lang-c")]
                "c" => Some(Language::C),
                #[cfg(feature = "lang-clojure")]
                "clojure" => Some(Language::Clojure),
                #[cfg(feature = "lang-csharp")]
                "csharp" => Some(Language::CSharp),
                #[cfg(feature = "lang-csv")]
                "csv" => Some(Language::CSV),
                #[cfg(feature = "lang-css")]
                "css" => Some(Language::CSS),
                #[cfg(feature = "lang-cpp")]
                "c++" => Some(Language::CPlusPlus),
                #[cfg(feature = "lang-elixir")]
                "elixir" => Some(Language::Elixir),
                #[cfg(feature = "lang-elm")]
                "elm" => Some(Language::Elm),
                #[cfg(feature = "lang-fsharp")]
                "fsharp" => Some(Language::FSharp),
                #[cfg(feature = "lang-gleam")]
                "gleam" => Some(Language::Gleam),
                #[cfg(feature = "lang-go")]
                "go" => Some(Language::Go),
                #[cfg(feature = "lang-haskell")]
                "haskell" => Some(Language::Haskell),
                #[cfg(feature = "lang-hcl")]
                "hcl" => Some(Language::HCL),
                #[cfg(feature = "lang-html")]
                "html" => Some(Language::HTML),
                #[cfg(feature = "lang-java")]
                "java" => Some(Language::Java),
                #[cfg(feature = "lang-javascript")]
                "js" | "js2" => Some(Language::JavaScript),
                #[cfg(feature = "lang-commonlisp")]
                "lisp" => Some(Language::CommonLisp),
                #[cfg(feature = "lang-nix")]
                "nix" => Some(Language::Nix),
                #[cfg(feature = "lang-xml")]
                "nxml" => Some(Language::XML),
                #[cfg(feature = "lang-objc")]
                "objc" => Some(Language::ObjC),
                #[cfg(feature = "lang-perl")]
                "perl" => Some(Language::Perl),
                #[cfg(feature = "lang-python")]
                "python" => Some(Language::Python),
                #[cfg(feature = "lang-ruby")]
                "ruby" => Some(Language::Ruby),
                #[cfg(feature = "lang-rust")]
                "rust" => Some(Language::Rust),
                #[cfg(feature = "lang-scala")]
                "scala" => Some(Language::Scala),
                #[cfg(feature = "lang-scss")]
                "scss" => Some(Language::SCSS),
                #[cfg(feature = "lang-bash")]
                "sh" => Some(Language::Bash),
                #[cfg(feature = "lang-sql")]
                "sql" => Some(Language::SQL),
                #[cfg(feature = "lang-surface")]
                "surface" => Some(Language::Surface),
                #[cfg(feature = "lang-swift")]
                "swift" => Some(Language::Swift),
                #[cfg(feature = "lang-toml")]
                "toml" => Some(Language::Toml),
                #[cfg(feature = "lang-typescript")]
                "typescript" => Some(Language::TypeScript),
                #[cfg(feature = "lang-tsx")]
                "tsx" => Some(Language::Tsx),
                #[cfg(feature = "lang-ocaml")]
                "tuareg" => Some(Language::OCaml),
                // "typescript" => Some(Language::TypeScript),
                #[cfg(feature = "lang-yaml")]
                "yaml" => Some(Language::YAML),
                #[cfg(feature = "lang-zig")]
                "zig" => Some(Language::Zig),
                _ => None,
            };
            if lang.is_some() {
                return lang;
            }
        }

        None
    }

    fn from_shebang(src: &str) -> Option<Language> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#! *(?:/usr/bin/env )?([^ ]+)").unwrap();
        }
        if let Some(first_line) = split_on_newlines(src).next() {
            if let Some(cap) = RE.captures(first_line) {
                let interpreter_path = Path::new(&cap[1]);
                if let Some(name) = interpreter_path.file_name() {
                    match name.to_string_lossy().as_ref() {
                        #[cfg(feature = "lang-typescript")]
                        "deno" | "ts-node" => return Some(Language::TypeScript),
                        #[cfg(feature = "lang-ocaml")]
                        "ocaml" | "ocamlrun" | "ocamlscript" => return Some(Language::OCaml),
                        #[cfg(feature = "lang-commonlisp")]
                        "lisp" | "sbc" | "ccl" | "clisp" | "ecl" => {
                            return Some(Language::CommonLisp);
                        }
                        #[cfg(feature = "lang-haskell")]
                        "runghc" | "runhaskell" | "runhugs" => return Some(Language::Haskell),
                        #[cfg(feature = "lang-bash")]
                        "ash" | "bash" | "dash" | "ksh" | "mksh" | "pdksh" | "rc" | "sh"
                        | "zsh" => return Some(Language::Bash),
                        #[cfg(feature = "lang-elixir")]
                        "elixir" => return Some(Language::Elixir),
                        #[cfg(feature = "lang-r")]
                        "Rscript" => return Some(Language::R),
                        #[cfg(feature = "lang-python")]
                        "python" | "python2" | "python3" => return Some(Language::Python),
                        #[cfg(feature = "lang-perl")]
                        "perl" => return Some(Language::Perl),
                        #[cfg(feature = "lang-ruby")]
                        "ruby" | "macruby" | "rake" | "jruby" | "rbx" => {
                            return Some(Language::Ruby);
                        }
                        #[cfg(feature = "lang-swift")]
                        "swift" => return Some(Language::Swift),
                        #[cfg(feature = "lang-c")]
                        "tcc" => return Some(Language::C),
                        _ => {}
                    }
                }
            }
        }

        None
    }

    /// Use a heuristic to determine if a '.h' file looks like Objective-C.
    /// We look for a line starting with '#import', '@interface' or '@protocol'
    /// near the top of the file.  These keywords are not valid C or C++, so this
    /// should not produce false positives.
    fn looks_like_objc(path: &Path, src: &str) -> bool {
        if let Some(extension) = path.extension() {
            if extension == "h" {
                return split_on_newlines(src).take(100).any(|line| {
                    ["#import", "@interface", "@protocol"]
                        .iter()
                        .any(|keyword| line.starts_with(keyword))
                });
            }
        }

        false
    }

    fn looks_like_xml(src: &str) -> bool {
        src.to_lowercase().starts_with("<?xml")
    }

    fn looks_like_html(src: &str) -> bool {
        src.to_lowercase().starts_with("<!doctype html")
    }

    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "lang-angular")]
            Language::Angular => "Angular",
            #[cfg(feature = "lang-asm")]
            Language::Assembly => "Assembly",
            #[cfg(feature = "lang-astro")]
            Language::Astro => "Astro",
            #[cfg(feature = "lang-bash")]
            Language::Bash => "Bash",
            #[cfg(feature = "lang-c")]
            Language::C => "C",
            #[cfg(feature = "lang-caddy")]
            Language::Caddy => "Caddy",
            #[cfg(feature = "lang-clojure")]
            Language::Clojure => "Clojure",
            #[cfg(feature = "lang-comment")]
            Language::Comment => "Comment",
            #[cfg(feature = "lang-commonlisp")]
            Language::CommonLisp => "Common Lisp",
            #[cfg(feature = "lang-cmake")]
            Language::CMake => "CMake",
            #[cfg(feature = "lang-csharp")]
            Language::CSharp => "C#",
            #[cfg(feature = "lang-csv")]
            Language::CSV => "CSV",
            #[cfg(feature = "lang-cpp")]
            Language::CPlusPlus => "C++",
            #[cfg(feature = "lang-css")]
            Language::CSS => "CSS",
            #[cfg(feature = "lang-dart")]
            Language::Dart => "Dart",
            Language::Diff => "Diff",
            #[cfg(feature = "lang-dockerfile")]
            Language::Dockerfile => "Dockerfile",
            #[cfg(feature = "lang-eex")]
            Language::EEx => "Eex",
            #[cfg(feature = "lang-ejs")]
            Language::EJS => "EJS",
            #[cfg(feature = "lang-erb")]
            Language::ERB => "ERB",
            #[cfg(feature = "lang-elixir")]
            Language::Elixir => "Elixir",
            #[cfg(feature = "lang-elm")]
            Language::Elm => "Elm",
            #[cfg(feature = "lang-erlang")]
            Language::Erlang => "Erlang",
            #[cfg(feature = "lang-fish")]
            Language::Fish => "Fish",
            #[cfg(feature = "lang-fsharp")]
            Language::FSharp => "F#",
            #[cfg(feature = "lang-gleam")]
            Language::Gleam => "Gleam",
            #[cfg(feature = "lang-glimmer")]
            Language::Glimmer => "Glimmer",
            #[cfg(feature = "lang-go")]
            Language::Go => "Go",
            #[cfg(feature = "lang-graphql")]
            Language::GraphQL => "GraphQL",
            #[cfg(feature = "lang-haskell")]
            Language::Haskell => "Haskell",
            #[cfg(feature = "lang-hcl")]
            Language::HCL => "HCL",
            #[cfg(feature = "lang-heex")]
            Language::HEEx => "HEEx",
            #[cfg(feature = "lang-html")]
            Language::HTML => "HTML",
            #[cfg(feature = "lang-iex")]
            Language::IEx => "IEx",
            #[cfg(feature = "lang-java")]
            Language::Java => "Java",
            #[cfg(feature = "lang-javascript")]
            Language::JavaScript => "JavaScript",
            #[cfg(feature = "lang-json")]
            Language::JSON => "JSON",
            #[cfg(feature = "lang-kotlin")]
            Language::Kotlin => "Kotlin",
            #[cfg(feature = "lang-latex")]
            Language::LaTeX => "LaTeX",
            #[cfg(feature = "lang-liquid")]
            Language::Liquid => "Liquid",
            #[cfg(feature = "lang-llvm")]
            Language::Llvm => "LLVM",
            #[cfg(feature = "lang-lua")]
            Language::Lua => "Lua",
            #[cfg(feature = "lang-objc")]
            Language::ObjC => "Objective-C",
            #[cfg(feature = "lang-ocaml")]
            Language::OCaml => "OCaml",
            #[cfg(feature = "lang-ocaml")]
            Language::OCamlInterface => "OCaml Interface",
            #[cfg(feature = "lang-make")]
            Language::Make => "Make",
            #[cfg(feature = "lang-markdown")]
            Language::Markdown => "Markdown",
            #[cfg(feature = "lang-markdown-inline")]
            Language::MarkdownInline => "Markdown Inline",
            #[cfg(feature = "lang-nix")]
            Language::Nix => "Nix",
            #[cfg(feature = "lang-perl")]
            Language::Perl => "Perl",
            #[cfg(feature = "lang-php")]
            Language::Php => "PHP",
            Language::PlainText => "Plain Text",
            #[cfg(feature = "lang-powershell")]
            Language::PowerShell => "PowerShell",
            #[cfg(feature = "lang-protobuf")]
            Language::ProtoBuf => "Protocol Buffer",
            #[cfg(feature = "lang-python")]
            Language::Python => "Python",
            #[cfg(feature = "lang-r")]
            Language::R => "R",
            #[cfg(feature = "lang-regex")]
            Language::Regex => "Regex",
            #[cfg(feature = "lang-ruby")]
            Language::Ruby => "Ruby",
            #[cfg(feature = "lang-rust")]
            Language::Rust => "Rust",
            #[cfg(feature = "lang-scala")]
            Language::Scala => "Scala",
            #[cfg(feature = "lang-scss")]
            Language::SCSS => "SCSS",
            #[cfg(feature = "lang-sql")]
            Language::SQL => "SQL",
            #[cfg(feature = "lang-surface")]
            Language::Surface => "Surface",
            #[cfg(feature = "lang-svelte")]
            Language::Svelte => "Svelte",
            #[cfg(feature = "lang-swift")]
            Language::Swift => "Swift",
            #[cfg(feature = "lang-toml")]
            Language::Toml => "TOML",
            #[cfg(feature = "lang-typescript")]
            Language::TypeScript => "TypeScript",
            #[cfg(feature = "lang-tsx")]
            Language::Tsx => "TSX",
            #[cfg(feature = "lang-typst")]
            Language::Typst => "Typst",
            #[cfg(feature = "lang-vim")]
            Language::Vim => "Vim",
            #[cfg(feature = "lang-vue")]
            Language::Vue => "Vue",
            #[cfg(feature = "lang-xml")]
            Language::XML => "XML",
            #[cfg(feature = "lang-yaml")]
            Language::YAML => "YAML",
            #[cfg(feature = "lang-zig")]
            Language::Zig => "Zig",
        }
    }

    pub fn id_name(&self) -> String {
        self.name().to_ascii_lowercase().replace(" ", "")
    }

    pub fn config(&self) -> &'static HighlightConfiguration {
        match self {
            #[cfg(feature = "lang-angular")]
            Language::Angular => &ANGULAR_CONFIG,
            #[cfg(feature = "lang-asm")]
            Language::Assembly => &ASM_CONFIG,
            #[cfg(feature = "lang-astro")]
            Language::Astro => &ASTRO_CONFIG,
            #[cfg(feature = "lang-bash")]
            Language::Bash => &BASH_CONFIG,
            #[cfg(feature = "lang-c")]
            Language::C => &C_CONFIG,
            #[cfg(feature = "lang-caddy")]
            Language::Caddy => &CADDY_CONFIG,
            #[cfg(feature = "lang-clojure")]
            Language::Clojure => &CLOJURE_CONFIG,
            #[cfg(feature = "lang-comment")]
            Language::Comment => &COMMENT_CONFIG,
            #[cfg(feature = "lang-commonlisp")]
            Language::CommonLisp => &COMMONLISP_CONFIG,
            #[cfg(feature = "lang-cmake")]
            Language::CMake => &CMAKE_CONFIG,
            #[cfg(feature = "lang-csharp")]
            Language::CSharp => &CSHARP_CONFIG,
            #[cfg(feature = "lang-csv")]
            Language::CSV => &CSV_CONFIG,
            #[cfg(feature = "lang-cpp")]
            Language::CPlusPlus => &CPP_CONFIG,
            #[cfg(feature = "lang-css")]
            Language::CSS => &CSS_CONFIG,
            #[cfg(feature = "lang-dart")]
            Language::Dart => &DART_CONFIG,
            Language::Diff => &DIFF_CONFIG,
            #[cfg(feature = "lang-dockerfile")]
            Language::Dockerfile => &DOCKERFILE_CONFIG,
            #[cfg(feature = "lang-eex")]
            Language::EEx => &EEX_CONFIG,
            #[cfg(feature = "lang-ejs")]
            Language::EJS => &EJS_CONFIG,
            #[cfg(feature = "lang-erb")]
            Language::ERB => &ERB_CONFIG,
            #[cfg(feature = "lang-elixir")]
            Language::Elixir => &ELIXIR_CONFIG,
            #[cfg(feature = "lang-elm")]
            Language::Elm => &ELM_CONFIG,
            #[cfg(feature = "lang-erlang")]
            Language::Erlang => &ERLANG_CONFIG,
            #[cfg(feature = "lang-fish")]
            Language::Fish => &FISH_CONFIG,
            #[cfg(feature = "lang-fsharp")]
            Language::FSharp => &FSHARP_CONFIG,
            #[cfg(feature = "lang-gleam")]
            Language::Gleam => &GLEAM_CONFIG,
            #[cfg(feature = "lang-glimmer")]
            Language::Glimmer => &GLIMMER_CONFIG,
            #[cfg(feature = "lang-go")]
            Language::Go => &GO_CONFIG,
            #[cfg(feature = "lang-graphql")]
            Language::GraphQL => &GRAPHQL_CONFIG,
            #[cfg(feature = "lang-haskell")]
            Language::Haskell => &HASKELL_CONFIG,
            #[cfg(feature = "lang-hcl")]
            Language::HCL => &HCL_CONFIG,
            #[cfg(feature = "lang-heex")]
            Language::HEEx => &HEEX_CONFIG,
            #[cfg(feature = "lang-html")]
            Language::HTML => &HTML_CONFIG,
            #[cfg(feature = "lang-iex")]
            Language::IEx => &IEX_CONFIG,
            #[cfg(feature = "lang-java")]
            Language::Java => &JAVA_CONFIG,
            #[cfg(feature = "lang-javascript")]
            Language::JavaScript => &JAVASCRIPT_CONFIG,
            #[cfg(feature = "lang-json")]
            Language::JSON => &JSON_CONFIG,
            #[cfg(feature = "lang-kotlin")]
            Language::Kotlin => &KOTLIN_CONFIG,
            #[cfg(feature = "lang-latex")]
            Language::LaTeX => &LATEX_CONFIG,
            #[cfg(feature = "lang-liquid")]
            Language::Liquid => &LIQUID_CONFIG,
            #[cfg(feature = "lang-llvm")]
            Language::Llvm => &LLVM_CONFIG,
            #[cfg(feature = "lang-lua")]
            Language::Lua => &LUA_CONFIG,
            #[cfg(feature = "lang-objc")]
            Language::ObjC => &OBJC_CONFIG,
            #[cfg(feature = "lang-ocaml")]
            Language::OCaml => &OCAML_CONFIG,
            #[cfg(feature = "lang-ocaml")]
            Language::OCamlInterface => &OCAML_INTERFACE_CONFIG,
            #[cfg(feature = "lang-make")]
            Language::Make => &MAKE_CONFIG,
            #[cfg(feature = "lang-markdown")]
            Language::Markdown => &MARKDOWN_CONFIG,
            #[cfg(feature = "lang-markdown-inline")]
            Language::MarkdownInline => &MARKDOWN_INLINE_CONFIG,
            #[cfg(feature = "lang-nix")]
            Language::Nix => &NIX_CONFIG,
            #[cfg(feature = "lang-perl")]
            Language::Perl => &PERL_CONFIG,
            #[cfg(feature = "lang-php")]
            Language::Php => &PHP_CONFIG,
            #[cfg(feature = "lang-powershell")]
            Language::PowerShell => &POWERSHELL_CONFIG,
            #[cfg(feature = "lang-protobuf")]
            Language::ProtoBuf => &PROTO_BUF_CONFIG,
            #[cfg(feature = "lang-python")]
            Language::Python => &PYTHON_CONFIG,
            #[cfg(feature = "lang-r")]
            Language::R => &R_CONFIG,
            #[cfg(feature = "lang-regex")]
            Language::Regex => &REGEX_CONFIG,
            #[cfg(feature = "lang-ruby")]
            Language::Ruby => &RUBY_CONFIG,
            #[cfg(feature = "lang-rust")]
            Language::Rust => &RUST_CONFIG,
            #[cfg(feature = "lang-scala")]
            Language::Scala => &SCALA_CONFIG,
            #[cfg(feature = "lang-scss")]
            Language::SCSS => &SCSS_CONFIG,
            #[cfg(feature = "lang-sql")]
            Language::SQL => &SQL_CONFIG,
            #[cfg(feature = "lang-surface")]
            Language::Surface => &SURFACE_CONFIG,
            #[cfg(feature = "lang-svelte")]
            Language::Svelte => &SVELTE_CONFIG,
            #[cfg(feature = "lang-swift")]
            Language::Swift => &SWIFT_CONFIG,
            #[cfg(feature = "lang-toml")]
            Language::Toml => &TOML_CONFIG,
            #[cfg(feature = "lang-typescript")]
            Language::TypeScript => &TYPESCRIPT_CONFIG,
            #[cfg(feature = "lang-tsx")]
            Language::Tsx => &TSX_CONFIG,
            #[cfg(feature = "lang-typst")]
            Language::Typst => &TYPST_CONFIG,
            #[cfg(feature = "lang-vim")]
            Language::Vim => &VIM_CONFIG,
            #[cfg(feature = "lang-vue")]
            Language::Vue => &VUE_CONFIG,
            #[cfg(feature = "lang-xml")]
            Language::XML => &XML_CONFIG,
            #[cfg(feature = "lang-yaml")]
            Language::YAML => &YAML_CONFIG,
            #[cfg(feature = "lang-zig")]
            Language::Zig => &ZIG_CONFIG,
            _ => &PLAIN_TEXT_CONFIG,
        }
    }
}

/// Returns a HashMap containing all supported languages with their details.
///
/// The key is the language's id_name (lowercase, no spaces).
/// The value is a tuple containing:
/// - The friendly name (e.g. "Elixir", "Common Lisp")
/// - A Vec of file extensions/patterns
///
/// # Examples
///
/// ## Basic usage - listing all languages
///
/// ```rust
/// use autumnus::languages::available_languages;
///
/// let languages = available_languages();
/// println!("Supported languages: {}", languages.len());
///
/// // Check if specific languages are supported
/// assert!(languages.contains_key("rust"));
/// assert!(languages.contains_key("elixir"));
/// assert!(languages.contains_key("javascript"));
/// ```
///
/// ## Getting language information
///
/// ```rust
/// use autumnus::languages::available_languages;
///
/// let languages = available_languages();
///
/// // Get details for Rust
/// let (name, extensions) = &languages["rust"];
/// assert_eq!(name, "Rust");
/// assert!(extensions.contains(&"*.rs".to_string()));
///
/// // Get details for Elixir
/// let (name, extensions) = &languages["elixir"];
/// assert_eq!(name, "Elixir");
/// assert!(extensions.contains(&"*.ex".to_string()));
/// assert!(extensions.contains(&"*.exs".to_string()));
///
/// // Languages with special characters in names
/// let (name, _) = &languages["c#"];
/// assert_eq!(name, "C#");
///
/// let (name, _) = &languages["c++"];
/// assert_eq!(name, "C++");
/// ```
///
/// ## Building a language selector UI
///
/// ```rust
/// use autumnus::languages::available_languages;
///
/// let languages = available_languages();
/// let mut sorted_languages: Vec<_> = languages.iter().collect();
/// sorted_languages.sort_by(|a, b| a.1.0.cmp(&b.1.0)); // Sort by friendly name
///
/// for (id, (name, extensions)) in sorted_languages {
///     println!("{} ({}): {}", name, id, extensions.join(", "));
/// }
/// ```
///
/// ## Finding languages by file extension
///
/// ```rust
/// use autumnus::languages::available_languages;
///
/// let languages = available_languages();
/// let target_extension = "*.py";
///
/// let python_languages: Vec<_> = languages
///     .iter()
///     .filter(|(_, (_, extensions))| extensions.contains(&target_extension.to_string()))
///     .collect();
///
/// assert!(!python_languages.is_empty());
/// let (id, (name, _)) = python_languages[0];
/// assert_eq!(id, "python");
/// assert_eq!(name, "Python");
/// ```
pub fn available_languages() -> HashMap<String, (String, Vec<String>)> {
    let mut languages = HashMap::new();

    for language in Language::iter() {
        let id_name = language.id_name();
        let friendly_name = language.name().to_string();
        let extensions: Vec<String> = Language::language_globs(language)
            .iter()
            .map(|p| p.to_string())
            .collect();

        languages.insert(id_name, (friendly_name, extensions));
    }

    languages
}

fn split_on_newlines(s: &str) -> impl Iterator<Item = &str> {
    s.split('\n').map(|l| {
        if let Some(l) = l.strip_suffix('\r') {
            l
        } else {
            l
        }
    })
}

#[cfg(feature = "lang-angular")]
static ANGULAR_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_angular) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "angular",
        ANGULAR_HIGHLIGHTS,
        ANGULAR_INJECTIONS,
        ANGULAR_LOCALS,
    )
    .expect("failed to create angular highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-asm")]
static ASM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_asm::LANGUAGE),
        "asm",
        ASM_HIGHLIGHTS,
        ASM_INJECTIONS,
        ASM_LOCALS,
    )
    .expect("failed to create asm highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-astro")]
static ASTRO_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_astro) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "astro",
        ASTRO_HIGHLIGHTS,
        ASTRO_INJECTIONS,
        ASTRO_LOCALS,
    )
    .expect("failed to create astro highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-bash")]
static BASH_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_bash::LANGUAGE),
        "bash",
        BASH_HIGHLIGHTS,
        BASH_INJECTIONS,
        BASH_LOCALS,
    )
    .expect("failed to create bash highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-c")]
static C_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_c::LANGUAGE),
        "c",
        C_HIGHLIGHTS,
        C_INJECTIONS,
        C_LOCALS,
    )
    .expect("failed to create c highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-caddy")]
static CADDY_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_caddy) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "caddy",
        CADDY_HIGHLIGHTS,
        CADDY_INJECTIONS,
        CADDY_LOCALS,
    )
    .expect("failed to create caddy highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-clojure")]
static CLOJURE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_clojure) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "clojure",
        CLOJURE_HIGHLIGHTS,
        CLOJURE_INJECTIONS,
        CLOJURE_LOCALS,
    )
    .expect("failed to create clojure highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-comment")]
static COMMENT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_comment::LANGUAGE),
        "comment",
        COMMENT_HIGHLIGHTS,
        COMMENT_INJECTIONS,
        COMMENT_LOCALS,
    )
    .expect("failed to create comment highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-commonlisp")]
static COMMONLISP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_commonlisp) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "common_lisp",
        COMMONLISP_HIGHLIGHTS,
        COMMONLISP_INJECTIONS,
        COMMONLISP_LOCALS,
    )
    .expect("failed to create common_lisp highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-cmake")]
static CMAKE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_cmake::LANGUAGE),
        "cmake",
        CMAKE_HIGHLIGHTS,
        CMAKE_INJECTIONS,
        CMAKE_LOCALS,
    )
    .expect("failed to create cmake highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-csharp")]
static CSHARP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_c_sharp::LANGUAGE),
        "csharp",
        C_SHARP_HIGHLIGHTS,
        C_SHARP_INJECTIONS,
        C_SHARP_LOCALS,
    )
    .expect("failed to create csharp highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-csv")]
static CSV_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_csv) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "csv",
        CSV_HIGHLIGHTS,
        CSV_INJECTIONS,
        CSV_LOCALS,
    )
    .expect("failed to create csv highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-dart")]
static DART_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_dart) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "dart",
        DART_HIGHLIGHTS,
        DART_INJECTIONS,
        DART_LOCALS,
    )
    .expect("failed to create dart highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-cpp")]
static CPP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_cpp::LANGUAGE),
        "cpp",
        CPP_HIGHLIGHTS,
        CPP_INJECTIONS,
        CPP_LOCALS,
    )
    .expect("failed to create cpp highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-css")]
static CSS_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_css::LANGUAGE),
        "css",
        CSS_HIGHLIGHTS,
        CSS_INJECTIONS,
        CSS_LOCALS,
    )
    .expect("failed to create css highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

static DIFF_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_diff::LANGUAGE),
        "diff",
        DIFF_HIGHLIGHTS,
        DIFF_INJECTIONS,
        DIFF_LOCALS,
    )
    .expect("failed to create diff highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-dockerfile")]
static DOCKERFILE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_dockerfile) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "dockerfile",
        DOCKERFILE_HIGHLIGHTS,
        DOCKERFILE_INJECTIONS,
        DOCKERFILE_LOCALS,
    )
    .expect("failed to create dockerfile highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-eex")]
static EEX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_eex) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "eex",
        EEX_HIGHLIGHTS,
        EEX_INJECTIONS,
        EEX_LOCALS,
    )
    .expect("failed to create eex highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-fish")]
static FISH_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_fish) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "fish",
        FISH_HIGHLIGHTS,
        FISH_INJECTIONS,
        FISH_LOCALS,
    )
    .expect("failed to create fish highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-ejs")]
static EJS_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_embedded_template::LANGUAGE),
        "ejs",
        EMBEDDED_TEMPLATE_HIGHLIGHTS,
        EMBEDDED_TEMPLATE_INJECTIONS,
        EMBEDDED_TEMPLATE_LOCALS,
    )
    .expect("failed to create ejs highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-erb")]
static ERB_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_embedded_template::LANGUAGE),
        "erb",
        EMBEDDED_TEMPLATE_HIGHLIGHTS,
        EMBEDDED_TEMPLATE_INJECTIONS,
        EMBEDDED_TEMPLATE_LOCALS,
    )
    .expect("failed to create erb highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-elixir")]
static ELIXIR_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_elixir::LANGUAGE),
        "elixir",
        ELIXIR_HIGHLIGHTS,
        ELIXIR_INJECTIONS,
        ELIXIR_LOCALS,
    )
    .expect("failed to create elixir highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-elm")]
static ELM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_elm::LANGUAGE),
        "elm",
        ELM_HIGHLIGHTS,
        ELM_INJECTIONS,
        ELM_LOCALS,
    )
    .expect("failed to create elm highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-erlang")]
static ERLANG_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_erlang::LANGUAGE),
        "erlang",
        ERLANG_HIGHLIGHTS,
        ERLANG_INJECTIONS,
        ERLANG_LOCALS,
    )
    .expect("failed to create erlang highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-fsharp")]
static FSHARP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_fsharp::LANGUAGE_FSHARP),
        "fsharp",
        FSHARP_HIGHLIGHTS,
        FSHARP_INJECTIONS,
        FSHARP_LOCALS,
    )
    .expect("failed to create fsharp highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-gleam")]
static GLEAM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_gleam::LANGUAGE),
        "gleam",
        GLEAM_HIGHLIGHTS,
        GLEAM_INJECTIONS,
        GLEAM_LOCALS,
    )
    .expect("failed to create gleam highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-glimmer")]
static GLIMMER_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_glimmer) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "glimmer",
        GLIMMER_HIGHLIGHTS,
        GLIMMER_INJECTIONS,
        GLIMMER_LOCALS,
    )
    .expect("failed to create glimmer highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-go")]
static GO_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_go::LANGUAGE),
        "go",
        GO_HIGHLIGHTS,
        GO_INJECTIONS,
        GO_LOCALS,
    )
    .expect("failed to create go highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-graphql")]
static GRAPHQL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_graphql) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "graphql",
        GRAPHQL_HIGHLIGHTS,
        GRAPHQL_INJECTIONS,
        GRAPHQL_LOCALS,
    )
    .expect("failed to create graphql highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-haskell")]
static HASKELL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_haskell::LANGUAGE),
        "haskell",
        HASKELL_HIGHLIGHTS,
        HASKELL_INJECTIONS,
        HASKELL_LOCALS,
    )
    .expect("failed to create haskell highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-hcl")]
static HCL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_hcl::LANGUAGE),
        "hcl",
        HCL_HIGHLIGHTS,
        HCL_INJECTIONS,
        HCL_LOCALS,
    )
    .expect("failed to create hcl highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-heex")]
static HEEX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_heex::LANGUAGE),
        "heex",
        HEEX_HIGHLIGHTS,
        HEEX_INJECTIONS,
        HEEX_LOCALS,
    )
    .expect("failed to create heex highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-html")]
static HTML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_html::LANGUAGE),
        "html",
        HTML_HIGHLIGHTS,
        HTML_INJECTIONS,
        HTML_LOCALS,
    )
    .expect("failed to create html highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-iex")]
static IEX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_iex) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "iex",
        IEX_HIGHLIGHTS,
        IEX_INJECTIONS,
        IEX_LOCALS,
    )
    .expect("failed to create iex highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-java")]
static JAVA_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_java::LANGUAGE),
        "java",
        JAVA_HIGHLIGHTS,
        JAVA_INJECTIONS,
        JAVA_LOCALS,
    )
    .expect("failed to create java highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-javascript")]
static JAVASCRIPT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_javascript::LANGUAGE),
        "javascript",
        JAVASCRIPT_HIGHLIGHTS,
        JAVASCRIPT_INJECTIONS,
        JAVASCRIPT_LOCALS,
    )
    .expect("failed to create javascript highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-json")]
static JSON_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_json::LANGUAGE),
        "json",
        JSON_HIGHLIGHTS,
        JSON_INJECTIONS,
        JSON_LOCALS,
    )
    .expect("failed to create json highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-kotlin")]
static KOTLIN_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_kotlin) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "kotlin",
        KOTLIN_HIGHLIGHTS,
        KOTLIN_INJECTIONS,
        KOTLIN_LOCALS,
    )
    .expect("failed to create kotlin highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-latex")]
static LATEX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_latex) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "latex",
        LATEX_HIGHLIGHTS,
        LATEX_INJECTIONS,
        LATEX_LOCALS,
    )
    .expect("failed to create latex highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-liquid")]
static LIQUID_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_liquid) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "liquid",
        LIQUID_HIGHLIGHTS,
        LIQUID_INJECTIONS,
        LIQUID_LOCALS,
    )
    .expect("failed to create liquid highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-llvm")]
static LLVM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_llvm) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "llvm",
        LLVM_HIGHLIGHTS,
        LLVM_INJECTIONS,
        LLVM_LOCALS,
    )
    .expect("failed to create llvm highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-lua")]
static LUA_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_lua::LANGUAGE),
        "lua",
        LUA_HIGHLIGHTS,
        LUA_INJECTIONS,
        LUA_LOCALS,
    )
    .expect("failed to create lua highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-objc")]
static OBJC_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_objc::LANGUAGE),
        "objc",
        OBJC_HIGHLIGHTS,
        OBJC_INJECTIONS,
        OBJC_LOCALS,
    )
    .expect("failed to create objc highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-ocaml")]
static OCAML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_ocaml::LANGUAGE_OCAML),
        "ocaml",
        OCAML_HIGHLIGHTS,
        OCAML_INJECTIONS,
        OCAML_LOCALS,
    )
    .expect("failed to create ocaml highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-ocaml")]
static OCAML_INTERFACE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_ocaml::LANGUAGE_OCAML_INTERFACE),
        "ocaml_interface",
        OCAML_INTERFACE_HIGHLIGHTS,
        OCAML_INTERFACE_INJECTIONS,
        OCAML_INTERFACE_LOCALS,
    )
    .expect("failed to create ocam_interface highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-make")]
static MAKE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_make) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "make",
        MAKE_HIGHLIGHTS,
        MAKE_INJECTIONS,
        MAKE_LOCALS,
    )
    .expect("failed to create make highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-markdown")]
static MARKDOWN_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_markdown) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "markdown",
        MARKDOWN_HIGHLIGHTS,
        MARKDOWN_INJECTIONS,
        MARKDOWN_LOCALS,
    )
    .expect("failed to create markdown highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-markdown-inline")]
static MARKDOWN_INLINE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_markdown_inline) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "markdown_inline",
        MARKDOWN_INLINE_HIGHLIGHTS,
        MARKDOWN_INLINE_INJECTIONS,
        MARKDOWN_INLINE_LOCALS,
    )
    .expect("failed to create markdown highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-nix")]
static NIX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_nix::LANGUAGE),
        "nix",
        NIX_HIGHLIGHTS,
        NIX_INJECTIONS,
        NIX_LOCALS,
    )
    .expect("failed to create nix configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-perl")]
static PERL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_perl) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "perl",
        PERL_HIGHLIGHTS,
        PERL_INJECTIONS,
        PERL_LOCALS,
    )
    .expect("failed to create perl highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-php")]
static PHP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_php::LANGUAGE_PHP_ONLY),
        "php",
        PHP_ONLY_HIGHLIGHTS,
        PHP_ONLY_INJECTIONS,
        PHP_ONLY_LOCALS,
    )
    .expect("failed to create php highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-powershell")]
static POWERSHELL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_powershell::LANGUAGE),
        "poweshell",
        POWERSHELL_HIGHLIGHTS,
        POWERSHELL_INJECTIONS,
        POWERSHELL_LOCALS,
    )
    .expect("failed to create poweshell highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-protobuf")]
static PROTO_BUF_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_proto::LANGUAGE),
        "protobuf",
        PROTO_HIGHLIGHTS,
        PROTO_INJECTIONS,
        PROTO_LOCALS,
    )
    .expect("failed to create protobuf highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

static PLAIN_TEXT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_diff::LANGUAGE),
        "plaintext",
        "",
        "",
        "",
    )
    .expect("failed to create plaintext highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-python")]
static PYTHON_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_python::LANGUAGE),
        "python",
        PYTHON_HIGHLIGHTS,
        PYTHON_INJECTIONS,
        PYTHON_LOCALS,
    )
    .expect("failed to create python highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-r")]
static R_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_r::LANGUAGE),
        "r",
        R_HIGHLIGHTS,
        R_INJECTIONS,
        R_LOCALS,
    )
    .expect("failed to create r highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-regex")]
static REGEX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_regex::LANGUAGE),
        "regex",
        REGEX_HIGHLIGHTS,
        REGEX_INJECTIONS,
        REGEX_LOCALS,
    )
    .expect("failed to create regex highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-ruby")]
static RUBY_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_ruby::LANGUAGE),
        "ruby",
        RUBY_HIGHLIGHTS,
        RUBY_INJECTIONS,
        RUBY_LOCALS,
    )
    .expect("failed to create ruby highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-rust")]
static RUST_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_rust::LANGUAGE),
        "rust",
        RUST_HIGHLIGHTS,
        RUST_INJECTIONS,
        RUST_LOCALS,
    )
    .expect("failed to create rust highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-scala")]
static SCALA_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_scala::LANGUAGE),
        "scala",
        SCALA_HIGHLIGHTS,
        SCALA_INJECTIONS,
        SCALA_LOCALS,
    )
    .expect("failed to create scala highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-scss")]
static SCSS_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_scss) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "scss",
        SCSS_HIGHLIGHTS,
        SCSS_INJECTIONS,
        SCSS_LOCALS,
    )
    .expect("failed to create scss highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-sql")]
static SQL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_sequel::LANGUAGE),
        "sql",
        SQL_HIGHLIGHTS,
        SQL_INJECTIONS,
        SQL_LOCALS,
    )
    .expect("failed to create sql highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-surface")]
static SURFACE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_surface) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "surface",
        SURFACE_HIGHLIGHTS,
        SURFACE_INJECTIONS,
        SURFACE_LOCALS,
    )
    .expect("failed to create surface highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-svelte")]
static SVELTE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_svelte_ng::LANGUAGE),
        "svelte",
        SVELTE_HIGHLIGHTS,
        SVELTE_INJECTIONS,
        SVELTE_LOCALS,
    )
    .expect("failed to create svelte highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-swift")]
static SWIFT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_swift::LANGUAGE),
        "swift",
        SWIFT_HIGHLIGHTS,
        SWIFT_INJECTIONS,
        SWIFT_LOCALS,
    )
    .expect("failed to create swift highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-toml")]
static TOML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_toml_ng::LANGUAGE),
        "toml",
        TOML_HIGHLIGHTS,
        TOML_INJECTIONS,
        TOML_LOCALS,
    )
    .expect("failed to create toml highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-typescript")]
static TYPESCRIPT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_typescript::LANGUAGE_TYPESCRIPT),
        "typescript",
        TYPESCRIPT_HIGHLIGHTS,
        TYPESCRIPT_INJECTIONS,
        TYPESCRIPT_LOCALS,
    )
    .expect("failed to create typescript highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-tsx")]
static TSX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_typescript::LANGUAGE_TSX),
        "tsx",
        TSX_HIGHLIGHTS,
        TSX_INJECTIONS,
        TSX_LOCALS,
    )
    .expect("failed to create tsx highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-typst")]
static TYPST_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_typst) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "typst",
        TYPST_HIGHLIGHTS,
        TYPST_INJECTIONS,
        TYPST_LOCALS,
    )
    .expect("failed to create typst highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-vim")]
static VIM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_vim) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "vim",
        VIM_HIGHLIGHTS,
        VIM_INJECTIONS,
        VIM_LOCALS,
    )
    .expect("failed to create vim highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-vue")]
static VUE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_vue) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "vue",
        VUE_HIGHLIGHTS,
        VUE_INJECTIONS,
        VUE_LOCALS,
    )
    .expect("failed to create vue highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-xml")]
static XML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_xml::LANGUAGE_XML),
        "xml",
        XML_HIGHLIGHTS,
        XML_INJECTIONS,
        XML_LOCALS,
    )
    .expect("failed to create xml highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-yaml")]
static YAML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_yaml::LANGUAGE),
        "yaml",
        YAML_HIGHLIGHTS,
        YAML_INJECTIONS,
        YAML_LOCALS,
    )
    .expect("failed to create yaml highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-zig")]
static ZIG_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_zig::LANGUAGE),
        "zig",
        ZIG_HIGHLIGHTS,
        ZIG_INJECTIONS,
        ZIG_LOCALS,
    )
    .expect("failed to create zig highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vendor::tree_sitter_highlight::Highlighter;

    #[test]
    fn test_match_exact_name() {
        let lang = Language::guess(Some("elixir"), "");
        assert_eq!(lang.name(), "Elixir");

        let lang = Language::guess(Some("Elixir"), "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_match_filename() {
        let lang = Language::guess(Some("app.ex"), "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_match_extension() {
        let lang = Language::guess(Some(".ex"), "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_match_path_with_extension() {
        let lang = Language::guess(Some("lib/app.ex"), "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_no_match_fallbacks_to_plain_text() {
        let lang = Language::guess(Some("none"), "");
        assert_eq!(lang.name(), "Plain Text");
    }

    #[test]
    #[cfg(feature = "lang-angular")]
    fn test_angular_config_loads() {
        let lang = Language::Angular;
        let config = lang.config();
        assert_eq!(lang.name(), "Angular");

        // Verify we can create a highlighter with this config
        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-astro")]
    fn test_astro_config_loads() {
        let lang = Language::Astro;
        let config = lang.config();
        assert_eq!(lang.name(), "Astro");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-bash")]
    fn test_bash_config_loads() {
        let lang = Language::Bash;
        let config = lang.config();
        assert_eq!(lang.name(), "Bash");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-c")]
    fn test_c_config_loads() {
        let lang = Language::C;
        let config = lang.config();
        assert_eq!(lang.name(), "C");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-caddy")]
    fn test_caddy_config_loads() {
        let lang = Language::Caddy;
        let config = lang.config();
        assert_eq!(lang.name(), "Caddy");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-cmake")]
    fn test_cmake_config_loads() {
        let lang = Language::CMake;
        let config = lang.config();
        assert_eq!(lang.name(), "CMake");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-cpp")]
    fn test_cpp_config_loads() {
        let lang = Language::CPlusPlus;
        let config = lang.config();
        assert_eq!(lang.name(), "C++");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-css")]
    fn test_css_config_loads() {
        let lang = Language::CSS;
        let config = lang.config();
        assert_eq!(lang.name(), "CSS");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-csv")]
    fn test_csv_config_loads() {
        let lang = Language::CSV;
        let config = lang.config();
        assert_eq!(lang.name(), "CSV");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-csharp")]
    fn test_csharp_config_loads() {
        let lang = Language::CSharp;
        let config = lang.config();
        assert_eq!(lang.name(), "C#");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-clojure")]
    fn test_clojure_config_loads() {
        let lang = Language::Clojure;
        let config = lang.config();
        assert_eq!(lang.name(), "Clojure");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-comment")]
    fn test_comment_config_loads() {
        let lang = Language::Comment;
        let config = lang.config();
        assert_eq!(lang.name(), "Comment");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-commonlisp")]
    fn test_commonlisp_config_loads() {
        let lang = Language::CommonLisp;
        let config = lang.config();
        assert_eq!(lang.name(), "Common Lisp");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    fn test_diff_config_loads() {
        let lang = Language::Diff;
        let config = lang.config();
        assert_eq!(lang.name(), "Diff");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-dockerfile")]
    fn test_dockerfile_config_loads() {
        let lang = Language::Dockerfile;
        let config = lang.config();
        assert_eq!(lang.name(), "Dockerfile");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-eex")]
    fn test_eex_config_loads() {
        let lang = Language::EEx;
        let config = lang.config();
        assert_eq!(lang.name(), "Eex");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-ejs")]
    fn test_ejs_config_loads() {
        let lang = Language::EJS;
        let config = lang.config();
        assert_eq!(lang.name(), "EJS");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-erb")]
    fn test_erb_config_loads() {
        let lang = Language::ERB;
        let config = lang.config();
        assert_eq!(lang.name(), "ERB");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-elixir")]
    fn test_elixir_config_loads() {
        let lang = Language::Elixir;
        let config = lang.config();
        assert_eq!(lang.name(), "Elixir");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-elm")]
    fn test_elm_config_loads() {
        let lang = Language::Elm;
        let config = lang.config();
        assert_eq!(lang.name(), "Elm");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-erlang")]
    fn test_erlang_config_loads() {
        let lang = Language::Erlang;
        let config = lang.config();
        assert_eq!(lang.name(), "Erlang");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-fish")]
    fn test_fish_config_loads() {
        let lang = Language::Fish;
        let config = lang.config();
        assert_eq!(lang.name(), "Fish");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-fsharp")]
    fn test_fsharp_config_loads() {
        let lang = Language::FSharp;
        let config = lang.config();
        assert_eq!(lang.name(), "F#");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-gleam")]
    fn test_gleam_config_loads() {
        let lang = Language::Gleam;
        let config = lang.config();
        assert_eq!(lang.name(), "Gleam");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-glimmer")]
    fn test_glimmer_config_loads() {
        let lang = Language::Glimmer;
        let config = lang.config();
        assert_eq!(lang.name(), "Glimmer");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-go")]
    fn test_go_config_loads() {
        let lang = Language::Go;
        let config = lang.config();
        assert_eq!(lang.name(), "Go");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-graphql")]
    fn test_graphql_config_loads() {
        let lang = Language::GraphQL;
        let config = lang.config();
        assert_eq!(lang.name(), "GraphQL");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-heex")]
    fn test_heex_config_loads() {
        let lang = Language::HEEx;
        let config = lang.config();
        assert_eq!(lang.name(), "HEEx");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-html")]
    fn test_html_config_loads() {
        let lang = Language::HTML;
        let config = lang.config();
        assert_eq!(lang.name(), "HTML");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-haskell")]
    fn test_haskell_config_loads() {
        let lang = Language::Haskell;
        let config = lang.config();
        assert_eq!(lang.name(), "Haskell");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-hcl")]
    fn test_hcl_config_loads() {
        let lang = Language::HCL;
        let config = lang.config();
        assert_eq!(lang.name(), "HCL");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-iex")]
    fn test_iex_config_loads() {
        let lang = Language::IEx;
        let config = lang.config();
        assert_eq!(lang.name(), "IEx");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-json")]
    fn test_json_config_loads() {
        let lang = Language::JSON;
        let config = lang.config();
        assert_eq!(lang.name(), "JSON");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-java")]
    fn test_java_config_loads() {
        let lang = Language::Java;
        let config = lang.config();
        assert_eq!(lang.name(), "Java");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-javascript")]
    fn test_javascript_config_loads() {
        let lang = Language::JavaScript;
        let config = lang.config();
        assert_eq!(lang.name(), "JavaScript");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-kotlin")]
    fn test_kotlin_config_loads() {
        let lang = Language::Kotlin;
        let config = lang.config();
        assert_eq!(lang.name(), "Kotlin");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-latex")]
    fn test_latex_config_loads() {
        let lang = Language::LaTeX;
        let config = lang.config();
        assert_eq!(lang.name(), "LaTeX");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-liquid")]
    fn test_liquid_config_loads() {
        let lang = Language::Liquid;
        let config = lang.config();
        assert_eq!(lang.name(), "Liquid");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-llvm")]
    fn test_llvm_config_loads() {
        let lang = Language::Llvm;
        let config = lang.config();
        assert_eq!(lang.name(), "LLVM");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-lua")]
    fn test_lua_config_loads() {
        let lang = Language::Lua;
        let config = lang.config();
        assert_eq!(lang.name(), "Lua");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-make")]
    fn test_make_config_loads() {
        let lang = Language::Make;
        let config = lang.config();
        assert_eq!(lang.name(), "Make");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-markdown")]
    fn test_markdown_config_loads() {
        let lang = Language::Markdown;
        let config = lang.config();
        assert_eq!(lang.name(), "Markdown");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-markdown-inline")]
    fn test_markdown_inline_config_loads() {
        let lang = Language::MarkdownInline;
        let config = lang.config();
        assert_eq!(lang.name(), "Markdown Inline");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-nix")]
    fn test_nix_config_loads() {
        let lang = Language::Nix;
        let config = lang.config();
        assert_eq!(lang.name(), "Nix");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-ocaml")]
    fn test_ocaml_config_loads() {
        let lang = Language::OCaml;
        let config = lang.config();
        assert_eq!(lang.name(), "OCaml");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-ocaml")]
    fn test_ocaml_interface_config_loads() {
        let lang = Language::OCamlInterface;
        let config = lang.config();
        assert_eq!(lang.name(), "OCaml Interface");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-objc")]
    fn test_objc_config_loads() {
        let lang = Language::ObjC;
        let config = lang.config();
        assert_eq!(lang.name(), "Objective-C");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-perl")]
    fn test_perl_config_loads() {
        let lang = Language::Perl;
        let config = lang.config();
        assert_eq!(lang.name(), "Perl");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-php")]
    fn test_php_config_loads() {
        let lang = Language::Php;
        let config = lang.config();
        assert_eq!(lang.name(), "PHP");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    fn test_plaintext_config_loads() {
        let lang = Language::PlainText;
        let config = lang.config();
        assert_eq!(lang.name(), "Plain Text");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-powershell")]
    fn test_powershell_config_loads() {
        let lang = Language::PowerShell;
        let config = lang.config();
        assert_eq!(lang.name(), "PowerShell");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-protobuf")]
    fn test_protobuf_config_loads() {
        let lang = Language::ProtoBuf;
        let config = lang.config();
        assert_eq!(lang.name(), "Protocol Buffer");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-python")]
    fn test_python_config_loads() {
        let lang = Language::Python;
        let config = lang.config();
        assert_eq!(lang.name(), "Python");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-r")]
    fn test_r_config_loads() {
        let lang = Language::R;
        let config = lang.config();
        assert_eq!(lang.name(), "R");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-regex")]
    fn test_regex_config_loads() {
        let lang = Language::Regex;
        let config = lang.config();
        assert_eq!(lang.name(), "Regex");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-ruby")]
    fn test_ruby_config_loads() {
        let lang = Language::Ruby;
        let config = lang.config();
        assert_eq!(lang.name(), "Ruby");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-rust")]
    fn test_rust_config_loads() {
        let lang = Language::Rust;
        let config = lang.config();
        assert_eq!(lang.name(), "Rust");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-scala")]
    fn test_scala_config_loads() {
        let lang = Language::Scala;
        let config = lang.config();
        assert_eq!(lang.name(), "Scala");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-scss")]
    fn test_scss_config_loads() {
        let lang = Language::SCSS;
        let config = lang.config();
        assert_eq!(lang.name(), "SCSS");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-sql")]
    fn test_sql_config_loads() {
        let lang = Language::SQL;
        let config = lang.config();
        assert_eq!(lang.name(), "SQL");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-surface")]
    fn test_surface_config_loads() {
        let lang = Language::Surface;
        let config = lang.config();
        assert_eq!(lang.name(), "Surface");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-svelte")]
    fn test_svelte_config_loads() {
        let lang = Language::Svelte;
        let config = lang.config();
        assert_eq!(lang.name(), "Svelte");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-swift")]
    fn test_swift_config_loads() {
        let lang = Language::Swift;
        let config = lang.config();
        assert_eq!(lang.name(), "Swift");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-toml")]
    fn test_toml_config_loads() {
        let lang = Language::Toml;
        let config = lang.config();
        assert_eq!(lang.name(), "TOML");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-tsx")]
    fn test_tsx_config_loads() {
        let lang = Language::Tsx;
        let config = lang.config();
        assert_eq!(lang.name(), "TSX");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-typescript")]
    fn test_typescript_config_loads() {
        let lang = Language::TypeScript;
        let config = lang.config();
        assert_eq!(lang.name(), "TypeScript");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-typst")]
    fn test_typst_config_loads() {
        let lang = Language::Typst;
        let config = lang.config();
        assert_eq!(lang.name(), "Typst");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-vim")]
    fn test_vim_config_loads() {
        let lang = Language::Vim;
        let config = lang.config();
        assert_eq!(lang.name(), "Vim");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-vue")]
    fn test_vue_config_loads() {
        let lang = Language::Vue;
        let config = lang.config();
        assert_eq!(lang.name(), "Vue");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-xml")]
    fn test_xml_config_loads() {
        let lang = Language::XML;
        let config = lang.config();
        assert_eq!(lang.name(), "XML");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-yaml")]
    fn test_yaml_config_loads() {
        let lang = Language::YAML;
        let config = lang.config();
        assert_eq!(lang.name(), "YAML");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    #[cfg(feature = "lang-zig")]
    fn test_zig_config_loads() {
        let lang = Language::Zig;
        let config = lang.config();
        assert_eq!(lang.name(), "Zig");

        let mut highlighter = Highlighter::new();
        let _ = highlighter
            .highlight(config, "".as_bytes(), None, |_| None)
            .unwrap();
    }

    #[test]
    fn test_available_languages() {
        let languages = available_languages();

        for (id_name, (friendly_name, _extensions)) in languages {
            assert!(!id_name.is_empty());
            assert!(!friendly_name.is_empty());
        }
    }
}
