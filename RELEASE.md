# Release

Merging a release pull request ships that package. Nothing else does.

## Ship

1. Open the [release pull requests](https://github.com/leandrocp/lumis/pulls?q=is%3Apr+is%3Aopen+head%3Arelease%2F).
   `release-prepare.yml` keeps one per package (`chore(release): <package> <version>`
   on `release/<package>`) up to date on every push to `main`.
2. Merge what you want to ship, one at a time, in publish order, waiting for each
   publish to finish.
3. `release-tag.yml` tags the merge commit `<package>/v<version>`; the tag publishes.

Not merged is not released.

## Publish order

```
 1. cargo-lumis-core
 2. cargo-lumis-build
 3. cargo-lumis-wasm-runtime     needs 1
 4. cargo-lumis                  needs 1, 2, 3
 5. cargo-lumis-cli              needs 1, 3
 6. npm-themes
 7. npm-lumis                    builds against 6
 8. npm-markdown-it-lumis        needs 7
 9. npm-rehype-lumis             needs 7
10. npm-react                    needs 7
11. npm-cli
12. npm-wasm-bundle-web
13. npm-wasm-bundle-web-extra
14. npm-wasm-bundle-system       12-16 need the @lumis-sh/wasm-* parser packages
15. npm-wasm-bundle-backend
16. npm-wasm-bundle-full
17. hex-lumis                    needs 1 and 3 published to crates.io
```

Merge order is publish order; nothing enforces it. This is every releasable package —
the list itself lives in `mise run release-packages`.

- `npm-lumis` and `npm-cli` publish their `@lumis-sh/lumis-native-*` / `@lumis-sh/cli-*`
  platform packages first, at the same version. `release-prepare` bumps them together
  and `mise run lint` fails on drift. `npm-cli` need not match `cargo-lumis-cli`.
- `hex-lumis` goes last — see [Elixir package](#elixir-package).
- `@lumis-sh/wasm-*` parser packages are outside this flow: `wasm-release.yml` publishes
  them on any push to `main` that touches parsers or queries.
  `mise run wasm-publish-needed` lists pending ones.
- An ordinary `0.26.x` WASM release needs no runtime release. Moving to a new
  Tree-sitter minor series in `mise.toml` does.

## By hand

```sh
mise run release-needed
mise run release-prepare npm-cli 0.7.0        # bare version, not v0.7.0
git add <every file it touched>
git commit -m "chore(release): npm-cli 0.7.0"
git push origin main
```

The push is the publish. One package per commit, one release per push —
`release-tag.yml` reads only the head commit.

The subject must be `chore(release): <package> <version>` and nothing else, bar the
`(#1234)` a squash merge appends. It is the whole interface: CI skips on it, and
`release-tag.yml` reads the package and version out of it.

Review every file `release-prepare` touches. Crate releases rewrite dependent
manifests too — see [Crate version requirements](#crate-version-requirements).

## No pull request for a package?

`mise run release-plan` skipped it because one of:

- Nothing to bump — only `chore` or `build(deps)` commits since its tag. Releasing on
  a dependency bump is a judgement call; prepare it by hand.
- The version file is already ahead of the tag — a merged release awaiting its tag.
- No `<package>/v*` tag exists — cut the first release by hand.

Below `1.0.0` only a breaking change bumps the minor; `feat` bumps the patch.

## Failed release

Nothing published yet: delete the failed tags, fix `main`, push corrected tags by hand
in publish order. No pull request reopens — the version file is ahead of the tag, which
is the state `release-plan` skips.

Already published: do not reuse the version. Cut a patch release.

## Crate version requirements

A `version` requirement on a lumis crate must equal that crate's version here, spelled
in full — `version = "2.5.0"`, never `"2"`, because `cargo set-version` only rewrites
requirements the new version falls outside of.

`release-prepare` keeps this current via `cargo set-version` plus
`mise run check-crate-deps --fix`, which catches registry-resolved manifests like the
Elixir NIF's. Commit every manifest they touch.

Nothing built here resolves these requirements except `crates/autumnus`, so
`check-crate-deps` is the only thing that can see drift. That is how
[#1118](https://github.com/leandrocp/lumis/issues/1118) shipped: `lumis` 0.12.1 called
`lumis-core` 2.2.0 API while requiring `"2"`.

## npm CLI

`npm-cli` owns its version and changelog. Shared Rust CLI changes publish through
`cargo-lumis-cli` first; once crates.io accepts them, `rust-release.yml` opens
`align-npm-cli` to update the package's `binaryVersion`. Merge that small pull request,
then `release-prepare.yml` opens the normal `npm-cli` release pull request. npm-only
changes continue through the normal flow without a Cargo release.

## Elixir package

`packages/elixir/lumis/native/lumis_nif/Cargo.lock` resolves `lumis-core` and
`lumis-wasm-runtime` from crates.io, so `hex-lumis` can only be prepared once those are
published. `mise run elixir-nif-lock-check --fix` refreshes it; `rust-release.yml` runs
it after each `cargo publish` and opens `refresh-nif-lock`. **Merge that before
`hex-lumis`.** A red `Standalone packaged NIF lockfile` on `main` means it has not
landed, and `hex-lumis` is `continue-on-error` in `release-prepare.yml` for the same
reason.

`elixir-release.yml` uploads NIFs to a GitHub Release and mirrors them to R2
(`artifacts.lumis.sh`); a missing `R2_*` secret fails the release. Checksums come from
GitHub, which `Lumis.Native.ArtifactURL` defaults to.

## CI on the release commit

Branch workflows skip a `chore(release):` head commit and any `release/*` pull request.

- **The pull request half keys on the branch, not the title, and must stay that way.**
  A title is contributor-controlled; keying on it would let anyone skip the whole matrix.
- Do not use `[skip ci]` — it keys on the same commit the tag points at, so it would
  suppress the publish too.
- A misspelled subject runs the full matrix against a bare version bump, and
  `Standalone packaged NIF lockfile` fails on a crates.io version that does not exist yet.
- Tag workflows and `release-prepare.yml` carry no guard and must not grow one.
  `crate-deps` in `rust.yml` is exempt on purpose — it reads manifests, not the registry.

## Tags

`<package>/v<version>` — `cargo-lumis-core/v2.5.0`, `npm-lumis/v0.7.1`, `hex-lumis/v0.3.0`.
Created with `CI_TOKEN`, not `GITHUB_TOKEN`, which raises no events and would never
start the publish.
