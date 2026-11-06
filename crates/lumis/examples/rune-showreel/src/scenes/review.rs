//! A code review of the shot before's loop. Lumis's word diff finds what changed inside each
//! line; those ranges go back in as annotations on the Rust, so the changed words sit on the
//! syntax colours rather than replacing them. The reviewer approves, and it sparkles.

use std::sync::OnceLock;

use lumis::annotations::Annotation;
use lumis::decorations::DiffWordKind;
use lumis::languages::Language;
use lumis::themes::Theme;
use rune::color::Color;
use rune::{El, col, el, row};

use crate::data::{Run, annotated, diff_words, fg, packed, scope, scope_bg, theme};
use crate::gfx::{GLOW, GRAIN, SPARKLES};
use crate::ide::{CODE_X, CODE_Y, Chrome, GUTTER, LH, SIDE, TABS, TITLE, lines, window, y_of};
use crate::kit::{H, W, ease, fx, mono, pointer, pressed, sp, t, track};
use crate::scenes::FILES;

pub(crate) const LEN: f64 = 3.0;
const THEME: &str = "github_light";
const TAP: f64 = 1.9;
const DIFF: &str = "@@ -4,9 +4,7 @@ pub fn total_price(cart: &Cart, rates: &Rates) -> f64 {
     for item in &cart.items {
         let rate = rates.lookup(&item.region);
-        for _ in 0..item.qty {
-            total += item.price * rate.factor();
-        }
+        total += item.unit_price * rate.factor() * f64::from(item.qty);
     }
-    let discount = cart.coupon.map(|c| c.percent);
-    total
+    total * cart.coupon.map_or(1.0, |c| 1.0 - c.percent)
 }";
/// The line the reviewer comments under: the end of the hunk.
const COMMENT_AFTER: usize = 11;

/// The diff without its `+`/`-`/` ` column, and the hunk header blanked: Rust that Lumis can
/// highlight, line for line with the diff.
fn rust() -> &'static str {
    static RUST: OnceLock<String> = OnceLock::new();
    RUST.get_or_init(|| {
        DIFF.lines()
            .map(|l| if l.starts_with("@@") { "" } else { &l[1..] })
            .collect::<Vec<_>>()
            .join("\n")
    })
}

/// Each changed word, moved from the diff's offsets to the Rust's.
fn words() -> &'static [Annotation<DiffWordKind>] {
    static WORDS: OnceLock<Vec<Annotation<DiffWordKind>>> = OnceLock::new();
    WORDS.get_or_init(|| {
        let starts: Vec<(usize, usize)> = DIFF
            .lines()
            .zip(rust().lines())
            .scan((0, 0), |(d, r), (dl, rl)| {
                let at = (*d, *r);
                *d += dl.len() + 1;
                *r += rl.len() + 1;
                Some(at)
            })
            .collect();
        diff_words(DIFF)
            .into_iter()
            .filter_map(|(range, kind)| {
                let text = &DIFF[range.clone()];
                let lead = text.len() - text.trim_start().len();
                let range = range.start + lead..range.start + text.trim_end().len();
                if range.is_empty() {
                    return None;
                }
                let &(d, r) = starts.iter().rev().find(|(d, _)| *d <= range.start)?;
                let shift = |o: usize| r + o - d - 1;
                Annotation::new(shift(range.start)..shift(range.end), kind).ok()
            })
            .collect()
    })
}

fn runs() -> &'static [Run<DiffWordKind>] {
    static RUNS: OnceLock<Vec<Run<DiffWordKind>>> = OnceLock::new();
    RUNS.get_or_init(|| annotated(rust(), Language::Rust, theme(THEME), words()))
}

fn sides(th: &Theme) -> [(Color, Color, Color); 2] {
    let side = |line: &str, word: &str, fallback: Color| {
        let ink = scope(th, line);
        let fill = scope_bg(th, line).unwrap_or_else(|| ink.mix(fallback, 0.8));
        let strong = scope_bg(th, word).unwrap_or_else(|| ink.mix(fallback, 0.55));
        (ink, fill, strong)
    };
    [
        side("diff.minus", "diff.minus.word", Color::WHITE),
        side("diff.plus", "diff.plus.word", Color::WHITE),
    ]
}

