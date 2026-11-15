//! Options written after a code fence's language.
//!
//! A Markdown fence's info string is a language followed by free text, the
//! meta, which `CommonMark` leaves to the renderer:
//!
//! ````markdown
//! ```elixir highlight_lines="1,3-5" theme=github_dark
//! ````
//!
//! [`parse_meta`] reads that text into the options a formatter takes. Names it
//! does not recognise are returned untouched, because other tools write their
//! own options in the same place.

use crate::formatter::html_inline::HighlightLinesStyle;
use std::fmt;
use std::ops::RangeInclusive;

/// The options a code fence's meta sets.
///
/// Every field is `None` when the meta does not set it, or sets it to an empty
/// or invalid value.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeFenceMeta {
    /// `theme=github_dark`: the theme to highlight with.
    pub theme: Option<String>,
    /// `pre_class="a b"`: classes to add to the `<pre>` element.
    pub pre_class: Option<String>,
    /// `highlight_lines="1,3-5"`: 1-based inclusive line ranges.
    pub highlight_lines: Option<Vec<RangeInclusive<usize>>>,
    /// `highlight_lines_style="background: yellow"`, or `theme` for the
    /// theme's own highlight color.
    pub highlight_lines_style: Option<HighlightLinesStyle>,
    /// `highlight_lines_class=emphasis`: a class for highlighted lines.
    pub highlight_lines_class: Option<String>,
    /// `include_highlights`, `include_highlights=true` or
    /// `include_highlights=false`: whether to name each token's highlight.
    pub include_highlights: Option<bool>,
    /// Every other word in the meta, in order.
    pub unrecognized: Vec<MetaAttribute>,
}

/// A word in a code fence's meta that [`parse_meta`] does not recognise.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetaAttribute {
    /// The text before the first `=`, or the whole word when it has none.
    pub name: String,
    /// The text after the first `=`, which may be empty.
    pub value: Option<String>,
}

/// Read the options in a code fence's meta, the info string after its
/// language.
///
/// Words are separated by ASCII whitespace and split into a name and a value
/// at their first `=`. Double or single quotes group text with spaces in it and
/// are removed; inside double quotes `\"` and `\\` escape, and any other
/// backslash is kept. A quote with no closing match is ordinary text.
///
/// When a recognised name appears more than once, the last one wins. A
/// malformed part of `highlight_lines` is dropped on its own, so one typo does
/// not turn the rest off.
///
/// ```rust
/// use lumis_core::code_fence::parse_meta;
///
/// let meta = parse_meta(r#"highlight_lines="1, 3-5" theme=github_dark title="app.ex""#);
///
/// assert_eq!(meta.theme.as_deref(), Some("github_dark"));
/// assert_eq!(meta.highlight_lines, Some(vec![1..=1, 3..=5]));
/// assert_eq!(meta.unrecognized[0].name, "title");
/// assert_eq!(meta.unrecognized[0].value.as_deref(), Some("app.ex"));
/// ```
#[must_use]
pub fn parse_meta(meta: &str) -> CodeFenceMeta {
    let mut parsed = CodeFenceMeta::default();

    for word in words(meta) {
        match word.split_once('=') {
            Some((name, value)) => parsed.set(name, Some(value)),
            None => parsed.set(&word, None),
        }
    }

    parsed
}

impl CodeFenceMeta {
    fn set(&mut self, name: &str, value: Option<&str>) {
        let text = value.filter(|value| !value.is_empty()).map(str::to_string);

        match name {
            "theme" => self.theme = text,
            "pre_class" => self.pre_class = text,
            "highlight_lines" => self.highlight_lines = value.and_then(lenient_line_ranges),
            "highlight_lines_style" => {
                self.highlight_lines_style = text.map(|style| match style.as_str() {
                    "theme" => HighlightLinesStyle::Theme,
                    _ => HighlightLinesStyle::Style(style),
                });
            }
            "highlight_lines_class" => self.highlight_lines_class = text,
            "include_highlights" => {
                self.include_highlights = match value {
                    None | Some("true") => Some(true),
                    Some("false") => Some(false),
                    Some(_) => None,
                };
            }
            _ => self.unrecognized.push(MetaAttribute {
                name: name.to_string(),
                value: value.map(str::to_string),
            }),
        }
    }
}

