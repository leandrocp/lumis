//! The opener: an editor full of plain grey code. A band of light sweeps across it and every
//! token it passes lights up in the colour Lumis gives it; then the cursor types a test below.

use lumis::languages::Language;
use rune::color::Color;
use rune::el;

use crate::data::{fg, styled, theme};
use crate::gfx::GRAIN;
use crate::ide::{CODE_Y, CW, Chrome, LH, lines, place, window, x_of, y_of};
use crate::kit::{H, W, ease, float, fx};
use crate::scenes::{FILES, PRICING};

pub(crate) const LEN: f64 = 5.0;
const THEME: &str = "tokyonight_night";
const SWEEP: (f64, f64) = (0.5, 1.5);
const TYPE: (f64, f64) = (2.4, 4.6);

/// How lit a token at `column` on `line` is: the band reaches it, it flares, it settles.
fn lit(t: f64, line: usize, column: usize) -> (f32, f32) {
    let at = SWEEP.0 + SWEEP.1 * (f64::from(float(column)) / 64.0) + 0.02 * f64::from(float(line));
    let k = ease(t, at, 0.22);
    (k, 4.0 * k * (1.0 - k))
}

/// How many bytes are on screen: the function, then the test typed out.
fn typed(t: f64) -> usize {
    let body = PRICING.find("\n\n#[cfg").unwrap_or(PRICING.len());
    let k = f64::from(ease(t, TYPE.0, TYPE.1 - TYPE.0));
    let extra = PRICING.len() - body;
    body + crate::kit::index(k * f64::from(float(extra)))
}

pub(crate) fn view(t_: f64) -> rune::El {
    let th = theme(THEME);
    let runs = styled(PRICING, Language::Rust, th);
    let upto = typed(t_);
    let grey = fg(th).alpha(0.38);
    let code = lines(PRICING, &runs, upto, grey, |_, run, s| {
        let (line, column) = place(PRICING, run.range.start);
        let (k, flare) = lit(t_, line, column);
        s.color = Some(grey.mix(run.color, k).mix(Color::WHITE, 0.8 * flare));
    });
    let (cl, cc) = place(PRICING, upto);
    let blink = if t_ < TYPE.0 || t_ > TYPE.1 {
        f32::from(u8::from((t_ * 2.0).fract() < 0.5))
    } else {
        1.0
    };
    let chrome = Chrome {
        title: "lumis — pricing.rs",
        files: FILES,
        active: 2,
        tabs: &["pricing.rs", "cart.rs", "rates.rs"],
        lang: "Rust",
        lines: PRICING[..upto].lines().count().max(1),
    };
    let band = x_of(0) + ((t_ - SWEEP.0) / SWEEP.1) as f32 * 64.0 * CW;
    let glare = ease(t_, SWEEP.0 - 0.2, 0.2) * (1.0 - ease(t_, SWEEP.0 + SWEEP.1 - 0.1, 0.3));
    window(th, &chrome, code)
        .child(
            el().absolute(x_of(cc), y_of(cl) + 2.0)
                .size(2.0, LH - 4.0)
                .fill(fg(th).alpha(blink)),
        )
        .child(
            el().absolute(band - 40.0, CODE_Y - 30.0)
                .size(80.0, 14.0 * LH)
                .fill(Color::WHITE.alpha(0.3 * glare))
                .blur(28.0)
                .rotate(0.18),
        )
        .child(
            fx(GRAIN, [0.04, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
        .scale(1.0 + 0.8 * (1.0 - ease(t_, 0.1, 2.6)))
        .origin(0.36, 0.3)
        .opacity(ease(t_, 0.0, 0.35))
}
