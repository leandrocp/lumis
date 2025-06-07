use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rayon::prelude::*;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    vendored_parsers();
    queries();
    themes();
}

struct TreeSitterParser {
    name: &'static str,
    src_dir: &'static str,
    extra_files: Vec<&'static str>,
}

impl TreeSitterParser {
    fn build(&self) {
        let dir = PathBuf::from(&self.src_dir);

        let mut c_files = vec!["parser.c"];
        let mut cpp_files = vec![];

        for file in &self.extra_files {
            if file.ends_with(".c") {
                c_files.push(file);
            } else {
                cpp_files.push(file);
            }
        }

        if !cpp_files.is_empty() {
            let mut cpp_build = cc::Build::new();
            cpp_build
                .include(&dir)
                .cpp(true)
                .std("c++14")
                .flag_if_supported("-Wno-implicit-fallthrough")
                .flag_if_supported("-Wno-unused-parameter")
                .flag_if_supported("-Wno-ignored-qualifiers")
                .link_lib_modifier("+whole-archive");

            for file in cpp_files {
                cpp_build.file(dir.join(file));
            }

            cpp_build.compile(&format!("{}-cpp", self.name));
        }

        let mut build = cc::Build::new();

        // if cfg!(target_env = "msvc") {
        //     build.flag("/utf-8");
        // }

        build.include(&dir).warnings(false);

        // Add unique prefix for symbols to avoid conflicts
        if self.name == "tree-sitter-angular" || self.name == "tree-sitter-vue" {
            build.flag(format!(
                "-DTAG_TYPES_BY_TAG_NAME={}_{}",
                self.name.replace("-", "_"),
                "TAG_TYPES_BY_TAG_NAME"
            ));
        }

        for file in c_files {
            build.file(dir.join(file));
        }

        build.link_lib_modifier("+whole-archive");

        build.compile(self.name);
    }
}

// https://github.com/Wilfred/difftastic/blob/8953c55cf854ceac2ccb6ece004d6a94a5bfa122/build.rs
// TODO: remove vendored parsers in favor of crates as soon as they implement LanguageFn
fn vendored_parsers() {
    let parsers = vec![
        TreeSitterParser {
            name: "tree-sitter-angular",
            src_dir: "vendored_parsers/tree-sitter-angular/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-astro",
            src_dir: "vendored_parsers/tree-sitter-astro/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-clojure",
            src_dir: "vendored_parsers/tree-sitter-clojure/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-commonlisp",
            src_dir: "vendored_parsers/tree-sitter-commonlisp/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-csv",
            src_dir: "vendored_parsers/tree-sitter-csv/csv/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-dockerfile",
            src_dir: "vendored_parsers/tree-sitter-dockerfile/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-eex",
            src_dir: "vendored_parsers/tree-sitter-eex/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-glimmer",
            src_dir: "vendored_parsers/tree-sitter-glimmer/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-graphql",
            src_dir: "vendored_parsers/tree-sitter-graphql/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-iex",
            src_dir: "vendored_parsers/tree-sitter-iex/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-kotlin",
            src_dir: "vendored_parsers/tree-sitter-kotlin/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-latex",
            src_dir: "vendored_parsers/tree-sitter-latex/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-liquid",
            src_dir: "vendored_parsers/tree-sitter-liquid/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-llvm",
            src_dir: "vendored_parsers/tree-sitter-llvm/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-make",
            src_dir: "vendored_parsers/tree-sitter-make/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-perl",
            src_dir: "vendored_parsers/tree-sitter-perl/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-scss",
            src_dir: "vendored_parsers/tree-sitter-scss/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-surface",
            src_dir: "vendored_parsers/tree-sitter-surface/src",
            extra_files: vec![],
        },
        TreeSitterParser {
            name: "tree-sitter-vim",
            src_dir: "vendored_parsers/tree-sitter-vim/src",
            extra_files: vec!["scanner.c"],
        },
        TreeSitterParser {
            name: "tree-sitter-vue",
            src_dir: "vendored_parsers/tree-sitter-vue/src",
            extra_files: vec!["scanner.c"],
        },
    ];

    for parser in &parsers {
        println!("cargo:rerun-if-changed={}", parser.src_dir);
    }

    parsers.par_iter().for_each(|p| p.build());
}

