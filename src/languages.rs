// Guess Language copied from https://github.com/Wilfred/difftastic/blob/f34a9014760efbaed01b972caba8b73754da16c9/src/parse/guess_language.rs

use crate::constants::HIGHLIGHT_NAMES;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::{path::Path, sync::LazyLock};
use strum::{EnumIter, IntoEnumIterator};
use tree_sitter_highlight::HighlightConfiguration;

extern "C" {
    fn tree_sitter_hcl() -> *const ();
    fn tree_sitter_angular() -> *const ();
    fn tree_sitter_astro() -> *const ();
    fn tree_sitter_clojure() -> *const ();
    fn tree_sitter_cmake() -> *const ();
    fn tree_sitter_comment() -> *const ();
    fn tree_sitter_commonlisp() -> *const ();
    fn tree_sitter_csv() -> *const ();
    fn tree_sitter_dockerfile() -> *const ();
    fn tree_sitter_eex() -> *const ();
    fn tree_sitter_elm() -> *const ();
    fn tree_sitter_glimmer() -> *const ();
    fn tree_sitter_graphql() -> *const ();
    fn tree_sitter_iex() -> *const ();
    fn tree_sitter_kotlin() -> *const ();
    fn tree_sitter_latex() -> *const ();
    fn tree_sitter_liquid() -> *const ();
    fn tree_sitter_llvm() -> *const ();
    fn tree_sitter_make() -> *const ();
    fn tree_sitter_perl() -> *const ();
    fn tree_sitter_powershell() -> *const ();
    fn tree_sitter_scss() -> *const ();
    fn tree_sitter_surface() -> *const ();
    fn tree_sitter_vim() -> *const ();
    fn tree_sitter_vue() -> *const ();
}

include!(concat!(env!("OUT_DIR"), "/queries_constants.rs"));

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Language {
    Angular,
    Astro,
    Bash,
    C,
    CMake,
    CPlusPlus,
    CSS,
    CSV,
    CSharp,
    Clojure,
    Comment,
    CommonLisp,
    Diff,
    Dockerfile,
    EEx,
    EJS,
    ERB,
    Elixir,
    Elm,
    Erlang,
    FSharp,
    Gleam,
    Glimmer,
    Go,
    GraphQL,
    HEEx,
    HTML,
    Haskell,
    HCL,
    IEx,
    JSON,
    Java,
    JavaScript,
    Kotlin,
    LaTeX,
    Liquid,
    Llvm,
    Lua,
    Make,
    Markdown,
    MarkdownInline,
    Nix,
    OCaml,
    OCamlInterface,
    ObjC,
    Perl,
    Php,
    PlainText,
    PowerShell,
    ProtoBuf,
    Python,
    R,
    Regex,
    Ruby,
    Rust,
    SCSS,
    SQL,
    Scala,
    Surface,
    Svelte,
    Swift,
    Toml,
    Tsx,
    TypeScript,
    Vim,
    Vue,
    XML,
    YAML,
    Zig,
}

