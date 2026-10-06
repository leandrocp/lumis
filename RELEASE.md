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
10. npm-vite                     needs 9
11. npm-react                    needs 7
12. npm-cli
13. hex-lumis                    needs 1 and 3 published to crates.io
```

Merge order is publish order; nothing enforces it. This is every releasable package —
the list itself lives in `mise run release-packages`.

`@lumis-sh/wasm-bundle-*` is not on that list and has no tag. A bundle is a
manifest and an import list derived from `languages.toml`, so it is generated at
publish time like the parser packages and versioned the same way: its dependency
list is its identity, and `wasm-release.yml` gives it the next patch whenever
that list changes. Nothing to bump and nothing to remember.

- `npm-lumis` and `npm-cli` publish their `@lumis-sh/lumis-native-*` / `@lumis-sh/cli-*`
  platform packages first, at the same version. `release-prepare` bumps them together
  and `mise run lint` fails on drift. `npm-cli` need not match `cargo-lumis-cli`.
- `npm-lumis` also updates the comparison manifest's version label. This changes
  metadata only; highlighting changes must regenerate the comparison before release.
- `hex-lumis` goes last — see [Elixir package](#elixir-package).
- `@lumis-sh/wasm-*` parser packages are outside this flow: `wasm-release.yml` publishes
  them on any push to `main` that touches parsers, queries or the package templates.
  `mise run wasm-publish-needed` lists pending ones.
- A change to what a parser or bundle package contains bumps `PACKAGE_FORMAT_VERSION`
  or `BUNDLE_FORMAT_VERSION` in `crates/dev`, and merging it republishes every parser
  or bundle on npm. That happens on the push to `main`, before any runtime release that
  reads the new format can ship, so the order takes care of itself. Let that WASM
  Release run finish before merging `npm-lumis`.
- Hex gets a release only when its own package changes: a new parser or queries, a
  new dependency in `mix.exs`, or a bundle's members. A format change that touches
  only the npm entry leaves Hex on the version it has, so Hex and npm versions can
  differ. When a format change does reach what Hex ships (`priv/parsers`, `mix.exs`,
  the README), raise `HEX_FORMAT_VERSION` to the new `PACKAGE_FORMAT_VERSION` too.
- An ordinary `0.26.x` WASM release needs no runtime release. Moving to a new
  Tree-sitter minor series in `mise.toml` does.
- `parser-sizes.json` is measured off what is published, so it lags a parser
  release by design. Run `mise run wasm-sizes` after one lands to refresh the
  `Size` and `Memory` columns in the language catalog. It downloads only what it
  has not already measured at that version, so the sweep after one parser
  release fetches that one parser; name a parser to narrow it further.

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

### Changelog template

`release-prepare` uses git-cliff 2.14.2 with the shared
[`release-notes.tera`](https://github.com/leandrocp/github-actions/blob/938d95526b75be259407b02aa1813d455033ffd5/git-cliff/release-notes.tera)
from `leandrocp/github-actions`. `mise run release-template` downloads that
revision into `tmp/release-templates/` on first use, then reuses the cached file.
The same task runs locally and in CI, before any version files change.

New entries prefer PR titles and PR authors, include scopes and breaking-change
markers, and follow the section order in `cliff.toml`. Entries without a PR title
use the commit message. Author credits require a PR author or commit-author
username from GitHub; without either, the entry has no author credit.
Credits omit `leandrocp`, while retaining PR links and external contributor
credits. The task inserts a blank line between the new release and existing
changelog history, preserving the history verbatim. Commit filters, package tag
patterns, and version-bump rules remain local to Lumis.

To update the template, change `revision` in the `release-template` task to a
published commit of `github-actions` and update the link above. Preview a package
before preparing its release:

```sh
template="$(mise run release-template)"
mise run release-cliff -- npm-themes --body-file "$template" --github-repo leandrocp/lumis --tag-pattern 'npm-themes/v[0-9].*' --include-path 'packages/javascript/themes/**/*' --unreleased
```

Planning, release detection, and changelogs use `release-cliff` so they agree on
which changes are breaking for each package. The Rust toolchain upgrade in
[#1702](https://github.com/leandrocp/lumis/pull/1702) is breaking for Cargo packages;
npm and Hex list it under Dependencies without a breaking marker or minor bump.
Other breaking changes retain their classification. `mise run test-release`
checks both cases.

## No pull request for a package?

`mise run release-plan` skipped it because one of:

- Nothing to bump — only `chore` or `build(deps)` commits since its tag. Releasing on
  a dependency bump is a judgement call; prepare it by hand. The generated npm CLI
  binary update is an exception: it triggers a patch release even with `chore`.
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

`release-prepare` updates the crate version and all dependent requirements through
`crates/dev`, including registry dependencies such as the Elixir NIF's. Only then
does it run Cargo's resolver to refresh the workspace lockfile. `cargo set-version`
cannot do this: it updates path dependencies and resolves before the NIF's
requirement changes, which breaks a release that moves Tree-sitter versions.
Commit every manifest and lockfile the preparation touches.

Nothing built here resolves these requirements except `crates/autumnus`, so
`check-crate-deps` is the only thing that can see drift. That is how
[#1118](https://github.com/leandrocp/lumis/issues/1118) shipped: `lumis` 0.12.1 called
`lumis-core` 2.2.0 API while requiring `"2"`.

## Publishing to npm

No token. The release workflows request `id-token: write` and npm exchanges that
OIDC identity for a short-lived publish token, which the registry grants only
because the package names the workflow that asked. `--provenance` was always the
same identity being used for attestation; trusted publishing keeps it and drops
the long-lived credential that sat beside it.

This is not a preference. Write-enabled granular tokens have been capped at 90
days since September 2025, classic tokens stopped being issued in November 2025,
and publishing a new version directly with a token is removed in January 2027 —
which is every `npm publish` here. Direct publish stays available to a trusted
publisher; only the token path goes away.

Trust is configured **per package**, and this repository publishes 142:

| | count | workflow |
| --- | --- | --- |
| `@lumis-sh/wasm-*` parsers | 113 | `wasm-release.yml` |
| `@lumis-sh/lumis-native-*` and the selector | 9 | `javascript-release.yml` |
| `@lumis-sh/cli-*` platform packages | 8 | `javascript-release.yml` |
| `@lumis-sh/wasm-bundle-*` | 5 | `wasm-release.yml` |
| `lumis`, `cli`, `themes`, `react`, `markdown-it-lumis`, `rehype-lumis`, `vite` | 7 | `javascript-release.yml` |

```sh
mise run npm-trust
```

That visits each one and skips those already configured. Both lists are derived
rather than written down — the parsers and bundles from `languages.toml`, the
rest from `mise run release-packages` and the platform package directories — so
a package added to either cannot be forgotten. `mise run npm-packages` prints
what it will visit.

A package that *moves* between workflows is the case the skip does not cover.
Trust is stored per package on npm, so the old workflow keeps its entry and the
new one has none until this runs; publishing from the new one answers `404 Not
Found` on the PUT, which reads like the package does not exist. Run
`mise run npm-trust` after changing which workflow publishes something.

Run it locally, in a terminal. Being logged in is not enough: reading and
writing a trusted publisher both need a 2FA challenge, npm refuses a bypass-2FA
token, and it will not prompt at all unless **stdout is a terminal** — so this
cannot run in CI, under a pipe, or with its output redirected to a file.

Expect to answer the challenge several times. npm's approval lasts about five
minutes and the calls are spaced two seconds apart as npm asks, so a full pass
over 142 packages outlives it. The task notices and re-prompts rather than
failing.

Two traps:

- **`--allow-publish` has to be explicit.** Configurations created after
  3 September 2026 default to `npm stage publish`, and that date has passed.
  Without the flag the configuration looks right and every publish fails on
  permissions.
- **npm 11.15.0 or later locally**, or `--allow-publish` never reaches the
  registry and the call fails with a `400` and no body.

The runners pin `npm@12` for the same family of reason. OIDC publishing needs
11.5.1 at minimum and Node 24 LTS bundles 11.19.0, so the pin is about knowing
which client publishes rather than reaching a floor — `lts/*` moves, and
`pnpm publish` shells out to whichever `npm` is on `PATH` rather than doing the
exchange itself.

It is the major rather than a version so there is nothing to maintain between
majors. npm publishes no `lts` or `stable` tag, and `latest` would let the next
major arrive in the middle of a release — npm 12 blocked dependency lifecycle
scripts by default and made unknown CLI flags throw, neither of which touches
this path, but neither of which was announced here either.

### What the trust relationship does and does not pin

npm records two claims: the **repository** and the **workflow filename**. There
is no branch claim, so anything that gets `javascript-release.yml` or
`wasm-release.yml` to run in this repository with `id-token: write` can mint a
publish token. `npm trust` says so while configuring: *anyone with GitHub
repository write access can publish*. That was equally true of `NPM_TOKEN`,
which any branch's workflow could read, so this is not a new exposure — but it
is not one trusted publishing removes either.

What holds the line today is the `github.ref == 'refs/heads/main'` guard on the
publishing jobs. Note what that is worth: `workflow_dispatch` runs the workflow
file **from the ref you pick**, so the guard is only as good as the file on that
branch. npm's third, optional claim is a GitHub **environment**, which would
move the check into repository settings where a branch cannot rewrite it. It is
deliberately not used. Adding it later means revoking and re-adding all 142
configurations, so it is a decision to revisit as a whole, not per package.

Two things do limit the damage:

- **The token is scoped to one package and lives for minutes.** The exchange is
  `POST /-/npm/v1/oidc/token/exchange/package/<pkg>`, so a run publishing
  `@lumis-sh/themes` cannot touch `@lumis-sh/lumis`. `NPM_TOKEN` covered the
  whole `@lumis-sh` scope and did not expire.
- **`id-token: write` is kept out of jobs that run third-party code.** In
  `javascript-release.yml` the `stage` job installs dependencies, runs their
  lifecycle scripts and builds each package, then packs tarballs and stops; only
  `publish` can reach npm, and it holds no source tree. That split is the whole
  reason `stage` packs rather than publishes. `wasm-release.yml` is the same
  shape: each parser's pipeline in `wasm-release-parser.yml` has a `build` job
  that compiles the grammar and stages both packages with `contents: read` and
  nothing more, and a `publish-npm` job that publishes the artifact it
  uploaded. Neither a grammar's C nor a crate's build script is ever in a job
  that can publish. npm checks the calling workflow's filename for a reusable
  workflow, so `wasm-release.yml` stays the trusted publisher.

Neither workflow has a `pull_request` trigger, so a fork cannot reach any of
this.

### A brand-new package

npm has no equivalent of PyPI's pending publishers: a package must exist before
it can name a trusted publisher. So the first release of a genuinely new
package — a language added to `languages.toml`, a new platform — cannot be
configured in advance, and its first publish fails. Publish that one version by
hand, then:

```sh
mise run npm-trust @lumis-sh/wasm-<name>
```

Everything already on npm is unaffected; this is only about packages npm has
never seen.

## npm CLI

`npm-cli` owns its version and changelog. Shared Rust CLI changes publish through
`cargo-lumis-cli` first; once crates.io accepts them, `rust-release.yml` opens
`align-npm-cli` to update the package's `binaryVersion`. Merge that small pull request,
then `release-prepare.yml` opens the normal `npm-cli` release pull request. npm-only
changes continue through the normal flow without a Cargo release.

The alignment PR uses `chore(javascript): update npm CLI binary to <version>`.
`cliff.toml` explicitly includes that subject under Dependencies, so it triggers
an npm patch release and appears in the changelog. Other chores stay excluded.
`release-needed` uses the same filters as `release-plan` and `release-prepare`.

## CLI binaries

`cli-binary-release.yml` publishes macOS, glibc Linux, musl Linux, and Windows
archives to the `cargo-lumis-cli/v*` GitHub Release. Unix archives use `.tar.gz`;
Windows uses `.zip`. Every archive has a matching SHA-256 file and build
provenance attestation. The shell and PowerShell installers, Homebrew,
`cargo-binstall`, and mise's GitHub backend all consume those GitHub Release
assets. GitHub is the canonical binary host; the CLI artifacts are not mirrored
to R2.

### Source archives

CLI source releases use the `cargo-lumis-cli/v*` tags. See the
[source build instructions](docs/content/cli/install.mdx#building-from-source)
for package maintainers.

`.gitattributes` excludes the crate-local `languages.toml` and `queries`
symlinks from GitHub source archives so MSYS2 can extract them. Source builds
read the root files, which remain in the archive. Keep the links in Git:
Cargo uses them to package those files when publishing crates from a checkout.

`release-packages` includes `.gitattributes` in the `cargo-lumis-cli` paths, so
archive packaging fixes prepare a CLI patch release and appear in its changelog.
Release paths can name individual files or directories.

Rust CI extracts a source archive with MSYS2 and tests the CLI from that
directory using MSVC.

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
