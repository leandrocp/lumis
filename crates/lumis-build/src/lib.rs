//! Build-time helpers shared by every Lumis runtime.
//!
//! The `license` feature adds the `license` module, which identifies the license of a
//! theme plugin during theme extraction.
//!
//! Query preprocessing in `crates/dev` rewrites
//! Neovim's `#lua-match?` predicates into Tree-sitter's built-in `#match?`
//! predicates so that Rust, the CLI, Elixir, JavaScript, and the browser all read
//! byte-identical `.scm` files.
//!
//! # Reference semantics
//!
//! Lumis queries come from `nvim-treesitter`, so Neovim is the reference for what
//! a predicate means:
//!
//! - `#lua-match?` is `string.find(node_text, pattern)` in
//!   `runtime/lua/vim/treesitter/query.lua`, i.e. a **Lua 5.1 pattern**, matched
//!   unanchored unless the pattern starts with `^`.
//! - `#match?` (and its `#vim-match?` alias) is a **Vim regex** in Neovim, but
//!   Lumis evaluates it with Tree-sitter's own implementation: the `regex` crate in
//!   Rust and `RegExp` in `web-tree-sitter`. Patterns that already use `#match?`
//!   are therefore passed through untouched.
//!
//! Translation must land on a regex that is **valid and equivalent in both** the
//! `regex` crate and JavaScript `RegExp` (no `u`/`v` flag). The two engines
//! disagree in ways that matter, so this module avoids the ambiguous constructs:
//!
//! - `[[A-Z]]` is a nested class in `regex` (`[A-Z]`) but `[` or `A-Z` followed by
//!   a literal `]` in `RegExp`. Character classes are therefore never nested; Lua
//!   classes expand to bare range fragments inside `[...]`.
//! - `\d`, `\w`, and `\s` are Unicode-aware in `regex` and ASCII-only in `RegExp`.
//!   Lua classes expand to explicit ASCII ranges instead, matching Lua's C-locale
//!   definitions exactly.
//! - `^*?` is accepted by `regex` and rejected by `RegExp`. Quantifiers are only
//!   emitted where Lua actually reads one.

use std::fmt;

#[cfg(feature = "license")]
pub mod license;

/// A Lua pattern construct that has no portable regular-expression translation.
///
/// None of these appear in the queries Lumis vendors today. The corpus test in
/// `tests/processed_queries.rs` fails if one is ever introduced, rather than
/// letting a silently wrong regex reach a released language package.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum LuaPatternError {
    /// `%bxy`, which matches balanced delimiters.
    BalancedMatch,
    /// `%f[set]`, a frontier pattern.
    Frontier,
    /// `%1`-`%9`, a capture back reference.
    BackReference(char),
    /// A complement class such as `%D` used inside `[...]`, which would need a
    /// nested negated class.
    ComplementClassInSet(char),
    /// A `[` that is never closed.
    UnterminatedSet,
    /// A trailing `%` with nothing to escape.
    DanglingEscape,
}

impl fmt::Display for LuaPatternError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BalancedMatch => {
                write!(formatter, "unsupported Lua balanced match '%b'")
            }
            Self::Frontier => write!(formatter, "unsupported Lua frontier pattern '%f'"),
            Self::BackReference(digit) => {
                write!(formatter, "unsupported Lua back reference '%{digit}'")
            }
            Self::ComplementClassInSet(class) => write!(
                formatter,
                "unsupported Lua complement class '%{class}' inside '[...]'"
            ),
            Self::UnterminatedSet => write!(formatter, "unterminated '[' in Lua pattern"),
            Self::DanglingEscape => write!(formatter, "trailing '%' in Lua pattern"),
        }
    }
}

impl std::error::Error for LuaPatternError {}

