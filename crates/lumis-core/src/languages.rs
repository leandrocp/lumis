//! Language detection and metadata.
//!
//! This module provides the [`Language`] enum that represents all supported programming
//! languages. It includes automatic language detection based on file extensions,
//! file names, content analysis, and shebangs.
//!
//! This module is independent of tree-sitter. For tree-sitter configuration,
//! see the `lumis` crate's `languages` module.

// Guess Language copied from https://github.com/Wilfred/difftastic/blob/dc2283500838fe5018a48c65682646c9637d7816/src/parse/guess_language.rs
//
// That is the last upstream commit to touch the file. The catalog itself lives
// in `languages.toml`, so only the detection logic is synced from upstream;
// upstream dropping a language is not a reason for Lumis to drop it.

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;
use typed_path::Utf8WindowsPath;

/// Declarative macro that generates the `Language` enum, iteration, `FromStr`,
/// `name()`, `id_name()`, `language_globs()`, `from_emacs_mode_header()`,
/// `from_shebang()`, and `available_languages()` — all with `#[cfg(feature)]`
/// gates on the gated variants.
macro_rules! define_languages {
    (
        always {
            $(
                $a_variant:ident {
                    id: $a_id:expr,
                    name: $a_name:expr,
                    from_str: [$($a_str:expr),* $(,)?],
                    globs: [$($a_glob:expr),* $(,)?],
                    emacs: [$($a_emacs:expr),* $(,)?],
                    shebang: [$($a_shebang:expr),* $(,)?]
                }
            ),* $(,)?
        }
        gated {
            $(
                [$g_feat:expr] $g_variant:ident {
                    id: $g_id:expr,
                    name: $g_name:expr,
                    from_str: [$($g_str:expr),* $(,)?],
                    globs: [$($g_glob:expr),* $(,)?],
                    emacs: [$($g_emacs:expr),* $(,)?],
                    shebang: [$($g_shebang:expr),* $(,)?]
                }
            ),* $(,)?
        }
    ) => {
        /// Every language id and the globs it claims, whatever the feature set.
        ///
        /// The same declarations as [`Language::globs`], without the gates.
        /// Several languages claim one extension, and a table that shrinks with
        /// the feature set would answer `*.m` with whichever of matlab and objc
        /// happened to be compiled in.
        static FILENAME_GLOBS: &[(&str, &[&str])] = &[
            $(($a_id, &[$($a_glob),*]),)*
            $(($g_id, &[$($g_glob),*]),)*
        ];

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum Language {
            $(
                $a_variant,
            )*
            $(
                #[cfg(feature = $g_feat)]
                $g_variant,
            )*
        }

        impl Default for Language {
            fn default() -> Self {
                Language::PlainText
            }
        }

        impl Language {
            /// Returns an iterator over all compiled-in language variants.
            pub fn iter() -> impl Iterator<Item = Language> {
                let always: &[Language] = &[
                    $(Language::$a_variant,)*
                ];
                let gated: &[Language] = &[
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant,
                    )*
                ];
                always.iter().copied().chain(gated.iter().copied())
            }

            pub fn language_globs(language: Language) -> Vec<glob::Pattern> {
                let glob_strs: &'static [&'static str] = match language {
                    $(
                        Language::$a_variant => &[$($a_glob),*],
                    )*
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant => &[$($g_glob),*],
                    )*
                };

                glob_strs
                    .iter()
                    .map(|name| glob::Pattern::new(name).expect("failed to guess language by path"))
                    .collect()
            }

            pub fn name(&self) -> &'static str {
                match self {
                    $(
                        Language::$a_variant => $a_name,
                    )*
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant => $g_name,
                    )*
                }
            }

            /// Alternative names this language answers to, without its id.
            ///
            /// ```
            /// # #[cfg(feature = "lang-bash")] {
            /// use lumis_core::languages::Language;
            /// assert!(Language::Bash.aliases().contains(&"sh"));
            /// # }
            /// ```
            pub fn aliases(&self) -> &'static [&'static str] {
                let all: &'static [&'static str] = match self {
                    $(
                        Language::$a_variant => &[$($a_str),*],
                    )*
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant => &[$($g_str),*],
                    )*
                };

                // The generated list leads with the id itself.
                match all.split_first() {
                    Some((first, rest)) if *first == self.id_name() => rest,
                    _ => all,
                }
            }

            /// File name patterns this language claims, e.g. `*.rs`.
            ///
            /// ```
            /// # #[cfg(feature = "lang-bash")] {
            /// use lumis_core::languages::Language;
            /// assert!(Language::Bash.globs().contains(&"PKGBUILD"));
            /// # }
            /// ```
            pub fn globs(&self) -> &'static [&'static str] {
                match self {
                    $(
                        Language::$a_variant => &[$($a_glob),*],
                    )*
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant => &[$($g_glob),*],
                    )*
                }
            }

            /// Emacs `mode:` names that select this language.
            ///
            /// ```
            /// # #[cfg(feature = "lang-bash")] {
            /// use lumis_core::languages::Language;
            /// assert!(Language::Bash.emacs_modes().contains(&"sh"));
            /// # }
            /// ```
            pub fn emacs_modes(&self) -> &'static [&'static str] {
                match self {
                    $(
                        Language::$a_variant => &[$($a_emacs),*],
                    )*
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant => &[$($g_emacs),*],
                    )*
                }
            }

            /// Interpreter names in a shebang line that select this language.
            ///
            /// ```
            /// # #[cfg(feature = "lang-bash")] {
            /// use lumis_core::languages::Language;
            /// assert!(Language::Bash.shebangs().contains(&"bash"));
            /// # }
            /// ```
            pub fn shebangs(&self) -> &'static [&'static str] {
                match self {
                    $(
                        Language::$a_variant => &[$($a_shebang),*],
                    )*
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant => &[$($g_shebang),*],
                    )*
                }
            }

            /// The globs that name a bare file extension, e.g. `*.rs`.
            ///
            /// ```
            /// # #[cfg(feature = "lang-bash")] {
            /// use lumis_core::languages::Language;
            /// // `PKGBUILD` is a glob but not an extension.
            /// assert!(!Language::Bash.extensions().contains(&"PKGBUILD"));
            /// # }
            /// ```
            pub fn extensions(&self) -> Vec<&'static str> {
                self.globs()
                    .iter()
                    .copied()
                    .filter(|glob| glob.starts_with("*."))
                    .collect()
            }

            /// Everything the catalog knows about this language.
            ///
            /// ```
            /// # #[cfg(feature = "lang-rust")] {
            /// use lumis_core::languages::Language;
            /// assert_eq!(Language::Rust.info().name, "Rust");
            /// # }
            /// ```
            pub fn info(&self) -> LanguageInfo {
                LanguageInfo {
                    id: self.id_name(),
                    name: self.name(),
                    aliases: self.aliases(),
                    extensions: self.extensions(),
                    globs: self.globs(),
                    emacs_modes: self.emacs_modes(),
                    shebangs: self.shebangs(),
                }
            }

            pub fn id_name(&self) -> &'static str {
                match self {
                    $(
                        Language::$a_variant => $a_id,
                    )*
                    $(
                        #[cfg(feature = $g_feat)]
                        Language::$g_variant => $g_id,
                    )*
                }
            }

            fn from_emacs_mode_header(src: &str) -> Option<Language> {
                // Emacs only needs a `;` between file variables, so a header that
                // sets nothing else has none: `; -*- mode: Lisp -*-`. Requiring one
                // meant the shorthand branch saw those and read `mode: lisp` as the
                // whole mode name. The `.*?` is the one difference from upstream,
                // which anchors `mode:` to the opening `-*-` and so stopped
                // matching `-*- coding: utf-8; mode: python; -*-`.
                static MODE_RE: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r"-\*-.*?mode: *([a-zA-Z0-9_+-]+).*-\*-").unwrap());
                static SHORTHAND_RE: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r"-\*-(.+)-\*-").unwrap());

                for line in split_on_newlines(src).take(2) {
                    let mode_name: String = match (MODE_RE.captures(line), SHORTHAND_RE.captures(line)) {
                        (Some(cap), _) | (_, Some(cap)) => cap[1].into(),
                        _ => "".into(),
                    };
                    let lang = match mode_name.to_ascii_lowercase().trim() {
                        $(
                            $($a_emacs => Some(Language::$a_variant),)*
                        )*
                        $(
                            $(
                                #[cfg(feature = $g_feat)]
                                $g_emacs => Some(Language::$g_variant),
                            )*
                        )*
                        _ => None,
                    };
                    if lang.is_some() {
                        return lang;
                    }
                }

                None
            }

            fn from_shebang(src: &str) -> Option<Language> {
                // Anchored, so a `#!` anywhere else on the first line is a comment:
                // `bar = 1 #!/bin/bash` is Python, not Bash.
                static RE: LazyLock<Regex> =
                    LazyLock::new(|| Regex::new(r"^#! *(?:/usr/bin/env )?([^ ]+)").unwrap());

                let first_line = split_on_newlines(src).next()?;
                let cap = RE.captures(first_line)?;
                let interpreter_path = Path::new(&cap[1]);
                let name = normalize_shebang_command(&interpreter_path.file_name()?.to_string_lossy());

                NORMALIZED_SHEBANGS
                    .iter()
                    .find(|(_, candidates)| candidates.iter().any(|candidate| *candidate == name))
                    .map(|(language, _)| *language)
            }
        }

        impl std::str::FromStr for Language {
            type Err = LanguageParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let original = s;
                let s = s.trim();
                if s.is_empty() {
                    return Ok(Language::PlainText);
                }

                let s_lower = s.to_ascii_lowercase();

                let exact = match s_lower.as_str() {
                    $(
                        $($a_str => Some(Language::$a_variant),)*
                    )*
                    $(
                        $(
                            #[cfg(feature = $g_feat)]
                            $g_str => Some(Language::$g_variant),
                        )*
                    )*
                    _ => None,
                };

                if let Some(lang) = exact {
                    return Ok(lang);
                }

                let path = Utf8WindowsPath::new(&s_lower);

                if let Some(lang) = Language::from_glob(path) {
                    return Ok(lang);
                }

                if let Some(lang) = Language::from_extension(&s_lower) {
                    return Ok(lang);
                }

                Err(LanguageParseError(original.to_string()))
            }
        }

        /// Every compiled-in language and what the catalog knows about it,
        /// sorted by id.
        ///
        /// Elixir's `Lumis.available_languages/0` and JavaScript's
        /// `availableLanguages()` return this same record.
        pub fn available_languages() -> Vec<LanguageInfo> {
            let mut languages: Vec<LanguageInfo> = Language::iter().map(|l| l.info()).collect();
            languages.sort_unstable_by_key(|language| language.id);
            languages
        }

        #[deprecated(note = "use `available_languages()`, which carries aliases and detection metadata")]
        pub fn available_languages_map() -> HashMap<String, (String, Vec<String>)> {
            Language::iter()
                .map(|language| {
                    (
                        language.id_name().to_string(),
                        (
                            language.name().to_string(),
                            language.globs().iter().map(|g| (*g).to_string()).collect(),
                        ),
                    )
                })
                .collect()
        }
    };
}

