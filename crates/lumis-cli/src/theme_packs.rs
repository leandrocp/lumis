use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub(crate) fn load(data_dir: &Path, explicit: &[PathBuf]) -> Result<()> {
    let executable = std::env::current_exe()?;
    let directory = executable.parent().context("executable has no parent")?;
    let relative = "theme-packs/copyleft/lumis-themes.json";
    let candidates = [
        data_dir.join(relative),
        directory.join(relative),
        directory.join("../share/lumis").join(relative),
    ];
    let mut contents = Vec::new();
    for path in candidates
        .iter()
        .filter(|path| path.exists())
        .chain(explicit)
    {
        contents.push(
            std::fs::read_to_string(path)
                .with_context(|| format!("cannot read theme pack {}", path.display()))?,
        );
    }
    let sources = contents.iter().map(String::as_str).collect::<Vec<_>>();
    lumis_core::themes::install_packs(&sources)?;
    Ok(())
}
