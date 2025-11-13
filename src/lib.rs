//! Syntax highlighter powered by Tree-sitter and Neovim themes.
//!
//! ## Quick Start
//!
//! Use the builder pattern for type-safe, ergonomic formatter creation:
//!
//! ```rust
//! use autumnus::{HtmlInlineBuilder, languages::Language, themes, formatter::Formatter};
//! use std::io::Write;
//!
//! let code = "fn main() { println!(\"Hello, world!\"); }";
//! let theme = themes::get("dracula").unwrap();
//!
//! let formatter = HtmlInlineBuilder::new()
//!     .source(code)
//!     .lang(Language::Rust)
//!     .theme(Some(theme))
//!     .pre_class(Some("code-block"))
//!     .build()
//!     .unwrap();
//!
//! let mut output = Vec::new();
//! formatter.format(&mut output).unwrap();
//! let html = String::from_utf8(output).unwrap();
//! ```
//!
//! ## Language Feature Flags
//!
//! By default, Autumnus includes support for all languages, which can result in longer
//! compilation times. You can reduce compilation time and binary size by enabling only
//! the languages you need:
//!
//! ```toml
//! [dependencies]
//! autumnus = { version = "0.3", default-features = false, features = ["lang-rust", "lang-javascript"] }
//! ```
//!
//! Available language features include: `lang-angular`, `lang-astro`, `lang-bash`, `lang-c`,
//! `lang-cpp`, `lang-css`, `lang-elixir`, `lang-go`, `lang-html`, `lang-java`, `lang-javascript`,
//! `lang-json`, `lang-markdown`, `lang-python`, `lang-rust`, `lang-typescript`, and many more.
//!
//! Use `all-languages` to enable all language support:
//!
//! ```toml
//! [dependencies]
//! autumnus = { version = "0.3", features = ["all-languages"] }
//! ```
//!
//! ## Available Builders
//!
//! - [`HtmlInlineBuilder`] - HTML output with inline CSS styles
//! - [`HtmlLinkedBuilder`] - HTML output with CSS classes (requires external CSS)
//! - [`TerminalBuilder`] - ANSI color codes for terminal output
//!
//! ## More Examples
//!
//! See the [`formatter`] module for detailed examples and usage patterns.
//!
//! ## Languages available
//!
//! | Language | File Extensions |
//! |----------|-----------------|
//! | Angular | *.angular, component.html |
//! | Assembly | *.s, *.asm, *.assembly |
//! | Astro | *.astro |
//! | Bash | *.bash, *.bats, *.cgi, *.command, *.env, *.fcgi, *.ksh, *.sh, *.sh.in, *.tmux, *.tool, *.zsh, .bash_aliases, .bash_history, .bash_logout, .bash_profile, .bashrc, .cshrc, .env, .env.example, .flaskenv, .kshrc, .login, .profile, .zlogin, .zlogout, .zprofile, .zshenv, .zshrc, 9fs, PKGBUILD, bash_aliases, bash_logout, bash_profile, bashrc, cshrc, ebuild, eclass, gradlew, kshrc, login, man, profile, zlogin, zlogout, zprofile, zshenv, zshrc |
//! | C | *.c |
//! | Caddy | Caddyfile |
//! | CMake | *.cmake, *.cmake.in, CMakeLists.txt |
//! | C++ | *.cc, *.cpp, *.h, *.hh, *.hpp, *.ino, *.cxx, *.cu, *.hxx |
//! | CSS | *.css |
//! | CSV | *.csv |
//! | C# | *.cs |
//! | Clojure | *.bb, *.boot, *.clj, *.cljc, *.clje, *.cljs, *.cljx, *.edn, *.joke, *.joker |
//! | Comment | |
//! | Common Lisp | *.lisp, *.lsp, *.asd |
//! | Dart | *.dart |
//! | Diff | *.diff |
//! | Dockerfile | Dockerfile, dockerfile, docker, Containerfile, container, *.dockerfile, *.docker, *.container |
//! | EEx | *.eex |
//! | EJS | *.ejs |
//! | ERB | *.erb |
//! | Elixir | *.ex, *.exs |
//! | Elm | *.elm |
//! | Erlang | *.erl, *.app, *.app.src, *.es, *.escript, *.hrl, *.xrl, *.yrl, Emakefile, rebar.config |
//! | Fish | *.fish |
//! | F# | *.fs, *.fsx, *.fsi |
//! | Gleam | *.gleam |
//! | Glimmer | *.hbs, *.handlebars, *.html.handlebars, *.glimmer |
//! | Go | *.go |
//! | GraphQL | |
//! | HEEx | *.heex, *.neex |
//! | HTML | *.html, *.htm, *.xhtml |
//! | Haskell | *.hs, *.hs-boot |
//! | HCL | *.hcl, *.nomad, *.tf, *.tfvars, *.workflow |
//! | IEx | *.iex |
//! | JSON | *.json, *.avsc, *.geojson, *.gltf, *.har, *.ice, *.JSON-tmLanguage, *.jsonl, *.mcmeta, *.tfstate, *.tfstate.backup, *.topojson, *.webapp, *.webmanifest, .arcconfig, .auto-changelog, .c8rc, .htmlhintrc, .imgbotconfig, .nycrc, .tern-config, .tern-project, .watchmanconfig, Pipfile.lock, composer.lock, mcmod.info, flake.lock |
//! | Java | *.java |
//! | JavaScript | *.cjs, *.js, *.mjs, *.snap, *.jsx |
//! | Kotlin | *.kt, *.ktm, *.kts |
//! | LaTeX | *.aux, *.cls, *.sty, *.tex |
//! | Liquid | *liquid |
//! | LLVM | *.llvm, *.ll |
//! | Lua | *.lua |
//! | Make | *.mak, *.d, *.make, *.makefile, *.mk, *.mkfile, *.dsp, BSDmakefile, GNUmakefile, Kbuild, Makefile, MAKEFILE, Makefile.am, Makefile.boot, Makefile.frag, Makefile*.in, Makefile.inc, Makefile.wat, makefile, makefile.sco, mkfile |
//! | Markdown | *.md, README, LICENSE |
//! | Markdown Inline | |
//! | Nix | *.nix |
//! | OCaml | *.ml |
//! | OCaml Interface | *.mli |
//! | Objective-C | *.m, *.objc |
//! | Perl | *.pm, *.pl, *.t |
//! | PHP | *.php, *.phtml, *.php3, *.php4, *.php5, *.php7, *.phps |
//! | Plain Text | |
//! | PowerShell | *.ps1, *.psm1 |
//! | Protocol Buffer | *.proto, *.protobuf, *.proto2, *.proto3 |
//! | Python | *.py, *.py3, *.pyi, *.bzl, TARGETS, BUCK, DEPS |
//! | R | *.R, *.r, *.rd, *.rsx, .Rprofile, expr-dist |
//! | Regex | *.regex |
//! | Ruby | *.rb, *.builder, *.spec, *.rake, Gemfile, Rakefile |
//! | Rust | *.rs |
//! | SCSS | *.scss |
//! | SQL | *.sql, *.pgsql |
//! | Scala | *.scala, *.sbt, *.sc |
//! | Surface | *.surface, *.sface |
//! | Svelte | *.svelte |
//! | Swift | *.swift |
//! | TOML | *.toml, Cargo.lock, Gopkg.lock, Pipfile, pdm.lock, poetry.lock, uv.lock |
//! | TSX | *.tsx |
//! | TypeScript | *.ts |
//! | Typst | *.typ, *.typst |
//! | Vim | *.vim, *.viml |
//! | Vue | *.vue |
//! | XML | *.ant, *.csproj, *.mjml, *.plist, *.resx, *.svg, *.ui, *.vbproj, *.xaml, *.xml, *.xsd, *.xsl, *.xslt, *.zcml, *.rng, App.config, nuget.config, packages.config, .classpath, .cproject, .project |
//! | YAML | *.yaml, *.yml |
//! | Zig | *.zig |
//!
//! ## Themes available
//!
//! | Theme Name |
//! | ---------- |
//! | aura_dark |
//! | aura_dark_soft_text |
//! | aura_soft_dark |
//! | aura_soft_dark_soft_text |
//! | ayu_dark |
//! | ayu_light |
//! | ayu_mirage |
//! | bamboo_light |
//! | bamboo_multiplex |
//! | bamboo_vulgaris |
//! | bluloco_dark |
//! | bluloco_light |
//! | carbonfox |
//! | catppuccin_frappe |
//! | catppuccin_latte |
//! | catppuccin_macchiato |
//! | catppuccin_mocha |
//! | cyberdream_dark |
//! | cyberdream_light |
//! | darkplus |
//! | dawnfox |
//! | dayfox |
//! | dracula |
//! | dracula_soft |
//! | duskfox |
//! | edge_aura |
//! | edge_dark |
//! | edge_light |
//! | edge_neon |
//! | everforest_dark |
//! | everforest_light |
//! | flexoki_dark |
//! | flexoki_light |
//! | github_dark |
//! | github_dark_colorblind |
//! | github_dark_default |
//! | github_dark_dimmed |
//! | github_dark_high_contrast |
//! | github_dark_tritanopia |
//! | github_light |
//! | github_light_colorblind |
//! | github_light_default |
//! | github_light_high_contrast |
//! | github_light_tritanopia |
//! | horizon_dark |
//! | horizon_light |
//! | iceberg |
//! | gruvbox_dark |
//! | gruvbox_dark_hard |
//! | gruvbox_dark_soft |
//! | gruvbox_light |
//! | gruvbox_light_hard |
//! | gruvbox_light_soft |
//! | kanagawa_dragon |
//! | kanagawa_lotus |
//! | kanagawa_wave |
//! | material_darker |
//! | material_deep_ocean |
//! | material_lighter |
//! | material_oceanic |
//! | material_palenight |
//! | matte_black |
//! | melange_dark |
//! | melange_light |
//! | molokai |
//! | modus_operandi |
//! | modus_vivendi |
//! | monokai_pro_dark |
//! | monokai_pro_machine |
//! | monokai_pro_ristretto |
//! | monokai_pro_spectrum |
//! | moonfly |
//! | moonlight |
//! | neosolarized_dark |
//! | neosolarized_light |
//! | neovim_dark |
//! | neovim_light |
//! | nightfly |
//! | nightfox |
//! | nord |
//! | nordfox |
//! | nordic |
//! | onedark |
//! | onedark_cool |
//! | onedark_darker |
//! | onedark_deep |
//! | onedark_light |
//! | onedark_warm |
//! | onedark_warmer |
//! | onedarkpro_dark |
//! | onedarkpro_vivid |
//! | onelight |
//! | papercolor_dark |
//! | papercolor_light |
//! | rosepine_dark |
//! | rosepine_dawn |
//! | rosepine_moon |
//! | solarized_autumn_dark |
//! | solarized_autumn_light |
//! | solarized_spring_dark |
//! | solarized_spring_light |
//! | solarized_summer_dark |
//! | solarized_summer_light |
//! | solarized_winter_dark |
//! | solarized_winter_light |
//! | srcery |
//! | terafox |
//! | tokyonight_day |
//! | tokyonight_moon |
//! | tokyonight_night |
//! | tokyonight_storm |
//! | vscode_dark |
//! | vscode_light |
//! | xcode_dark |
//! | xcode_dark_hc |
//! | xcode_light |
//! | xcode_light_hc |
//! | xcode_wwdc |
//! | zenburn |
//! | zephyr_dark |

