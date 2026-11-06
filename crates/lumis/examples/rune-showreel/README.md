# Lumis × Rune showreel

Every bundled Lumis language and theme, animated in [Rune](https://github.com/anomalyco/rune), a
GPU-rendered Rust UI framework. Lumis tokenises each snippet once; Rune draws it as rich text and
animates it, with [rune-fx](https://github.com/anomalyco/rune/tree/dev/crates/rune-fx) shaders
behind it.

Four scenes, 33 seconds back to back:

- **type**: the Rune integration code types itself into a card over a lava lamp tinted with the
  theme's own token colours.
- **themes**: one Elixir snippet through every bundled theme, each token crossfading from one
  theme's colour to the next.
- **annotate**: search hits, compiler diagnostics and a profile drawn over the same Rust code, each
  one an `Annotation` whose range Lumis cuts into the tokens it covers. The hottest line catches
  fire.
- **langs**: one card per language in `samples/`, each in a different theme, while the camera
  pulls back to show the whole wall.

From this directory, run it in a window:

```sh
cargo run --release
```

Render it headless. Scenes run off Rune's virtual clock, so the recording is frame-exact:

```sh
cargo run --release -- --scene reel --video reel.mp4 --size 960x540 --scale 2 --duration 33
cargo run --release -- --scene themes --at 3 --snap themes.png
```

On Linux without a GPU, use Mesa's software Vulkan driver:

```sh
VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json cargo run --release -- --scene reel --video reel.mp4
```

Under that driver the headless recorder holds about 20 MB per frame, so a whole reel does not
fit in 8 GB. `reel@N` starts the reel N seconds in on a fresh clock (N = 2, 4 … 32); record 2 s
segments and join them with `ffmpeg -f concat -c copy`.

To colour Markdown code fences in a Rune app with Lumis, use `rune_kit::syntax::Highlighter`; the
code typed in the first scene is that integration.

Fonts in `assets/` are Inter and IBM Plex Mono, under the SIL Open Font License.
