//! A `#match?` filter on every `comment` injection, derived from the comment
//! language's own highlights query.
//!
//! Most comments contain nothing the comment query colors, yet each one costs a
//! parsed injection layer. The filter rejects a comment before that layer exists
//! unless its text contains something a highlights pattern requires: an
//! `#any-of?` or `#eq?` literal, a `#match?` regex, or a `uri` token. A pattern
//! that requires none of those could color any comment, so it fails derivation
//! instead of being guessed at.

use std::fmt;

use regex_syntax::ast::{self, Ast};
use tree_sitter::{Node, Parser, Tree};

use super::{escape_regex_for_query_string, escape_regex_literal, unescape_query_string};

/// What the comment grammar lexes as a `uri`: `https?://` followed by at least
/// one more character. Requiring only the scheme keeps the filter a superset.
const URI_REGEX: &str = "https?://";

/// A query the comment injection filter cannot be derived from or attached to.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CommentFilterError {
    /// The query text does not parse as a Tree-sitter query.
    InvalidQuery,
    /// A highlights pattern that can match without requiring any literal,
    /// regex, or `uri` token, so no filter can bound what it colors.
    UnboundedPattern(String),
    /// A `#match?` regex that cannot be searched for inside a longer text.
    UnsupportedRegex(String),
    /// A `comment` injection whose content capture the filter cannot test
    /// faithfully, such as a quantified or combined one.
    UnsupportedInjection(String),
}

impl fmt::Display for CommentFilterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuery => write!(formatter, "query does not parse"),
            Self::UnboundedPattern(pattern) => write!(
                formatter,
                "comment highlights pattern requires no literal, regex or uri: {pattern}"
            ),
            Self::UnsupportedRegex(regex) => {
                write!(
                    formatter,
                    "cannot search for #match? regex {regex:?} unanchored"
                )
            }
            Self::UnsupportedInjection(pattern) => {
                write!(formatter, "cannot filter comment injection: {pattern}")
            }
        }
    }
}

impl std::error::Error for CommentFilterError {}

/// Derive a regex that matches every comment `comment_highlights` can color.
///
/// The result is unanchored and portable between the `regex` crate and
/// JavaScript `RegExp`. It is a superset: a comment it rejects produces no
/// capture from any pattern in the query.
///
/// # Errors
///
/// Fails when a pattern can match without requiring a literal, a regex, or a
/// `uri` token, or when one of its `#match?` regexes cannot be unanchored.
///
/// # Examples
///
/// ```
/// use lumis_build::comment_injection_filter;
///
/// let query = r#"("text" @comment.todo (#any-of? @comment.todo "TODO" "WIP"))
/// ("text" @number (#match? @number "^#[0-9]+$"))
/// (uri) @string.special.url"#;
/// assert_eq!(
///     comment_injection_filter(query).unwrap(),
///     "TODO|WIP|#[0-9]+|https?://",
/// );
/// ```
pub fn comment_injection_filter(comment_highlights: &str) -> Result<String, CommentFilterError> {
    let tree = parse_query(comment_highlights)?;
    let mut literals: Vec<String> = Vec::new();
    let mut regexes: Vec<String> = Vec::new();

    for pattern in top_level_patterns(&tree) {
        let witnesses = pattern_witnesses(pattern, comment_highlights)?;
        if witnesses.is_empty() {
            return Err(CommentFilterError::UnboundedPattern(
                text(pattern, comment_highlights).to_owned(),
            ));
        }
        for witness in witnesses {
            match witness {
                Witness::Literal(literal) => literals.push(literal),
                Witness::Regex(regex) => regexes.push(regex),
            }
        }
    }

    // A literal containing another adds nothing: `WARNING` contains `WARN`.
    let mut kept: Vec<&String> = Vec::new();
    for literal in &literals {
        let redundant = literals
            .iter()
            .any(|other| other != literal && literal.contains(other.as_str()));
        if !redundant && !kept.contains(&literal) {
            kept.push(literal);
        }
    }

    let mut alternatives: Vec<String> = kept.iter().map(|l| escape_regex_literal(l)).collect();
    for regex in regexes {
        if !alternatives.contains(&regex) {
            alternatives.push(regex);
        }
    }
    Ok(alternatives.join("|"))
}

