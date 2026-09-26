# HTML line screenshot QA

```sh
mise run qa-html-lines
```

This builds the local packages, generates real highlighted HTML through Rust,
the CLI, Elixir, Node native and Node Wasm, then independently highlights the same
inputs through browser Wasm in Chromium, Firefox and WebKit. It does not reuse
Rust's HTML as another runtime's output.

## Coverage

- Three formatters: inline, linked and multi-theme (named `dark` default).
- Both gutter settings; adjacent rows 1–2 highlighted; Dracula theme with italics.
- Nine source cases: basic, empty middle, final LF, trailing empty, CRLF with
  Unicode, multiline string, horizontal overflow, newline-only and empty input.
- Every page contains all 54 variants. Each runtime gets a separate full-page
  screenshot at the left and right scroll positions in every browser.
- Layout assertions check line tags, numbers, text, row count, row height and
  highlight coverage across the full scroll width, including short/empty rows.

The page only loads generated layout CSS inside the linked panels. Inline and
multi-theme panels must supply their own structural styles. Shared presentation
CSS sets the font, panel grid, scrolling and gutter spacing; it does not repair
line layout. Gutters receive only inline padding, not an additional inline-block
box that could introduce its own baseline sizing.

## Acceptance

Every HTML variant must equal the native Rust reference byte for byte. Every
PNG must equal that browser's Rust screenshot byte for byte: identical PNG bytes
prove identical pixels. No masks, cropping, resizing, antialiasing threshold or
allowed changed pixels are used.

A one-CSS-pixel magenta overlay at device scale 1 is deliberately introduced in
each browser after the passing captures. The same comparison must reject it.
The negative-control images are evidence of the guard, not passing runtime
screenshots.

Browsers are compared **within each engine**, not across engines. Different
engines can rasterize text differently; cross-engine differences are not runtime
differences. The report records the browser versions and host platform. Code uses the bundled
Roboto Mono variable font, loaded explicitly at regular and bold weights before
layout checks. This avoids host Menlo/fallback metrics changing numbered row
height in Firefox. Neither geometry assertions nor pixel checks are relaxed.

`fonts/RobotoMono.ttf` is the unmodified `RobotoMono[wght].ttf` from
[google/fonts at 23e54b51](https://github.com/google/fonts/tree/23e54b51ddffbc7713c583748e3bd86f62b1fa4a/ofl/robotomono),
licensed under [SIL OFL 1.1](fonts/OFL.txt). Its SHA-256 is
`66a80e79d17e4c7cabd162e2916578a4cc08fd19eef6e2a643305eae9c567b2b`.
The font is a QA fixture only, not a published runtime dependency.

## Evidence

`generated/report.json` records the implementation commit, source hashes, HTML
hashes, screenshot hashes, browser versions and comparison results. It covers:

- 432 actual HTML renders: five non-browser lanes plus browser Wasm in three engines.
- 36 runtime screenshots: six lanes × three engines × two scroll positions.
- 30 comparisons against the six native-Rust reference screenshots.
- Three deliberately rejected one-pixel controls.

Images live in `generated/{chromium,firefox,webkit}/`; per-runtime HTML lives in
`generated/html/`. Matching files are separate captures; Git deduplicates their
identical contents. `target/html-line-qa/` contains disposable intermediate data.

Java is explicitly excluded. It is maintained in
[`roastedroot/lumis4j`](https://github.com/roastedroot/lumis4j), outside this
repository. Its current bridge pins Lumis 0.9.0 and exposes neither multi-theme
formatting nor line-number/highlight options, so it cannot exercise this PR's
current implementation or this matrix.