impl Language {
    /// Guess the language based on the provided language name, file path, or source content.
    pub fn guess(lang_or_file: &str, src: &str) -> Self {
        let lang_or_file = lang_or_file.to_ascii_lowercase();

        let exact = match lang_or_file.as_str() {
            "angular" => Some(Language::Angular),
            "astro" => Some(Language::Astro),
            "bash" => Some(Language::Bash),
            "c" => Some(Language::C),
            "clojure" => Some(Language::Clojure),
            "comment" => Some(Language::Comment),
            "commonlisp" => Some(Language::CommonLisp),
            "c++" | "cpp" => Some(Language::CPlusPlus),
            "cmake" => Some(Language::CMake),
            "c#" | "csharp" => Some(Language::CSharp),
            "csv" => Some(Language::CSV),
            "css" => Some(Language::CSS),
            "diff" => Some(Language::Diff),
            "dockerfile" | "docker" => Some(Language::Dockerfile),
            "eex" => Some(Language::EEx),
            "ejs" => Some(Language::EJS),
            "erb" => Some(Language::ERB),
            "elixir" => Some(Language::Elixir),
            "elm" => Some(Language::Elm),
            "erlang" => Some(Language::Erlang),
            "f#" | "fsharp" => Some(Language::FSharp),
            "gleam" => Some(Language::Gleam),
            "ember" | "glimmer" | "handlebars" => Some(Language::Glimmer),
            "go" => Some(Language::Go),
            "graphql" => Some(Language::GraphQL),
            "haskell" => Some(Language::Haskell),
            "hcl" | "terraform" => Some(Language::HCL),
            "heex" => Some(Language::HEEx),
            "html" => Some(Language::HTML),
            "iex" => Some(Language::IEx),
            "java" => Some(Language::Java),
            "jsx" | "javascript" => Some(Language::JavaScript),
            "json" => Some(Language::JSON),
            "kotlin" => Some(Language::Kotlin),
            "latex" => Some(Language::LaTeX),
            "liquid" => Some(Language::Liquid),
            "llvm" => Some(Language::Llvm),
            "lua" => Some(Language::Lua),
            "objc" | "objective-c" => Some(Language::ObjC),
            "ocaml" => Some(Language::OCaml),
            "ocaml_interface" => Some(Language::OCamlInterface),
            "perl" => Some(Language::Perl),
            "make" => Some(Language::Make),
            "markdown" => Some(Language::Markdown),
            "markdown_inline" => Some(Language::MarkdownInline),
            "nix" => Some(Language::Nix),
            "php" => Some(Language::Php),
            "powershell" => Some(Language::PowerShell),
            "protobuf" => Some(Language::ProtoBuf),
            "python" => Some(Language::Python),
            "r" => Some(Language::R),
            "regex" => Some(Language::Regex),
            "ruby" => Some(Language::Ruby),
            "rust" => Some(Language::Rust),
            "scala" => Some(Language::Scala),
            "scss" => Some(Language::SCSS),
            "sql" => Some(Language::SQL),
            "surface" => Some(Language::Surface),
            "svelte" => Some(Language::Svelte),
            "swift" => Some(Language::Swift),
            "toml" => Some(Language::Toml),
            "typescript" => Some(Language::TypeScript),
            "tsx" => Some(Language::Tsx),
            "vim" | "viml" | "vimscript" => Some(Language::Vim),
            "vue" => Some(Language::Vue),
            "xml" => Some(Language::XML),
            "yaml" => Some(Language::YAML),
            "zig" => Some(Language::Zig),
            _ => None,
        };

        match exact {
            Some(lang) => lang,
            None => {
                let path = Path::new(&lang_or_file);

                if let Some(lang) = Self::from_glob(path) {
                    return lang;
                }

                if let Some(lang) = Self::from_extension(&lang_or_file) {
                    return lang;
                }

                if let Some(lang) = Self::from_emacs_mode_header(src) {
                    return lang;
                }

                if let Some(lang) = Self::from_shebang(src) {
                    return lang;
                }

                if Self::looks_like_html(src) {
                    return Language::HTML;
                }

                if Self::looks_like_xml(src) {
                    return Language::XML;
                }

                if Self::looks_like_objc(path, src) {
                    return Language::ObjC;
                }

                Language::PlainText
            }
        }
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
        let token_pattern = format!("*.{}", token);

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
            Language::Angular => &["*.angular", "component.html"],
            Language::Astro => &["*.astro"],
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
            Language::C => &["*.c"],
            Language::Clojure => &[
                "*.bb", "*.boot", "*.clj", "*.cljc", "*.clje", "*.cljs", "*.cljx", "*.edn",
                "*.joke", "*.joker",
            ],
            Language::Comment => &[],
            Language::CommonLisp => &["*.lisp", "*.lsp", "*.asd"],
            Language::CMake => &["*.cmake", "*.cmake.in", "CMakeLists.txt"],
            Language::CSharp => &["*.cs"],
            Language::CSV => &["*.csv"],
            Language::CPlusPlus => &[
                "*.cc", "*.cpp", "*.h", "*.hh", "*.hpp", "*.ino", "*.cxx", "*.cu", "*.hxx",
            ],
            Language::CSS => &["*.css"],
            Language::Diff => &["*.diff"],
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
            Language::EEx => &["*.eex"],
            Language::EJS => &["*.ejs"],
            Language::ERB => &["*.erb"],
            Language::Elixir => &["*.ex", "*.exs"],
            Language::Elm => &["*.elm"],
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
            Language::FSharp => &["*.fs", "*.fsx", "*.fsi"],
            Language::Gleam => &["*.gleam"],
            Language::Glimmer => &["*.hbs", "*.handlebars", "*.html.handlebars", "*.glimmer"],
            Language::Go => &["*.go"],
            Language::GraphQL => &[],
            Language::Haskell => &["*.hs", "*.hs-boot"],
            Language::HCL => &["*.hcl", "*.nomad", "*.tf", "*.tfvars", "*.workflow"],
            Language::HEEx => &["*.heex", "*.neex"],
            Language::HTML => &["*.html", "*.htm", "*.xhtml"],
            Language::IEx => &["*.iex"],
            Language::Java => &["*.java"],
            Language::JavaScript => &["*.cjs", "*.js", "*.mjs", "*.snap", "*.jsx"],
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
            Language::Kotlin => &["*.kt", "*.ktm", "*.kts"],
            Language::LaTeX => &["*.aux", "*.cls", "*.sty", "*.tex"],
            Language::Liquid => &["*liquid"],
            Language::Llvm => &["*.llvm", "*.ll"],
            Language::Lua => &["*.lua"],
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
            Language::Markdown => &["*.md", "README", "LICENSE"],
            Language::MarkdownInline => &[],
            Language::Nix => &["*.nix"],
            Language::ObjC => &["*.m", "*.objc"],
            Language::OCaml => &["*.ml"],
            Language::OCamlInterface => &["*.mli"],
            Language::Perl => &["*.pm", "*.pl", "*.t"],
            Language::Php => &[
                "*.php", "*.phtml", "*.php3", "*.php4", "*.php5", "*.php7", "*.phps",
            ],
            Language::PowerShell => &["*.ps1", "*.psm1"],
            Language::ProtoBuf => &["*.proto", "*.protobuf", "*.proto2", "*.proto3"],
            Language::PlainText => &[],
            Language::Python => &["*.py", "*.py3", "*.pyi", "*.bzl", "TARGETS", "BUCK", "DEPS"],
            Language::R => &["*.R", "*.r", "*.rd", "*.rsx", ".Rprofile", "expr-dist"],
            Language::Regex => &["*.regex"],
            Language::Ruby => &[
                "*.rb",
                "*.builder",
                "*.spec",
                "*.rake",
                "Gemfile",
                "Rakefile",
            ],
            Language::Rust => &["*.rs"],
            Language::Scala => &["*.scala", "*.sbt", "*.sc"],
            Language::SCSS => &["*.scss"],
            Language::SQL => &["*.sql", "*.pgsql"],
            Language::Surface => &["*.surface", "*.sface"],
            Language::Svelte => &["*.svelte"],
            Language::Swift => &["*.swift"],
            Language::Toml => &[
                "*.toml",
                "Cargo.lock",
                "Gopkg.lock",
                "Pipfile",
                "pdm.lock",
                "poetry.lock",
                "uv.lock",
            ],
            Language::TypeScript => &["*.ts"],
            Language::Tsx => &["*.tsx"],
            Language::Vim => &["*.vim", "*.viml"],
            Language::Vue => &["*.vue"],
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
            Language::YAML => &["*.yaml", "*.yml"],
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
                "c" => Some(Language::C),
                "clojure" => Some(Language::Clojure),
                "csharp" => Some(Language::CSharp),
                "csv" => Some(Language::CSV),
                "css" => Some(Language::CSS),
                "c++" => Some(Language::CPlusPlus),
                "elixir" => Some(Language::Elixir),
                "elm" => Some(Language::Elm),
                "fsharp" => Some(Language::FSharp),
                "gleam" => Some(Language::Gleam),
                "go" => Some(Language::Go),
                "haskell" => Some(Language::Haskell),
                "hcl" => Some(Language::HCL),
                "html" => Some(Language::HTML),
                "java" => Some(Language::Java),
                "js" | "js2" => Some(Language::JavaScript),
                "lisp" => Some(Language::CommonLisp),
                "nix" => Some(Language::Nix),
                "nxml" => Some(Language::XML),
                "objc" => Some(Language::ObjC),
                "perl" => Some(Language::Perl),
                "python" => Some(Language::Python),
                "ruby" => Some(Language::Ruby),
                "rust" => Some(Language::Rust),
                "scala" => Some(Language::Scala),
                "scss" => Some(Language::SCSS),
                "sh" => Some(Language::Bash),
                "sql" => Some(Language::SQL),
                "surface" => Some(Language::Surface),
                "swift" => Some(Language::Swift),
                "toml" => Some(Language::Toml),
                "typescript" => Some(Language::TypeScript),
                "tsx" => Some(Language::Tsx),
                "tuareg" => Some(Language::OCaml),
                // "typescript" => Some(Language::TypeScript),
                "yaml" => Some(Language::YAML),
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
                    match name.to_string_lossy().borrow() {
                        "deno" | "ts-node" => return Some(Language::TypeScript),
                        "ocaml" | "ocamlrun" | "ocamlscript" => return Some(Language::OCaml),
                        "lisp" | "sbc" | "ccl" | "clisp" | "ecl" => {
                            return Some(Language::CommonLisp)
                        }
                        "runghc" | "runhaskell" | "runhugs" => return Some(Language::Haskell),
                        "ash" | "bash" | "dash" | "ksh" | "mksh" | "pdksh" | "rc" | "sh"
                        | "zsh" => return Some(Language::Bash),
                        "elixir" => return Some(Language::Elixir),
                        "Rscript" => return Some(Language::R),
                        "python" | "python2" | "python3" => return Some(Language::Python),
                        "perl" => return Some(Language::Perl),
                        "ruby" | "macruby" | "rake" | "jruby" | "rbx" => {
                            return Some(Language::Ruby)
                        }
                        "swift" => return Some(Language::Swift),
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
            Language::Angular => "Angular",
            Language::Astro => "Astro",
            Language::Bash => "Bash",
            Language::C => "C",
            Language::Clojure => "Clojure",
            Language::Comment => "Comment",
            Language::CommonLisp => "Common Lisp",
            Language::CMake => "CMake",
            Language::CSharp => "C#",
            Language::CSV => "CSV",
            Language::CPlusPlus => "C++",
            Language::CSS => "CSS",
            Language::Diff => "Diff",
            Language::Dockerfile => "Dockerfile",
            Language::EEx => "Eex",
            Language::EJS => "EJS",
            Language::ERB => "ERB",
            Language::Elixir => "Elixir",
            Language::Elm => "Elm",
            Language::Erlang => "Erlang",
            Language::FSharp => "F#",
            Language::Gleam => "Gleam",
            Language::Glimmer => "Glimmer",
            Language::Go => "Go",
            Language::GraphQL => "GraphQL",
            Language::Haskell => "Haskell",
            Language::HCL => "HCL",
            Language::HEEx => "HEEx",
            Language::HTML => "HTML",
            Language::IEx => "IEx",
            Language::Java => "Java",
            Language::JavaScript => "JavaScript",
            Language::JSON => "JSON",
            Language::Kotlin => "Kotlin",
            Language::LaTeX => "LaTeX",
            Language::Liquid => "Liquid",
            Language::Llvm => "LLVM",
            Language::Lua => "Lua",
            Language::ObjC => "Objective-C",
            Language::OCaml => "OCaml",
            Language::OCamlInterface => "OCaml Interface",
            Language::Make => "Make",
            Language::Markdown => "Markdown",
            Language::MarkdownInline => "Markdown Inline",
            Language::Nix => "Nix",
            Language::Perl => "Perl",
            Language::Php => "PHP",
            Language::PlainText => "Plain Text",
            Language::PowerShell => "PowerShell",
            Language::ProtoBuf => "Protocol Buffer",
            Language::Python => "Python",
            Language::R => "R",
            Language::Regex => "Regex",
            Language::Ruby => "Ruby",
            Language::Rust => "Rust",
            Language::Scala => "Scala",
            Language::SCSS => "SCSS",
            Language::SQL => "SQL",
            Language::Surface => "Surface",
            Language::Svelte => "Svelte",
            Language::Swift => "Swift",
            Language::Toml => "TOML",
            Language::TypeScript => "TypeScript",
            Language::Tsx => "TSX",
            Language::Vim => "Vim",
            Language::Vue => "Vue",
            Language::XML => "XML",
            Language::YAML => "YAML",
            Language::Zig => "Zig",
        }
    }