#[doc(hidden)]
pub mod constants;
pub mod formatter;
pub mod languages;
pub mod themes;

#[cfg(feature = "elixir-nif")]
#[doc(hidden)]
pub mod elixir;

use crate::formatter::Formatter;
use crate::languages::Language;
use crate::themes::Theme;
use std::io::{self, Write};

// Re-export builders for easier access
pub use crate::formatter::{HtmlInlineBuilder, HtmlLinkedBuilder, TerminalBuilder};

/// Output formatter configuration for syntax highlighting.
///
/// This enum specifies how syntax highlighted code should be formatted for output.
/// Each variant provides different output formats suitable for different use cases:
/// web pages, documentation, terminal display, etc.
///
/// # Default Behavior
///
/// The default is [`FormatterOption::HtmlInline`] with no theme, no additional CSS classes,
/// no italics, and no debug information.
///
/// # Examples
///
/// ## HTML with inline styles (default)
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption, themes};
///
/// let code = "fn hello() { println!(\"world\"); }";
///
/// let options = Options {
///     language: Some("rust"),
///     formatter: FormatterOption::HtmlInline {
///         theme: themes::get("dracula").ok(),
///         pre_class: Some("code-block"),
///         italic: true,
///         include_highlights: false,
///         highlight_lines: None,
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// // Produces: <pre class="athl code-block" style="...">...</pre>
/// ```
///
/// ## HTML with CSS classes (requires external CSS)
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption};
///
/// let code = "const greeting = 'Hello, World!';";
///
/// let options = Options {
///     language: Some("javascript"),
///     formatter: FormatterOption::HtmlLinked {
///         pre_class: Some("syntax-highlight"),
///         highlight_lines: None,
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// // Produces: <pre class="athl syntax-highlight"><code class="language-javascript">...</code></pre>
/// // Remember to include CSS: <link rel="stylesheet" href="css/your-theme.css">
/// ```
///
/// ## Terminal output with ANSI colors
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption, themes};
///
/// let code = "print('Hello from Python!')";
///
/// let options = Options {
///     language: Some("python"),
///     formatter: FormatterOption::Terminal {
///         theme: themes::get("github_light").ok(),
///     },
/// };
///
/// let ansi_output = highlight(code, options);
/// // Produces: ANSI escape codes for terminal colors
/// ```
///
/// ## Line highlighting in HTML
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption, themes};
/// use autumnus::formatter::html_inline::{HighlightLines, HighlightLinesStyle};
///
/// let code = "line 1\nline 2\nline 3\nline 4";
///
/// let highlight_lines = HighlightLines {
///     lines: vec![2..=3],  // Highlight lines 2 and 3
///     style: Some(HighlightLinesStyle::Style("background-color: yellow".to_string())),
///     class: None,
/// };
///
/// let options = Options {
///     language: Some("text"),
///     formatter: FormatterOption::HtmlInline {
///         theme: themes::get("catppuccin_mocha").ok(),
///         pre_class: None,
///         italic: false,
///         include_highlights: false,
///         highlight_lines: Some(highlight_lines),
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// ```
///
/// ## Debug mode with highlight scope names
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption};
///
/// let code = "const x = 42;";
///
/// let options = Options {
///     language: Some("javascript"),
///     formatter: FormatterOption::HtmlInline {
///         theme: None,
///         pre_class: None,
///         italic: false,
///         include_highlights: true,  // Adds data-highlight attributes
///         highlight_lines: None,
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// // Produces: <span data-highlight="keyword">const</span>
/// ```
///
/// ## HTML with custom wrapper elements
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption};
/// use autumnus::formatter::HtmlElement;
///
/// let code = "const greeting = 'Hello';";
///
/// let options = Options {
///     language: Some("javascript"),
///     formatter: FormatterOption::HtmlInline {
///         theme: None,
///         pre_class: None,
///         italic: false,
///         include_highlights: false,
///         highlight_lines: None,
///         header: Some(HtmlElement {
///             open_tag: "<div class=\"code-wrapper\" data-lang=\"js\">".to_string(),
///             close_tag: "</div>".to_string(),
///         }),
///     },
/// };
///
/// let html = highlight(code, options);
/// // Produces: <div class="code-wrapper" data-lang="js"><pre class="athl">...</pre></div>
/// ```
#[derive(Clone, Debug)]
pub enum FormatterOption<'a> {
    /// HTML output with inline CSS styles.
    ///
    /// This formatter generates HTML where syntax highlighting colors are applied
    /// directly as `style` attributes on each element. This is convenient for
    /// simple use cases where you don't want to manage external CSS files.
    ///
    /// **Pros**: Self-contained, no external dependencies, works immediately
    /// **Cons**: Larger output size, harder to customize styling globally
    HtmlInline {
        /// Theme to use for syntax highlighting colors.
        ///
        /// If `None`, elements will have no color styling applied.
        /// Use [`themes::get`] to retrieve a theme by name.
        theme: Option<&'a Theme>,

        /// Additional CSS class to add to the `<pre>` tag.
        ///
        /// The `<pre>` tag always gets the class `"athl"`. If this field is
        /// `Some("my-class")`, the final class will be `"athl my-class"`.
        pre_class: Option<&'a str>,

        /// Whether to apply italic styling to appropriate syntax elements.
        ///
        /// When `true`, elements that should be italic (like comments in many themes)
        /// will have `font-style: italic` added to their inline styles.
        italic: bool,

        /// Whether to include highlight scope names as data attributes.
        ///
        /// When `true`, each syntax element gets a `data-highlight` attribute
        /// containing the Tree-sitter highlight scope name (e.g., "keyword", "string").
        /// Useful for debugging or custom JavaScript processing.
        include_highlights: bool,

        /// Configuration for highlighting specific lines.
        ///
        /// Allows you to visually emphasize certain lines with background colors
        /// or other styling. See [`formatter::html_inline::HighlightLines`] for details.
        highlight_lines: Option<formatter::html_inline::HighlightLines>,

        /// Optional header element to wrap the entire code block.
        ///
        /// When provided, the code block will be wrapped with the specified opening
        /// and closing tags. Useful for adding custom containers, sections, or other
        /// structural elements. See [`formatter::HtmlElement`] for details.
        header: Option<formatter::HtmlElement>,
    },

    /// HTML output with CSS classes instead of inline styles.
    ///
    /// This formatter generates HTML with semantic CSS classes that you can style
    /// with external CSS files. Pre-generated CSS files for all themes are
    /// available in the `css/` directory of this crate.
    ///
    /// **Pros**: Smaller output, easier global styling, better caching
    /// **Cons**: Requires external CSS files, additional setup
    ///
    /// ## Required CSS
    ///
    /// You must include a CSS file corresponding to your desired theme:
    ///
    /// ```html
    /// <link rel="stylesheet" href="css/dracula.css">
    /// <link rel="stylesheet" href="css/github_light.css">
    /// <link rel="stylesheet" href="css/catppuccin_mocha.css">
    /// <!-- etc. -->
    /// ```
    HtmlLinked {
        /// Additional CSS class to add to the `<pre>` tag.
        ///
        /// The `<pre>` tag always gets the class `"athl"`. If this field is
        /// `Some("my-class")`, the final class will be `"athl my-class"`.
        pre_class: Option<&'a str>,

        /// Configuration for highlighting specific lines with CSS classes.
        ///
        /// Allows you to add CSS classes to specific lines for custom styling.
        /// See [`formatter::html_linked::HighlightLines`] for details.
        highlight_lines: Option<formatter::html_linked::HighlightLines>,

        /// Optional header element to wrap the entire code block.
        ///
        /// When provided, the code block will be wrapped with the specified opening
        /// and closing tags. Useful for adding custom containers, sections, or other
        /// structural elements. See [`formatter::HtmlElement`] for details.
        header: Option<formatter::HtmlElement>,
    },

    /// Terminal output with ANSI color escape codes.
    ///
    /// This formatter generates output suitable for display in terminals,
    /// using ANSI escape sequences to apply colors. The colors are derived
    /// from the theme's color definitions.
    ///
    /// **Use cases**: Command-line tools, terminal-based editors, console output
    Terminal {
        /// Theme to use for color mapping to ANSI codes.
        ///
        /// Theme colors are converted to the closest ANSI RGB equivalents.
        /// If `None`, a default color scheme is used to ensure visibility.
        theme: Option<&'a Theme>,
    },
}

