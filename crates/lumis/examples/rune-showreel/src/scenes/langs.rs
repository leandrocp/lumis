//! Every language in the bundle, one card each in a different theme, popping in outward from
//! the centre while the camera pulls back to show the whole wall.

use rune::{El, col, el};

use crate::data::{bg, code, fg, snippets, theme, themes};
use crate::gfx::{GRAIN, SPARKLES};
use crate::kit::{H, W, ease, float, fx, mono, sp, t, track};

const COLS: usize = 12;
const CARD: (f32, f32) = (236.0, 150.0);
const GAP: f32 = 14.0;
const PULL: (f64, f64) = (0.6, 5.2);
const END: f64 = 5.6;

pub(crate) fn view(t_: f64) -> El {
    let all = themes();
    let snips = snippets();
    let rows = snips.len().div_ceil(COLS);
    let (ww, wh) = (
        float(COLS) * (CARD.0 + GAP) - GAP,
        float(rows) * (CARD.1 + GAP) - GAP,
    );
    let fit = (W / ww).min(H / wh) * 0.94;
    let z = 1.6 + (fit - 1.6) * ease(t_, PULL.0, PULL.1 - PULL.0);
    let (cc, cr) = ((float(COLS) - 1.0) / 2.0, (float(rows) - 1.0) / 2.0);

    let mut wall = el()
        .absolute((W - ww) / 2.0, (H - wh) / 2.0)
        .size(ww, wh)
        .scale(z);
    for (i, s) in snips.iter().enumerate() {
        let (c, r) = (float(i % COLS), float(i / COLS));
        let d = f64::from(((c - cc).powi(2) + (r - cr).powi(2)).sqrt());
        let pop = track(t_, 0.0, &[(0.05 + d * 0.13, 1.0, sp(0.55, 0.3))]);
        let th = all[(i * 41 + 7) % all.len()];
        wall = wall.child(
            col()
                .absolute(c * (CARD.0 + GAP), r * (CARD.1 + GAP))
                .size(CARD.0, CARD.1)
                .padding(12.0)
                .gap(8.0)
                .radius(10.0)
                .fill(bg(th))
                .border(1.0, fg(th).alpha(0.12))
                .clip_rounded(10.0)
                .scale(0.5 + 0.5 * pop)
                .opacity(pop.clamp(0.0, 1.0))
                .child(t(s.name, 11.0, 600.0, fg(th).alpha(0.65)))
                .child(code(s, th, th, 0.0, usize::MAX, mono(9.5))),
        );
    }

    let langs = lumis::languages::Language::iter()
        .filter(|l| *l != lumis::languages::Language::PlainText)
        .count();
    let base = theme("tokyonight_night");
    let fin = ease(t_, END, 0.5);
    let pop = track(t_, 0.0, &[(END, 1.0, sp(0.55, 0.35))]);
    let ti = all.iter().position(|x| x.name == base.name).unwrap_or(0);
    let ti = float(ti);
    let banner = col()
        .gap(10.0)
        .items_center()
        .scale(0.8 + 0.2 * pop)
        .opacity(fin)
        .child(t(
            format!(
                "{} languages · {} themes",
                langs,
                lumis::themes::available().count()
            ),
            44.0,
            600.0,
            fg(base),
        ))
        .child(t("lumis.sh", 18.0, 500.0, fg(base).alpha(0.7)).font(mono(18.0)));

    el().w(W)
        .h(H)
        .fill(bg(base))
        .child(wall)
        .child(
            el().absolute(0.0, 0.0)
                .size(W, H)
                .fill(bg(base).alpha(0.7 * fin)),
        )
        .child(col().absolute(0.0, 0.0).size(W, H).center().child(banner))
        .child(
            fx(SPARKLES, [(t_ - END) as f32, 30.0, 10.0, ti])
                .absolute(W / 2.0 - 260.0, H / 2.0 - 60.0)
                .size(520.0, 120.0)
                .effect_bleed(60.0),
        )
        .child(
            fx(GRAIN, [0.05, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
}
