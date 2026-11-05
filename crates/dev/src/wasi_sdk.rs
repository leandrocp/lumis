use anyhow::{bail, ensure, Context, Result};
use etcetera::BaseStrategy as _;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_dir as symlink;

/// What the Tree-sitter release in `crates/lumis-tree-sitter/upstream.toml` pins in
/// `crates/loader/wasi-sdk-version`.
pub(crate) const VERSION: &str = "34.0";

/// The SDK directory, downloaded beside the CLI's own in Tree-sitter's cache on first use.
pub(crate) fn ensure() -> Result<PathBuf> {
    let cache = etcetera::choose_base_strategy()?
        .cache_dir()
        .join("tree-sitter");
    let dir = cache.join(format!("wasi-sdk-{VERSION}"));
    if is_installed(&dir) {
        return Ok(dir);
    }

    let (platform, sha256) = host_archive()?;
    let major = VERSION.split('.').next().unwrap_or(VERSION);
    let url = format!(
        "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-{major}/wasi-sdk-{VERSION}-{platform}.tar.gz"
    );
    println!("* downloading {url}");
    let archive = super::download_bytes(&ureq::Agent::new_with_defaults(), &url)?;
    install(&archive, sha256, platform, &dir)?;
    Ok(dir)
}

fn is_installed(dir: &Path) -> bool {
    dir.join("bin/clang").exists()
}

fn host_archive() -> Result<(&'static str, &'static str)> {
    Ok(match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => (
            "x86_64-linux",
            "b761e3a0721dbae9c09a0059e5fdb2bf917d1b4a8a7b430fb3b5aafb0984b2c4",
        ),
        ("linux", "aarch64") => (
            "arm64-linux",
            "f7e243dff54d60bcc576e94d6166b69f410f2500ae4a9ceef34315be10e77971",
        ),
        ("macos", "x86_64") => (
            "x86_64-macos",
            "87d27fa8adc68dee59bfbf2e22a6d34ef717c34d6bf1d8af2a56fc929d9ce0eb",
        ),
        ("macos", "aarch64") => (
            "arm64-macos",
            "9c59398106b417f8f14913380fdf0097a8cc0ff4af9eb3ce0065a859e88d49e9",
        ),
        (os, arch) => bail!("no WASI SDK {VERSION} is pinned for {arch} {os}"),
    })
}

fn install(archive: &[u8], sha256: &str, platform: &str, dir: &Path) -> Result<()> {
    let actual = super::sha256_hex(archive);
    ensure!(
        actual == sha256,
        "WASI SDK {VERSION} for {platform} has SHA-256 {actual}, expected {sha256}"
    );

    let staging = dir.with_file_name(format!(".wasi-sdk-{VERSION}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&staging);
    tar::Archive::new(flate2::read::GzDecoder::new(archive))
        .unpack(&staging)
        .with_context(|| format!("failed to unpack the WASI SDK into {}", staging.display()))?;
    let root = staging.join(format!("wasi-sdk-{VERSION}-{platform}"));

    // The 0.26 CLI targets `wasm32-unknown-wasi`, a sysroot SDK 34 only ships as `wasm32-wasip1`.
    for kind in ["include", "lib"] {
        let link = root
            .join("share/wasi-sysroot")
            .join(kind)
            .join("wasm32-wasi");
        symlink("wasm32-wasip1", &link)
            .with_context(|| format!("failed to link {}", link.display()))?;
    }

    // Moved in whole, so an interrupted install never passes for an SDK.
    // A concurrent install of the same archive may have won; its SDK serves.
    let moved = fs::rename(&root, dir);
    if moved.is_err() && !is_installed(dir) {
        return moved.with_context(|| format!("failed to move {}", root.display()));
    }
    fs::remove_dir_all(&staging)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLATFORM: &str = "x86_64-linux";

    fn sdk_archive() -> Vec<u8> {
        let gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut tar = tar::Builder::new(gzip);
        for (path, contents) in [
            ("bin/clang", b"clang".as_slice()),
            (
                "share/wasi-sysroot/include/wasm32-wasip1/stdio.h",
                b"header",
            ),
            ("share/wasi-sysroot/lib/wasm32-wasip1/libc.a", b"library"),
        ] {
            let mut header = tar::Header::new_gnu();
            header.set_size(contents.len() as u64);
            header.set_mode(0o755);
            tar.append_data(
                &mut header,
                format!("wasi-sdk-{VERSION}-{PLATFORM}/{path}"),
                contents,
            )
            .unwrap();
        }
        tar.into_inner().unwrap().finish().unwrap()
    }

    fn scratch(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("lumis-wasi-sdk-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        root
    }

    #[test]
    fn the_sysroot_answers_to_the_target_the_cli_passes() {
        let root = scratch("alias");
        let dir = root.join("wasi-sdk");
        let archive = sdk_archive();

        let installed = install(
            &archive,
            &super::super::sha256_hex(&archive),
            PLATFORM,
            &dir,
        );
        let clang = dir.join("bin/clang").exists();
        let header = fs::read(dir.join("share/wasi-sysroot/include/wasm32-wasi/stdio.h"));
        let library = fs::read(dir.join("share/wasi-sysroot/lib/wasm32-wasi/libc.a"));
        let left = fs::read_dir(&root).map(Iterator::count);
        let _ = fs::remove_dir_all(&root);

        installed.unwrap();
        assert!(clang, "the archive's top directory must be stripped");
        assert_eq!(header.unwrap(), b"header");
        assert_eq!(library.unwrap(), b"library");
        assert_eq!(left.unwrap(), 1, "the staging directory must be cleaned up");
    }

    #[test]
    fn an_install_that_lost_the_race_accepts_the_winner() {
        let root = scratch("race");
        let dir = root.join("wasi-sdk");
        let archive = sdk_archive();
        let sha256 = super::super::sha256_hex(&archive);

        let winner = install(&archive, &sha256, PLATFORM, &dir);
        let loser = install(&archive, &sha256, PLATFORM, &dir);
        let clang = dir.join("bin/clang").exists();
        let left = fs::read_dir(&root).map(Iterator::count);
        let _ = fs::remove_dir_all(&root);

        winner.unwrap();
        loser.unwrap();
        assert!(clang);
        assert_eq!(
            left.unwrap(),
            1,
            "the loser must clean up its staging directory"
        );
    }

    #[test]
    fn a_target_that_is_not_an_sdk_is_still_an_error() {
        let root = scratch("occupied");
        let dir = root.join("wasi-sdk");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("stray"), b"stray").unwrap();
        let archive = sdk_archive();

        let installed = install(
            &archive,
            &super::super::sha256_hex(&archive),
            PLATFORM,
            &dir,
        );
        let _ = fs::remove_dir_all(&root);

        assert!(installed.is_err());
    }

    #[test]
    fn an_archive_that_misses_its_pin_installs_nothing() {
        let root = scratch("mismatch");
        let dir = root.join("wasi-sdk");

        let installed = install(&sdk_archive(), &"0".repeat(64), PLATFORM, &dir);
        let present = root.exists();
        let _ = fs::remove_dir_all(&root);

        assert!(installed.is_err());
        assert!(!present, "a rejected archive must not be unpacked");
    }
}
