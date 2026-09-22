# Deployment

Lumis downloads and compiles a parser the first time it is used. Warm the
languages your application needs at startup so no request pays that cost.

Call `Lumis.Languages.async_load/1` from your application's `start/2`:

```elixir
def start(_type, _args) do
  Lumis.Languages.async_load(~w(markdown elixir javascript rust css html comment))

  Supervisor.start_link(children(), strategy: :one_for_one, name: MyApp.Supervisor)
end
```

It returns immediately, so the boot never waits on the network, and the result
is deliberately not matched on: a warm-up must not be able to stop an
application from starting. Failures are logged, and highlighting still loads on
demand, so the worst case is the cost this moves off the first request coming
back rather than an outage.

Include the root languages and any injected languages, such as the languages in
Markdown code fences. The API accepts language names or bundles such as
`:bundle_web` and `:bundle_system`. Downloads run concurrently, and every
failure is reported instead of stopping at the first.

Use `Lumis.Languages.load/1` instead when you do want to wait — a release task,
or a smoke test that should fail if a parser is unreachable.

By default, Lumis writes parser metadata, verified WASM, and compiled modules to
its `priv/lumis` directory. For a release, configure an absolute writable or
persistent directory such as `config :lumis, data_dir: "/app/lumis"`.

If cache preparation belongs in an image build instead of application startup,
use the standalone CLI and copy the resulting directory into the runtime image:

```dockerfile
FROM node:22-bookworm-slim AS parsers
RUN npx --yes @lumis-sh/cli --data-dir /app/lumis languages download markdown elixir javascript rust css html comment

FROM debian:bookworm-20250428-slim
COPY --from=parsers --chown=nobody:root /app/lumis /app/lumis
```

`npx` is the shortest way to get the CLI for one command; the Node image is
only that stage's, and nothing from it lands in the runtime image.

Point the release at that directory with `config :lumis, data_dir: "/app/lumis"`
or `LUMIS_DATA_DIR=/app/lumis`. Keep the `async_load/1` call: against a prepared
directory it needs no network and only moves the parsers into the VM, which is
the half a prepared directory cannot do for you.

`Lumis.Languages.download/2` does the same preparation from Elixir rather than the
CLI, for a release task or a migration step that runs before the VM that serves.
Inside a running application prefer `async_load/1`, which keeps what it loads
instead of compiling and discarding it.

## Deploying with a lock

If your project has a `lumis-lock.toml`, one step changes and one trap opens.

The step: run `mix lumis.install` in the build. It downloads exactly what the
lock names and leaves a copy of the lock in the data directory.

The trap: a release ships `priv/` but not your project directory, so a released
node cannot find `lumis-lock.toml` by walking up from wherever it happens to
start. That copy in the data directory is the only lock it can see. Miss it and
the application boots with no lock and loads whatever a document names — the
behaviour the lock was added to remove, restored silently in production only.

So `data_dir` has to be the same absolute path in the build and at runtime:

```elixir
# config/runtime.exs
config :lumis, data_dir: System.get_env("LUMIS_DATA_DIR") || "/app/lumis"
```

```dockerfile
# Build: after config/runtime.exs, which is where data_dir is set.
ENV LUMIS_DATA_DIR="/app/lumis"
COPY lumis-lock.toml ./
RUN mix lumis.install

# Runtime: the parsers and the lock copy the build prepared.
ENV LUMIS_DATA_DIR="/app/lumis"
COPY --from=builder --chown=nobody:root /app/lumis /app/lumis
```

To check you got it right, run the image with no network at all. Highlighting
should work from the prepared directory, and a language the lock does not pin
should be refused rather than fetched:

```
$ docker run --network none ... bin/my_app rpc 'IO.inspect Lumis.Languages.load("haskell")'
{:error, :not_locked}
```

The CLI stage above is a different tool with a different store, and it has no
lock — `lumis languages download` fills a directory, it does not consult
`lumis-lock.toml` or check what it downloaded against it. Using both means the
set you download and the set your application may load are declared in two
places that nothing keeps in agreement. Prefer `mix lumis.install`, which reads
the one file that decides.

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

This NIF build cache is separate from the parser data directory described
above. The release contains the NIF; parsers still need an absolute writable
`data_dir` at runtime.