    pub fn id_name(&self) -> String {
        self.name().to_ascii_lowercase().replace(" ", "")
    }

    pub fn config(&self) -> &'static HighlightConfiguration {
        match self {
            Language::Angular => &ANGULAR_CONFIG,
            Language::Astro => &ASTRO_CONFIG,
            Language::Bash => &BASH_CONFIG,
            Language::C => &C_CONFIG,
            Language::Clojure => &CLOJURE_CONFIG,
            Language::Comment => &COMMENT_CONFIG,
            Language::CommonLisp => &COMMONLISP_CONFIG,
            Language::CMake => &CMAKE_CONFIG,
            Language::CSharp => &CSHARP_CONFIG,
            Language::CSV => &CSV_CONFIG,
            Language::CPlusPlus => &CPP_CONFIG,
            Language::CSS => &CSS_CONFIG,
            Language::Diff => &DIFF_CONFIG,
            Language::Dockerfile => &DOCKERFILE_CONFIG,
            Language::EEx => &EEX_CONFIG,
            Language::EJS => &EJS_CONFIG,
            Language::ERB => &ERB_CONFIG,
            Language::Elixir => &ELIXIR_CONFIG,
            Language::Elm => &ELM_CONFIG,
            Language::Erlang => &ERLANG_CONFIG,
            Language::FSharp => &FSHARP_CONFIG,
            Language::Gleam => &GLEAM_CONFIG,
            Language::Glimmer => &GLIMMER_CONFIG,
            Language::Go => &GO_CONFIG,
            Language::GraphQL => &GRAPHQL_CONFIG,
            Language::Haskell => &HASKELL_CONFIG,
            Language::HCL => &HCL_CONFIG,
            Language::HEEx => &HEEX_CONFIG,
            Language::HTML => &HTML_CONFIG,
            Language::IEx => &IEX_CONFIG,
            Language::Java => &JAVA_CONFIG,
            Language::JavaScript => &JAVASCRIPT_CONFIG,
            Language::JSON => &JSON_CONFIG,
            Language::Kotlin => &KOTLIN_CONFIG,
            Language::LaTeX => &LATEX_CONFIG,
            Language::Liquid => &LIQUID_CONFIG,
            Language::Llvm => &LLVM_CONFIG,
            Language::Lua => &LUA_CONFIG,
            Language::ObjC => &OBJC_CONFIG,
            Language::OCaml => &OCAML_CONFIG,
            Language::OCamlInterface => &OCAML_INTERFACE_CONFIG,
            Language::Make => &MAKE_CONFIG,
            Language::Markdown => &MARKDOWN_CONFIG,
            Language::MarkdownInline => &MARKDOWN_INLINE_CONFIG,
            Language::Nix => &NIX_CONFIG,
            Language::Perl => &PERL_CONFIG,
            Language::Php => &PHP_CONFIG,
            Language::PowerShell => &POWERSHELL_CONFIG,
            Language::ProtoBuf => &PROTO_BUF_CONFIG,
            Language::Python => &PYTHON_CONFIG,
            Language::R => &R_CONFIG,
            Language::Regex => &REGEX_CONFIG,
            Language::Ruby => &RUBY_CONFIG,
            Language::Rust => &RUST_CONFIG,
            Language::Scala => &SCALA_CONFIG,
            Language::SCSS => &SCSS_CONFIG,
            Language::SQL => &SQL_CONFIG,
            Language::Surface => &SURFACE_CONFIG,
            Language::Svelte => &SVELTE_CONFIG,
            Language::Swift => &SWIFT_CONFIG,
            Language::Toml => &TOML_CONFIG,
            Language::TypeScript => &TYPESCRIPT_CONFIG,
            Language::Tsx => &TSX_CONFIG,
            Language::Vim => &VIM_CONFIG,
            Language::Vue => &VUE_CONFIG,
            Language::XML => &XML_CONFIG,
            Language::YAML => &YAML_CONFIG,
            Language::Zig => &ZIG_CONFIG,
            _ => &PLAIN_TEXT_CONFIG,
        }
    }
}