/// Rewrite every Lua pattern predicate in `content` into its Tree-sitter
/// equivalent, replacing an untranslatable pattern with an escaped literal.
///
/// Prefer [`try_convert_lua_matches`] in code generation, where an untranslatable
/// pattern should abort the build instead of silently changing meaning.
///
/// # Examples
///
/// ```
/// use lumis_build::convert_lua_matches;
///
/// let query = r#"((identifier) @type (#lua-match? @type "^%u"))"#;
/// assert_eq!(
///     convert_lua_matches(query),
///     r#"((identifier) @type (#match? @type "^[A-Z]"))"#,
/// );
/// ```
pub fn convert_lua_matches(content: &str) -> String {
    // `EscapeLiteral` never rejects a pattern, so the error arm cannot be reached.
    // Returning the input keeps this total instead of introducing a panic path.
    convert_content(content, PatternFallback::EscapeLiteral).unwrap_or_else(|_| content.to_owned())
}

/// Rewrite every `#lua-match?`, `#not-lua-match?`, `#any-lua-match?`, and
/// `#any-not-lua-match?` predicate in `content` into `#match?`, `#not-match?`,
/// `#any-match?`, and `#any-not-match?` with a portable regular expression, and
/// expand `(?i)` in predicates that were already regular expressions.
///
/// # Errors
///
/// Returns the first [`LuaPatternError`] encountered, so a query that cannot be
/// translated faithfully fails generation rather than shipping a regex that means
/// something else.
///
/// # Examples
///
/// ```
/// use lumis_build::{try_convert_lua_matches, LuaPatternError};
///
/// let query = r#"((comment) @doc (#lua-match? @doc "^/%*%*[^*]"))"#;
/// assert_eq!(
///     try_convert_lua_matches(query).unwrap(),
///     r#"((comment) @doc (#match? @doc "^/\\*\\*[^*]"))"#,
/// );
///
/// let unsupported = r#"((s) @x (#lua-match? @x "%bxy"))"#;
/// assert_eq!(
///     try_convert_lua_matches(unsupported),
///     Err(LuaPatternError::BalancedMatch),
/// );
/// ```
pub fn try_convert_lua_matches(content: &str) -> Result<String, LuaPatternError> {
    convert_content(content, PatternFallback::Reject)
}

/// How [`convert_content`] handles a pattern that cannot be translated.
#[derive(Clone, Copy, Eq, PartialEq)]
enum PatternFallback {
    EscapeLiteral,
    Reject,
}

fn convert_content(content: &str, fallback: PatternFallback) -> Result<String, LuaPatternError> {
    let mut result = String::with_capacity(content.len());

    for line in content.lines() {
        result.push_str(&convert_line(line, fallback)?);
        result.push('\n');
    }

    if !content.ends_with('\n') && result.ends_with('\n') {
        result.pop();
    }

    Ok(result)
}

/// Predicate operators whose argument is a Lua pattern, paired with the
/// Tree-sitter operator that replaces them.
const LUA_MATCH_OPERATORS: [(&str, &str); 4] = [
    ("#not-lua-match?", "#not-match?"),
    ("#lua-match?", "#match?"),
    ("#any-lua-match?", "#any-match?"),
    ("#any-not-lua-match?", "#any-not-match?"),
];

/// Convert one line, translating only the string argument that belongs to a
/// `#lua-match?` style operator.
///
/// Scanning per operator rather than "the first quoted string on the line" keeps
/// a second predicate on the same line, such as `(#eq? @name "foo")`, untouched.
fn convert_line(line: &str, fallback: PatternFallback) -> Result<String, LuaPatternError> {
    let mut result = String::with_capacity(line.len());
    let mut rest = line;

    while let Some((offset, lua_operator, regex_operator)) = next_lua_match_operator(rest) {
        result.push_str(&expand_query_case_insensitive_regexes(&rest[..offset]));
        result.push_str(regex_operator);
        rest = &rest[offset + lua_operator.len()..];

        let Some(pattern_start) = rest.find('"') else {
            break;
        };
        let Some(pattern_length) = rest[pattern_start + 1..].find('"') else {
            break;
        };
        let pattern_end = pattern_start + 1 + pattern_length;
        // Tree-sitter resolves the string escapes before a predicate ever sees the
        // argument, so `"[ \t]"` reaches Neovim's `string.find` as a real tab. The
        // Lua pattern is the unescaped text, not the source text.
        let lua_pattern = unescape_query_string(&rest[pattern_start + 1..pattern_end]);

        let regex = match (convert_lua_pattern(&lua_pattern), fallback) {
            (Ok(regex), _) => regex,
            (Err(_), PatternFallback::EscapeLiteral) => escape_regex_literal(&lua_pattern),
            (Err(error), PatternFallback::Reject) => return Err(error),
        };

        result.push_str(&rest[..=pattern_start]);
        result.push_str(&escape_regex_for_query_string(&regex));
        result.push('"');
        rest = &rest[pattern_end + 1..];
    }

    result.push_str(&expand_query_case_insensitive_regexes(rest));
    Ok(result)
}