fn lenient_line_ranges(spec: &str) -> Option<Vec<RangeInclusive<usize>>> {
    let ranges: Vec<_> = parse_line_ranges(spec).filter_map(Result::ok).collect();
    (!ranges.is_empty()).then_some(ranges)
}

fn words(meta: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut rest = meta;
    // A quote nothing closes means no later quote of that kind is closed either:
    // an earlier search reached each later one as an escaped `\"` and carried on
    // from there just as a search starting at it would. Remembering that keeps
    // the walk linear instead of rescanning the tail at every quote.
    let mut unclosed = Vec::new();

    while let Some(ch) = rest.chars().next() {
        rest = &rest[ch.len_utf8()..];

        match ch {
            '"' | '\'' if unclosed.contains(&ch) => word.push(ch),
            '"' | '\'' => {
                if let Some((content, after)) = quoted(rest, ch) {
                    word.push_str(&content);
                    rest = after;
                } else {
                    unclosed.push(ch);
                    word.push(ch);
                }
            }
            ch if ch.is_ascii_whitespace() => {
                if !word.is_empty() {
                    words.push(std::mem::take(&mut word));
                }
            }
            ch => word.push(ch),
        }
    }

    if !word.is_empty() {
        words.push(word);
    }

    words
}

/// The text up to the `quote` that closes it, and what follows that quote, or
/// `None` when nothing closes it.
fn quoted(rest: &str, quote: char) -> Option<(String, &str)> {
    let mut content = String::new();
    let mut chars = rest.char_indices();

    while let Some((index, ch)) = chars.next() {
        if ch == quote {
            return Some((content, &rest[index + ch.len_utf8()..]));
        }

        if quote == '"' && ch == '\\' {
            if let Some((_, escaped @ ('"' | '\\'))) = chars.clone().next() {
                chars.next();
                content.push(escaped);
                continue;
            }
        }

        content.push(ch);
    }

    None
}

/// Why one part of a line spec such as `"1,3-5"` is not a line or a range.
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LineRangeError {
    /// The text is not a line number.
    InvalidNumber(String),
    /// Line numbers start at 1.
    Zero,
    /// The range ends before it starts.
    Reversed {
        /// The first line of the range.
        start: usize,
        /// The last line of the range.
        end: usize,
    },
}

impl fmt::Display for LineRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber(text) => write!(f, "Invalid line number: '{text}'"),
            Self::Zero => f.write_str("Line numbers must be greater than 0"),
            Self::Reversed { start, end } => write!(
                f,
                "Start line ({start}) must be less than or equal to end line ({end})"
            ),
        }
    }
}

impl std::error::Error for LineRangeError {}

/// Each comma-separated part of a line spec such as `"1, 3-5"`, as an
/// inclusive range or the reason it is not one. Empty parts are skipped.
///
/// A fence's `highlight_lines` drops the parts that fail; the CLI's
/// `--highlight-lines` rejects them.
#[doc(hidden)]
pub fn parse_line_ranges(
    spec: &str,
) -> impl Iterator<Item = Result<RangeInclusive<usize>, LineRangeError>> + '_ {
    spec.split(',')
        .map(str::trim_ascii)
        .filter(|part| !part.is_empty())
        .map(parse_line_range)
}

fn parse_line_range(part: &str) -> Result<RangeInclusive<usize>, LineRangeError> {
    let (start, end) = if let Some((start, end)) = part.split_once('-') {
        (line_number(start)?, line_number(end)?)
    } else {
        let line = line_number(part)?;
        (line, line)
    };

    if start == 0 || end == 0 {
        return Err(LineRangeError::Zero);
    }
    if start > end {
        return Err(LineRangeError::Reversed { start, end });
    }

    Ok(start..=end)
}