/// What Lumis knows about one language.
///
/// Every runtime returns this same shape.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "lang-rust")] {
/// use lumis_core::languages::Language;
///
/// let info = Language::Rust.info();
///
/// assert_eq!(info.id, "rust");
/// assert_eq!(info.name, "Rust");
/// assert!(info.globs.contains(&"*.rs"));
/// # }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct LanguageInfo {
    /// Stable identifier, e.g. `rust`.
    pub id: &'static str,
    /// Human-readable name, e.g. `Rust`.
    pub name: &'static str,
    /// Alternative names this language answers to.
    pub aliases: &'static [&'static str],
    /// The subset of [`globs`](Self::globs) that name a bare extension.
    pub extensions: Vec<&'static str>,
    /// File name patterns, e.g. `*.rs` and `Cargo.lock`.
    pub globs: &'static [&'static str],
    /// Emacs `mode:` names that select this language.
    pub emacs_modes: &'static [&'static str],
    /// Interpreter names in a shebang line that select this language.
    pub shebangs: &'static [&'static str],
}

include!(concat!(env!("OUT_DIR"), "/languages_data.rs"));

/// Error returned when a language cannot be determined from input.
///
/// This error occurs when using [`std::str::FromStr`] or the `.parse()` method
/// with an unrecognized language name, file extension, or file path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageParseError(String);