fn next_lua_match_operator(haystack: &str) -> Option<(usize, &'static str, &'static str)> {
    LUA_MATCH_OPERATORS
        .iter()
        .filter_map(|(lua_operator, regex_operator)| {
            haystack
                .find(lua_operator)
                .map(|offset| (offset, *lua_operator, *regex_operator))
        })
        .min_by_key(|(offset, lua_operator, _)| (*offset, std::cmp::Reverse(lua_operator.len())))
}

/// Translate a single Lua pattern into a regular expression that behaves
/// identically in the `regex` crate and in JavaScript `RegExp`.
///
/// Lua's positional rules are honoured: `^` only anchors at the start of the
/// pattern, `$` only at the end, and `*`, `+`, `-`, and `?` are quantifiers only
/// where they directly follow a single pattern item. Everywhere else they are
/// literal characters, exactly as `string.find` reads them.
///
/// # Errors
///
/// Returns a [`LuaPatternError`] for a Lua construct that regular expressions
/// cannot express, such as `%b`, `%f`, or a back reference.
///
/// # Examples
///
/// ```
/// use lumis_build::convert_lua_pattern;
///
/// // `-` after `^` is a literal dash in Lua, not a lazy quantifier.
/// assert_eq!(convert_lua_pattern("^-%>").unwrap(), "^->");
/// // `-` after a pattern item is Lua's lazy `*`.
/// assert_eq!(convert_lua_pattern("a-b").unwrap(), "a*?b");
/// // `^` directly after `[` negates the set.
/// assert_eq!(convert_lua_pattern("[^*]").unwrap(), "[^*]");
/// // Classes expand to bare ranges inside a set so nothing nests.
/// assert_eq!(convert_lua_pattern("[%u]").unwrap(), "[A-Z]");
/// ```
pub fn convert_lua_pattern(lua_pattern: &str) -> Result<String, LuaPatternError> {
    let pattern: Vec<char> = lua_pattern.chars().collect();
    let mut regex = String::with_capacity(lua_pattern.len() * 2);
    let mut index = 0;
    // True when the previous token was a single pattern item, which is the only
    // position where Lua reads `*`, `+`, `-`, or `?` as a quantifier.
    let mut quantifiable = false;

    while index < pattern.len() {
        let character = pattern[index];

        // `^` anchors only as the first character of the whole pattern.
        if character == '^' && index == 0 {
            regex.push('^');
            index += 1;
            quantifiable = false;
            continue;
        }

        // `$` anchors only as the last character of the whole pattern.
        if character == '$' && index + 1 == pattern.len() {
            regex.push('$');
            index += 1;
            quantifiable = false;
            continue;
        }

        if let Some(quantifier) = lua_quantifier(character) {
            if quantifiable {
                regex.push_str(quantifier);
                quantifiable = false;
            } else {
                push_regex_literal(&mut regex, character);
                quantifiable = true;
            }
            index += 1;
            continue;
        }

        match character {
            // Lua's `.` matches any character including newlines; regex `.` stops
            // at newlines unless the engine is in "dot matches all" mode, which is
            // not portable. `[\s\S]` is exact in both engines.
            '.' => {
                regex.push_str("[\\s\\S]");
                index += 1;
                quantifiable = true;
            }
            // Lua cannot quantify a capture, so a `*` after `)` stays literal.
            '(' | ')' => {
                regex.push(character);
                index += 1;
                quantifiable = false;
            }
            '%' => {
                let Some(&escaped) = pattern.get(index + 1) else {
                    return Err(LuaPatternError::DanglingEscape);
                };
                match escaped {
                    'b' => return Err(LuaPatternError::BalancedMatch),
                    'f' => return Err(LuaPatternError::Frontier),
                    '0'..='9' => return Err(LuaPatternError::BackReference(escaped)),
                    _ => match lua_class_regex(escaped) {
                        Some(class) => regex.push_str(class),
                        // Lua compares an unknown `%X` against the literal `X`.
                        None => push_regex_literal(&mut regex, escaped),
                    },
                }
                index += 2;
                quantifiable = true;
            }
            '[' => {
                let (set, next_index) = convert_lua_set(&pattern, index)?;
                regex.push_str(&set);
                index = next_index;
                quantifiable = true;
            }
            _ => {
                push_regex_literal(&mut regex, character);
                index += 1;
                quantifiable = true;
            }
        }
    }

    Ok(regex)
}