/// Add `(#match? @injection.content "<filter>")` to every pattern in
/// `injections` that sets `injection.language` to `"comment"`.
///
/// # Errors
///
/// Fails when such a pattern combines its injections, or captures its content
/// with a quantifier or inside an alternative, where a per-node `#match?` would
/// not see what the comment parser sees.
///
/// # Examples
///
/// ```
/// use lumis_build::filter_comment_injections;
///
/// let query = "((comment) @injection.content\n  (#set! injection.language \"comment\"))";
/// assert_eq!(
///     filter_comment_injections(query, "TODO|https?://").unwrap(),
///     "((comment) @injection.content\n  (#set! injection.language \"comment\")\n  (#match? @injection.content \"TODO|https?://\"))",
/// );
/// ```
pub fn filter_comment_injections(
    injections: &str,
    filter: &str,
) -> Result<String, CommentFilterError> {
    let tree = parse_query(injections)?;
    let predicate = format!(
        "(#match? @injection.content \"{}\")",
        escape_regex_for_query_string(filter)
    );
    let mut insertions = Vec::new();

    for pattern in top_level_patterns(&tree) {
        let Some(language) = descendants(pattern)
            .into_iter()
            .find(|node| is_comment_language_directive(*node, injections))
        else {
            continue;
        };
        check_filterable(pattern, injections)?;
        let indent = line_indent(injections, language.start_byte());
        insertions.push((language.end_byte(), format!("\n{indent}{predicate}")));
    }

    let mut result = injections.to_owned();
    for (offset, insertion) in insertions.into_iter().rev() {
        result.insert_str(offset, &insertion);
    }
    Ok(result)
}

enum Witness {
    Literal(String),
    Regex(String),
}

fn parse_query(source: &str) -> Result<Tree, CommentFilterError> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_tsquery::LANGUAGE.into())
        .map_err(|_| CommentFilterError::InvalidQuery)?;
    let tree = parser
        .parse(source, None)
        .ok_or(CommentFilterError::InvalidQuery)?;
    if tree.root_node().has_error() {
        return Err(CommentFilterError::InvalidQuery);
    }
    Ok(tree)
}

fn top_level_patterns(tree: &Tree) -> Vec<Node<'_>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    root.named_children(&mut cursor)
        .filter(|node| node.kind() != "comment")
        .collect()
}

fn descendants(node: Node<'_>) -> Vec<Node<'_>> {
    let mut found = Vec::new();
    let mut pending = vec![node];
    while let Some(current) = pending.pop() {
        found.push(current);
        let mut cursor = current.walk();
        pending.extend(current.named_children(&mut cursor));
    }
    found
}

fn text<'a>(node: Node<'_>, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

/// The named node in `field`. A predicate's `name` field also holds its `#`
/// token, so the first child of the field is not necessarily the name.
fn identifier<'a>(node: Node<'_>, field: &str, source: &'a str) -> Option<&'a str> {
    let mut cursor = node.walk();
    let found = node
        .children_by_field_name(field, &mut cursor)
        .find(Node::is_named)
        .map(|child| text(child, source));
    found
}

/// The value of a query string node, with Tree-sitter's escapes resolved.
fn string_value(node: Node<'_>, source: &str) -> Option<String> {
    (node.kind() == "string").then(|| {
        let content = node
            .named_child(0)
            .map_or("", |content| text(content, source));
        unescape_query_string(content)
    })
}

/// A predicate's operator, such as `any-of?` or `set!`, and its parameters.
fn predicate_parts<'a>(node: Node<'a>, source: &str) -> Option<(String, Vec<Node<'a>>)> {
    if node.kind() != "predicate" {
        return None;
    }
    let name = identifier(node, "name", source)?;
    let kind = identifier(node, "type", source)?;
    let parameters = node.child_by_field_name("parameters")?;
    let mut cursor = parameters.walk();
    let parameters = parameters.named_children(&mut cursor).collect();
    Some((format!("{name}{kind}"), parameters))
}