impl std::fmt::Display for LanguageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown language or file type: {}", self.0)
    }
}

impl std::error::Error for LanguageParseError {}

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
    pub fn guess(language: Option<&str>, src: &str) -> Self {
        if let Some(input) = language {
            if let Ok(lang) = input.parse() {
                return lang;
            }
        }

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

        Language::PlainText
    }

    fn from_glob(path: &Utf8WindowsPath) -> Option<Self> {
        Self::first_matching_glob(path.file_name()?)
    }

    fn from_extension(token: &str) -> Option<Self> {
        let token = token.strip_prefix('.').unwrap_or(token);
        Self::first_matching_glob(&format!("*.{}", token.to_ascii_lowercase()))
    }

    /// The first language whose globs match `candidate`, of those compiled in.
    fn first_matching_glob(candidate: &str) -> Option<Self> {
        let id = id_matching_glob(candidate)?;
        Language::iter().find(|language| language.id_name() == id)
    }

    #[allow(dead_code)]
    fn looks_like_xml(src: &str) -> bool {
        src.trim_start().to_lowercase().starts_with("<?xml")
    }

    #[allow(dead_code)]
    fn looks_like_html(src: &str) -> bool {
        src.trim_start()
            .to_lowercase()
            .starts_with("<!doctype html")
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id_name())
    }
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