fn lua_quantifier(character: char) -> Option<&'static str> {
    match character {
        '*' => Some("*"),
        '+' => Some("+"),
        '?' => Some("?"),
        // Lua's `-` is the lazy zero-or-more quantifier.
        '-' => Some("*?"),
        _ => None,
    }
}

/// Translate a Lua `[...]` set starting at `start`, returning the regex class and
/// the index just past the closing `]`.
fn convert_lua_set(pattern: &[char], start: usize) -> Result<(String, usize), LuaPatternError> {
    let mut index = start + 1;
    let mut body = String::new();
    let mut negated = false;

    // A `^` directly after `[` negates the set in Lua exactly as it does in regex.
    if pattern.get(index) == Some(&'^') {
        negated = true;
        index += 1;
    }

    // Lua's `classend` consumes the first member unconditionally, so a `]` in that
    // position is a literal member rather than the terminator.
    let mut members = 0usize;

    loop {
        let Some(&character) = pattern.get(index) else {
            return Err(LuaPatternError::UnterminatedSet);
        };

        if character == ']' && members > 0 {
            index += 1;
            break;
        }
        members += 1;

        if character == '%' {
            let Some(&escaped) = pattern.get(index + 1) else {
                return Err(LuaPatternError::DanglingEscape);
            };
            match lua_class_set_body(escaped) {
                Some(fragment) => body.push_str(fragment),
                None if is_lua_complement_class(escaped) => {
                    return Err(LuaPatternError::ComplementClassInSet(escaped));
                }
                None => push_set_literal(&mut body, escaped),
            }
            index += 2;
            continue;
        }

        // `-` separates a range in Lua sets just as it does in regex, and is a
        // literal member when it is first or last.
        if character == '-' {
            body.push('-');
            index += 1;
            continue;
        }

        push_set_literal(&mut body, character);
        index += 1;
    }

    let mut set = String::with_capacity(body.len() + 3);
    set.push('[');
    if negated {
        set.push('^');
    }
    set.push_str(&body);
    set.push(']');
    Ok((set, index))
}

/// A Lua character class as a standalone regex fragment.
///
/// Ranges are spelled out instead of using `\d`, `\w`, or `\s` because those are
/// Unicode-aware in the `regex` crate and ASCII-only in JavaScript. The explicit
/// forms match Lua's C-locale definitions in both engines.
fn lua_class_regex(class: char) -> Option<&'static str> {
    Some(match class {
        'a' => "[a-zA-Z]",
        'A' => "[^a-zA-Z]",
        'c' => "[\\x00-\\x1f\\x7f]",
        'C' => "[^\\x00-\\x1f\\x7f]",
        'd' => "[0-9]",
        'D' => "[^0-9]",
        'g' => "[\\x21-\\x7e]",
        'G' => "[^\\x21-\\x7e]",
        'l' => "[a-z]",
        'L' => "[^a-z]",
        'p' => "[!-/:-@\\[-`{-~]",
        'P' => "[^!-/:-@\\[-`{-~]",
        's' => "[\\t-\\r ]",
        'S' => "[^\\t-\\r ]",
        'u' => "[A-Z]",
        'U' => "[^A-Z]",
        'w' => "[0-9a-zA-Z]",
        'W' => "[^0-9a-zA-Z]",
        'x' => "[0-9a-fA-F]",
        'X' => "[^0-9a-fA-F]",
        _ => return None,
    })
}