/// The name a capture node binds, or a predicate parameter refers to.
fn capture_name<'a>(node: Node<'_>, source: &'a str) -> Option<&'a str> {
    (node.kind() == "capture")
        .then(|| identifier(node, "name", source))
        .flatten()
}

/// A capture written on a node, as opposed to one named in a predicate.
fn is_binding(capture: Node<'_>) -> bool {
    capture
        .parent()
        .is_some_and(|parent| parent.kind() != "parameters")
}

/// Whether the node a capture or a node pattern stands for is present in every
/// match of its top-level pattern.
///
/// A `?` or `*` quantifier, or membership in a `[...]` alternation, lets the
/// pattern match without it. A predicate on an absent capture passes, so it
/// requires nothing.
fn is_mandatory(node: Node<'_>) -> bool {
    let mut current = node;
    while let Some(parent) = current.parent() {
        if parent.kind() == "program" {
            return true;
        }
        let quantifier = current.child_by_field_name("quantifier");
        let optional = quantifier.is_some_and(|q| q.kind() == "quantifier" && is_optional(q));
        if optional || parent.kind() == "list" {
            return false;
        }
        current = parent;
    }
    true
}

fn is_optional(quantifier: Node<'_>) -> bool {
    let mut cursor = quantifier.walk();
    let optional = quantifier
        .children(&mut cursor)
        .any(|token| matches!(token.kind(), "?" | "*"));
    optional
}