fn rows(th: &Theme) -> El {
    let [minus, plus] = sides(th);
    let mut out = el();
    for (n, l) in DIFF.lines().enumerate() {
        let (sign, fill) = match l.as_bytes().first() {
            Some(b'-') => ("−", Some(minus)),
            Some(b'+') => ("+", Some(plus)),
            _ => ("", None),
        };
        let y = y_of(n) - CODE_Y;
        let mut r = row()
            .absolute(-GUTTER - 12.0, y)
            .size(W - SIDE, LH)
            .center_y()
            .child(
                t(sign, 13.0, 700.0, fill.map_or(fg(th), |f| f.0))
                    .w(GUTTER)
                    .px(16.0),
            );
        if let Some((_, f, _)) = fill {
            r = r.fill(f.alpha(0.55));
        }
        if l.starts_with("@@") {
            r = r
                .fill(fg(th).alpha(0.05))
                .child(t(l, 12.5, 400.0, fg(th).alpha(0.5)).font(mono(12.5)));
        }
        out = out.child(r);
    }
    out
}

fn comment(th: &Theme, k: f32) -> El {
    col()
        .absolute(CODE_X - 8.0, y_of(COMMENT_AFTER + 1) + 4.0)
        .w(460.0)
        .padding(12.0)
        .gap(6.0)
        .radius(10.0)
        .fill(Color::WHITE)
        .border(1.0, fg(th).alpha(0.15))
        .shadow(10.0, 30.0, Color::BLACK.alpha(0.15))
        .opacity(k.clamp(0.0, 1.0))
        .offset(0.0, 10.0 * (1.0 - k))
        .child(
            row()
                .gap(8.0)
                .center_y()
                .child(
                    el().size(20.0, 20.0)
                        .radius(10.0)
                        .fill(scope(th, "function")),
                )
                .child(t("kai", 12.5, 600.0, fg(th)))
                .child(t("just now", 11.5, 400.0, fg(th).alpha(0.5))),
        )
        .child(t(
            "One multiply instead of a loop per unit. Ship it.",
            13.0,
            400.0,
            fg(th).alpha(0.85),
        ))
}

fn approve(th: &Theme, t_: f64) -> (El, [f32; 2]) {
    let green = scope(th, "diff.plus");
    let done = ease(t_, TAP, 0.15);
    let pop = track(t_, 0.0, &[(TAP, 1.0, sp(0.35, 0.55))]);
    let (x, y) = (W - 150.0, TITLE + 4.0);
    let button = row()
        .absolute(x, y)
        .size(130.0, TABS - 8.0)
        .radius(7.0)
        .center()
        .gap(6.0)
        .fill(green.alpha(0.12 + 0.88 * done))
        .border(1.0, green.alpha(0.6))
        .scale(1.0 - 0.06 * pressed(t_, TAP) + 0.05 * (pop - pop.min(1.0)))
        .child(t(
            if t_ >= TAP { "Approved ✓" } else { "Approve" },
            12.5,
            600.0,
            green.mix(Color::WHITE, done),
        ));
    (button, [x + 65.0, y + (TABS - 8.0) / 2.0])
}

pub(crate) fn view(t_: f64) -> El {
    let th = theme(THEME);
    let [minus, plus] = sides(th);
    let reveal = ease(t_, 0.15, 0.5);
    let code = lines(rust(), runs(), rust().len(), fg(th), |_, run, s| {
        if let Some(kind) = run.marks.last() {
            let (_, _, strong) = if **kind == DiffWordKind::Removed {
                minus
            } else {
                plus
            };
            s.bg = Some(strong.alpha(reveal));
            s.bg_pad = 1.5;
        }
    });
    let chrome = Chrome {
        title: "Pull request #42 — Hoist the per-unit loop",
        files: FILES,
        active: 2,
        tabs: &["Files changed  1", "Conversation", "Checks"],
        lang: "Rust",
        lines: 0,
    };
    let (button, at) = approve(th, t_);
    let age = (t_ - TAP) as f32;
    let green = scope(th, "diff.plus");
    let note = track(t_, 0.0, &[(0.7, 1.0, sp(0.45, 0.3))]);
    window(th, &chrome, el().child(rows(th)).child(code))
        .child(comment(th, note))
        .child(
            fx(
                GLOW,
                [
                    1.4 * ease(t_, TAP, 0.2) * (1.0 - ease(t_, TAP + 0.4, 0.6)),
                    22.0,
                    0.0,
                    packed(green),
                ],
            )
            .absolute(at[0] - 65.0, at[1] - 12.0)
            .size(130.0, 24.0)
            .radius(7.0)
            .effect_bleed(60.0),
        )
        .child(button)
        .child(
            fx(
                SPARKLES,
                [
                    if age >= 0.0 { age } else { -1.0 },
                    36.0,
                    5.0,
                    packed(green),
                ],
            )
            .absolute(at[0] - 65.0, at[1] - 12.0)
            .size(130.0, 24.0)
            .effect_bleed(140.0),
        )
        .child(pointer(
            t_,
            [W - 240.0, H - 60.0],
            &[(1.2, at[0] + 10.0, at[1] + 4.0)],
            &[TAP],
        ))
        .child(
            fx(GRAIN, [0.03, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
}