/// A Lua character class as a fragment for the inside of a regex class.
///
/// Returning a bare fragment is what keeps `[%u]` from becoming the nested
/// `[[A-Z]]`, which the two engines read differently.
fn lua_class_set_body(class: char) -> Option<&'static str> {
    Some(match class {
        'a' => "a-zA-Z",
        'c' => "\\x00-\\x1f\\x7f",
        'd' => "0-9",
        'g' => "\\x21-\\x7e",
        'l' => "a-z",
        'p' => "!-/:-@\\[-`{-~",
        's' => "\\t-\\r ",
        'u' => "A-Z",
        'w' => "0-9a-zA-Z",
        'x' => "0-9a-fA-F",
        _ => return None,
    })
}

fn is_lua_complement_class(class: char) -> bool {
    matches!(
        class,
        'A' | 'C' | 'D' | 'G' | 'L' | 'P' | 'S' | 'U' | 'W' | 'X'
    )
}

/// Escape one character so both engines read it literally outside a class.
///
/// Only true metacharacters are escaped. JavaScript rejects arbitrary identity
/// escapes once the `u` flag is in play, so `\>` and `\-` are avoided even though
/// `web-tree-sitter` compiles without it today.
fn push_regex_literal(regex: &mut String, character: char) {
    if let Some(escape) = control_character_escape(character) {
        regex.push_str(escape);
        return;
    }
    if matches!(
        character,
        '\\' | '^' | '$' | '.' | '|' | '?' | '*' | '+' | '(' | ')' | '[' | ']' | '{' | '}'
    ) {
        regex.push('\\');
    }
    regex.push(character);
}

/// Spell a control character as a regex escape so the generated `.scm` stays
/// readable and never carries a raw tab or newline inside a string literal.
fn control_character_escape(character: char) -> Option<&'static str> {
    match character {
        '\t' => Some("\\t"),
        '\n' => Some("\\n"),
        '\r' => Some("\\r"),
        '\0' => Some("\\0"),
        _ => None,
    }
}

/// Escape one character so both engines read it literally inside a class.
///
/// `[` and `&` are escaped because the `regex` crate treats `[` as class nesting
/// and `&&` as set intersection, neither of which JavaScript supports.
fn push_set_literal(body: &mut String, character: char) {
    if let Some(escape) = control_character_escape(character) {
        body.push_str(escape);
        return;
    }
    if matches!(character, '\\' | ']' | '^' | '[' | '&' | '~') {
        body.push('\\');
    }
    body.push(character);
}

/// Resolve the escapes Tree-sitter applies when it parses a query string.
///
/// An unrecognised escape keeps its backslash, matching how a literal backslash
/// reaches a Lua pattern, where `\` has no special meaning.
fn unescape_query_string(raw: &str) -> String {
    let mut unescaped = String::with_capacity(raw.len());
    let mut characters = raw.chars();

    while let Some(character) = characters.next() {
        if character != '\\' {
            unescaped.push(character);
            continue;
        }
        match characters.next() {
            Some('\\') | None => unescaped.push('\\'),
            Some('"') => unescaped.push('"'),
            Some('n') => unescaped.push('\n'),
            Some('r') => unescaped.push('\r'),
            Some('t') => unescaped.push('\t'),
            Some('0') => unescaped.push('\0'),
            Some(other) => {
                unescaped.push('\\');
                unescaped.push(other);
            }
        }
    }

    unescaped
}

fn escape_regex_literal(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len() * 2);
    for character in text.chars() {
        push_regex_literal(&mut escaped, character);
    }
    escaped
}

