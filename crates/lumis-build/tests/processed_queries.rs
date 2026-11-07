//! Corpus checks over every generated query in `queries/processed`.
//!
//! These tests deliberately fail instead of skipping. The regressions in
//! `REVIEW.md` §1 reached the branch because the only per-language query
//! check silently skipped two thirds of the catalog, so nothing here is allowed to
//! opt out.
//!
//! The JavaScript half of the same guarantee lives in
//! `packages/javascript/lumis/test/query-patterns.test.ts`, which compiles the same
//! patterns with `RegExp`.

// A table-driven test's branches are its coverage; splitting one to satisfy
// the cognitive complexity ceiling would hide what it asserts.
#![allow(clippy::cognitive_complexity)]

use lumis_build::{convert_lua_pattern, LuaPatternError};
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate lives two directories below the repository root")
        .to_path_buf()
}

fn scm_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()));
        for entry in entries {
            let path = entry.expect("failed to read directory entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "scm") {
                files.push(path);
            }
        }
    }

    files.sort();
    files
}

/// One `#match?` style predicate argument, with query-string escapes resolved.
struct Pattern {
    file: PathBuf,
    line: usize,
    operator: String,
    regex: String,
}

/// Collect the regex argument of every `#match?` style predicate.
///
/// Tree-sitter resolves `\\` to `\` and `\"` to `"` when it parses a query string,
/// so the stored text is unescaped the same way before it is compiled.
fn regex_predicates(files: &[PathBuf]) -> Vec<Pattern> {
    const OPERATORS: [&str; 4] = ["#match?", "#not-match?", "#any-match?", "#any-not-match?"];
    let mut patterns = Vec::new();

    for file in files {
        let content = fs::read_to_string(file)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", file.display()));

        for (index, line) in content.lines().enumerate() {
            if line.trim_start().starts_with(';') {
                continue;
            }

            let mut rest = line;
            while let Some((offset, operator)) = OPERATORS
                .iter()
                .filter_map(|operator| rest.find(operator).map(|offset| (offset, *operator)))
                .min_by_key(|(offset, operator)| (*offset, std::cmp::Reverse(operator.len())))
            {
                rest = &rest[offset + operator.len()..];
                let Some((raw, remainder)) = next_query_string(rest) else {
                    break;
                };
                rest = remainder;
                patterns.push(Pattern {
                    file: file.clone(),
                    line: index + 1,
                    operator: operator.to_string(),
                    regex: unescape_query_string(raw),
                });
            }
        }
    }

    patterns
}

fn next_query_string(text: &str) -> Option<(&str, &str)> {
    let start = text.find('"')?;
    let bytes = text.as_bytes();
    let mut index = start + 1;

    while index < bytes.len() {
        match bytes[index] {
            b'\\' => index += 2,
            b'"' => return Some((&text[start + 1..index], &text[index + 1..])),
            _ => index += 1,
        }
    }

    None
}

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
            Some(other) => {
                unescaped.push('\\');
                unescaped.push(other);
            }
        }
    }

    unescaped
}

fn processed_queries() -> Vec<PathBuf> {
    let directory = repo_root().join("queries/processed");
    assert!(
        directory.is_dir(),
        "missing {}; run `mise run langs-preprocess-queries`",
        directory.display()
    );
    let files = scm_files(&directory);
    assert!(
        files.len() > 100,
        "expected the full processed query corpus, found {} files",
        files.len()
    );
    files
}

#[test]
fn every_predicate_regex_compiles_in_rust() {
    let patterns = regex_predicates(&processed_queries());
    assert!(
        patterns.len() > 200,
        "expected the full predicate corpus, found {}",
        patterns.len()
    );

    let failures: Vec<String> = patterns
        .iter()
        .filter_map(|pattern| {
            regex::bytes::Regex::new(&pattern.regex).err().map(|error| {
                format!(
                    "{}:{} ({} {:?}): {}",
                    pattern.file.display(),
                    pattern.line,
                    pattern.operator,
                    pattern.regex,
                    error.to_string().lines().next().unwrap_or_default()
                )
            })
        })
        .collect();

    assert!(
        failures.is_empty(),
        "predicate regexes rejected by the regex crate:\n{}",
        failures.join("\n")
    );
}

