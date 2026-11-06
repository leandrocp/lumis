//! What the scenes draw: every sample in `samples/`, cut to its most colourful window and
//! tokenised once by Lumis, and every bundled theme. Lumis colours a snippet once per theme; the
//! segments are the same in every theme, so two themes can crossfade token by token.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;
use std::sync::OnceLock;

use lumis::HighlightOptions;
use lumis::annotations::Annotation;
use lumis::decorations::{Decoration, DiffWordKind};
use lumis::events::HighlightEvent;
use lumis::highlight::{highlight_events_with_options, highlight_iter};
use lumis::languages::Language;
use lumis::themes::{self, Appearance, Theme};
use rune::color::{Color, hex};
use rune::{Span, rich};

pub(crate) struct Snippet {
    pub(crate) name: &'static str,
    pub(crate) lang: Language,
    pub(crate) code: String,
    pub(crate) tokens: Vec<(Range<usize>, &'static str)>,
}

const SAMPLES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../samples");

pub(crate) fn themes() -> &'static [&'static Theme] {
    static THEMES: OnceLock<Vec<&'static Theme>> = OnceLock::new();
    THEMES.get_or_init(|| themes::available().collect())
}

pub(crate) fn theme(name: &str) -> &'static Theme {
    themes()
        .iter()
        .copied()
        .find(|t| t.name == name)
        .expect("bundled theme")
}

pub(crate) fn snippets() -> &'static [Snippet] {
    static SNIPPETS: OnceLock<Vec<Snippet>> = OnceLock::new();
    SNIPPETS.get_or_init(|| {
        let mut paths: Vec<_> = std::fs::read_dir(SAMPLES)
            .expect("samples/")
            .flatten()
            .map(|e| e.path())
            .collect();
        paths.sort();
        let mut out: Vec<Snippet> = paths
            .iter()
            .filter_map(|p| {
                let src = std::fs::read_to_string(p).ok()?;
                let lang = Language::guess(p.to_str(), &src);
                if lang == Language::PlainText {
                    return None;
                }
                std::panic::catch_unwind(|| window(lang, &src, 9))
                    .inspect_err(|_| eprintln!("skipped {}: highlighting panicked", lang.name()))
                    .ok()
            })
            .collect();
        out.dedup_by_key(|s| s.name);
        out
    })
}

fn tokenize(lang: Language, code: &str) -> Vec<(Range<usize>, &'static str)> {
    let mut out = Vec::new();
    let _ = highlight_iter(code, lang, None, |_, _, range, scope, _| {
        if !scope.is_empty() && !code[range.clone()].trim().is_empty() {
            out.push((range, scope));
        }
        Ok::<_, std::fmt::Error>(())
    });
    out
}

/// The `n` consecutive lines (each at most 64 columns) with the most distinct scopes.
fn window(lang: Language, src: &str, n: usize) -> Snippet {
    let lines: Vec<String> = src
        .lines()
        .map(|l| l.replace('\t', "    ").chars().take(64).collect())
        .collect();
    let all = lines.join("\n");
    let tokens = tokenize(lang, &all);
    let starts: Vec<usize> = lines
        .iter()
        .scan(0, |at, l| {
            let s = *at;
            *at += l.len() + 1;
            Some(s)
        })
        .collect();
    let mut best = (0, 0);
    for i in 0..lines.len().saturating_sub(n).max(1) {
        let (a, b) = (starts[i], starts.get(i + n).copied().unwrap_or(all.len()));
        let mut seen: Vec<&str> = tokens
            .iter()
            .filter(|(r, _)| r.start >= a && r.end <= b)
            .map(|(_, s)| *s)
            .collect();
        seen.sort_unstable();
        seen.dedup();
        let blank = usize::from(lines[i].trim().is_empty());
        if seen.len().saturating_sub(blank) > best.1 {
            best = (i, seen.len() - blank);
        }
    }
    let code = lines[best.0..(best.0 + n).min(lines.len())]
        .join("\n")
        .trim_end()
        .to_string();
    Snippet {
        name: lang.name(),
        lang,
        tokens: tokenize(lang, &code),
        code,
    }
}

pub(crate) fn color(s: &str) -> Color {
    hex(s).unwrap_or(Color::WHITE)
}

/// A theme's text colour; themes that leave it to the terminal get one for their appearance.
pub(crate) fn fg(t: &Theme) -> Color {
    color(
        t.fg()
            .unwrap_or(if dark(t) { "#d4d4d4" } else { "#24292f" }),
    )
}