fn normalize_shebang_command(command: &str) -> String {
    static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+(?:\.\d+)*$").unwrap());

    let command = command.to_ascii_lowercase();
    VERSION_RE.replace(&command, "").into_owned()
}

/// `FILENAME_GLOBS` arranged for lookup rather than for iteration.
///
/// Asking `glob::Pattern` about all 383 globs costs about 11 µs for a name no
/// language claims, and `@injection.filename` runs once per changed line. 381 of
/// them are plain strings, so only two are left to walk.
struct FilenameIndex {
    /// Globs naming a whole file, e.g. `Dockerfile`.
    literals: HashMap<String, &'static str>,
    /// Globs naming a bare extension, e.g. `*.rs`, keyed by `rs`.
    extensions: HashMap<String, &'static str>,
    /// Whatever is left, e.g. `*.blade.php`.
    patterns: Vec<(&'static str, glob::Pattern)>,
}

impl FilenameIndex {
    fn id_for(&self, candidate: &str) -> Option<&'static str> {
        let mut best = self.literals.get(candidate).copied();

        if let Some((_, extension)) = candidate.rsplit_once('.') {
            best = lower_id(best, self.extensions.get(extension).copied());
        }

        for (id, pattern) in &self.patterns {
            if pattern.matches(candidate) {
                best = lower_id(best, Some(id));
            }
        }

        best
    }
}

fn lower_id(left: Option<&'static str>, right: Option<&'static str>) -> Option<&'static str> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (found, None) | (None, found) => found,
    }
}

