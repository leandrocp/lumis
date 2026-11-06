//! Code types itself into a card, highlighted as it lands, over a lava lamp in the theme's own
//! colours.

use rune::{El, col, el, row, spacer};

use std::sync::OnceLock;

use crate::data::{Snippet, bg, code, custom, fg, theme, themes};
use crate::gfx::{GLOW, GRAIN, LAVA};
use crate::kit::{H, W, ease, float, fx, index, mono, sp, t, track};

pub(crate) const LEN: f64 = 6.0;
const THEME: &str = "catppuccin_mocha";
const CARD: (f32, f32) = (720.0, 330.0);
const HERO: &str = r#"use lumis::{highlight::highlight_iter, themes};
use rune_kit::syntax::{Highlighter, Paint};

let theme = themes::get("catppuccin_mocha")?;
let lumis = Highlighter::new(move |lang, code| {
    let mut out = Vec::new();
    highlight_iter(code, guess(lang, code), Some(theme), |_, _, r, _, s| {
        out.push((r, Paint::Color(hex(s.fg?))));
    });
    out
});
Markdown::new(answer).highlighter(lumis)"#;

fn hero() -> &'static Snippet {
    static HERO_SNIPPET: OnceLock<Snippet> = OnceLock::new();
    HERO_SNIPPET.get_or_init(|| custom("rust", HERO))
}

pub(crate) fn view(t_: f64) -> El {
    let th = theme(THEME);
    let ti = themes().iter().position(|x| x.name == THEME).unwrap_or(0);
    let ti = float(ti);
    let s = hero();
    let chars = index((t_ - 0.9) * 120.0);
    let rise = track(t_, 0.0, &[(0.35, 1.0, sp(0.7, 0.25))]);
    let title = ease(t_, 0.1, 0.6);
    let out = ease(t_, LEN - 0.35, 0.35);

    let dot = |c: u32| {
        el().size(11.0, 11.0)
            .radius(5.5)
            .fill(rune::color::hex(&format!("#{c:06x}")).unwrap())
    };
    let card = col()
        .absolute(0.0, 0.0)
        .size(CARD.0, CARD.1)
        .padding(22.0)
        .gap(16.0)
        .radius(18.0)
        .fill(bg(th).alpha(0.88))
        .border(1.0, fg(th).alpha(0.1))
        .shadow(18.0, 48.0, rune::Color::BLACK.alpha(0.45))
        .clip_rounded(18.0)
        .child(
            row()
                .gap(8.0)
                .center_y()
                .child(dot(0x00ff_5f57))
                .child(dot(0x00fe_bc2e))
                .child(dot(0x0028_c840))
                .child(el().w(10.0))
                .child(t("main.rs", 13.0, 500.0, fg(th).alpha(0.6)))
                .child(spacer())
                .child(t(
                    format!("{} · {}", s.name, THEME),
                    12.0,
                    500.0,
                    fg(th).alpha(0.4),
                )),
        )
        .child(code(s, th, th, 0.0, chars, mono(14.0)));

    let stage = el()
        .size(CARD.0, CARD.1)
        .scale(0.9 + 0.1 * rise - 0.04 * out)
        .opacity(rise.min(1.0) * (1.0 - out))
        .offset(0.0, 40.0 * (1.0 - rise))
        .child(
            fx(GLOW, [ti, 0.55 * rise, 34.0, 0.0])
                .absolute(0.0, 0.0)
                .size(CARD.0, CARD.1)
                .radius(18.0)
                .effect_bleed(90.0),
        )
        .child(card);

    let heading = col()
        .gap(6.0)
        .items_center()
        .opacity(title * (1.0 - out))
        .offset(0.0, -10.0 * (1.0 - title))
        .child(t("Lumis × Rune", 40.0, 600.0, fg(th)))
        .child(t(
            "Tree-sitter highlighting with Neovim themes, on the GPU",
            16.0,
            400.0,
            fg(th).alpha(0.6),
        ));

    el().w(W)
        .h(H)
        .fill(bg(th))
        .child(
            fx(LAVA, [ti, ti, 0.0, 0.55 * ease(t_, 0.0, 1.0)])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
        .child(
            col()
                .absolute(0.0, 0.0)
                .size(W, H)
                .center()
                .gap(28.0)
                .child(heading)
                .child(stage),
        )
        .child(
            fx(GRAIN, [0.05, 24.0, 0.0, 0.0])
                .absolute(0.0, 0.0)
                .size(W, H),
        )
}
