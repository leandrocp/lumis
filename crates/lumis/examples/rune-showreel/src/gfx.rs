//! The compositor: every `El::effect` slot becomes one rune-fx layer. The slot's mark says which
//! effect and its `effect_data` the effect's live numbers. The lava takes its colours from Lumis
//! themes: `[theme a, theme b, mix, intensity]`. Effects run on the scenes' clock, `start + t`,
//! so a recording that starts partway into the reel draws the same frames as the whole reel.

use rune::compose::{Compositor, Frame};
use rune::render::{EffectPhase, EffectSite, KIND_EFFECT};
use rune_fx::{
    FireParams, FireStyle, Fx, FxFrame, FxLayer, FxOptions, FxRenderer, FxSlot, GlowParams,
    GlowStyle, GrainParams, MetaballParams, Rgb, Shape, SparkleParams, Stops, rgb,
};

use crate::State;
use crate::data::{bg_hex, palette, themes};
use crate::kit::index;

pub(crate) const LAVA: u32 = 1;
pub(crate) const GLOW: u32 = 2;
pub(crate) const SPARKLES: u32 = 3;
pub(crate) const GRAIN: u32 = 4;
pub(crate) const FIRE: u32 = 5;

fn mix(a: Rgb, b: Rgb, k: f32) -> Rgb {
    Rgb([0, 1, 2].map(|i| a.0[i] + (b.0[i] - a.0[i]) * k))
}

fn theme_rgb(i: f32, slot: usize) -> Rgb {
    let all = themes();
    let p = palette(all[index(f64::from(i)).min(all.len() - 1)]);
    rgb(p[slot % p.len()])
}

fn theme_bg(i: f32) -> Rgb {
    let all = themes();
    let hex = bg_hex(all[index(f64::from(i)).min(all.len() - 1)]);
    rgb(
        u32::from_str_radix(hex.trim_start_matches('#').get(..6).unwrap_or("000000"), 16)
            .unwrap_or(0),
    )
}

fn layer(kind: u32, d: [f32; 4], shape: Shape, t: f32) -> Option<FxLayer> {
    let fx = match kind {
        LAVA => {
            let colors: Vec<Rgb> = (0..5)
                .map(|s| mix(theme_rgb(d[0], s), theme_rgb(d[1], s), d[2]))
                .collect();
            let base = mix(theme_bg(d[0]), theme_bg(d[1]), d[2]);
            let p = MetaballParams {
                colors: Stops::new(&colors),
                base,
                count: 9,
                size: 0.12,
                speed: 0.45,
                gloss: 0.3,
            };
            return Some(Fx::fill(p).on(shape).at(t).seed(7).opacity(d[3]));
        }
        GLOW => Fx::glow(GlowParams {
            style: GlowStyle::Aura,
            color: theme_rgb(d[0], 0),
            intensity: d[1],
            radius: d[2],
            aura_speed: 0.35,
            ..Default::default()
        }),
        SPARKLES if d[0] < 0.0 => return None,
        SPARKLES => Fx::sparkles(SparkleParams {
            burst: Some(t - d[0]),
            count: index(f64::from(d[1])) as u32,
            size: d[2],
            color: theme_rgb(d[3], 1),
            margin: 30.0,
            ..Default::default()
        }),
        GRAIN => Fx::grain(GrainParams {
            amount: d[0],
            fps: d[1],
            ..Default::default()
        }),
        FIRE => {
            let p = FireParams {
                style: FireStyle::Top,
                height: d[0],
                intensity: d[1],
                smoke: d[3],
                embers: 24,
                scale: 26.0,
                ..FireParams::default()
            };
            return Some(Fx::fire(p).on(shape).at(t).opacity(d[2]));
        }
        _ => return None,
    };
    Some(fx.on(shape).at(t))
}

#[derive(Default)]
pub(crate) struct Gfx {
    fx: Option<FxRenderer>,
}

impl Compositor<State> for Gfx {
    fn animating(&self) -> bool {
        true
    }

    fn render(&mut self, frame: &mut Frame<'_, State>) {
        let (device, queue, size) = (frame.device, frame.queue, frame.size);
        let time = (frame.rt.state.start + frame.rt.state.t) as f32;
        let marks = &frame.rt.marks;
        let mut at: Vec<u32> = Vec::new();
        let mut layers: Vec<FxLayer> = Vec::new();
        let mut placements: Vec<FxSlot> = Vec::new();
        let mut fire = false;
        for (slot, inst) in frame.rt.instances.iter().enumerate() {
            if inst.params[1] != KIND_EFFECT {
                continue;
            }
            let kind = marks.get(slot).copied().unwrap_or(0) & 0xff;
            let scale = inst.params[3];
            let [x, y, w, h] = inst.rect.map(|v| v / scale);
            let radius = inst.radii.iter().fold(0.0f32, |m, v| m.max(v.abs())) / scale;
            let Some(fx_layer) = layer(kind, inst.uv, Shape::rounded(x, y, w, h, radius), time)
            else {
                continue;
            };
            fire |= kind == FIRE;
            at.push(slot as u32);
            layers.push(fx_layer);
            placements.push(FxSlot::default().scale(scale).clip(inst.clip, inst.fade));
        }
        let fx = self.fx.get_or_insert_with(|| {
            let mut fx = FxRenderer::new(device, frame.format, FxOptions { dither: true });
            fx.set_fire_sim(device, true);
            fx
        });
        fx.set_srgb_blend(frame.srgb_blend());
        fx.add_format(device, wgpu::TextureFormat::Rgba16Float);
        fx.prepare_with_slots(
            device,
            queue,
            &FxFrame {
                viewport: [size.0, size.1],
                scale: 1.0,
                layers: &layers,
                backdrop: None,
                sources: &[],
            },
            &placements,
        );
        if fire {
            fx.simulate(queue, frame.encoder, None);
        }
        let clear = frame.rt.clear_color();
        frame.ui.prepare(
            device,
            queue,
            &mut frame.rt.text.atlas,
            &frame.rt.instances,
            size,
        );
        frame
            .ui
            .draw_with_backdrop(frame.encoder, frame.target, clear, &mut |phase,
                                                                          s: &EffectSite<
                '_,
            >| {
                let Some(k) = at.iter().position(|&slot| slot == s.index) else {
                    return;
                };
                match phase {
                    EffectPhase::Backdrop(enc) => {
                        if let Some(bd) = s.backdrop {
                            fx.blur_backdrop(device, queue, enc, bd, k);
                        }
                    }
                    EffectPhase::Draw(pass) => fx.draw_layer_in(pass, s.format, s.origin, k),
                }
            });
    }
}