impl Default for FormatterOption<'_> {
    fn default() -> Self {
        Self::HtmlInline {
            theme: None,
            pre_class: None,
            italic: false,
            include_highlights: false,
            highlight_lines: None,
            header: None,
        }
    }
}

/// Configuration options for syntax highlighting.
///
/// This struct provides all the configuration needed to highlight source code,
/// including language detection and output formatting options. It's used with
/// the [`highlight`] and [`write_highlight`] functions.
///
/// # Language Detection
///
/// The `language` field supports multiple input formats:
/// - **Language names**: `"rust"`, `"python"`, `"javascript"`
/// - **File paths**: `"src/main.rs"`, `"app.py"`, `"script.js"`
/// - **File extensions**: `"rs"`, `"py"`, `"js"`
/// - **None**: Try to auto-detect from source content
///
/// # Default Behavior
///
/// When using [`Options::default()`], you get:
/// - Automatic language detection attempt (`language: None`)
/// - HTML inline formatter with no theme (`FormatterOption::HtmlInline`)
///
/// # Examples
///
/// ## Basic usage with defaults
///
/// ```rust
/// use autumnus::{highlight, Options};
///
/// let code = r#"
/// #!/usr/bin/env python3
/// print("Hello, World!")
/// "#;
///
/// // Language auto-detected from shebang, HTML inline output
/// let html = highlight(code, Options::default());
/// ```
///
/// ## Explicit language specification
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption};
///
/// let code = "fn main() { println!(\"Hello\"); }";
///
/// let options = Options {
///     language: Some("rust"),  // Explicit language
///     formatter: FormatterOption::HtmlInline {
///         theme: None,
///         pre_class: Some("code-block"),
///         italic: false,
///         include_highlights: false,
///         highlight_lines: None,
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// ```
///
/// ## File path-based detection
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption, themes};
///
/// let code = "defmodule MyApp do\n  def hello, do: :world\nend";
///
/// let options = Options {
///     language: Some("lib/my_app.ex"),  // Language detected from .ex extension
///     formatter: FormatterOption::HtmlInline {
///         theme: themes::get("dracula").ok(),
///         pre_class: None,
///         italic: true,
///         include_highlights: false,
///         highlight_lines: None,
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// ```
///
/// ## Parsing languages from strings
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption, languages::Language};
///
/// let code = "const greeting = 'Hello, World!';";
///
/// // Parse language from string
/// let lang: Language = "javascript".parse().unwrap();
///
/// let options = Options {
///     language: Some("javascript"),  // Also accepts: "js", "app.js"
///     formatter: FormatterOption::HtmlInline {
///         theme: None,
///         pre_class: None,
///         italic: false,
///         include_highlights: false,
///         highlight_lines: None,
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// ```
///
/// ## Terminal output
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption, themes};
///
/// let code = "SELECT * FROM users WHERE active = true;";
///
/// let options = Options {
///     language: Some("sql"),
///     formatter: FormatterOption::Terminal {
///         theme: themes::get("github_light").ok(),
///     },
/// };
///
/// let ansi = highlight(code, options);
/// ```
///
/// ## HTML with linked CSS
///
/// ```rust
/// use autumnus::{highlight, Options, FormatterOption};
///
/// let code = "<div class=\"container\">Hello</div>";
///
/// let options = Options {
///     language: Some("html"),
///     formatter: FormatterOption::HtmlLinked {
///         pre_class: Some("syntax-highlight"),
///         highlight_lines: None,
///         header: None,
///     },
/// };
///
/// let html = highlight(code, options);
/// // Remember to include the corresponding CSS file for your theme
/// ```
#[derive(Debug)]
pub struct Options<'a> {
    /// Optional language hint for syntax highlighting.
    ///
    /// This field controls language detection and can accept:
    /// - **Language names**: `"rust"`, `"python"`, `"javascript"`, etc.
    /// - **File paths**: `"src/main.rs"`, `"app.py"`, `"Dockerfile"`
    /// - **File extensions**: `"rs"`, `"py"`, `"js"`
    /// - **None**: Try to auto-detect from source content (shebang, doctype, etc.)
    ///
    /// When `None`, the highlighter will analyze the source content to detect
    /// the language using shebangs, file content patterns, and other heuristics.
    pub language: Option<&'a str>,

    /// The output formatter to use.
    ///
    /// Determines the output format and styling:
    /// - [`FormatterOption::HtmlInline`] - HTML with inline CSS styles
    /// - [`FormatterOption::HtmlLinked`] - HTML with CSS classes (requires external CSS)
    /// - [`FormatterOption::Terminal`] - ANSI color codes for terminal output
    ///
    /// See [`FormatterOption`] documentation for detailed configuration options.
    pub formatter: FormatterOption<'a>,
}