#[test]
fn no_predicate_regex_nests_a_character_class() {
    // `[[A-Z]]` is `[A-Z]` in the regex crate and `[` or `A-Z` followed by a
    // literal `]` in JavaScript. A generated query must never depend on that.
    let failures: Vec<String> = regex_predicates(&processed_queries())
        .iter()
        .filter(|pattern| has_nested_character_class(&pattern.regex))
        .map(|pattern| {
            format!(
                "{}:{} {:?}",
                pattern.file.display(),
                pattern.line,
                pattern.regex
            )
        })
        .collect();

    assert!(
        failures.is_empty(),
        "predicate regexes with a nested character class:\n{}",
        failures.join("\n")
    );
}

#[test]
fn no_predicate_regex_uses_rust_only_inline_flags() {
    // The regex crate supports `(?i)`; JavaScript `RegExp` does not. Case folding
    // is expanded at generation time instead.
    let failures: Vec<String> = regex_predicates(&processed_queries())
        .iter()
        .filter(|pattern| pattern.regex.contains("(?"))
        .map(|pattern| {
            format!(
                "{}:{} {:?}",
                pattern.file.display(),
                pattern.line,
                pattern.regex
            )
        })
        .collect();

    assert!(
        failures.is_empty(),
        "predicate regexes with a non-portable inline group:\n{}",
        failures.join("\n")
    );
}

