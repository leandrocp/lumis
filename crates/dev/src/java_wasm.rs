use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

const TARGET: &str = "wasm32-wasip1";
const PROFILE: &str = "java-wasm";
const TARGET_DIR: &str = "target";
const OUTPUT: &str = "packages/java/lumis/lumis.wasm";

/// Builds the module the Java package compiles to bytecode from the workspace `lumis` crate.
pub(crate) fn build() -> Result<()> {
    let sdk = super::wasi_sdk::ensure()?;
    let target_dir = Path::new(TARGET_DIR).join(TARGET).join(PROFILE);
    let compiled = target_dir.join("lumis_java_wasm.wasm");
    let initialized = target_dir.join("lumis_java_wasm.initialized.wasm");

    run(Command::new("cargo")
        .args(["build", "-p", "lumis-java-wasm"])
        .args(["--target", TARGET, "--profile", PROFILE])
        .args(["--target-dir", TARGET_DIR])
        .env("WASI_SDK_PATH", &sdk)
        .env("WASI_SYSROOT", sdk.join("share/wasi-sysroot"))
        .env("CC_wasm32_wasip1", sdk.join("bin/clang"))
        .env("CXX_wasm32_wasip1", sdk.join("bin/clang++"))
        .env("AR_wasm32_wasip1", sdk.join("bin/llvm-ar")))?;
    // Runs `wizer.initialize`, so no grammar is set up when the JVM loads the module.
    run(Command::new("wizer")
        .arg("--allow-wasi")
        .arg(&compiled)
        .arg("-o")
        .arg(&initialized))?;
    run(Command::new("wasm-opt")
        .args(["-Oz", "--strip-debug"])
        .arg(&initialized)
        .arg("-o")
        .arg(OUTPUT))?;
    println!("* wrote {OUTPUT}");
    Ok(())
}

fn run(command: &mut Command) -> Result<()> {
    let status = command
        .status()
        .with_context(|| format!("failed to run: {command:?}"))?;
    if !status.success() {
        bail!("command failed: {command:?}");
    }
    Ok(())
}
