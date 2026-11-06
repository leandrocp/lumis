//! What the scenes draw: every sample in `samples/`, cut to its most colourful window and
//! tokenised once by Lumis, and every bundled theme. Lumis colours a snippet once per theme; the
//! segments are the same in every theme, so two themes can crossfade token by token.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Range;
use std::rc::Rc;
use std::sync::OnceLock;

use lumis::highlight::highlight_iter;
use lumis::languages::Language;
use lumis::themes::{self, Appearance, Theme};
use rune::color::{Color, hex};
use rune::{Span, rich};

pub(crate) struct Snippet {
    pub(crate) name: &'static str,
    lang: Language,
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

/// A hand-written snippet in `lang` (a name or alias), tokenised once.
pub(crate) fn custom(lang: &str, code: &str) -> Snippet {
    let l = Language::guess(Some(lang), code);
    Snippet {
        name: l.name(),
        lang: l,
        tokens: tokenize(l, code),
        code: code.to_string(),
    }
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
