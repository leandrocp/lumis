//! Language detection and Tree-sitter configuration.
//!
//! This module provides the [`Language`] enum that represents
//! all supported programming languages and their Tree-sitter configurations. It includes
//! automatic language detection based on file extensions, file names, content analysis,
//! and shebangs.
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
//! use lumis::languages::Language;
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
//! use lumis::languages::Language;
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
//! # #[cfg(all(feature = "lang-rust", feature = "lang-csharp", feature = "lang-bash"))] {
//! use lumis::languages::{Language, available_languages};
//!
//! // Get friendly name
//! assert_eq!(Language::Rust.name(), "Rust");
//! assert_eq!(Language::CSharp.name(), "C#");
//!
//! // Get all supported languages, sorted by id
//! let languages = available_languages();
//!
//! let rust = languages.iter().find(|language| language.id == "rust").unwrap();
//! assert_eq!(rust.name, "Rust");
//! assert!(rust.extensions.contains(&"*.rs"));
//! assert!(rust.globs.contains(&"*.rs"));
//!
//! // Aliases, Emacs modes and shebangs are part of the same record
//! let bash = languages.iter().find(|language| language.id == "bash").unwrap();
//! assert!(bash.aliases.contains(&"sh"));
//! assert!(bash.shebangs.contains(&"bash"));
//! # }
//! ```
//!
pub use lumis_core::languages::{
    available_languages, language_id_for_filename, Language, LanguageInfo, LanguageParseError,
};

use lumis_core::highlights::HIGHLIGHT_NAMES;
use lumis_wasm_runtime::tree_sitter_highlight::HighlightConfiguration;
use std::sync::LazyLock;

unsafe extern "C" {
    fn tree_sitter_diff() -> *const ();
    #[cfg(feature = "lang-editorconfig")]
    fn tree_sitter_editorconfig() -> *const ();
    #[cfg(feature = "lang-angular")]
    fn tree_sitter_angular() -> *const ();
    #[cfg(feature = "lang-astro")]
    fn tree_sitter_astro() -> *const ();
    #[cfg(feature = "lang-bash")]
    fn tree_sitter_bash() -> *const ();
    #[cfg(feature = "lang-csv")]
    fn tree_sitter_csv() -> *const ();
    #[cfg(feature = "lang-dart")]
    fn tree_sitter_dart() -> *const ();
    #[cfg(feature = "lang-dockerfile")]
    fn tree_sitter_dockerfile() -> *const ();
    #[cfg(feature = "lang-eex")]
    fn tree_sitter_eex() -> *const ();
    #[cfg(feature = "lang-gitattributes")]
    fn tree_sitter_gitattributes() -> *const ();
    #[cfg(feature = "lang-glimmer")]
    fn tree_sitter_glimmer() -> *const ();
    #[cfg(feature = "lang-haskell")]
    fn tree_sitter_haskell() -> *const ();
    #[cfg(feature = "lang-html")]
    fn tree_sitter_html() -> *const ();
    #[cfg(feature = "lang-http")]
    fn tree_sitter_http() -> *const ();
    #[cfg(feature = "lang-iex")]
    fn tree_sitter_iex() -> *const ();
    #[cfg(feature = "lang-jinja-inline")]
    fn tree_sitter_jinja_inline() -> *const ();
    #[cfg(feature = "lang-jq")]
    fn tree_sitter_jq() -> *const ();
    #[cfg(feature = "lang-kdl")]
    fn tree_sitter_kdl() -> *const ();
    #[cfg(feature = "lang-kotlin")]
    fn tree_sitter_kotlin() -> *const ();
    #[cfg(feature = "lang-latex")]
    fn tree_sitter_latex() -> *const ();
    #[cfg(feature = "lang-liquid")]
    fn tree_sitter_liquid() -> *const ();
    #[cfg(feature = "lang-luadoc")]
    fn tree_sitter_luadoc() -> *const ();
    #[cfg(feature = "lang-dot")]
    fn tree_sitter_dot() -> *const ();
    #[cfg(feature = "lang-nim")]
    fn tree_sitter_nim() -> *const ();
    #[cfg(feature = "lang-protobuf")]
    fn tree_sitter_proto() -> *const ();
    #[cfg(feature = "lang-python")]
    fn tree_sitter_python() -> *const ();
    #[cfg(feature = "lang-perl")]
    fn tree_sitter_perl() -> *const ();
    #[cfg(feature = "lang-ruby")]
    fn tree_sitter_ruby() -> *const ();
    #[cfg(feature = "lang-scss")]
    fn tree_sitter_scss() -> *const ();
    #[cfg(feature = "lang-surface")]
    fn tree_sitter_surface() -> *const ();
    #[cfg(feature = "lang-svelte")]
    fn tree_sitter_svelte() -> *const ();
    #[cfg(feature = "lang-terraform")]
    fn tree_sitter_terraform() -> *const ();
    #[cfg(feature = "lang-toon")]
    fn tree_sitter_toon() -> *const ();
    #[cfg(feature = "lang-nushell")]
    fn tree_sitter_nu() -> *const ();
    #[cfg(feature = "lang-vim")]
    fn tree_sitter_vim() -> *const ();
    #[cfg(feature = "lang-vue")]
    fn tree_sitter_vue() -> *const ();
    #[cfg(feature = "lang-wat")]
    fn tree_sitter_wat() -> *const ();
    #[cfg(feature = "lang-wgsl")]
    fn tree_sitter_wgsl() -> *const ();
    #[cfg(feature = "lang-xml")]
    fn tree_sitter_xml() -> *const ();
    #[cfg(feature = "lang-yaml")]
    fn tree_sitter_yaml() -> *const ();
}

include!(concat!(env!("OUT_DIR"), "/queries_constants.rs"));

/// Internal extension trait mapping each language to its highlight configuration.
pub(crate) trait LanguageConfig {
    fn config(&self) -> &'static HighlightConfiguration;
}

