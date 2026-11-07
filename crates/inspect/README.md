# Lumis inspector

A window for working out why a token has the colour it has: pick a sample and a
theme, put the caret on a byte, and see the captures over it, the theme keys they
resolved to, the painted style and its contrast, and the syntax tree.

```sh
mise run inspect
```

Headless renders, for screenshots and checks:

```sh
cargo run --release --manifest-path crates/inspect/Cargo.toml -- --list-scenes
cargo run --release --manifest-path crates/inspect/Cargo.toml -- --scene injection --snap inspect.png --size 1440x900
```

[CONTRIBUTING.md](../../CONTRIBUTING.md#inspecting-a-highlight) describes what each part of
the window shows.
