#!/bin/sh

set -eu

repository="https://github.com/leandrocp/lumis"
releases_api="https://api.github.com/repos/leandrocp/lumis/releases?per_page=100"

fail() {
  printf 'lumis installer: %s\n' "$*" >&2
  exit 1
}

latest_version() {
  releases="$(curl --proto '=https' --tlsv1.2 -LsSf \
    -H 'Accept: application/vnd.github+json' \
    -H 'X-GitHub-Api-Version: 2022-11-28' \
    "$releases_api")"
  printf '%s\n' "$releases" |
    sed -n 's/.*"tag_name": "cargo-lumis-cli\/v\([^"]*\)".*/\1/p' |
    head -n 1
}

detect_target() {
  case "$(uname -m)" in
    arm64 | aarch64) architecture="aarch64" ;;
    x86_64 | amd64) architecture="x86_64" ;;
    *) fail "unsupported architecture: $(uname -m)" ;;
  esac

  case "$(uname -s)" in
    Darwin) printf '%s-apple-darwin\n' "$architecture" ;;
    Linux)
      libc="gnu"
      if { command -v ldd >/dev/null 2>&1 && ldd --version 2>&1 | grep -qi musl; } ||
        find /lib /usr/lib -maxdepth 1 -name 'ld-musl-*.so.1' -print -quit 2>/dev/null | grep -q .; then
        libc="musl"
      fi
      printf '%s-unknown-linux-%s\n' "$architecture" "$libc"
      ;;
    *) fail "unsupported operating system: $(uname -s)" ;;
  esac
}

verify_checksum() {
  expected="$(awk -v archive="$archive" '$2 == archive { print $1 }' "$checksum_file")"
  [ -n "$expected" ] || fail "checksum for $archive is missing"

  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$archive_path" | awk '{ print $1 }')"
  elif command -v shasum >/dev/null 2>&1; then
    actual="$(shasum -a 256 "$archive_path" | awk '{ print $1 }')"
  else
    fail "sha256sum or shasum is required"
  fi

  [ "$actual" = "$expected" ] || fail "checksum verification failed for $archive"
}

add_to_path() {
  case ":${PATH:-}:" in
    *:"$install_dir":*) return ;;
  esac

  case "${SHELL:-}" in
    */zsh) profile="$HOME/.zshrc" ;;
    */bash) profile="$HOME/.bashrc" ;;
    */fish)
      profile="$HOME/.config/fish/config.fish"
      mkdir -p "${profile%/*}"
      escaped_dir="$(printf '%s' "$install_dir" | sed 's/[\\"$`]/\\&/g')"
      path_line="fish_add_path \"${escaped_dir}\""
      ;;
    *) profile="$HOME/.profile" ;;
  esac

  if [ -z "${path_line:-}" ]; then
    escaped_dir="$(printf '%s' "$install_dir" | sed 's/[\\"$`]/\\&/g')"
    path_line="export PATH=\"${escaped_dir}:\$PATH\""
  fi
  if [ ! -f "$profile" ] || ! grep -Fqx "$path_line" "$profile"; then
    printf '\n# Lumis\n%s\n' "$path_line" >>"$profile"
  fi
  printf 'Added %s to PATH in %s. Restart your shell to use it.\n' "$install_dir" "$profile"
}

version="${LUMIS_VERSION:-}"
[ -n "$version" ] || version="$(latest_version)"
[ -n "$version" ] || fail "could not find the latest CLI release"
version="${version#v}"
printf '%s\n' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$' ||
  fail "invalid CLI version: $version"

target="$(detect_target)"
archive="lumis-${target}.tar.gz"
tag="cargo-lumis-cli/v${version}"
download_base="${repository}/releases/download/${tag}"
install_dir="${LUMIS_INSTALL_DIR:-$HOME/.local/bin}"
temp_dir="$(mktemp -d)"
trap 'rm -rf "$temp_dir"' EXIT HUP INT TERM

archive_path="$temp_dir/$archive"
checksum_file="$temp_dir/lumis-${target}.sha256"

printf 'Downloading lumis %s for %s\n' "$version" "$target"
curl --proto '=https' --tlsv1.2 -LsSf -o "$archive_path" "$download_base/$archive"
curl --proto '=https' --tlsv1.2 -LsSf -o "$checksum_file" "$download_base/lumis-${target}.sha256"
verify_checksum

tar -xzf "$archive_path" -C "$temp_dir"
mkdir -p "$install_dir"
install -m 755 "$temp_dir/lumis" "$install_dir/lumis"

printf 'Installed lumis %s to %s/lumis\n' "$version" "$install_dir"
if [ "${LUMIS_NO_MODIFY_PATH:-0}" = "1" ]; then
  case ":${PATH:-}:" in
    *:"$install_dir":*) ;;
    *) printf 'Add %s to PATH to use lumis.\n' "$install_dir" ;;
  esac
else
  add_to_path
fi