/// A theme's background, as [`fg`] picks its text colour.
pub(crate) fn bg_hex(t: &Theme) -> &str {
    t.bg()
        .unwrap_or(if dark(t) { "#1e1e1e" } else { "#ffffff" })
}

pub(crate) fn bg(t: &Theme) -> Color {
    color(bg_hex(t))
}

pub(crate) fn dark(t: &Theme) -> bool {
    t.appearance == Appearance::Dark
}

/// One token's colour, weight and slant in a theme.
#[derive(Clone, Copy)]
struct Look {
    color: Color,
    bold: bool,
    italic: bool,
}

/// A snippet and a theme, by address: both live for the whole program.
type Pair = (usize, usize);

/// How Lumis styles each of the snippet's tokens in `theme`, highlighted once per pair.
fn looks(s: &'static Snippet, theme: &'static Theme) -> Rc<[Look]> {
    thread_local! {
        static LOOKS: RefCell<HashMap<Pair, Rc<[Look]>>> = RefCell::default();
    }
    let key = (
        std::ptr::from_ref(s) as usize,
        std::ptr::from_ref(theme) as usize,
    );
    if let Some(hit) = LOOKS.with_borrow(|m| m.get(&key).cloned()) {
        return hit;
    }
    let mut out = Vec::with_capacity(s.tokens.len());
    let _ = highlight_iter(
        &s.code,
        s.lang,
        Some(theme.clone()),
        |text, _, _, scope, st| {
            if !scope.is_empty() && !text.trim().is_empty() {
                out.push(Look {
                    color: st.fg.as_deref().map_or_else(|| fg(theme), color),
                    bold: st.bold,
                    italic: st.italic,
                });
            }
            Ok::<_, std::fmt::Error>(())
        },
    );
    assert_eq!(
        out.len(),
        s.tokens.len(),
        "{} segments differ by theme",
        s.name
    );
    let out: Rc<[Look]> = out.into();
    LOOKS.with_borrow_mut(|m| m.insert(key, out.clone()));
    out
}

/// The snippet's first `chars` characters, coloured partway (`k`) from theme `a` to theme `b`.
pub(crate) fn code(
    s: &'static Snippet,
    a: &'static Theme,
    b: &'static Theme,
    k: f32,
    chars: usize,
    style: rune::TextStyle,
) -> rune::El {
    let end = s
        .code
        .char_indices()
        .nth(chars)
        .map_or(s.code.len(), |(i, _)| i);
    let (la, lb) = (looks(s, a), looks(s, b));
    let spans = s
        .tokens
        .iter()
        .zip(la.iter().zip(lb.iter()))
        .take_while(|((r, _), _)| r.start < end)
        .map(|((r, _), (la, lb))| {
            let near = if k < 0.5 { la } else { lb };
            let mut sp = Span::new(r.start, r.end.min(end));
            sp.color = Some(la.color.mix(lb.color, k));
            sp.weight = near.bold.then_some(600.0);
            sp.italic = near.italic;
            sp
        })
        .collect();
    rich(&s.code[..end], spans)
        .font(style)
        .color(fg(a).mix(fg(b), k))
}

/// Up to six of a theme's token colours, most characteristic first, for the lava behind it.
pub(crate) fn palette(t: &Theme) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();
    for scope in [
        "keyword",
        "string",
        "function",
        "type",
        "constant",
        "number",
        "property",
        "operator",
        "variable.builtin",
        "tag",
    ] {
        if let Some(c) = t
            .get_style(scope)
            .and_then(|s| s.fg.as_deref())
            .and_then(|h| u32::from_str_radix(h.trim_start_matches('#').get(..6)?, 16).ok())
            && !out.contains(&c)
        {
            out.push(c);
        }
        if out.len() == 6 {
            break;
        }
    }
    if out.is_empty() {
        out.push(0x0088_8888);
    }
    out
}

/// A stretch of source with one Lumis style and the same annotations open over it.
pub(crate) struct Run<M: 'static> {
    pub(crate) range: Range<usize>,
    pub(crate) color: Color,
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    /// The caller's annotations around this stretch, outermost first.
    pub(crate) marks: Vec<&'static M>,
}