impl LanguageConfig for Language {
    fn config(&self) -> &'static HighlightConfiguration {
        match self {
            #[cfg(feature = "lang-arduino")]
            Language::Arduino => &ARDUINO_CONFIG,
            #[cfg(feature = "lang-angular")]
            Language::Angular => &ANGULAR_CONFIG,
            #[cfg(feature = "lang-asm")]
            Language::Assembly => &ASM_CONFIG,
            #[cfg(feature = "lang-astro")]
            Language::Astro => &ASTRO_CONFIG,
            #[cfg(feature = "lang-bash")]
            Language::Bash => &BASH_CONFIG,
            #[cfg(feature = "lang-bicep")]
            Language::Bicep => &BICEP_CONFIG,
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
            #[cfg(feature = "lang-d")]
            Language::D => &D_CONFIG,
            #[cfg(feature = "lang-dart")]
            Language::Dart => &DART_CONFIG,
            Language::Diff => &DIFF_CONFIG,
            #[cfg(feature = "lang-dockerfile")]
            Language::Dockerfile => &DOCKERFILE_CONFIG,
            #[cfg(feature = "lang-dot")]
            Language::Dot => &DOT_CONFIG,
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
            #[cfg(feature = "lang-editorconfig")]
            Language::Editorconfig => &EDITORCONFIG_CONFIG,
            #[cfg(feature = "lang-erlang")]
            Language::Erlang => &ERLANG_CONFIG,
            #[cfg(feature = "lang-fish")]
            Language::Fish => &FISH_CONFIG,
            #[cfg(feature = "lang-fortran")]
            Language::Fortran => &FORTRAN_CONFIG,
            #[cfg(feature = "lang-fsharp")]
            Language::FSharp => &FSHARP_CONFIG,
            #[cfg(feature = "lang-gitattributes")]
            Language::Gitattributes => &GITATTRIBUTES_CONFIG,
            #[cfg(feature = "lang-gleam")]
            Language::Gleam => &GLEAM_CONFIG,
            #[cfg(feature = "lang-glimmer")]
            Language::Glimmer => &GLIMMER_CONFIG,
            #[cfg(feature = "lang-glsl")]
            Language::Glsl => &GLSL_CONFIG,
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
            #[cfg(feature = "lang-http")]
            Language::HTTP => &HTTP_CONFIG,
            #[cfg(feature = "lang-iex")]
            Language::IEx => &IEX_CONFIG,
            #[cfg(feature = "lang-ini")]
            Language::INI => &INI_CONFIG,
            #[cfg(feature = "lang-java")]
            Language::Java => &JAVA_CONFIG,
            #[cfg(feature = "lang-javascript")]
            Language::JavaScript => &JAVASCRIPT_CONFIG,
            #[cfg(feature = "lang-json")]
            Language::JSON => &JSON_CONFIG,
            #[cfg(feature = "lang-julia")]
            Language::Julia => &JULIA_CONFIG,
            #[cfg(feature = "lang-jinja")]
            Language::Jinja => &JINJA_CONFIG,
            #[cfg(feature = "lang-jinja-inline")]
            Language::JinjaInline => &JINJA_INLINE_CONFIG,
            #[cfg(feature = "lang-javadoc")]
            Language::Javadoc => &JAVADOC_CONFIG,
            #[cfg(feature = "lang-jq")]
            Language::Jq => &JQ_CONFIG,
            #[cfg(feature = "lang-kdl")]
            Language::Kdl => &KDL_CONFIG,
            #[cfg(feature = "lang-kotlin")]
            Language::Kotlin => &KOTLIN_CONFIG,
            #[cfg(feature = "lang-latex")]
            Language::LaTeX => &LATEX_CONFIG,
            #[cfg(feature = "lang-liquid")]
            Language::Liquid => &LIQUID_CONFIG,
            #[cfg(feature = "lang-lua")]
            Language::Lua => &LUA_CONFIG,
            #[cfg(feature = "lang-luadoc")]
            Language::Luadoc => &LUADOC_CONFIG,
            #[cfg(feature = "lang-objc")]
            Language::ObjC => &OBJC_CONFIG,
            #[cfg(feature = "lang-ocaml")]
            Language::OCaml => &OCAML_CONFIG,
            #[cfg(feature = "lang-ocaml")]
            Language::OCamlInterface => &OCAML_INTERFACE_CONFIG,
            #[cfg(feature = "lang-make")]
            Language::Make => &MAKE_CONFIG,
            #[cfg(feature = "lang-matlab")]
            Language::Matlab => &MATLAB_CONFIG,
            #[cfg(feature = "lang-markdown")]
            Language::Markdown => &MARKDOWN_CONFIG,
            #[cfg(feature = "lang-markdown-inline")]
            Language::MarkdownInline => &MARKDOWN_INLINE_CONFIG,
            #[cfg(feature = "lang-mdx")]
            Language::Mdx => &MDX_CONFIG,
            #[cfg(feature = "lang-nginx")]
            Language::Nginx => &NGINX_CONFIG,
            #[cfg(feature = "lang-nim")]
            Language::Nim => &NIM_CONFIG,
            #[cfg(feature = "lang-nix")]
            Language::Nix => &NIX_CONFIG,
            #[cfg(feature = "lang-nushell")]
            Language::Nushell => &NUSHELL_CONFIG,
            #[cfg(feature = "lang-perl")]
            Language::Perl => &PERL_CONFIG,
            #[cfg(feature = "lang-pascal")]
            Language::Pascal => &PASCAL_CONFIG,
            #[cfg(feature = "lang-php")]
            Language::Php => &PHP_CONFIG,
            #[cfg(feature = "lang-powershell")]
            Language::PowerShell => &POWERSHELL_CONFIG,
            #[cfg(feature = "lang-protobuf")]
            Language::ProtoBuf => &PROTO_BUF_CONFIG,
            #[cfg(feature = "lang-puppet")]
            Language::Puppet => &PUPPET_CONFIG,
            #[cfg(feature = "lang-python")]
            Language::Python => &PYTHON_CONFIG,
            #[cfg(feature = "lang-qmljs")]
            Language::QMLJS => &QMLJS_CONFIG,
            #[cfg(feature = "lang-r")]
            Language::R => &R_CONFIG,
            #[cfg(feature = "lang-racket")]
            Language::Racket => &RACKET_CONFIG,
            #[cfg(feature = "lang-regex")]
            Language::Regex => &REGEX_CONFIG,
            #[cfg(feature = "lang-rst")]
            Language::RST => &RST_CONFIG,
            #[cfg(feature = "lang-ruby")]
            Language::Ruby => &RUBY_CONFIG,
            #[cfg(feature = "lang-rust")]
            Language::Rust => &RUST_CONFIG,
            #[cfg(feature = "lang-scala")]
            Language::Scala => &SCALA_CONFIG,
            #[cfg(feature = "lang-scss")]
            Language::SCSS => &SCSS_CONFIG,
            #[cfg(feature = "lang-scheme")]
            Language::Scheme => &SCHEME_CONFIG,
            #[cfg(feature = "lang-sql")]
            Language::SQL => &SQL_CONFIG,
            #[cfg(feature = "lang-solidity")]
            Language::Solidity => &SOLIDITY_CONFIG,
            #[cfg(feature = "lang-surface")]
            Language::Surface => &SURFACE_CONFIG,
            #[cfg(feature = "lang-svelte")]
            Language::Svelte => &SVELTE_CONFIG,
            #[cfg(feature = "lang-swift")]
            Language::Swift => &SWIFT_CONFIG,
            #[cfg(feature = "lang-systemverilog")]
            Language::SystemVerilog => &SYSTEMVERILOG_CONFIG,
            #[cfg(feature = "lang-terraform")]
            Language::Terraform => &TERRAFORM_CONFIG,
            #[cfg(feature = "lang-toon")]
            Language::Toon => &TOON_CONFIG,
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
            #[cfg(feature = "lang-vhdl")]
            Language::VHDL => &VHDL_CONFIG,
            #[cfg(feature = "lang-vue")]
            Language::Vue => &VUE_CONFIG,
            #[cfg(feature = "lang-wat")]
            Language::Wat => &WAT_CONFIG,
            #[cfg(feature = "lang-wgsl")]
            Language::Wgsl => &WGSL_CONFIG,
            #[cfg(feature = "lang-xml")]
            Language::XML => &XML_CONFIG,
            #[cfg(feature = "lang-yaml")]
            Language::YAML => &YAML_CONFIG,
            #[cfg(feature = "lang-zig")]
            Language::Zig => &ZIG_CONFIG,
            #[cfg(feature = "lang-zsh")]
            Language::Zsh => &ZSH_CONFIG,
            _ => &PLAIN_TEXT_CONFIG,
        }
    }
}