/// The node a binding capture is written on.
fn captured_node(capture: Node<'_>) -> Option<Node<'_>> {
    capture.parent()
}

/// Whether `name` is bound to a node every match of `pattern` contains.
fn binds_mandatory_node(pattern: Node<'_>, name: &str, source: &str) -> bool {
    descendants(pattern).into_iter().any(|node| {
        capture_name(node, source) == Some(name)
            && is_binding(node)
            && captured_node(node).is_some_and(is_mandatory)
    })
}

fn pattern_witnesses(pattern: Node<'_>, source: &str) -> Result<Vec<Witness>, CommentFilterError> {
    let mut witnesses = Vec::new();
    for node in descendants(pattern) {
        if node.kind() == "named_node"
            && identifier(node, "name", source) == Some("uri")
            && is_mandatory(node)
        {
            witnesses.push(Witness::Regex(URI_REGEX.to_owned()));
        }
        if let Some((operator, parameters)) = predicate_parts(node, source) {
            witnesses.extend(predicate_witnesses(
                pattern,
                &operator,
                &parameters,
                source,
            )?);
        }
    }
    Ok(witnesses)
}

/// What a text predicate requires of the comment, when its capture is always
/// present: `#eq?` and `#any-of?` a literal, `#match?` its regex.
fn predicate_witnesses(
    pattern: Node<'_>,
    operator: &str,
    parameters: &[Node<'_>],
    source: &str,
) -> Result<Vec<Witness>, CommentFilterError> {
    let Some((first, rest)) = parameters.split_first() else {
        return Ok(Vec::new());
    };
    let Some(name) = capture_name(*first, source) else {
        return Ok(Vec::new());
    };
    if !binds_mandatory_node(pattern, name, source) {
        return Ok(Vec::new());
    }
    let values: Vec<String> = rest
        .iter()
        .map_while(|node| string_value(*node, source))
        .collect();
    if values.len() != rest.len() || values.is_empty() {
        return Ok(Vec::new());
    }

    match operator {
        "eq?" | "any-of?" => Ok(values.into_iter().map(Witness::Literal).collect()),
        "match?" | "vim-match?" => Ok(vec![Witness::Regex(unanchored(&values[0])?)]),
        _ => Ok(Vec::new()),
    }
}

/// `regex` with its outer `^` and `$` removed, so it finds the captured node's
/// text inside the whole comment.
///
/// A node matching `^#[0-9]+$` sits somewhere in the comment, so the comment
/// contains a match of `#[0-9]+`. Any other assertion, such as `\b` or an
/// anchor inside a group, can hold at the node's edges and fail in the comment,
/// so it is rejected rather than dropped.
fn unanchored(regex: &str) -> Result<String, CommentFilterError> {
    let unsupported = || CommentFilterError::UnsupportedRegex(regex.to_owned());
    let parsed = ast::parse::Parser::new()
        .parse(regex)
        .map_err(|_| unsupported())?;

    let (start, end) = match &parsed {
        Ast::Concat(concat) => {
            let items = &concat.asts;
            let skip_start = items.first().is_some_and(|item| is_anchor(item, true));
            let skip_end =
                items.len() > 1 && items.last().is_some_and(|item| is_anchor(item, false));
            let first = usize::from(skip_start);
            let last = items.len() - usize::from(skip_end);
            if first >= last {
                return Err(unsupported());
            }
            (
                items[first].span().start.offset,
                items[last - 1].span().end.offset,
            )
        }
        other => (other.span().start.offset, other.span().end.offset),
    };

    let inner = &regex[start..end];
    let inner_ast = ast::parse::Parser::new()
        .parse(inner)
        .map_err(|_| unsupported())?;
    if has_assertion_or_flags(&inner_ast) {
        return Err(unsupported());
    }
    Ok(if matches!(inner_ast, Ast::Alternation(_)) {
        format!("(?:{inner})")
    } else {
        inner.to_owned()
    })
}

fn is_anchor(node: &Ast, start: bool) -> bool {
    use ast::AssertionKind::{EndLine, EndText, StartLine, StartText};
    let Ast::Assertion(assertion) = node else {
        return false;
    };
    if start {
        matches!(assertion.kind, StartLine | StartText)
    } else {
        matches!(assertion.kind, EndLine | EndText)
    }
}

fn has_assertion_or_flags(node: &Ast) -> bool {
    match node {
        Ast::Assertion(_) | Ast::Flags(_) => true,
        Ast::Repetition(repetition) => has_assertion_or_flags(&repetition.ast),
        Ast::Group(group) => group.flags().is_some() || has_assertion_or_flags(&group.ast),
        Ast::Alternation(alternation) => alternation.asts.iter().any(has_assertion_or_flags),
        Ast::Concat(concat) => concat.asts.iter().any(has_assertion_or_flags),
        Ast::Empty(_)
        | Ast::Literal(_)
        | Ast::Dot(_)
        | Ast::ClassUnicode(_)
        | Ast::ClassPerl(_)
        | Ast::ClassBracketed(_) => false,
    }
}

/// `(#set! injection.language "comment")`.
fn is_comment_language_directive(node: Node<'_>, source: &str) -> bool {
    let Some((operator, parameters)) = predicate_parts(node, source) else {
        return false;
    };
    operator == "set!"
        && parameters.len() == 2
        && parameters[0].kind() == "identifier"
        && text(parameters[0], source) == "injection.language"
        && string_value(parameters[1], source).as_deref() == Some("comment")
}

/// The filter tests one node's text. A combined injection parses many nodes as
/// one document, and a quantified or optional capture can hand the parser
/// several nodes or none, so neither is filtered.
fn check_filterable(pattern: Node<'_>, source: &str) -> Result<(), CommentFilterError> {
    let unsupported = || CommentFilterError::UnsupportedInjection(text(pattern, source).to_owned());
    let nodes = descendants(pattern);

    let combined = nodes.iter().any(|node| {
        predicate_parts(*node, source).is_some_and(|(operator, parameters)| {
            operator == "set!"
                && parameters
                    .first()
                    .is_some_and(|key| text(*key, source) == "injection.combined")
        })
    });
    let contents: Vec<Node<'_>> = nodes
        .into_iter()
        .filter(|node| {
            capture_name(*node, source) == Some("injection.content") && is_binding(*node)
        })
        .filter_map(captured_node)
        .collect();
    let [content] = contents.as_slice() else {
        return Err(unsupported());
    };
    if combined || !is_mandatory(*content) || content.child_by_field_name("quantifier").is_some() {
        return Err(unsupported());
    }
    Ok(())
}

fn line_indent(source: &str, offset: usize) -> &str {
    let line_start = source[..offset].rfind('\n').map_or(0, |index| index + 1);
    let prefix = &source[line_start..offset];
    if prefix.chars().all(char::is_whitespace) {
        prefix
    } else {
        "  "
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals_containing_another_literal_are_dropped() {
        let query = r#"("text" @x (#any-of? @x "FIXME" "FIX" "WARNING" "WARN" "FIX"))"#;
        assert_eq!(comment_injection_filter(query).unwrap(), "FIX|WARN");
    }

    #[test]
    fn a_predicate_on_an_optional_capture_requires_nothing() {
        // `(user)?` can be absent, and an absent capture passes `#eq?`.
        let optional = r#"((tag (name) (user)? @u) (#eq? @u "me"))"#;
        assert!(matches!(
            comment_injection_filter(optional),
            Err(CommentFilterError::UnboundedPattern(_))
        ));
        let alternative = r#"([(name) @n (user)] (#eq? @n "TODO"))"#;
        assert!(matches!(
            comment_injection_filter(alternative),
            Err(CommentFilterError::UnboundedPattern(_))
        ));
    }

    #[test]
    fn a_capture_with_no_requirement_is_unbounded() {
        assert!(matches!(
            comment_injection_filter("(user) @constant"),
            Err(CommentFilterError::UnboundedPattern(_))
        ));
        assert!(matches!(
            comment_injection_filter(r#"("text" @x (#not-any-of? @x "TODO"))"#),
            Err(CommentFilterError::UnboundedPattern(_))
        ));
    }

    #[test]
    fn match_regexes_lose_only_their_outer_anchors() {
        assert_eq!(unanchored("^#[0-9]+$").unwrap(), "#[0-9]+");
        assert_eq!(unanchored("a|b").unwrap(), "(?:a|b)");
        assert_eq!(unanchored("^(a|b)").unwrap(), "(a|b)");
        assert!(unanchored("^a|b$").is_err());
        assert!(unanchored("\\bTODO\\b").is_err());
        assert!(unanchored("(?i)todo").is_err());
    }

    #[test]
    fn unfilterable_comment_injections_are_rejected() {
        for query in [
            "((comment)+ @injection.content (#set! injection.language \"comment\"))",
            "((comment) @injection.content (#set! injection.language \"comment\") (#set! injection.combined))",
            "(x (comment)? @injection.content (#set! injection.language \"comment\"))",
        ] {
            assert!(
                matches!(
                    filter_comment_injections(query, "TODO"),
                    Err(CommentFilterError::UnsupportedInjection(_))
                ),
                "{query}"
            );
        }
    }

    #[test]
    fn only_comment_injections_are_filtered() {
        let query = "; (#set! injection.language \"comment\")\n((string) @injection.content\n  (#set! injection.language \"regex\"))\n([\n  (line_comment)\n  (block_comment)\n] @injection.content\n  (#set! injection.language \"comment\"))\n";
        let expected = "; (#set! injection.language \"comment\")\n((string) @injection.content\n  (#set! injection.language \"regex\"))\n([\n  (line_comment)\n  (block_comment)\n] @injection.content\n  (#set! injection.language \"comment\")\n  (#match? @injection.content \"a\\\\.b\"))\n";
        assert_eq!(filter_comment_injections(query, "a\\.b").unwrap(), expected);
    }
}
