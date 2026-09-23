# Deployment

Parsers are dependencies. Add the languages your application highlights to
`mix.exs`, and `mix release` carries them the way it carries any dependency's
assets:

```elixir
defp deps do
  [
    {:lumis, "~> 0.8"},
    {:lumis_wasm_bundle_web, "~> 0.26"},
    {:lumis_wasm_elixir, "~> 0.26"},
    {:lumis_wasm_markdown, "~> 0.26"}
  ]
end
```

There is nothing to prepare and nothing to copy. No image stage that downloads
parsers, no directory to stage, and no network at boot or at render — the bytes
are inside the release, under `lib/lumis_wasm_elixir-0.26.3/priv/parsers/`.

```elixir
Lumis.Languages.load("haskell")
#=> {:error, :not_installed}
```

Include the languages a document can *inject*, not only the ones it names.
Markdown fences reach whatever language they label, HTML reaches `css` and
`javascript`, and Elixir reaches `comment`. A language you missed costs that
block its highlighting and nothing else — the page still renders.

## Docker

The Dockerfile `mix phx.gen.release --docker` generates needs no changes.
Parsers arrive with `mix deps.get` and travel inside the release, so the
standard build and runner stages already carry them.

Optionally, point the compiled-module cache at a writable path:

```dockerfile
ENV LUMIS_DATA_DIR="/app/lumis"
```

## Warm-up

Loading a parser still costs a WASM compile the first time. Move it off the
first request with `Lumis.Languages.async_load/1` from your application's
`start/2`:

```elixir
def start(_type, _args) do
  Lumis.Languages.async_load(~w(markdown elixir javascript rust css html comment))

  Supervisor.start_link(children(), strategy: :one_for_one, name: MyApp.Supervisor)
end
```

It returns immediately and the result is deliberately not matched on: a warm-up
must not be able to stop an application from starting. Failures are logged, and
highlighting still loads on demand, so the worst case is that the cost this
moves comes back.

Use `Lumis.Languages.load/1` instead when you do want to wait — a release task,
or a smoke test that should fail if a parser is missing.

## The compiled-module cache

`config :lumis, :data_dir` does not decide where parsers come from. It decides
where wasmtime keeps compiled modules:

```elixir
config :lumis, data_dir: "/app/lumis"   # or LUMIS_DATA_DIR
```

It defaults to the `lumis` application's own `priv/`, which a release owns, and
is created on first write. Point it somewhere writable and persistent and a
restart skips recompiling. Lose it and the first render of each language is
slower; no request fails. On a read-only filesystem it is never written.

## Build with Nix

A sandboxed Nix build cannot download the precompiled Lumis NIF while
`mixRelease` compiles dependencies. The failure may be reported as
`Error while downloading precompiled NIF: erofs` or `eacces` because
`rustler_precompiled` cannot create its cache under the builder's home
directory.

Download the NIF while `fetchMixDeps` has fixed-output network access, keep the
archive in that derivation, and point the release build at the cached copy:

```nix
let
  pname = "my_app";
  version = "0.1.0";
  src = ./.;

  mixDeps = beamPackages.fetchMixDeps {
    pname = "mix-deps-${pname}";
    inherit src version;
    hash = "sha256-...";
    mixEnv = "prod";

    postInstall = ''
      export RUSTLER_PRECOMPILED_GLOBAL_CACHE_PATH="$out/.rustler-precompiled"
      mix deps.compile nimble_options --no-deps-check
      mix deps.compile rustler_precompiled --no-deps-check
      mix deps.compile lumis --no-deps-check
      rm -f "$RUSTLER_PRECOMPILED_GLOBAL_CACHE_PATH"/metadata-*.exs
    '';
  };
in
beamPackages.mixRelease {
  inherit pname src version;
  mixFodDeps = mixDeps;
  mixEnv = "prod";

  preConfigure = ''
    export RUSTLER_PRECOMPILED_GLOBAL_CACHE_PATH="$MIX_DEPS_PATH/.rustler-precompiled"
  '';
}
```

The NIF archive becomes part of the `mixFodDeps` hash. That hash is now specific
to the Nix system, so provide one per system and update it when the dependency
set, Lumis version, or artifact-selection settings such as
`LUMIS_USE_LEGACY_ARTIFACTS` change. Use `preConfigure`, not `preBuild`:
`mixRelease` compiles dependencies during its configure phase.

For a portable Linux x86_64 release, export
`LUMIS_USE_LEGACY_ARTIFACTS=true` in both phases so the NIF does not require
AVX/FMA support from the runtime host.

Building the NIF from source is also possible with
`config :rustler_precompiled, :force_build, lumis: true`, but the Nix build must
then provide Rustler, a Rust toolchain, and an offline Cargo dependency source.
Prefetching the released NIF is usually simpler.

This NIF build cache is separate from the compiled-module cache described above,
and from parsers entirely: the release contains the NIF, and parsers are
dependencies inside it. Neither needs the network at runtime.