#[test]
fn no_processed_query_still_uses_a_lua_predicate() {
    let failures: Vec<String> = processed_queries()
        .iter()
        .filter_map(|file| {
            let content = fs::read_to_string(file).expect("failed to read query");
            content.contains("lua-match?").then(|| {
                file.strip_prefix(repo_root())
                    .unwrap_or(file)
                    .display()
                    .to_string()
            })
        })
        .collect();

    assert!(
        failures.is_empty(),
        "processed queries must not contain `#lua-match?`:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_upstream_lua_pattern_is_translatable() {
    // Upstream is the input side of the conversion. If nvim-treesitter introduces
    // `%b`, `%f`, or a back reference, generation must fail here rather than emit a
    // silently wrong regex.
    let directory = repo_root().join("queries/upstream");
    assert!(directory.is_dir(), "missing {}", directory.display());

    let mut failures = Vec::new();
    for file in scm_files(&directory) {
        let content = fs::read_to_string(&file).expect("failed to read query");
        for (index, line) in content.lines().enumerate() {
            if line.trim_start().starts_with(';') || !line.contains("lua-match?") {
                continue;
            }
            let mut rest = line;
            while let Some(offset) = rest.find("lua-match?") {
                rest = &rest[offset + "lua-match?".len()..];
                let Some((raw, remainder)) = next_query_string(rest) else {
                    break;
                };
                rest = remainder;
                let lua_pattern = unescape_query_string(raw);
                if let Err(error) = convert_lua_pattern(&lua_pattern) {
                    failures.push(format!(
                        "{}:{} {:?}: {error}",
                        file.display(),
                        index + 1,
                        lua_pattern
                    ));
                }
            }
        }
    }

    assert!(
        failures.is_empty(),
        "untranslatable Lua patterns:\n{}",
        failures.join("\n")
    );
}

#[test]
fn documented_defects_stay_fixed() {
    // REVIEW.md §1.1: a leading `-` is a literal dash, not Lua's lazy `*`.
    let clojure = convert_lua_pattern("^-%>[^>].*").expect("clojure constructor pattern");
    assert_eq!(clojure, "^->[^>][\\s\\S]*");
    regex::bytes::Regex::new(&clojure).expect("must compile");
    assert!(regex::bytes::Regex::new(&clojure)
        .unwrap()
        .is_match(b"->foo"));

    // REVIEW.md §1.2: `[^*]` must stay a negated class.
    let documentation =
        convert_lua_pattern("^/[*][*][^*].*[*]/$").expect("documentation comment pattern");
    let documentation = regex::bytes::Regex::new(&documentation).expect("must compile");
    // The broken `[\^*]` matched only a caret or a star here, so a real doc comment
    // was rejected and a `/***` block was accepted. Both directions are pinned.
    assert!(documentation.is_match(b"/** hi */"));
    assert!(documentation.is_match(b"/**x hi */"));
    assert!(!documentation.is_match(b"/*** hi */"));

    // REVIEW.md §1.3: `[%u]` must not become the nested `[[A-Z]]`.
    let upper = convert_lua_pattern("^[%u]").expect("uppercase pattern");
    assert_eq!(upper, "^[A-Z]");
    assert!(!has_nested_character_class(&upper));

    // REVIEW.md §1.4: Lua's `.` crosses newlines and `$` only anchors last.
    let multiline = regex::bytes::Regex::new(
        &convert_lua_pattern("^/[*][*][^*].*[*]/$").expect("documentation comment pattern"),
    )
    .expect("must compile");
    assert!(multiline.is_match(b"/**\n * hi\n */"));
    let powershell = convert_lua_pattern("^$env:").expect("powershell pattern");
    assert_eq!(powershell, "^\\$env:");
    assert!(regex::bytes::Regex::new(&powershell)
        .unwrap()
        .is_match(b"$env:PATH"));
}

#[test]
fn unsupported_lua_constructs_are_reported_not_guessed() {
    assert_eq!(
        convert_lua_pattern("%b()"),
        Err(LuaPatternError::BalancedMatch)
    );
    assert_eq!(
        convert_lua_pattern("%f[%a]"),
        Err(LuaPatternError::Frontier)
    );
    assert_eq!(
        convert_lua_pattern("[%D]"),
        Err(LuaPatternError::ComplementClassInSet('D'))
    );
}

/// Detect an unescaped `[` inside a character class.
fn has_nested_character_class(regex: &str) -> bool {
    let characters: Vec<char> = regex.chars().collect();
    let mut index = 0;
    let mut depth = 0usize;

    while index < characters.len() {
        match characters[index] {
            '\\' => index += 1,
            '[' if depth == 0 => {
                depth = 1;
                // A `]` in the first member position is a literal.
                if characters.get(index + 1) == Some(&'^') {
                    index += 1;
                }
                if characters.get(index + 1) == Some(&']') {
                    index += 1;
                }
            }
            '[' => return true,
            ']' if depth == 1 => depth = 0,
            _ => {}
        }
        index += 1;
    }

    false
}

fn comment_highlights() -> String {
    fs::read_to_string(repo_root().join("queries/processed/comment/highlights.scm"))
        .expect("failed to read the comment highlights query")
}

/// The filter every `comment` injection in the corpus carries, which must be
/// the one the comment highlights query derives.
#[test]
fn every_comment_injection_carries_the_derived_filter() {
    let filter = lumis_build::comment_injection_filter(&comment_highlights())
        .expect("comment highlights derive a filter");
    let expected = format!(
        "(#match? @injection.content \"{}\")",
        filter.replace('\\', "\\\\")
    );

    let mut injections = 0;
    let mut failures = Vec::new();
    for file in processed_queries() {
        if file.file_name().is_none_or(|name| name != "injections.scm") {
            continue;
        }
        let content = fs::read_to_string(&file).expect("failed to read query");
        let directives = content
            .matches("(#set! injection.language \"comment\")")
            .count();
        let filters = content.matches(&expected).count();
        injections += directives;
        if directives != filters {
            failures.push(format!(
                "{}: {directives} comment injections, {filters} filtered",
                file.display()
            ));
        }
    }

    assert!(
        injections > 100,
        "expected the full corpus, found {injections}"
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// A deterministic stream of comment-like texts, built from the pieces the
/// comment grammar and its query care about, so tags, users, issue numbers
/// and URIs appear in every position and spelling.
fn generated_comments() -> Vec<String> {
    const PIECES: &[&str] = &[
        "TODO", "WIP", "NOTE", "XXX", "INFO", "DOCS", "PERF", "TEST", "HACK", "WARNING", "WARN",
        "FIX", "FIXME", "BUG", "ERROR", "todo", "Note", "TOD", "FIXM", "ERR", "XX", "TODOS", "A_B",
        "X1", ":", ": ", "(", ")", "(me)", " (me)", "()", " ", "\n", "\t", "#", "#1", "#42", "1",
        "x", "http", "https", "://", "http://", "https://", "a.b", ".", ",", "/", "//", "/*", "*/",
        "--", "<!--", "-->", ";", "?", "!", "'", "\"", "`", "]", "}", ">", "-", "_", "é", "日本",
    ];
    let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
    let mut next = move |bound: usize| {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        usize::try_from(state % bound as u64).expect("fits")
    };

    let mut comments: Vec<String> = PIECES.iter().map(|piece| (*piece).to_owned()).collect();
    for first in PIECES {
        for second in PIECES {
            comments.push(format!("{first}{second}"));
            comments.push(format!("// {first}{second} x"));
        }
    }
    for _ in 0..60_000 {
        let length = 1 + next(8);
        comments.push((0..length).map(|_| PIECES[next(PIECES.len())]).collect());
    }
    comments
}

/// The filter is a superset of the comment query: any comment the query
/// would put a capture on must pass it, or that capture is lost.
#[test]
fn comment_injection_filter_admits_every_comment_the_query_colors() {
    use tree_sitter::StreamingIterator;

    let highlights = comment_highlights();
    let filter = lumis_build::comment_injection_filter(&highlights)
        .expect("comment highlights derive a filter");
    let filter = regex::Regex::new(&filter).expect("filter compiles");

    let language: tree_sitter::Language = tree_sitter_comment::LANGUAGE.into();
    let query = tree_sitter::Query::new(&language, &highlights).expect("comment query compiles");
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .expect("comment grammar loads");
    let mut cursor = tree_sitter::QueryCursor::new();

    let mut colored = 0;
    let mut skipped = 0;
    let mut captured_names = std::collections::BTreeSet::new();
    let mut failures = Vec::new();
    for comment in generated_comments() {
        let tree = parser.parse(&comment, None).expect("comment parses");
        let mut captures = cursor.captures(&query, tree.root_node(), comment.as_bytes());
        let mut names = Vec::new();
        while let Some((found, index)) = captures.next() {
            names.push(query.capture_names()[found.captures()[*index].index as usize]);
        }
        if names.is_empty() {
            if !filter.is_match(&comment) {
                skipped += 1;
            }
            continue;
        }
        colored += 1;
        captured_names.extend(names.iter().copied());
        if !filter.is_match(&comment) && failures.len() < 20 {
            failures.push(format!("{comment:?} colors {names:?}"));
        }
    }

    assert!(
        failures.is_empty(),
        "filter rejects colored comments:\n{}",
        failures.join("\n")
    );
    // A generator that never reaches a pattern proves nothing about it.
    for name in [
        "comment.todo",
        "comment.note",
        "comment.warning",
        "comment.error",
        "constant",
        "punctuation.bracket",
        "punctuation.delimiter",
        "number",
        "string.special.url",
    ] {
        assert!(
            captured_names.contains(name),
            "no generated comment captured @{name}"
        );
    }
    assert!(
        colored > 10_000,
        "only {colored} generated comments were colored"
    );
    assert!(
        skipped > 1_000,
        "only {skipped} generated comments were skipped"
    );
}
