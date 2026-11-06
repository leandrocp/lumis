//! Lumis in Rune: a 30-second reel of an editor whose code Lumis colours, cut like a trailer.
//! Every frame is a function of the (virtual) clock, so a headless `--video` run records it
//! frame-exact:
//!
//!     lumis-rune-showreel --video reel.mp4 --size 960x540 --scale 2 --duration 30
//!
//! `reel@N` starts the reel N seconds in, on a fresh clock, so a long recording can be made in
//! segments without simulating the frames before each one.

mod data;
mod gfx;
mod ide;
mod kit;
mod scenes;

use rune::{AppSpec, Cx, El};

#[derive(Default)]
pub(crate) struct State {
    /// Seconds since the scene started (Rune's virtual clock when headless).
    pub(crate) t: f64,
    /// Seconds into the reel the clock starts at.
    pub(crate) start: f64,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    scenes::reel(s.start + s.t).instant_changes()
}

fn main() {
    // Tokenise every sample before the first frame rather than during it.
    let _ = data::snippets();
    let app = rune::App::new(AppSpec::<State> {
        scenes: &[
            ("reel", |_| {}),
            ("reel@2", |s| s.start = 2.0),
            ("reel@4", |s| s.start = 4.0),
            ("reel@6", |s| s.start = 6.0),
            ("reel@8", |s| s.start = 8.0),
            ("reel@10", |s| s.start = 10.0),
            ("reel@12", |s| s.start = 12.0),
            ("reel@14", |s| s.start = 14.0),
            ("reel@16", |s| s.start = 16.0),
            ("reel@18", |s| s.start = 18.0),
            ("reel@20", |s| s.start = 20.0),
            ("reel@22", |s| s.start = 22.0),
            ("reel@24", |s| s.start = 24.0),
            ("reel@26", |s| s.start = 26.0),
            ("reel@28", |s| s.start = 28.0),
            ("reel@30", |s| s.start = 30.0),
        ],
        fonts: &[
            include_bytes!("../assets/Inter-Regular.ttf"),
            include_bytes!("../assets/Inter-Medium.ttf"),
            include_bytes!("../assets/Inter-SemiBold.ttf"),
            include_bytes!("../assets/IBMPlexMono-Regular.ttf"),
            include_bytes!("../assets/IBMPlexMono-Medium.ttf"),
        ],
        families: ("Inter", "IBM Plex Mono"),
        // Every frame is drawn from the clock, so colours land at once rather than springing:
        // a theme that changes every few frames would otherwise average out to grey.
        poll: Some(|s, t, tx| {
            s.t = t;
            tx.instant();
            true
        }),
        ..AppSpec::new("Lumis × Rune", (kit::W, kit::H), State::default, view)
    })
    .compositor(gfx::Gfx::default());
    app.run();
}