fn read_query_file(path: &Path, language: &str, query: &str) -> String {
    if !path.exists() {
        return String::new();
    }

    let mut query_content: Vec<String> = Vec::new();

    let original_content = fs::read_to_string(path).expect("failed to ready query file");

    // fix incompatible patterns
    let content = original_content
        .replace("@spell", "")
        .replace("@nospell", "")
        .replace("; inherits html_tags", "; inherits: html_tags")
        .replace(
            "#set! @string.special.url url @string.special.url",
            "#set! @string.special.url url \"string.special.url\"",
        )
        .replace(
            "#set! @_label url @_url",
            "#set! @_label url \"markup.link.url\"",
        )
        .replace(
            "#set! @_url url @_url",
            "#set! @_url highlight \"markup.link.url\"",
        )
        .replace(
            "#set! @_hyperlink url @markup.link.url",
            "#set! @_hyperlink highlight \"markup.link.url\"",
        )
        .replace("\\\\c", "(?i)")
        .replace("^{[-]|[^|]", "^\\{[-]|^\\{[^|]")
        .replace(r#""^\\if"#, r#""^if"#);

    let content = convert_lua_matches(&content);

    for line in content.lines() {
        if line.starts_with("; inherits: ") {
            let inherits_str = line.trim_start_matches("; inherits: ").trim();

            let parent_languages: Vec<String> = inherits_str
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            for parent_language in parent_languages {
                let parent_path =
                    PathBuf::from(format!("queries/{}/{}.scm", parent_language, query));
                let parent_content = read_query_file(&parent_path, &parent_language, query);
                query_content.push(parent_content.clone());
            }
        }
    }

    query_content.push(format!("\n; query: {}", language));
    query_content.push(content.clone());

    let overwrite_path = PathBuf::from(format!("overwrites/{}/{}.scm", language, query));
    if overwrite_path.exists() {
        println!(
            "cargo:warning=appending {} into {}",
            overwrite_path.display(),
            path.display()
        );
        let overwrite_content =
            fs::read_to_string(&overwrite_path).expect("failed to read overwrite file");
        query_content.push(format!("\n; overwrite: {}", overwrite_path.display()));
        query_content.push(overwrite_content);
    }

    query_content.join("\n")
}

fn queries() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest_path = out_dir.join("queries_constants.rs");

    let queries_path = PathBuf::from("queries");
    let mut generated_code = TokenStream::new();

    let entries = fs::read_dir(&queries_path).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let language = path.file_name().unwrap().to_str().unwrap();
        println!("cargo:rerun-if-changed=queries/{}", language);
        println!("cargo:rerun-if-changed=overwrites/{}", language);

        let lang_upper = language.to_uppercase();
        let queries = ["highlights", "injections", "locals"];

        for query in queries {
            let file_path = path.join(format!("{}.scm", query));
            let const_name = format_ident!("{}_{}", lang_upper, query.to_uppercase());
            let processed_content = read_query_file(&file_path, language, query);

            generated_code.extend(quote! {
                #[doc(hidden)]
                pub const #const_name: &str = #processed_content;
            });

            generated_code.extend(quote! {});
        }

        generated_code.extend(quote! {});
    }

    let mut output_file = File::create(&dest_path).unwrap();

    write!(
        output_file,
        "{}",
        prettyplease::unparse(&syn::parse2::<syn::File>(generated_code).unwrap())
    )
    .unwrap();
}

fn convert_lua_matches(content: &str) -> String {
    let mut result = String::new();
    let lines: Vec<&str> = content.lines().collect();

    for line in lines {
        let line = line
            .replace("#lua-match?", "#match?")
            .replace("#not-lua-match?", "#not-match?");

        if line.contains("#match?") || line.contains("#not-match?") {
            if let Some(pattern_start) = line.find('"') {
                if let Some(pattern_end) = line[pattern_start + 1..].find('"') {
                    let pattern_end = pattern_start + 1 + pattern_end;
                    let lua_pattern = &line[pattern_start + 1..pattern_end];

                    let rust_pattern = convert_lua_pattern_to_rust_regex(lua_pattern);

                    let mut new_line = line[..pattern_start + 1].to_string();
                    new_line.push_str(&rust_pattern);
                    new_line.push_str(&line[pattern_end..]);

                    result.push_str(&new_line);
                    result.push('\n');
                    continue;
                }
            }
        }

        result.push_str(&line);
        result.push('\n');
    }

    if !content.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    result
}