impl Default for Options<'_> {
    fn default() -> Self {
        Self {
            language: None,
            formatter: FormatterOption::HtmlInline {
                pre_class: None,
                italic: false,
                include_highlights: false,
                theme: None,
                highlight_lines: None,
                header: None,
            },
        }
    }
}

/// Highlights source code and returns it as a string with syntax highlighting.
///
/// This function takes the source code and options as input,
/// and returns a string with the source code highlighted according to the specified formatter.
///
/// For a more ergonomic API, consider using the builder pattern - see [`formatter`] for examples.
///
/// # Arguments
///
/// * `source` - A string slice that represents the source code to be highlighted.
/// * `options` - An `Options` struct that contains the configuration options for the highlighter,
///   including the optional language/file path and formatter type to use.
///
/// # Examples
///
/// Basic usage with HTML inline styles (default):
///
/// ```rust
/// use autumnus::highlight;
/// use autumnus::Options;
/// use autumnus::FormatterOption;
///
/// let code = r#"
/// fn main() {
///     println!("Hello, world!");
/// }
/// "#;
///
/// let html = highlight(
///     code,
///     Options {
///         language: Some("rust"),
///         formatter: FormatterOption::HtmlInline {
///             pre_class: None,
///             italic: false,
///             include_highlights: false,
///             theme: None,
///             highlight_lines: None,
///             header: None,
///         },
///     }
/// );
/// ```
///
/// Output with HTML inline styles (default):
/// ```html
/// <pre class="athl" style="color: #c6d0f5; background-color: #303446;">
///   <code class="language-rust" translate="no" tabindex="0">
///     <div class="line" data-line="1">
///       <span style="color: #ca9ee6;">fn</span> <span style="color: #8caaee;">main</span>() {
///     </div>
///     <div class="line" data-line="2">
///       <span style="color: #8caaee;">println!</span>(<span style="color: #a6d189;">"Hello, world!"</span>);
///     </div>
///     <div class="line" data-line="3">}</div>
///   </code>
/// </pre>
/// ```
///
/// Using HTML with linked styles:
///
/// ```rust
/// use autumnus::highlight;
/// use autumnus::Options;
/// use autumnus::FormatterOption;
///
/// let code = r#"
/// fn main() {
///     println!("Hello, world!");
/// }
/// "#;
///
/// let html = highlight(
///     code,
///     Options {
///         language: Some("rust"),
///         formatter: FormatterOption::HtmlLinked {
///             pre_class: Some("my-code-block"),
///             highlight_lines: None,
///             header: None,
///         },
///     }
/// );
/// ```
///
/// Output with HTML linked styles:
/// ```html
/// <pre class="athl my-code-block">
///   <code class="language-rust" translate="no" tabindex="0">
///     <div class="line" data-line="1">
///       <span class="keyword-function">fn</span> <span class="function">main</span>() {
///     </div>
///     <div class="line" data-line="2">
///       <span class="function-macro">println!</span>(<span class="string">"Hello, world!"</span>);
///     </div>
///     <div class="line" data-line="3">}</div>
///   </code>
/// </pre>
/// ```
///
/// When using `FormatterOption::HtmlLinked`, you need to include the corresponding CSS file for your chosen theme.
/// CSS files for all themes are available in the `css/` directory:
///
/// ```html
/// <link rel="stylesheet" href="css/dracula.css">
/// ```
///
/// Using terminal output:
///
/// ```rust
/// use autumnus::highlight;
/// use autumnus::Options;
/// use autumnus::FormatterOption;
///
/// let code = r#"
/// fn main() {
///     println!("Hello, world!");
/// }
/// "#;
///
/// let ansi = highlight(
///     code,
///     Options {
///         language: Some("rust"),
///         formatter: FormatterOption::Terminal {
///             theme: None,
///         },
///     }
/// );
/// ```
///
/// Output with ANSI terminal colors:
/// ```text
/// [38;2;202;158;230mfn[0m [38;2;140;170;238mmain[0m() {
///     [38;2;140;170;238mprintln![0m([38;2;166;209;137m"Hello, world!"[0m);
/// }
/// ```
///
pub fn highlight(source: &str, options: Options) -> String {
    let lang = Language::guess(options.language, source);

    let formatter: Box<dyn Formatter> = match options.formatter {
        FormatterOption::HtmlInline {
            theme,
            pre_class,
            italic,
            include_highlights,
            highlight_lines,
            header,
        } => Box::new(
            crate::formatter::HtmlInlineBuilder::new()
                .source(source)
                .lang(lang)
                .italic(italic)
                .include_highlights(include_highlights)
                .theme(theme)
                .pre_class(pre_class)
                .highlight_lines(highlight_lines)
                .header(header)
                .build()
                .unwrap(),
        ),
        FormatterOption::HtmlLinked {
            pre_class,
            highlight_lines,
            header,
        } => Box::new(
            crate::formatter::HtmlLinkedBuilder::new()
                .source(source)
                .lang(lang)
                .pre_class(pre_class)
                .highlight_lines(highlight_lines)
                .header(header)
                .build()
                .unwrap(),
        ),
        FormatterOption::Terminal { theme } => Box::new(
            crate::formatter::TerminalBuilder::new()
                .source(source)
                .lang(lang)
                .theme(theme)
                .build()
                .unwrap(),
        ),
    };

    let mut buffer = Vec::new();
    let _ = formatter.format(&mut buffer);
    String::from_utf8(buffer).unwrap()
}