#[cfg(feature = "lang-arduino")]
static ARDUINO_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_arduino::LANGUAGE),
        "arduino",
        ARDUINO_HIGHLIGHTS,
        ARDUINO_INJECTIONS,
        ARDUINO_LOCALS,
    )
    .expect("failed to create arduino highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

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
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_bash) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "bash",
        BASH_HIGHLIGHTS,
        BASH_INJECTIONS,
        BASH_LOCALS,
    )
    .expect("failed to create bash highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-bicep")]
static BICEP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_bicep::LANGUAGE),
        "bicep",
        BICEP_HIGHLIGHTS,
        BICEP_INJECTIONS,
        BICEP_LOCALS,
    )
    .expect("failed to create bicep highlight configuration");
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
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_caddy::LANGUAGE),
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
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_clojure_orchard::LANGUAGE),
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
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_commonlisp::LANGUAGE_COMMONLISP),
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

#[cfg(feature = "lang-d")]
static D_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_d::LANGUAGE),
        "d",
        D_HIGHLIGHTS,
        D_INJECTIONS,
        D_LOCALS,
    )
    .expect("failed to create d highlight configuration");
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
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_diff) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
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

#[cfg(feature = "lang-dot")]
static DOT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(unsafe {
            tree_sitter_language::LanguageFn::from_raw(tree_sitter_dot)
        }),
        "dot",
        DOT_HIGHLIGHTS,
        DOT_INJECTIONS,
        DOT_LOCALS,
    )
    .expect("failed to create dot highlight configuration");
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
    let mut config = HighlightConfiguration::new(
        tree_sitter_fish::language(),
        "fish",
        FISH_HIGHLIGHTS,
        FISH_INJECTIONS,
        FISH_LOCALS,
    )
    .expect("failed to create fish highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-fortran")]
static FORTRAN_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_fortran::LANGUAGE),
        "fortran",
        FORTRAN_HIGHLIGHTS,
        FORTRAN_INJECTIONS,
        FORTRAN_LOCALS,
    )
    .expect("failed to create fortran highlight configuration");
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

#[cfg(feature = "lang-editorconfig")]
static EDITORCONFIG_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_editorconfig) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "editorconfig",
        EDITORCONFIG_HIGHLIGHTS,
        EDITORCONFIG_INJECTIONS,
        EDITORCONFIG_LOCALS,
    )
    .expect("failed to create editorconfig highlight configuration");
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

#[cfg(feature = "lang-gitattributes")]
static GITATTRIBUTES_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(unsafe {
            tree_sitter_language::LanguageFn::from_raw(tree_sitter_gitattributes)
        }),
        "gitattributes",
        GITATTRIBUTES_HIGHLIGHTS,
        GITATTRIBUTES_INJECTIONS,
        GITATTRIBUTES_LOCALS,
    )
    .expect("failed to create gitattributes highlight configuration");
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

#[cfg(feature = "lang-glsl")]
static GLSL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_glsl::LANGUAGE_GLSL),
        "glsl",
        GLSL_HIGHLIGHTS,
        GLSL_INJECTIONS,
        GLSL_LOCALS,
    )
    .expect("failed to create glsl highlight configuration");
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
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_graphql::LANGUAGE),
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
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_haskell) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
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
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_html) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "html",
        HTML_HIGHLIGHTS,
        HTML_INJECTIONS,
        HTML_LOCALS,
    )
    .expect("failed to create html highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-http")]
static HTTP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_http) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "http",
        HTTP_HIGHLIGHTS,
        HTTP_INJECTIONS,
        HTTP_LOCALS,
    )
    .expect("failed to create http highlight configuration");
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