/// Feature gates are deliberately not applied. Several languages claim one
/// extension, and a table that shrinks with the feature set would answer `*.m`
/// with whichever of matlab and objc happened to be compiled in.
/// [`Language::first_matching_glob`] applies the gates afterwards.
///
/// Patterns are lowercased because the name they are matched against is, so a
/// `CMakeLists.txt` hint and a `cmakelists.txt` hint reach the same language.
static FILENAME_INDEX: LazyLock<FilenameIndex> = LazyLock::new(|| {
    let mut index = FilenameIndex {
        literals: HashMap::new(),
        extensions: HashMap::new(),
        patterns: Vec::new(),
    };

    // Sorted because `define_languages!` splits the always-on languages out from
    // the gated ones, so its order is not `languages.toml`'s. Inserting in id
    // order makes the first write per key the winner.
    let mut entries: Vec<_> = FILENAME_GLOBS.iter().collect();
    entries.sort_by_key(|(id, _)| *id);

    for (id, globs) in entries {
        for glob in *globs {
            let glob = glob.to_ascii_lowercase();

            if let Some(extension) = glob.strip_prefix("*.") {
                if !extension.contains(['*', '?', '[']) {
                    index.extensions.entry(extension.to_string()).or_insert(id);
                    continue;
                }
            }

            if glob.contains(['*', '?', '[']) {
                let pattern = glob::Pattern::new(&glob).expect("catalog glob is a valid pattern");
                index.patterns.push((id, pattern));
            } else {
                index.literals.entry(glob).or_insert(id);
            }
        }
    }

    index
});

/// The id of the first language whose globs match `candidate`.
///
/// `candidate` is already lowercased, and may be a file name or a `*.ext` glob,
/// which resolves through the extension map like the name it stands for.
fn id_matching_glob(candidate: &str) -> Option<&'static str> {
    FILENAME_INDEX.id_for(candidate)
}

/// The id of the language claiming `path`, by file name.
///
/// Returns an id rather than a [`Language`] because a caller that loads parsers
/// at runtime can use a language this build did not compile in;
/// [`Language::from_str`] is the gated view of the same table.
///
/// ```
/// use lumis_core::languages::language_id_for_filename;
///
/// assert_eq!(language_id_for_filename("lib/varsel.ex"), Some("elixir"));
/// assert_eq!(language_id_for_filename("Dockerfile"), Some("dockerfile"));
/// assert_eq!(language_id_for_filename("/dev/null"), None);
/// ```
pub fn language_id_for_filename(path: &str) -> Option<&'static str> {
    let path = path.trim().to_ascii_lowercase();
    id_matching_glob(Utf8WindowsPath::new(&path).file_name()?)
}

/// Every catalog shebang, normalized, in catalog order.
///
/// The candidates are static, so normalizing them per detection ran the version
/// regex over the whole catalog on every call.
static NORMALIZED_SHEBANGS: LazyLock<Vec<(Language, Vec<String>)>> = LazyLock::new(|| {
    Language::iter()
        .map(|language| {
            let commands = language
                .shebangs()
                .iter()
                .map(|shebang| normalize_shebang_command(shebang))
                .collect();
            (language, commands)
        })
        .collect()
});

#[cfg(test)]
mod tests {
    use super::*;

    /// Stripping a trailing version turns `python3` into `python`, which is the
    /// point. It would also turn `perl6` into `perl`, and Perl 6 is Raku — a
    /// different language. Nothing else warns if the catalog grows a shebang
    /// like that, so the collision is what this asserts.
    #[test]
    fn no_two_languages_share_a_normalized_shebang() {
        let mut owners: HashMap<&str, Vec<&'static str>> = HashMap::new();

        for (language, commands) in NORMALIZED_SHEBANGS.iter() {
            for command in commands {
                owners
                    .entry(command.as_str())
                    .or_default()
                    .push(language.id_name());
            }
        }

        let collisions: Vec<_> = owners
            .iter()
            .filter(|(_, languages)| {
                languages
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    > 1
            })
            .collect();

        assert!(
            collisions.is_empty(),
            "shebangs normalize onto one another: {collisions:?}"
        );
    }