fn convert_lua_pattern_to_rust_regex(lua_pattern: &str) -> String {
    let mut result = String::new();
    let mut chars = lua_pattern.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(&next_char) = chars.peek() {
                match next_char {
                    'd' => {
                        result.push_str("\\d");
                        chars.next();
                    }
                    's' => {
                        result.push_str("\\s");
                        chars.next();
                    }
                    'l' => {
                        result.push_str("[a-z]");
                        chars.next();
                    }
                    'u' => {
                        result.push_str("[A-Z]");
                        chars.next();
                    }
                    'A' => {
                        result.push_str("[^a-zA-Z]");
                        chars.next();
                    }
                    'S' => {
                        result.push_str("\\S");
                        chars.next();
                    }
                    '.' => {
                        result.push_str("\\.");
                        chars.next();
                    }
                    '%' => {
                        result.push('%');
                        chars.next();
                    }
                    '{' => {
                        result.push_str("\\{");
                        chars.next();
                    }
                    '}' => {
                        result.push_str("\\}");
                        chars.next();
                    }
                    '$' => {
                        // Special handling for $
                        result.push_str("\\$");
                        chars.next();
                        // Check if next char is {, which needs special handling in Rust regex
                        if let Some(&next) = chars.peek() {
                            if next == '{' {
                                result.push('\\'); // Add extra escape for ${
                            }
                        }
                    }
                    '^' => {
                        result.push_str("\\^");
                        chars.next();
                    }
                    _ => {
                        result.push('\\');
                        result.push(next_char);
                        chars.next();
                    }
                }
            } else {
                result.push('%');
            }
        } else if c == '\\' {
            result.push('\\');
            result.push('\\');
            if let Some(&next_char) = chars.peek() {
                result.push(next_char);
                chars.next();
            }
        } else if c == '$' {
            // Handle special $ character
            result.push_str("\\$");
            // Check if next char is {, which needs special handling in Rust regex
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    result.push('\\'); // Add extra escape for ${
                }
            }
        } else if c == '.'
            || c == '*'
            || c == '+'
            || c == '?'
            || c == '('
            || c == ')'
            || c == '['
            || c == ']'
            || c == '{'
            || c == '}'
            || c == '|'
            || (c == '^' && !result.is_empty())
        {
            result.push('\\');
            result.push(c);
        } else {
            result.push(c);
        }
    }

    result
}

fn themes() {
    println!("cargo:rerun-if-changed=themes");

    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("theme_data.rs");
    let themes_dir = Path::new(&manifest_dir).join("themes");

    let theme_names: Vec<String> = fs::read_dir(&themes_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                path.file_stem().and_then(|s| s.to_str()).map(String::from)
            } else {
                None
            }
        })
        .collect();

    let theme_constants = theme_names.iter().map(|name| {
        let constant_name = format_ident!("{}", name.to_uppercase());
        let json_path = format!("{}/{}.json", themes_dir.display(), name);

        quote! {
            #[doc(hidden)]
            pub(crate) static #constant_name: LazyLock<Theme> = LazyLock::new(|| {
                let theme_str = include_str!(#json_path);
                 crate::themes::from_json(theme_str).unwrap_or_else(|_| panic!("failed to load theme: {}", #name))
            });
        }
    });

    let theme_refs = theme_names.iter().map(|name| {
        let constant_name = format_ident!("{}", name.to_uppercase());
        quote! { &#constant_name }
    });

    let theme_name_matches = theme_names.iter().map(|name| {
        let constant_name = format_ident!("{}", name.to_uppercase());
        let name_str = name.to_lowercase();
        quote! { #name_str => Ok(&#constant_name), }
    });

    let output = quote! {
        use std::sync::LazyLock;

        #(#theme_constants)*

        #[doc(hidden)]
        pub static ALL_THEMES: LazyLock<Vec<&'static Theme>> = LazyLock::new(|| vec![
            #(#theme_refs),*
        ]);

        /// Retrieves a theme by its name.
        ///
        /// # Examples
        ///
        /// ```
        /// use autumnus::themes;
        ///
        /// let theme = themes::get("github_light").expect("Theme not found");
        /// assert_eq!(theme.name, "github_light");
        ///
        /// let theme = themes::get("non_existent_theme");
        /// assert!(theme.is_err());
        /// ```
        pub fn get(name: &str) -> Result<&'static Theme, ThemeError> {
            match name {
                #(#theme_name_matches)*
                _ => Err(ThemeError::NotFound(name.to_string())),
            }
        }
    };

    fs::write(dest_path, output.to_string()).unwrap();
}