/// Write syntax highlighted output directly to a writer.
///
/// This function performs the same syntax highlighting as [`highlight`] but writes
/// the output directly to any type that implements [`Write`] instead of returning
/// a string. This is more memory efficient for large outputs and allows streaming
/// to files, network connections, or other destinations.
///
/// For a more ergonomic API, consider using the builder pattern - see [`formatter`] for examples.
///
/// # Arguments
///
/// * `output` - The writer to send highlighted output to
/// * `source` - The source code to highlight
/// * `options` - Configuration options for highlighting and formatting
///
/// # Returns
///
/// * `Ok(())` - Successfully wrote highlighted output
/// * `Err(io::Error)` - Write operation failed
///
/// # Examples
///
/// ## Writing to a file
///
/// ```rust,no_run
/// use autumnus::{write_highlight, Options, FormatterOption, themes};
/// use std::fs::File;
/// use std::io::BufWriter;
///
/// let code = r#"
/// fn fibonacci(n: u32) -> u32 {
///     match n {
///         0 => 0,
///         1 => 1,
///         _ => fibonacci(n - 1) + fibonacci(n - 2),
///     }
/// }
/// "#;
///
/// let mut file = BufWriter::new(File::create("highlighted.html")?);
///
/// write_highlight(&mut file, code, Options {
///     language: Some("rust"),
///     formatter: FormatterOption::HtmlInline {
///         theme: themes::get("dracula").ok(),
///         pre_class: Some("code-block"),
///         italic: true,
///         include_highlights: false,
///         highlight_lines: None,
///         header: None,
///     },
/// })?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Writing to stdout
///
/// ```rust
/// use autumnus::{write_highlight, Options, FormatterOption, themes};
/// use std::io;
///
/// let code = "print('Hello, World!')";
///
/// write_highlight(&mut io::stdout(), code, Options {
///     language: Some("python"),
///     formatter: FormatterOption::Terminal {
///         theme: themes::get("github_light").ok(),
///     },
/// }).expect("Failed to write to stdout");
/// ```
///
/// ## Writing to a vector (in-memory buffer)
///
/// ```rust
/// use autumnus::{write_highlight, Options, FormatterOption};
///
/// let code = "const x = 42;";
/// let mut buffer = Vec::new();
///
/// write_highlight(&mut buffer, code, Options {
///     language: Some("javascript"),
///     formatter: FormatterOption::HtmlInline {
///         theme: None,
///         pre_class: None,
///         italic: false,
///         include_highlights: false,
///         highlight_lines: None,
///         header: None,
///     },
/// }).expect("Failed to write to buffer");
///
/// let result = String::from_utf8(buffer).expect("Invalid UTF-8");
/// println!("Highlighted: {}", result);
/// ```
///
/// ## Streaming large files
///
/// ```rust,no_run
/// use autumnus::{write_highlight, Options, FormatterOption};
/// use std::fs::File;
/// use std::io::{BufReader, BufWriter, Read};
///
/// // Read source code from large file
/// let mut source = String::new();
/// BufReader::new(File::open("large_source.rs")?)
///     .read_to_string(&mut source)?;
///
/// // Stream highlighted output to another file
/// let mut output_file = BufWriter::new(File::create("highlighted_output.html")?);
///
/// write_highlight(&mut output_file, &source, Options {
///     language: Some("rust"),
///     formatter: FormatterOption::HtmlLinked {
///         pre_class: Some("large-code"),
///         highlight_lines: None,
///         header: None,
///     },
/// })?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Error handling
///
/// ```rust
/// use autumnus::{write_highlight, Options, FormatterOption};
/// use std::io;
///
/// let code = "invalid source";
/// let mut buffer = Vec::new();
///
/// match write_highlight(&mut buffer, code, Options::default()) {
///     Ok(()) => println!("Successfully highlighted {} bytes", buffer.len()),
///     Err(e) => eprintln!("Failed to highlight: {}", e),
/// }
/// ```
///
pub fn write_highlight(output: &mut dyn Write, source: &str, options: Options) -> io::Result<()> {
    let lang = Language::guess(options.language, source);

    let formatter: Box<dyn Formatter> = match options.formatter {
        FormatterOption::HtmlInline {
            theme,
            pre_class,
            italic,
            include_highlights,
            highlight_lines,
            header,
        } => Box::new(
            crate::formatter::HtmlInlineBuilder::new()
                .source(source)
                .lang(lang)
                .italic(italic)
                .include_highlights(include_highlights)
                .theme(theme)
                .pre_class(pre_class)
                .highlight_lines(highlight_lines)
                .header(header)
                .build()
                .unwrap(),
        ),
        FormatterOption::HtmlLinked {
            pre_class,
            highlight_lines,
            header,
        } => Box::new(
            crate::formatter::HtmlLinkedBuilder::new()
                .source(source)
                .lang(lang)
                .pre_class(pre_class)
                .highlight_lines(highlight_lines)
                .header(header)
                .build()
                .unwrap(),
        ),
        FormatterOption::Terminal { theme } => Box::new(
            crate::formatter::TerminalBuilder::new()
                .source(source)
                .lang(lang)
                .theme(theme)
                .build()
                .unwrap(),
        ),
    };

    formatter.format(output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // println!("{}", result);
    // std::fs::write("result.html", result.clone()).unwrap();

    #[test]
    fn test_write_highlight() {
        let code = r#"const = 1"#;

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-javascript" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #ca9ee6;">const</span> <span style="color: #99d1db;">=</span> <span style="color: #ef9f76;">1</span>
</div></code></pre>"#;

        let mut buffer = Vec::new();

        write_highlight(
            &mut buffer,
            code,
            Options {
                language: Some("javascript"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        )
        .unwrap();

        let result = String::from_utf8(buffer).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline() {
        let code = r#"defmodule Foo do
  @moduledoc """
  Test Module
  """

  @projects ["Phoenix", "MDEx"]

  def projects, do: @projects
end
"#;

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #ca9ee6;">defmodule</span> <span style="color: #e5c890;">Foo</span> <span style="color: #ca9ee6;">do</span>
</div><div class="line" data-line="2">  <span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">@</span><span style="color: #949cbb;">moduledoc</span> <span style="color: #949cbb;">&quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="3"><span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">  Test Module</span></span></span>
</div><div class="line" data-line="4"><span style="color: #99d1db;"><span style="color: #949cbb;"><span style="color: #949cbb;">  &quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="5">
</div><div class="line" data-line="6">  <span style="color: #99d1db;"><span style="color: #ef9f76;">@<span style="color: #8caaee;"><span style="color: #ef9f76;">projects <span style="color: #949cbb;">[</span><span style="color: #a6d189;">&quot;Phoenix&quot;</span><span style="color: #949cbb;">,</span> <span style="color: #a6d189;">&quot;MDEx&quot;</span><span style="color: #949cbb;">]</span></span></span></span></span>
</div><div class="line" data-line="7">
</div><div class="line" data-line="8">  <span style="color: #ca9ee6;">def</span> <span style="color: #c6d0f5;">projects</span><span style="color: #949cbb;">,</span> <span style="color: #eebebe;">do: </span><span style="color: #99d1db;"><span style="color: #ef9f76;">@<span style="color: #ef9f76;">projects</span></span></span>
</div><div class="line" data-line="9"><span style="color: #ca9ee6;">end</span>
</div></code></pre>"#;

        let result = highlight(
            code,
            Options {
                language: Some("elixir"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_include_highlights() {
        let code = r#"defmodule Foo do
  @lang :elixir
end
"#;

        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span data-highlight="keyword" style="color: #ca9ee6;">defmodule</span> <span data-highlight="module" style="color: #e5c890;">Foo</span> <span data-highlight="keyword" style="color: #ca9ee6;">do</span>
</div><div class="line" data-line="2">  <span data-highlight="operator" style="color: #99d1db;"><span data-highlight="constant" style="color: #ef9f76;">@<span data-highlight="function.call" style="color: #8caaee;"><span data-highlight="constant" style="color: #ef9f76;">lang <span data-highlight="string.special.symbol" style="color: #eebebe;">:elixir</span></span></span></span></span>
</div><div class="line" data-line="3"><span data-highlight="keyword" style="color: #ca9ee6;">end</span>
</div></code></pre>"#;

        let result = highlight(
            code,
            Options {
                language: Some("elixir"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: true,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_inline_escape_curly_braces() {
        let expected = r#"<pre class="athl" style="color: #c6d0f5; background-color: #303446;"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span style="color: #949cbb;">&lbrace;</span><span style="color: #eebebe;">:ok</span><span style="color: #949cbb;">,</span> <span style="color: #eebebe;">char: </span><span style="color: #81c8be;">&#39;&lbrace;&#39;</span><span style="color: #949cbb;">&rbrace;</span>
</div></code></pre>"#;

        let result = highlight(
            "{:ok, char: '{'}",
            Options {
                language: Some("elixir"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_linked() {
        let code = r#"defmodule Foo do
  @moduledoc """
  Test Module
  """

  @projects ["Phoenix", "MDEx"]

  def projects, do: @projects
end
"#;

        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="keyword">defmodule</span> <span class="module">Foo</span> <span class="keyword">do</span>
</div><div class="line" data-line="2">  <span class="operator"><span class="comment-documentation"><span class="comment">@</span><span class="comment">moduledoc</span> <span class="comment">&quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="3"><span class="operator"><span class="comment-documentation"><span class="comment">  Test Module</span></span></span>
</div><div class="line" data-line="4"><span class="operator"><span class="comment-documentation"><span class="comment">  &quot;&quot;&quot;</span></span></span>
</div><div class="line" data-line="5">
</div><div class="line" data-line="6">  <span class="operator"><span class="constant">@<span class="function-call"><span class="constant">projects <span class="punctuation-bracket">[</span><span class="string">&quot;Phoenix&quot;</span><span class="punctuation-delimiter">,</span> <span class="string">&quot;MDEx&quot;</span><span class="punctuation-bracket">]</span></span></span></span></span>
</div><div class="line" data-line="7">
</div><div class="line" data-line="8">  <span class="keyword">def</span> <span class="variable">projects</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">do: </span><span class="operator"><span class="constant">@<span class="constant">projects</span></span></span>
</div><div class="line" data-line="9"><span class="keyword">end</span>
</div></code></pre>"#;

        let result = highlight(
            code,
            Options {
                language: Some("elixir"),
                formatter: FormatterOption::HtmlLinked {
                    pre_class: None,
                    highlight_lines: None,
                    header: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_highlight_html_linked_escape_curly_braces() {
        let expected = r#"<pre class="athl"><code class="language-elixir" translate="no" tabindex="0"><div class="line" data-line="1"><span class="punctuation-bracket">&lbrace;</span><span class="string-special-symbol">:ok</span><span class="punctuation-delimiter">,</span> <span class="string-special-symbol">char: </span><span class="character">&#39;&lbrace;&#39;</span><span class="punctuation-bracket">&rbrace;</span>
</div></code></pre>"#;

        let result = highlight(
            "{:ok, char: '{'}",
            Options {
                language: Some("elixir"),
                formatter: FormatterOption::HtmlLinked {
                    pre_class: None,
                    highlight_lines: None,
                    header: None,
                },
            },
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_guess_language_by_file_name() {
        let result = highlight(
            "foo = 1",
            Options {
                language: Some("app.ex"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_file_extension() {
        let result = highlight(
            "# Title",
            Options {
                language: Some("md"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );
        assert!(result.as_str().contains("language-markdown"));

        let result = highlight(
            "foo = 1",
            Options {
                language: Some("ex"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_guess_language_by_shebang() {
        let result = highlight(
            "#!/usr/bin/env elixir",
            Options {
                language: Some("test"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );
        assert!(result.as_str().contains("language-elixir"));
    }

    #[test]
    fn test_fallback_to_plain_text() {
        let result = highlight(
            "source code",
            Options {
                language: Some("none"),
                formatter: FormatterOption::HtmlInline {
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    theme: themes::get("catppuccin_frappe").ok(),
                    highlight_lines: None,
                    header: None,
                },
            },
        );
        assert!(result.as_str().contains("language-plaintext"));
    }

    #[test]
    fn test_highlight_terminal() {
        let options = Options {
            language: Some("ruby"),
            formatter: FormatterOption::Terminal {
                theme: themes::get("dracula").ok(),
            },
        };
        let code = "puts 'Hello from Ruby!'";
        let ansi = highlight(code, options);

        assert!(ansi.as_str().contains("[38;2;241;250;140mHello from Ruby!"));
    }

    #[test]
    fn test_formatter_option_with_header() {
        let code = "fn main() { println!(\"Hello\"); }";

        // Test HtmlInline with header
        let inline_result = highlight(
            code,
            Options {
                language: Some("rust"),
                formatter: FormatterOption::HtmlInline {
                    theme: None,
                    pre_class: None,
                    italic: false,
                    include_highlights: false,
                    highlight_lines: None,
                    header: Some(formatter::HtmlElement {
                        open_tag: "<div class=\"code-container\">".to_string(),
                        close_tag: "</div>".to_string(),
                    }),
                },
            },
        );

        assert!(inline_result.starts_with("<div class=\"code-container\">"));
        assert!(inline_result.ends_with("</div>"));
        assert!(inline_result.contains("<pre class=\"athl\">"));

        // Test HtmlLinked with header
        let linked_result = highlight(
            code,
            Options {
                language: Some("rust"),
                formatter: FormatterOption::HtmlLinked {
                    pre_class: None,
                    highlight_lines: None,
                    header: Some(formatter::HtmlElement {
                        open_tag: "<section class=\"code-section\">".to_string(),
                        close_tag: "</section>".to_string(),
                    }),
                },
            },
        );

        assert!(linked_result.starts_with("<section class=\"code-section\">"));
        assert!(linked_result.ends_with("</section>"));
        assert!(linked_result.contains("<pre class=\"athl\">"));
    }
}