/// Returns a BTreeMap containing all supported languages with their details.
///
/// The key is the language's id_name (lowercase, no spaces).
/// The value is a tuple containing:
/// - The friendly name (e.g. "Elixir", "Common Lisp")
/// - A Vec of file extensions/patterns
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

static COMMENT_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_comment) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "comment",
        COMMENT_HIGHLIGHTS,
        COMMENT_INJECTIONS,
        COMMENT_LOCALS,
    )
    .expect("failed to create comment highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

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

static CMAKE_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_cmake) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "cmake",
        CMAKE_HIGHLIGHTS,
        CMAKE_INJECTIONS,
        CMAKE_LOCALS,
    )
    .expect("failed to create cmake highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

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

static ELM_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_elm) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "elm",
        ELM_HIGHLIGHTS,
        ELM_INJECTIONS,
        ELM_LOCALS,
    )
    .expect("failed to create elm highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

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

static HCL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_hcl) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "hcl",
        HCL_HIGHLIGHTS,
        HCL_INJECTIONS,
        HCL_LOCALS,
    )
    .expect("failed to create hcl highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

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

static PHP_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(tree_sitter_php::LANGUAGE_PHP),
        "php",
        PHP_HIGHLIGHTS,
        PHP_INJECTIONS,
        PHP_LOCALS,
    )
    .expect("failed to create php highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

static POWERSHELL_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let language_fn = unsafe { tree_sitter_language::LanguageFn::from_raw(tree_sitter_powershell) };

    let mut config = HighlightConfiguration::new(
        tree_sitter::Language::new(language_fn),
        "poweshell",
        POWERSHELL_HIGHLIGHTS,
        POWERSHELL_INJECTIONS,
        POWERSHELL_LOCALS,
    )
    .expect("failed to create poweshell highlight configuration");
    config.configure(&HIGHLIGHT_NAMES);
    config
});

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
    use tree_sitter_highlight::Highlighter;

    #[test]
    fn test_match_exact_name() {
        let lang = Language::guess("elixir", "");
        assert_eq!(lang.name(), "Elixir");

        let lang = Language::guess("Elixir", "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_match_filename() {
        let lang = Language::guess("app.ex", "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_match_extension() {
        let lang = Language::guess(".ex", "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_match_path_with_extension() {
        let lang = Language::guess("lib/app.ex", "");
        assert_eq!(lang.name(), "Elixir");
    }

    #[test]
    fn test_no_match_fallbacks_to_plain_text() {
        let lang = Language::guess("none", "");
        assert_eq!(lang.name(), "Plain Text");
    }

    #[test]
    fn test_highlight_all_languages() {
        for language in Language::iter() {
            let name = Language::id_name(&language);
            let lang = Language::guess(&name, "");
            let mut highlighter = Highlighter::new();

            let _ = highlighter
                .highlight(lang.config(), "".as_bytes(), None, |_| None)
                .unwrap();
        }
    }

    #[test]
    fn test_available_languages() {
        let languages = available_languages();

        assert!(!languages.is_empty());
        assert!(languages.contains_key("elixir"));
        assert!(languages.contains_key("rust"));
        assert!(languages.contains_key("python"));

        let (friendly_name, extensions) = languages.get("elixir").unwrap();
        assert_eq!(friendly_name, "Elixir");
        assert!(extensions.contains(&"*.ex".to_string()));
        assert!(extensions.contains(&"*.exs".to_string()));

        for (id_name, (friendly_name, _extensions)) in languages {
            assert!(!id_name.is_empty());
            assert!(!friendly_name.is_empty());
        }
    }
}
