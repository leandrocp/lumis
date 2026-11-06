//! A cut on every beat: one language after another, each in a different theme, filling the
//! editor. Nothing is prepared per language; Lumis highlights each one as it is cut to.

use rune::{El, el, row, spacer};

use crate::data::{fg, palette, snippets, styled, themes};
use crate::gfx::GRAIN;
use crate::ide::{Chrome, hex, lines, window};
use crate::kit::{H, W, ease, float, fx, index, t};

pub(crate) const LEN: f64 = 4.0;
const BEAT: f64 = 0.25;

pub(crate) fn view(t_: f64) -> El {
    let (snips, all) = (snippets(), themes());
    let n = index(t_ / BEAT);
    let into = t_ - f64::from(float(n)) * BEAT;
    let s = &snips[(n * 7 + 3) % snips.len()];
    let th = all[(n * 53 + 11) % all.len()];
    let runs = styled(&s.code, s.lang, th);
    let code = lines(&s.code, &runs, s.code.len(), fg(th), |_, _, _| {});
    let name = [s.name];
    let chrome = Chrome {
        title: s.name,
        files: &name,
        active: 0,
        tabs: &name,
        lang: s.name,
        lines: s.code.lines().count(),
    };
    let punch = 1.0 + 0.07 * (1.0 - ease(into, 0.0, 0.2));
    el().w(W)
        .h(H)
        .child(
            window(th, &chrome, code)
                .scale(1.4 * punch)
                .origin(0.2, 0.1),
        )
        .child(
            row()
                .absolute(0.0, H - 96.0)
                .w(W - 44.0)
                .child(spacer())
                .child(t(s.name, 56.0, 600.0, hex(palette(th)[0])).scale(punch)),
        )
        .child(
            fx(GRAIN, [0.05, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
}