#[cfg(feature = "lang-ini")]
static INI_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_ini::LANGUAGE),
        "ini",
        INI_HIGHLIGHTS,
        INI_INJECTIONS,
        INI_LOCALS,
    )
    .expect("failed to create ini highlight configuration");
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

#[cfg(feature = "lang-julia")]
static JULIA_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_julia::LANGUAGE),
        "julia",
        JULIA_HIGHLIGHTS,
        JULIA_INJECTIONS,
        JULIA_LOCALS,
    )
    .expect("failed to create julia highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-jinja")]
static JINJA_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter_jinja::language(),
        "jinja",
        JINJA_HIGHLIGHTS,
        JINJA_INJECTIONS,
        JINJA_LOCALS,
    )
    .expect("failed to create jinja highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-jinja-inline")]
static JINJA_INLINE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn =
        unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_jinja_inline) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "jinja_inline",
        JINJA_INLINE_HIGHLIGHTS,
        JINJA_INLINE_INJECTIONS,
        JINJA_INLINE_LOCALS,
    )
    .expect("failed to create jinja_inline highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-javadoc")]
static JAVADOC_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_javadoc::LANGUAGE),
        "javadoc",
        JAVADOC_HIGHLIGHTS,
        JAVADOC_INJECTIONS,
        JAVADOC_LOCALS,
    )
    .expect("failed to create javadoc highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-jq")]
static JQ_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_jq) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "jq",
        JQ_HIGHLIGHTS,
        JQ_INJECTIONS,
        JQ_LOCALS,
    )
    .expect("failed to create jq highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-kdl")]
static KDL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(unsafe {
            tree_sitter_language::LanguageFn::from_raw(tree_sitter_kdl)
        }),
        "kdl",
        KDL_HIGHLIGHTS,
        KDL_INJECTIONS,
        KDL_LOCALS,
    )
    .expect("failed to create kdl highlight configuration");
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

#[cfg(feature = "lang-luadoc")]
static LUADOC_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(unsafe {
            tree_sitter_language::LanguageFn::from_raw(tree_sitter_luadoc)
        }),
        "luadoc",
        LUADOC_HIGHLIGHTS,
        LUADOC_INJECTIONS,
        LUADOC_LOCALS,
    )
    .expect("failed to create luadoc highlight configuration");
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
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_make::LANGUAGE),
        "make",
        MAKE_HIGHLIGHTS,
        MAKE_INJECTIONS,
        MAKE_LOCALS,
    )
    .expect("failed to create make highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-matlab")]
static MATLAB_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_matlab::LANGUAGE),
        "matlab",
        MATLAB_HIGHLIGHTS,
        MATLAB_INJECTIONS,
        MATLAB_LOCALS,
    )
    .expect("failed to create matlab highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-markdown")]
static MARKDOWN_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_md::LANGUAGE),
        "markdown",
        MARKDOWN_HIGHLIGHTS,
        MARKDOWN_INJECTIONS,
        MARKDOWN_LOCALS,
    )
    .expect("failed to create markdown highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-nginx")]
static NGINX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_nginx::LANGUAGE),
        "nginx",
        NGINX_HIGHLIGHTS,
        NGINX_INJECTIONS,
        NGINX_LOCALS,
    )
    .expect("failed to create nginx highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-markdown-inline")]
static MARKDOWN_INLINE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_md::INLINE_LANGUAGE),
        "markdown_inline",
        MARKDOWN_INLINE_HIGHLIGHTS,
        MARKDOWN_INLINE_INJECTIONS,
        MARKDOWN_INLINE_LOCALS,
    )
    .expect("failed to create markdown highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-mdx")]
static MDX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_md::LANGUAGE),
        "mdx",
        MDX_HIGHLIGHTS,
        MDX_INJECTIONS,
        MDX_LOCALS,
    )
    .expect("failed to create mdx highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-nim")]
static NIM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_nim) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "nim",
        NIM_HIGHLIGHTS,
        NIM_INJECTIONS,
        NIM_LOCALS,
    )
    .expect("failed to create nim highlight configuration");
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

#[cfg(feature = "lang-nushell")]
static NUSHELL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_nu) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "nu",
        NU_HIGHLIGHTS,
        NU_INJECTIONS,
        NU_LOCALS,
    )
    .expect("failed to create nushell highlight configuration");
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

#[cfg(feature = "lang-pascal")]
static PASCAL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_pascal::LANGUAGE),
        "pascal",
        PASCAL_HIGHLIGHTS,
        PASCAL_INJECTIONS,
        PASCAL_LOCALS,
    )
    .expect("failed to create pascal highlight configuration");
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
        tree_sitter::Language::new(unsafe {
            tree_sitter_language::LanguageFn::from_raw(tree_sitter_proto)
        }),
        "protobuf",
        PROTO_HIGHLIGHTS,
        PROTO_INJECTIONS,
        PROTO_LOCALS,
    )
    .expect("failed to create protobuf highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-puppet")]
static PUPPET_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_puppet::LANGUAGE),
        "puppet",
        PUPPET_HIGHLIGHTS,
        PUPPET_INJECTIONS,
        PUPPET_LOCALS,
    )
    .expect("failed to create puppet highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

static PLAIN_TEXT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    // Plain text has no grammar of its own; it borrows one and asks it nothing.
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_diff) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
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
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_python) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "python",
        PYTHON_HIGHLIGHTS,
        PYTHON_INJECTIONS,
        PYTHON_LOCALS,
    )
    .expect("failed to create python highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-qmljs")]
static QMLJS_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_qmljs::LANGUAGE),
        "qmljs",
        QMLJS_HIGHLIGHTS,
        QMLJS_INJECTIONS,
        QMLJS_LOCALS,
    )
    .expect("failed to create qmljs highlight configuration");
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

#[cfg(feature = "lang-racket")]
static RACKET_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_racket::LANGUAGE),
        "racket",
        RACKET_HIGHLIGHTS,
        RACKET_INJECTIONS,
        RACKET_LOCALS,
    )
    .expect("failed to create racket highlight configuration");
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

