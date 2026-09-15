use anyhow::{Context, Result};
use etcetera::BaseStrategy;
use serde::Deserialize;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct Config {
    pub highlight: HighlightConfig,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct HighlightConfig {
    pub theme: Option<String>,
}

/// `etcetera` resolves `XDG_CONFIG_HOME` on Unix and `%APPDATA%` on Windows, the
/// same base strategy [`lumis_wasm_runtime::store::default_data_dir`] uses.
///
/// Errors rather than panics when there is no home directory, since `--config`
/// and `LUMIS_CONFIG` both name a file without needing one.
pub(crate) fn default_path() -> Result<PathBuf> {
    Ok(etcetera::choose_base_strategy()
        .context("failed to determine the home directory holding config.toml")?
        .config_dir()
        .join("lumis")
        .join("config.toml"))
}

impl Config {
    pub(crate) fn load(path: &Path) -> Result<Self> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Self::default()),
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to read config file {}", path.display()));
            }
        };

        toml::from_str(&contents)
            .with_context(|| format!("failed to parse config file {}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `etcetera` owns the base directories; what is ours is choosing the config
    /// directory, naming the file under it, and preserving its platform-specific
    /// relationship with the data directory.
    #[test]
    fn default_path_is_lumis_config_toml_under_the_config_dir() {
        let strategy =
            etcetera::choose_base_strategy().expect("failed to determine home directory");

        let config_path = strategy.config_dir().join("lumis").join("config.toml");
        let data_path = strategy.data_dir().join("lumis").join("config.toml");

        assert_eq!(default_path().unwrap(), config_path);

        #[cfg(windows)]
        assert_eq!(config_path, data_path, "Windows uses %APPDATA% for both");

        #[cfg(not(windows))]
        assert_ne!(config_path, data_path);
    }
}