    /// [`Language::from_str`] matches an id or an alias before it consults a
    /// glob, so a name two parsers both claim resolves by whichever `match` arm
    /// the macro emitted first, and a name that is also somebody's extension
    /// takes that extension over. Neither is an error anywhere: the catalog
    /// keeps working, at a different language than the one it names.
    #[test]
    fn no_name_is_claimed_twice_or_shadows_a_glob() {
        let mut owners: HashMap<String, &'static str> = HashMap::new();

        for language in Language::iter() {
            let id = language.id_name();

            for name in std::iter::once(id).chain(language.aliases().iter().copied()) {
                let name = name.to_ascii_lowercase();

                if let Some(owner) = owners.insert(name.clone(), id) {
                    panic!("`{name}` is claimed twice, by {owner} and by {id}");
                }

                if let Some(owner) = id_matching_glob(&name) {
                    assert_eq!(owner, id, "`{name}` names {id} but is {owner}'s file name");
                }

                if let Some(owner) = id_matching_glob(&format!("*.{name}")) {
                    assert_eq!(owner, id, "`{name}` names {id} but is {owner}'s extension");
                }
            }
        }

        // Counting `owners` would only say how many languages this feature set
        // compiled in. `FILENAME_GLOBS` is generated without gates, so it
        // answers for the whole catalog however few of them are on, and an
        // empty one is the generator bug worth catching here. That aliases are
        // read at all is `Language::aliases`'s own doctest.
        assert!(
            FILENAME_GLOBS.len() > 110,
            "found {} languages in the catalog, expected the whole of it",
            FILENAME_GLOBS.len()
        );
    }

    /// An alias repeating its own id is dead weight that the published catalog
    /// table renders as a column entry.
    #[test]
    fn no_alias_repeats_its_own_id() {
        for language in Language::iter() {
            let id = language.id_name();
            for alias in language.aliases() {
                assert!(
                    !alias.eq_ignore_ascii_case(id),
                    "{id} lists its own id as an alias"
                );
            }
        }
    }

    #[test]
    fn plaintext_metadata_and_detection_are_consistent() {
        let plaintext = Language::PlainText.info();

        assert_eq!(plaintext.name, "Plain Text");
        assert_eq!(plaintext.aliases, ["text", "txt", "plain"]);
        assert_eq!(plaintext.emacs_modes, ["fundamental", "text"]);
        assert!(
            plaintext.extensions.is_empty(),
            "plaintext claims an extension"
        );
        assert!(plaintext.globs.is_empty(), "plaintext claims a glob");
        assert!(plaintext.shebangs.is_empty(), "plaintext claims a shebang");

        for name in std::iter::once(plaintext.id).chain(plaintext.aliases.iter().copied()) {
            assert_eq!(
                Language::guess(Some(name), "#!/usr/bin/env bash"),
                Language::PlainText
            );
        }
        assert_eq!(
            Language::guess(None, "// -*- mode: text -*-\nfn main() {}"),
            Language::PlainText
        );
    }

    /// Emacs writes a `;` between file variables, not after the last one, so a
    /// header that sets only the mode has none at all.
    #[cfg(feature = "lang-commonlisp")]
    #[test]
    fn emacs_mode_header_needs_no_semicolon() {
        assert_eq!(
            Language::guess(None, "; -*- mode: Lisp -*-"),
            Language::CommonLisp
        );
        assert_eq!(
            Language::guess(None, "; -*- mode: Lisp; eval: (auto-fill-mode 1); -*-"),
            Language::CommonLisp
        );
    }

    /// `mode:` is rarely the first file variable in the wild — `coding:` usually
    /// comes first. Upstream anchors `mode:` to the opening `-*-`; this does not.
    #[cfg(feature = "lang-python")]
    #[test]
    fn emacs_mode_header_may_follow_another_file_variable() {
        assert_eq!(
            Language::guess(None, "# -*- coding: utf-8; mode: python; -*-"),
            Language::Python
        );
    }

    /// A `#!` that opens the line is a shebang; the same bytes further along are
    /// a comment, and reading them as a shebang made every commented-out command
    /// rename the file's language.
    #[test]
    fn a_shebang_has_to_open_the_line() {
        assert_eq!(
            Language::guess(None, "bar = 1 #!/bin/bash"),
            Language::PlainText
        );

        #[cfg(feature = "lang-bash")]
        assert_eq!(Language::guess(None, "#!/bin/bash"), Language::Bash);
    }
}