#[cfg(feature = "lang-rst")]
static RST_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_rst::LANGUAGE),
        "rst",
        RST_HIGHLIGHTS,
        RST_INJECTIONS,
        RST_LOCALS,
    )
    .expect("failed to create rst highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-ruby")]
static RUBY_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_ruby) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
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

#[cfg(feature = "lang-scheme")]
static SCHEME_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_scheme::LANGUAGE),
        "scheme",
        SCHEME_HIGHLIGHTS,
        SCHEME_INJECTIONS,
        SCHEME_LOCALS,
    )
    .expect("failed to create scheme highlight configuration");
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

#[cfg(feature = "lang-solidity")]
static SOLIDITY_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_solidity::LANGUAGE),
        "solidity",
        SOLIDITY_HIGHLIGHTS,
        SOLIDITY_INJECTIONS,
        SOLIDITY_LOCALS,
    )
    .expect("failed to create solidity highlight configuration");
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
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_svelte) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
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

#[cfg(feature = "lang-systemverilog")]
static SYSTEMVERILOG_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_systemverilog::LANGUAGE),
        "systemverilog",
        SYSTEMVERILOG_HIGHLIGHTS,
        SYSTEMVERILOG_INJECTIONS,
        SYSTEMVERILOG_LOCALS,
    )
    .expect("failed to create systemverilog highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-terraform")]
static TERRAFORM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_terraform) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "terraform",
        TERRAFORM_HIGHLIGHTS,
        TERRAFORM_INJECTIONS,
        TERRAFORM_LOCALS,
    )
    .expect("failed to create terraform highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-toon")]
static TOON_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_toon) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "toon",
        TOON_HIGHLIGHTS,
        TOON_INJECTIONS,
        TOON_LOCALS,
    )
    .expect("failed to create toon highlight configuration");
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
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(codebook_tree_sitter_typst::LANGUAGE),
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

#[cfg(feature = "lang-vhdl")]
static VHDL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_vhdl::LANGUAGE),
        "vhdl",
        VHDL_HIGHLIGHTS,
        VHDL_INJECTIONS,
        VHDL_LOCALS,
    )
    .expect("failed to create vhdl highlight configuration");
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

#[cfg(feature = "lang-wat")]
static WAT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_wat) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "wat",
        WAT_HIGHLIGHTS,
        WAT_INJECTIONS,
        WAT_LOCALS,
    )
    .expect("failed to create wat highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-wgsl")]
static WGSL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(unsafe {
            tree_sitter_language::LanguageFn::from_raw(tree_sitter_wgsl)
        }),
        "wgsl",
        WGSL_HIGHLIGHTS,
        WGSL_INJECTIONS,
        WGSL_LOCALS,
    )
    .expect("failed to create wgsl highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

#[cfg(feature = "lang-xml")]
static XML_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_xml) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
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
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_yaml) };
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
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