fn line_number(text: &str) -> Result<usize, LineRangeError> {
    let text = text.trim_ascii();
    text.parse()
        .map_err(|_| LineRangeError::InvalidNumber(text.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::path::PathBuf;

    #[derive(Deserialize)]
    struct Manifest {
        cases: Vec<Case>,
    }

    #[derive(Deserialize)]
    struct Case {
        name: String,
        meta: String,
        expected: Expected,
    }

    #[derive(Debug, Default, Deserialize, PartialEq, Eq)]
    #[serde(default, rename_all = "camelCase", deny_unknown_fields)]
    struct Expected {
        theme: Option<String>,
        pre_class: Option<String>,
        highlight_lines: Option<Vec<(usize, usize)>>,
        highlight_lines_style: Option<String>,
        highlight_lines_class: Option<String>,
        include_highlights: Option<bool>,
        unrecognized: Vec<ExpectedAttribute>,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    struct ExpectedAttribute {
        name: String,
        #[serde(default)]
        value: Option<String>,
    }

    impl From<CodeFenceMeta> for Expected {
        fn from(meta: CodeFenceMeta) -> Self {
            Self {
                theme: meta.theme,
                pre_class: meta.pre_class,
                highlight_lines: meta.highlight_lines.map(|ranges| {
                    ranges
                        .into_iter()
                        .map(|range| (*range.start(), *range.end()))
                        .collect()
                }),
                highlight_lines_style: meta.highlight_lines_style.map(|style| match style {
                    HighlightLinesStyle::Theme => "theme".to_string(),
                    HighlightLinesStyle::Style(css) => css,
                }),
                highlight_lines_class: meta.highlight_lines_class,
                include_highlights: meta.include_highlights,
                unrecognized: meta
                    .unrecognized
                    .into_iter()
                    .map(|attribute| ExpectedAttribute {
                        name: attribute.name,
                        value: attribute.value,
                    })
                    .collect(),
            }
        }
    }

    #[test]
    fn matches_the_shared_fixture() {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/code-fence-meta.json");
        let manifest: Manifest =
            serde_json::from_str(&std::fs::read_to_string(path).expect("read fixture"))
                .expect("parse fixture");
        assert!(
            manifest.cases.len() > 40,
            "found {} cases",
            manifest.cases.len()
        );

        for case in manifest.cases {
            assert_eq!(
                Expected::from(parse_meta(&case.meta)),
                case.expected,
                "{}: {:?}",
                case.name,
                case.meta
            );
        }
    }

    #[test]
    fn reports_why_a_line_spec_part_fails() {
        let results: Vec<_> = parse_line_ranges("1, x, 0, 4-2, 2-0, 3 - 5").collect();
        assert_eq!(
            results,
            vec![
                Ok(1..=1),
                Err(LineRangeError::InvalidNumber("x".to_string())),
                Err(LineRangeError::Zero),
                Err(LineRangeError::Reversed { start: 4, end: 2 }),
                Err(LineRangeError::Zero),
                Ok(3..=5),
            ]
        );

        let messages: Vec<_> = results
            .iter()
            .filter_map(|result| result.as_ref().err().map(ToString::to_string))
            .collect();
        assert_eq!(
            messages,
            [
                "Invalid line number: 'x'",
                "Line numbers must be greater than 0",
                "Start line (4) must be less than or equal to end line (2)",
                "Line numbers must be greater than 0",
            ]
        );
    }

    #[test]
    fn theme_style_is_the_theme_variant_not_css() {
        assert_eq!(
            parse_meta("highlight_lines_style=theme").highlight_lines_style,
            Some(HighlightLinesStyle::Theme)
        );
        assert_eq!(
            parse_meta("highlight_lines_style=Theme").highlight_lines_style,
            Some(HighlightLinesStyle::Style("Theme".to_string()))
        );
    }
}