/// Escape a regex so it survives being written inside a Tree-sitter query string.
fn escape_regex_for_query_string(regex: &str) -> String {
    regex.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Rewrite `"(?i)..."` predicate arguments into explicit character classes.
///
/// The `regex` crate supports the inline `(?i)` flag and JavaScript `RegExp` does
/// not, so the case folding is expanded at generation time instead.
fn expand_query_case_insensitive_regexes(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    let mut rest = line;

    while let Some(start) = rest.find("\"(?i)") {
        result.push_str(&rest[..=start]);
        rest = &rest[start + 5..];
        let Some(end) = rest.find('"') else {
            result.push_str("(?i)");
            result.push_str(rest);
            return result;
        };
        result.push_str(&expand_case_insensitive_ascii(&rest[..end]));
        result.push('"');
        rest = &rest[end + 1..];
    }

    result.push_str(rest);
    result
}

fn expand_case_insensitive_ascii(regex: &str) -> String {
    let mut result = String::with_capacity(regex.len());
    let mut characters = regex.chars();
    let mut in_character_class = false;

    while let Some(character) = characters.next() {
        match character {
            // Query strings arrive escaped, so a regex escape reads as `\\x`.
            // Consume all three characters to leave it untouched.
            '\\' => {
                result.push(character);
                if let Some(escaped) = characters.next() {
                    result.push(escaped);
                    if escaped == '\\' {
                        if let Some(regex_escape) = characters.next() {
                            result.push(regex_escape);
                        }
                    }
                }
            }
            '[' => {
                in_character_class = true;
                result.push(character);
            }
            ']' => {
                in_character_class = false;
                result.push(character);
            }
            character if !in_character_class && character.is_ascii_alphabetic() => {
                result.push('[');
                result.push(character.to_ascii_lowercase());
                result.push(character.to_ascii_uppercase());
                result.push(']');
            }
            _ => result.push(character),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert(lua_pattern: &str) -> String {
        convert_lua_pattern(lua_pattern).expect("pattern should be portable")
    }

    // -- Lua character classes --

    #[test]
    fn classes_expand_to_explicit_ascii_ranges() {
        // `\d`, `\w`, and `\s` are Unicode-aware in the `regex` crate and
        // ASCII-only in JavaScript, so neither runtime sees a shorthand.
        assert_eq!(convert("%d"), "[0-9]");
        assert_eq!(convert("%a"), "[a-zA-Z]");
        assert_eq!(convert("%l"), "[a-z]");
        assert_eq!(convert("%u"), "[A-Z]");
        assert_eq!(convert("%w"), "[0-9a-zA-Z]");
        assert_eq!(convert("%x"), "[0-9a-fA-F]");
        assert_eq!(convert("%s"), "[\\t-\\r ]");
    }

    #[test]
    fn complement_classes_negate() {
        assert_eq!(convert("%A"), "[^a-zA-Z]");
        assert_eq!(convert("%D"), "[^0-9]");
        assert_eq!(convert("%S"), "[^\\t-\\r ]");
        assert_eq!(convert("%U"), "[^A-Z]");
    }

    #[test]
    fn percent_escapes_non_alphanumeric_literals() {
        assert_eq!(convert("%."), "\\.");
        assert_eq!(convert("%%"), "%");
        assert_eq!(convert("%{%}"), "\\{\\}");
        assert_eq!(convert("%$"), "\\$");
        assert_eq!(convert("%^"), "\\^");
        assert_eq!(convert("%-"), "-");
    }

    #[test]
    fn unknown_percent_escape_matches_the_literal_character() {
        // Lua's `match_class` falls through to `cl == c` for unknown classes.
        assert_eq!(convert("%>"), ">");
        assert_eq!(convert("%Z"), "Z");
    }

    #[test]
    fn dangling_percent_is_rejected() {
        assert_eq!(
            convert_lua_pattern("%"),
            Err(LuaPatternError::DanglingEscape)
        );
    }

    // -- 1.1 positional quantifiers and anchors --

    #[test]
    fn dash_after_a_pattern_item_is_the_lazy_quantifier() {
        assert_eq!(convert("a-b"), "a*?b");
        assert_eq!(convert("[a-z]-x"), "[a-z]*?x");
    }

    #[test]
    fn leading_dash_is_a_literal_not_a_quantifier() {
        // Regression for the Clojure `->` predicate, which produced the
        // JavaScript-invalid `^*?\>[\^>].*`.
        assert_eq!(convert("^-%>[^>].*"), "^->[^>][\\s\\S]*");
        assert_eq!(convert("-foo"), "-foo");
    }

    #[test]
    fn quantifier_after_a_quantifier_is_a_literal() {
        // Lua starts a new pattern item after consuming a quantifier.
        assert_eq!(convert("a*-b"), "a*-b");
        assert_eq!(convert("a+*"), "a+\\*");
    }

    #[test]
    fn leading_quantifier_characters_are_literals() {
        assert_eq!(convert("*foo"), "\\*foo");
        assert_eq!(convert("+foo"), "\\+foo");
        assert_eq!(convert("?foo"), "\\?foo");
    }

    #[test]
    fn caret_anchors_only_at_the_start() {
        assert_eq!(convert("^foo"), "^foo");
        assert_eq!(convert("a^b"), "a\\^b");
    }

    #[test]
    fn dollar_anchors_only_at_the_end() {
        assert_eq!(convert("foo$"), "foo$");
        // Regression for the PowerShell `$env:` predicate, which produced the
        // never-matching `^$env:`.
        assert_eq!(convert("^$env:"), "^\\$env:");
    }

    // -- 1.2 negated character classes --

    #[test]
    fn caret_after_open_bracket_negates_the_set() {
        assert_eq!(convert("[^*]"), "[^*]");
        assert_eq!(convert("[^%s]"), "[^\\t-\\r ]");
    }

    #[test]
    fn documentation_comment_predicate_keeps_its_negation() {
        // Regression for `@comment.documentation` across 20 languages, where
        // `[^*]` had become `[\^*]` and inverted the predicate.
        assert_eq!(convert("^/[*][*][^*].*[*]/$"), "^/[*][*][^*][\\s\\S]*[*]/$");
    }

    #[test]
    fn caret_inside_a_set_but_not_first_is_a_member() {
        assert_eq!(convert("[a^]"), "[a\\^]");
    }

    #[test]
    fn closing_bracket_as_the_first_member_is_a_literal() {
        assert_eq!(convert("[]]"), "[\\]]");
        assert_eq!(convert("[^]]"), "[^\\]]");
    }

    #[test]
    fn unterminated_set_is_rejected() {
        assert_eq!(
            convert_lua_pattern("[abc"),
            Err(LuaPatternError::UnterminatedSet)
        );
    }

    // -- 1.3 no nested character classes --

    #[test]
    fn classes_inside_a_set_expand_to_bare_ranges() {
        // `[[A-Z]]` reads as `[A-Z]` in the `regex` crate but as `[` or `A-Z`
        // followed by a literal `]` in JavaScript.
        assert_eq!(convert("[%u]"), "[A-Z]");
        assert_eq!(convert("^[%u]"), "^[A-Z]");
        assert_eq!(convert("[%u][^/%s]*$"), "[A-Z][^/\\t-\\r ]*$");
        assert_eq!(convert("[%u%l]"), "[A-Za-z]");
        assert_eq!(convert("[%u@][%u%d_]+$"), "[A-Z@][A-Z0-9_]+$");
    }

    #[test]
    fn literal_bracket_inside_a_set_is_escaped() {
        // An unescaped `[` would start a nested class in the `regex` crate.
        assert_eq!(convert("[%[]"), "[\\[]");
    }

    #[test]
    fn complement_class_inside_a_set_is_rejected() {
        assert_eq!(
            convert_lua_pattern("[%D]"),
            Err(LuaPatternError::ComplementClassInSet('D'))
        );
    }

    // -- 1.4 `.` matches newlines in Lua --

    #[test]
    fn dot_matches_any_character_including_newlines() {
        assert_eq!(convert("."), "[\\s\\S]");
        assert_eq!(convert("^m_.*$"), "^m_[\\s\\S]*$");
    }

    #[test]
    fn dot_inside_a_set_stays_literal() {
        assert_eq!(convert("[.]"), "[.]");
    }

    // -- unsupported constructs --

    #[test]
    fn balanced_and_frontier_and_backreferences_are_rejected() {
        assert_eq!(
            convert_lua_pattern("%b()"),
            Err(LuaPatternError::BalancedMatch)
        );
        assert_eq!(
            convert_lua_pattern("%f[%a]"),
            Err(LuaPatternError::Frontier)
        );
        assert_eq!(
            convert_lua_pattern("(a)%1"),
            Err(LuaPatternError::BackReference('1'))
        );
    }

    // -- convert_lua_matches (full line processing) --

    #[test]
    fn lua_match_predicate_converted() {
        let input = r#"((identifier) @var (#lua-match? @var "^%u"))"#;
        let expected = r#"((identifier) @var (#match? @var "^[A-Z]"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn not_lua_match_predicate_converted() {
        let input = r#"((identifier) @var (#not-lua-match? @var "^%d"))"#;
        let expected = r#"((identifier) @var (#not-match? @var "^[0-9]"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn query_string_preserves_literal_interpolation_marker() {
        let input = r#"((text) @injection.content (#lua-match? @injection.content "%${"))"#;
        let expected = r#"((text) @injection.content (#match? @injection.content "\\$\\{"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn query_string_escapes_are_resolved_before_translation() {
        // Tree-sitter turns `\t` into a tab before the predicate runs, so the Lua
        // pattern contains a tab and the regex must say `\t`, not a backslash.
        let input = r#"((comment) @d (#lua-match? @d "^#![ \t]*/"))"#;
        let expected = r#"((comment) @d (#match? @d "^#![ \\t]*/"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn a_literal_backslash_in_a_lua_pattern_stays_literal() {
        // `\` has no meaning in a Lua pattern, so an escaped backslash in the query
        // string is matched literally.
        let input = r#"((s) @x (#lua-match? @x "a\\b"))"#;
        let expected = r#"((s) @x (#match? @x "a\\\\b"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn only_the_pattern_argument_is_rewritten() {
        let input = r#"((a) @x (#lua-match? @x "%d") (#eq? @y "a-b"))"#;
        let expected = r#"((a) @x (#match? @x "[0-9]") (#eq? @y "a-b"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn multiple_lua_predicates_on_one_line_are_all_converted() {
        let input = r#"(#lua-match? @a "%u") (#not-lua-match? @b "%d")"#;
        let expected = r#"(#match? @a "[A-Z]") (#not-match? @b "[0-9]")"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn non_matching_line_unchanged() {
        let input = "(identifier) @variable";
        assert_eq!(convert_lua_matches(input), input);
    }

    #[test]
    fn existing_regex_predicate_unchanged() {
        let input =
            r#"((inline) @injection.content (#match? @injection.content "^(import|export)\\s"))"#;
        assert_eq!(convert_lua_matches(input), input);
    }

    #[test]
    fn case_insensitive_regex_is_portable() {
        let input = r#"((identifier) @keyword (#match? @keyword "(?i)^(continue|break)$"))"#;
        let expected = r#"((identifier) @keyword (#match? @keyword "^([cC][oO][nN][tT][iI][nN][uU][eE]|[bB][rR][eE][aA][kK])$"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn case_insensitive_expansion_preserves_character_classes_and_escapes() {
        let input = r#"((identifier) @x (#match? @x "(?i)^[a-z]\\sFoo$"))"#;
        let expected = r#"((identifier) @x (#match? @x "^[a-z]\\s[fF][oO][oO]$"))"#;
        assert_eq!(convert_lua_matches(input), expected);
    }

    #[test]
    fn multiline_input() {
        let input = "(identifier) @var\n(#lua-match? @var \"^%u\")\n(string) @str";
        let output = convert_lua_matches(input);
        assert!(output.contains("(#match? @var \"^[A-Z]\")"));
        assert!(output.contains("(identifier) @var"));
        assert!(output.contains("(string) @str"));
    }

    #[test]
    fn preserves_no_trailing_newline() {
        let input = "no newline at end";
        assert_eq!(convert_lua_matches(input), input);
    }

    #[test]
    fn preserves_trailing_newline() {
        let input = "with newline at end\n";
        assert_eq!(convert_lua_matches(input), input);
    }
}