#[cfg(feature = "lang-zsh")]
static ZSH_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_zsh::LANGUAGE),
        "zsh",
        ZSH_HIGHLIGHTS,
        ZSH_INJECTIONS,
        ZSH_LOCALS,
    )
    .expect("failed to create zsh highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

// A table-driven test's branches are its coverage; splitting one to satisfy the
// cognitive complexity ceiling would hide what it asserts.
#[allow(clippy::cognitive_complexity)]
#[cfg(test)]
mod tests {
    use super::*;
    use lumis_wasm_runtime::tree_sitter_highlight::Highlighter;
    use serde::Deserialize;
    use std::{fs, path::PathBuf};

    #[derive(Debug, Deserialize)]
    struct GuessCase {
        name: String,
        input: Option<String>,
        source: String,
        expected: String,
    }

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

    fn samples_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples")
    }

    fn guess_cases() -> Vec<GuessCase> {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/language-guess-cases.json");
        serde_json::from_str(&fs::read_to_string(path).expect("guess cases should exist"))
            .expect("guess cases should be valid json")
    }

    #[test]
    fn shared_guess_cases_match_expected_results() {
        for case in guess_cases() {
            let language = Language::guess(case.input.as_deref(), &case.source);
            assert_eq!(
                language.id_name(),
                case.expected,
                "case failed: {}",
                case.name
            );
        }
    }

    fn assert_language_sample_highlights(lang: Language, expected_name: &str, sample_file: &str) {
        let config = lang.config();
        let source =
            fs::read_to_string(samples_dir().join(sample_file)).expect("sample should exist");

        assert_eq!(lang.name(), expected_name);

        let mut highlighter = Highlighter::new();
        let events = highlighter
            .highlight(config, source.as_bytes(), None, |injected| {
                injected
                    .parse::<Language>()
                    .ok()
                    .map(|language| language.config())
            })
            .expect("highlight should start");

        let rendered = events
            .collect::<Result<Vec<_>, _>>()
            .expect("highlight should finish");
        assert!(
            !rendered.is_empty(),
            "expected highlight events for {sample_file}"
        );
    }

    fn sample_file_for_language_id(id: &str) -> Option<String> {
        let expected_stem = match id {
            "asm" => "assembly",
            "ocaml_interface" => "ocamlinterface",
            other => other,
        };

        fs::read_dir(samples_dir())
            .ok()?
            .filter_map(Result::ok)
            .find_map(|entry| {
                let path = entry.path();
                let stem = path.file_stem()?.to_str()?.to_ascii_lowercase();
                (stem == expected_stem).then(|| entry.file_name().to_string_lossy().into_owned())
            })
    }

    macro_rules! language_sample_tests {
        ($($(#[$meta:meta])* $name:ident => ($lang:expr, $expected:literal, $sample:literal);)+) => {
            $(
                #[test]
                $(#[$meta])*
                fn $name() {
                    assert_language_sample_highlights($lang, $expected, $sample);
                }
            )+
        };
    }

    #[test]
    fn test_all_available_languages_have_samples_and_highlight() {
        let ids: Vec<&str> = available_languages()
            .iter()
            .map(|language| language.id)
            .collect();

        for id in ids {
            let language = if id == "plaintext" {
                Language::PlainText
            } else {
                id.parse::<Language>()
                    .unwrap_or_else(|_| panic!("unknown language id: {id}"))
            };
            let sample_file = sample_file_for_language_id(id)
                .unwrap_or_else(|| panic!("missing sample for language id: {id}"));

            assert_language_sample_highlights(language, language.name(), &sample_file);
        }
    }

    #[cfg(feature = "lang-python")]
    #[test]
    fn python_class_attributes_are_local_fields() {
        use streaming_iterator::StreamingIterator;
        use tree_sitter::{Parser, Query, QueryCursor};

        let source: &[u8] = b"class Human:\n    species = \"H. sapiens\"\n";
        let config = Language::Python.config();

        let mut parser = Parser::new();
        parser
            .set_language(&config.language)
            .expect("Python parser language should load");
        let tree = parser
            .parse(source, None)
            .expect("Python source should parse");

        let query = Query::new(&config.language, PYTHON_LOCALS)
            .expect("Python locals query should compile");
        let field_capture = query
            .capture_names()
            .iter()
            .position(|name| *name == "local.definition.field")
            .expect("Python locals query should define the field capture")
            as u32;

        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source);
        let mut fields = Vec::new();

        while let Some(query_match) = matches.next() {
            for capture in query_match.captures {
                if capture.index == field_capture {
                    fields.push(
                        capture
                            .node
                            .utf8_text(source)
                            .expect("captured Python field should be UTF-8"),
                    );
                }
            }
        }

        assert!(
            fields.contains(&"species"),
            "expected class attribute to be captured as a local field, got {fields:?}"
        );
    }

    language_sample_tests! {
        #[cfg(feature = "lang-arduino")]
        test_arduino_config_loads => (Language::Arduino, "Arduino", "arduino.ino");
        #[cfg(feature = "lang-angular")]
        test_angular_config_loads => (Language::Angular, "Angular", "angular.angular");
        #[cfg(feature = "lang-asm")]
        test_asm_config_loads => (Language::Assembly, "Assembly", "assembly.s");
        #[cfg(feature = "lang-astro")]
        test_astro_config_loads => (Language::Astro, "Astro", "astro.astro");
        #[cfg(feature = "lang-bash")]
        test_bash_config_loads => (Language::Bash, "Bash", "bash.sh");
        #[cfg(feature = "lang-bicep")]
        test_bicep_config_loads => (Language::Bicep, "Bicep", "bicep.bicep");
        #[cfg(feature = "lang-c")]
        test_c_config_loads => (Language::C, "C", "c.c");
        #[cfg(feature = "lang-caddy")]
        test_caddy_config_loads => (Language::Caddy, "Caddy", "caddy.caddyfile");
        #[cfg(feature = "lang-clojure")]
        test_clojure_config_loads => (Language::Clojure, "Clojure", "clojure.clj");
        #[cfg(feature = "lang-cmake")]
        test_cmake_config_loads => (Language::CMake, "CMake", "cmake.cmake");
        #[cfg(feature = "lang-comment")]
        test_comment_config_loads => (Language::Comment, "Comment", "comment.txt");
        #[cfg(feature = "lang-commonlisp")]
        test_commonlisp_config_loads => (Language::CommonLisp, "Common Lisp", "commonlisp.lisp");
        #[cfg(feature = "lang-cpp")]
        test_cpp_config_loads => (Language::CPlusPlus, "C++", "cpp.cpp");
        #[cfg(feature = "lang-csharp")]
        test_csharp_config_loads => (Language::CSharp, "C#", "csharp.cs");
        #[cfg(feature = "lang-css")]
        test_css_config_loads => (Language::CSS, "CSS", "css.css");
        #[cfg(feature = "lang-csv")]
        test_csv_config_loads => (Language::CSV, "CSV", "csv.csv");
        #[cfg(feature = "lang-d")]
        test_d_config_loads => (Language::D, "D", "d.d");
        #[cfg(feature = "lang-dart")]
        test_dart_config_loads => (Language::Dart, "Dart", "dart.dart");
        test_diff_config_loads => (Language::Diff, "Diff", "diff.diff");
        #[cfg(feature = "lang-dockerfile")]
        test_dockerfile_config_loads => (Language::Dockerfile, "Dockerfile", "dockerfile.dockerfile");
        #[cfg(feature = "lang-dot")]
        test_dot_config_loads => (Language::Dot, "Dot", "dot.dot");
        #[cfg(feature = "lang-eex")]
        test_eex_config_loads => (Language::EEx, "Eex", "eex.eex");
        #[cfg(feature = "lang-ejs")]
        test_ejs_config_loads => (Language::EJS, "EJS", "ejs.ejs");
        #[cfg(feature = "lang-erb")]
        test_erb_config_loads => (Language::ERB, "ERB", "erb.erb");
        #[cfg(feature = "lang-elixir")]
        test_elixir_config_loads => (Language::Elixir, "Elixir", "elixir.ex");
        #[cfg(feature = "lang-elm")]
        test_elm_config_loads => (Language::Elm, "Elm", "elm.elm");
        #[cfg(feature = "lang-editorconfig")]
        test_editorconfig_config_loads => (Language::Editorconfig, "Editorconfig", "editorconfig.editorconfig");
        #[cfg(feature = "lang-erlang")]
        test_erlang_config_loads => (Language::Erlang, "Erlang", "erlang.erl");
        #[cfg(feature = "lang-fish")]
        test_fish_config_loads => (Language::Fish, "Fish", "fish.fish");
        #[cfg(feature = "lang-fortran")]
        test_fortran_config_loads => (Language::Fortran, "Fortran", "fortran.f90");
        #[cfg(feature = "lang-fsharp")]
        test_fsharp_config_loads => (Language::FSharp, "F#", "fsharp.fs");
        #[cfg(feature = "lang-gitattributes")]
        test_gitattributes_config_loads => (Language::Gitattributes, "Git Attributes", "gitattributes.gitattributes");
        #[cfg(feature = "lang-gitignore")]
        test_gitignore_config_loads => (Language::GitIgnore, "Git Ignore", "gitignore.gitignore");
        #[cfg(feature = "lang-gleam")]
        test_gleam_config_loads => (Language::Gleam, "Gleam", "gleam.gleam");
        #[cfg(feature = "lang-glimmer")]
        test_glimmer_config_loads => (Language::Glimmer, "Glimmer", "glimmer.hbs");
        #[cfg(feature = "lang-glsl")]
        test_glsl_config_loads => (Language::Glsl, "GLSL", "glsl.glsl");
        #[cfg(feature = "lang-go")]
        test_go_config_loads => (Language::Go, "Go", "go.go");
        #[cfg(feature = "lang-graphql")]
        test_graphql_config_loads => (Language::GraphQL, "GraphQL", "graphql.graphql");
        #[cfg(feature = "lang-haskell")]
        test_haskell_config_loads => (Language::Haskell, "Haskell", "haskell.hs");
        #[cfg(feature = "lang-hcl")]
        test_hcl_config_loads => (Language::HCL, "HCL", "hcl.hcl");
        #[cfg(feature = "lang-heex")]
        test_heex_config_loads => (Language::HEEx, "HEEx", "heex.heex");
        #[cfg(feature = "lang-html")]
        test_html_config_loads => (Language::HTML, "HTML", "html.html");
        #[cfg(feature = "lang-http")]
        test_http_config_loads => (Language::HTTP, "HTTP", "http.http");
        #[cfg(feature = "lang-iex")]
        test_iex_config_loads => (Language::IEx, "IEx", "iex.iex");
        #[cfg(feature = "lang-ini")]
        test_ini_config_loads => (Language::INI, "INI", "ini.ini");
        #[cfg(feature = "lang-java")]
        test_java_config_loads => (Language::Java, "Java", "java.java");
        #[cfg(feature = "lang-javascript")]
        test_javascript_config_loads => (Language::JavaScript, "JavaScript", "javascript.js");
        #[cfg(feature = "lang-json")]
        test_json_config_loads => (Language::JSON, "JSON", "json.json");
        #[cfg(feature = "lang-json5")]
        test_json5_config_loads => (Language::JSON5, "JSON5", "json5.json5");
        #[cfg(feature = "lang-julia")]
        test_julia_config_loads => (Language::Julia, "Julia", "julia.jl");
        #[cfg(feature = "lang-kdl")]
        test_kdl_config_loads => (Language::Kdl, "KDL", "kdl.kdl");
        #[cfg(feature = "lang-jinja")]
        test_jinja_config_loads => (Language::Jinja, "Jinja", "jinja.jinja");
        #[cfg(feature = "lang-jinja-inline")]
        test_jinja_inline_config_loads => (Language::JinjaInline, "Jinja Inline", "jinja_inline.txt");
        #[cfg(feature = "lang-javadoc")]
        test_javadoc_config_loads => (Language::Javadoc, "Javadoc", "javadoc.java");
        #[cfg(feature = "lang-jq")]
        test_jq_config_loads => (Language::Jq, "jq", "jq.jq");
        #[cfg(feature = "lang-just")]
        test_just_config_loads => (Language::Just, "Just", "just.just");
        #[cfg(feature = "lang-kotlin")]
        test_kotlin_config_loads => (Language::Kotlin, "Kotlin", "kotlin.kt");
        #[cfg(feature = "lang-latex")]
        test_latex_config_loads => (Language::LaTeX, "LaTeX", "latex.tex");
        #[cfg(feature = "lang-liquid")]
        test_liquid_config_loads => (Language::Liquid, "Liquid", "liquid.liquid");
        #[cfg(feature = "lang-llvm")]
        test_llvm_config_loads => (Language::Llvm, "LLVM", "llvm.ll");
        #[cfg(feature = "lang-lua")]
        test_lua_config_loads => (Language::Lua, "Lua", "lua.lua");
        #[cfg(feature = "lang-luadoc")]
        test_luadoc_config_loads => (Language::Luadoc, "LuaDoc", "luadoc.lua");
        #[cfg(feature = "lang-make")]
        test_make_config_loads => (Language::Make, "Make", "make.mk");
        #[cfg(feature = "lang-matlab")]
        test_matlab_config_loads => (Language::Matlab, "MATLAB", "matlab.m");
        #[cfg(feature = "lang-markdown")]
        test_markdown_config_loads => (Language::Markdown, "Markdown", "markdown.md");
        #[cfg(feature = "lang-markdown-inline")]
        test_markdown_inline_config_loads => (Language::MarkdownInline, "Markdown Inline", "markdown_inline.txt");
        #[cfg(feature = "lang-mermaid")]
        test_mermaid_config_loads => (Language::Mermaid, "Mermaid", "mermaid.mmd");
        #[cfg(feature = "lang-nginx")]
        test_nginx_config_loads => (Language::Nginx, "Nginx", "nginx.nginx");
        #[cfg(feature = "lang-nim")]
        test_nim_config_loads => (Language::Nim, "Nim", "nim.nim");
        #[cfg(feature = "lang-nix")]
        test_nix_config_loads => (Language::Nix, "Nix", "nix.nix");
        #[cfg(feature = "lang-nushell")]
        test_nushell_config_loads => (Language::Nushell, "Nushell", "nushell.nu");
        #[cfg(feature = "lang-objc")]
        test_objc_config_loads => (Language::ObjC, "Objective-C", "objc.m");
        #[cfg(feature = "lang-ocaml")]
        test_ocaml_config_loads => (Language::OCaml, "OCaml", "ocaml.ml");
        #[cfg(feature = "lang-ocaml")]
        test_ocaml_interface_config_loads => (Language::OCamlInterface, "OCaml Interface", "ocamlinterface.mli");
        #[cfg(feature = "lang-perl")]
        test_perl_config_loads => (Language::Perl, "Perl", "perl.pm");
        #[cfg(feature = "lang-pascal")]
        test_pascal_config_loads => (Language::Pascal, "Pascal", "pascal.pas");
        #[cfg(feature = "lang-php")]
        test_php_config_loads => (Language::Php, "PHP", "php.php");
        test_plaintext_config_loads => (Language::PlainText, "Plain Text", "plaintext.txt");
        #[cfg(feature = "lang-powershell")]
        test_powershell_config_loads => (Language::PowerShell, "PowerShell", "powershell.ps1");
        #[cfg(feature = "lang-prisma")]
        test_prisma_config_loads => (Language::Prisma, "Prisma", "prisma.prisma");
        #[cfg(feature = "lang-protobuf")]
        test_protobuf_config_loads => (Language::ProtoBuf, "Protocol Buffer", "protobuf.proto");
        #[cfg(feature = "lang-puppet")]
        test_puppet_config_loads => (Language::Puppet, "Puppet", "puppet.pp");
        #[cfg(feature = "lang-python")]
        test_python_config_loads => (Language::Python, "Python", "python.py");
        #[cfg(feature = "lang-qmljs")]
        test_qmljs_config_loads => (Language::QMLJS, "QML/JS", "qmljs.qml");
        #[cfg(feature = "lang-r")]
        test_r_config_loads => (Language::R, "R", "r.r");
        #[cfg(feature = "lang-racket")]
        test_racket_config_loads => (Language::Racket, "Racket", "racket.rkt");
        #[cfg(feature = "lang-regex")]
        test_regex_config_loads => (Language::Regex, "Regex", "regex.regex");
        #[cfg(feature = "lang-rst")]
        test_rst_config_loads => (Language::RST, "reStructuredText", "rst.rst");
        #[cfg(feature = "lang-ruby")]
        test_ruby_config_loads => (Language::Ruby, "Ruby", "ruby.rb");
        #[cfg(feature = "lang-rust")]
        test_rust_config_loads => (Language::Rust, "Rust", "rust.rs");
        #[cfg(feature = "lang-scala")]
        test_scala_config_loads => (Language::Scala, "Scala", "scala.scala");
        #[cfg(feature = "lang-scss")]
        test_scss_config_loads => (Language::SCSS, "SCSS", "scss.scss");
        #[cfg(feature = "lang-scheme")]
        test_scheme_config_loads => (Language::Scheme, "Scheme", "scheme.scm");
        #[cfg(feature = "lang-sql")]
        test_sql_config_loads => (Language::SQL, "SQL", "sql.sql");
        #[cfg(feature = "lang-solidity")]
        test_solidity_config_loads => (Language::Solidity, "Solidity", "solidity.sol");
        #[cfg(feature = "lang-surface")]
        test_surface_config_loads => (Language::Surface, "Surface", "surface.sface");
        #[cfg(feature = "lang-svelte")]
        test_svelte_config_loads => (Language::Svelte, "Svelte", "svelte.svelte");
        #[cfg(feature = "lang-swift")]
        test_swift_config_loads => (Language::Swift, "Swift", "swift.swift");
        #[cfg(feature = "lang-systemverilog")]
        test_systemverilog_config_loads => (Language::SystemVerilog, "SystemVerilog", "systemverilog.sv");
        #[cfg(feature = "lang-tcl")]
        test_tcl_config_loads => (Language::Tcl, "Tcl", "tcl.tcl");
        #[cfg(feature = "lang-terraform")]
        test_terraform_config_loads => (Language::Terraform, "Terraform", "terraform.tf");
        #[cfg(feature = "lang-toml")]
        test_toml_config_loads => (Language::Toml, "TOML", "toml.toml");
        #[cfg(feature = "lang-tsx")]
        test_tsx_config_loads => (Language::Tsx, "TSX", "tsx.tsx");
        #[cfg(feature = "lang-typescript")]
        test_typescript_config_loads => (Language::TypeScript, "TypeScript", "typescript.ts");
        #[cfg(feature = "lang-typst")]
        test_typst_config_loads => (Language::Typst, "Typst", "typst.typ");
        #[cfg(feature = "lang-vim")]
        test_vim_config_loads => (Language::Vim, "Vim", "vim.vim");
        #[cfg(feature = "lang-vhdl")]
        test_vhdl_config_loads => (Language::VHDL, "VHDL", "vhdl.vhdl");
        #[cfg(feature = "lang-vue")]
        test_vue_config_loads => (Language::Vue, "Vue", "vue.vue");
        #[cfg(feature = "lang-wat")]
        test_wat_config_loads => (Language::Wat, "WAT", "wat.wat");
        #[cfg(feature = "lang-wgsl")]
        test_wgsl_config_loads => (Language::Wgsl, "WGSL", "wgsl.wgsl");
        #[cfg(feature = "lang-xml")]
        test_xml_config_loads => (Language::XML, "XML", "xml.xml");
        #[cfg(feature = "lang-yaml")]
        test_yaml_config_loads => (Language::YAML, "YAML", "yaml.yml");
        #[cfg(feature = "lang-zig")]
        test_zig_config_loads => (Language::Zig, "Zig", "zig.zig");
        #[cfg(feature = "lang-zsh")]
        test_zsh_config_loads => (Language::Zsh, "Zsh", "zsh.zsh");
    }

    #[test]
    fn test_available_languages() {
        let languages = available_languages();
        assert!(!languages.is_empty(), "catalog looks empty");

        for language in &languages {
            assert!(!language.id.is_empty(), "a language has no id");
            assert!(!language.name.is_empty(), "a language has no name");
            assert!(
                !language.aliases.contains(&language.id),
                "{} lists its own id as an alias",
                language.id
            );
            assert!(
                language.extensions.iter().all(|ext| ext.starts_with("*.")),
                "{} has an extension that is not a bare extension glob",
                language.id
            );
        }

        // The catalog is feature-gated, so anything asserting a specific
        // language declares the feature that compiles it in.
        #[cfg(feature = "lang-rust")]
        {
            let rust = languages
                .iter()
                .find(|language| language.id == "rust")
                .expect("lang-rust is enabled");
            assert_eq!(rust.name, "Rust");
            assert!(rust.globs.contains(&"*.rs"));
        }

        #[cfg(feature = "lang-bash")]
        {
            let bash = languages
                .iter()
                .find(|language| language.id == "bash")
                .expect("lang-bash is enabled");
            assert!(bash.aliases.contains(&"sh"));
            assert!(bash.shebangs.contains(&"bash"));
            assert!(bash.globs.contains(&"PKGBUILD"));
            assert!(
                !bash.extensions.contains(&"PKGBUILD"),
                "extensions must exclude non-extension globs"
            );
        }
    }
}