/// `code` cut wherever its syntax or its annotations change. Lumis splits the source at every
/// annotation boundary, even inside a token, and reports which annotations are open over each
/// piece; the style of each piece is the one Lumis gives the token it falls in.
pub(crate) fn annotated<M>(
    code: &str,
    lang: Language,
    theme: &Theme,
    annotations: &'static [Annotation<M>],
) -> Vec<Run<M>> {
    let mut looks: Vec<(Range<usize>, Look)> = Vec::new();
    let _ = highlight_iter(code, lang, Some(theme.clone()), |_, _, range, _, st| {
        looks.push((
            range,
            Look {
                color: st.fg.as_deref().map_or_else(|| fg(theme), color),
                bold: st.bold,
                italic: st.italic,
            },
        ));
        Ok::<_, std::fmt::Error>(())
    });
    let plain = Look {
        color: fg(theme),
        bold: false,
        italic: false,
    };
    let options = HighlightOptions::new().annotations(annotations);
    let events = highlight_events_with_options(code, lang, options).expect("annotations in range");
    let mut open: Vec<&'static M> = Vec::new();
    let mut out = Vec::new();
    for event in events {
        match event {
            HighlightEvent::AnnotationStart { data, .. } => open.push(data),
            HighlightEvent::AnnotationEnd => {
                open.pop();
            }
            HighlightEvent::Source { start, end } => {
                let look = looks
                    .iter()
                    .find(|(r, _)| r.start <= start && start < r.end)
                    .map_or(plain, |(_, l)| *l);
                out.push(Run {
                    range: start..end,
                    color: look.color,
                    bold: look.bold,
                    italic: look.italic,
                    marks: open.clone(),
                });
            }
            _ => {}
        }
    }
    out
}

type Styled = Rc<[Run<()>]>;

/// `code` as Lumis styles it in `theme`, highlighted once per pair.
pub(crate) fn styled(code: &'static str, lang: Language, theme: &'static Theme) -> Rc<[Run<()>]> {
    thread_local! {
        static STYLED: RefCell<HashMap<(usize, Pair), Styled>> = RefCell::default();
    }
    let key = (
        code.len(),
        (code.as_ptr() as usize, std::ptr::from_ref(theme) as usize),
    );
    if let Some(hit) = STYLED.with_borrow(|m| m.get(&key).cloned()) {
        return hit;
    }
    let out: Rc<[Run<()>]> = annotated(code, lang, theme, &[]).into();
    STYLED.with_borrow_mut(|m| m.insert(key, out.clone()));
    out
}

/// The words Lumis marks as changed between a diff's removed lines and the added lines that
/// replace them.
pub(crate) fn diff_words(diff: &str) -> Vec<(Range<usize>, DiffWordKind)> {
    let options = HighlightOptions::new().word_diff(true);
    let events = highlight_events_with_options::<()>(diff, Language::Diff, options).expect("diff");
    let mut open: Vec<Option<DiffWordKind>> = Vec::new();
    let mut out = Vec::new();
    for event in events {
        match event {
            HighlightEvent::DecorationStart { decoration } => open.push(match decoration {
                Decoration::DiffWord { kind } => Some(kind),
                _ => None,
            }),
            HighlightEvent::DecorationEnd => {
                open.pop();
            }
            HighlightEvent::Source { start, end } => {
                if let Some(kind) = open.iter().rev().find_map(|k| *k) {
                    out.push((start..end, kind));
                }
            }
            _ => {}
        }
    }
    out
}

/// A colour packed as `0xRRGGBB` in an `f32`, for effect data.
#[allow(clippy::cast_precision_loss)]
pub(crate) fn packed(c: Color) -> f32 {
    u32::from_str_radix(&c.hex()[1..7], 16).unwrap_or(0) as f32
}

/// A theme's colour for `scope`, or its text colour.
pub(crate) fn scope(t: &Theme, scope: &str) -> Color {
    first(t, &[scope])
}

/// A theme's colour for the first of `scopes` it sets, or its text colour.
pub(crate) fn first(t: &Theme, scopes: &[&str]) -> Color {
    scopes
        .iter()
        .find_map(|s| t.get_style(s).and_then(|st| st.fg.as_deref()))
        .map_or_else(|| fg(t), color)
}

/// The theme's colour for errors and removals.
pub(crate) fn danger(t: &Theme) -> Color {
    first(t, &["diff.minus", "error", "comment.error"])
}

/// A theme's background for `scope`, if it sets one.
pub(crate) fn scope_bg(t: &Theme, scope: &str) -> Option<Color> {
    t.get_style(scope).and_then(|s| s.bg.as_deref()).map(color)
}
