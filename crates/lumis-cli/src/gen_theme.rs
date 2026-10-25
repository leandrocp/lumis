use anyhow::{Context, Result};
use lumis_build::license;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

const EXTRACT_THEME_LUA: &str = include_str!(concat!(env!("OUT_DIR"), "/extract_theme.lua"));

pub(crate) fn generate_theme(
    url: &str,
    colorscheme: &str,
    setup: Option<&str>,
    output: Option<&str>,
    appearance: Option<&str>,
) -> Result<()> {
    let temp_dir = TempDir::new().context("Failed to create temporary directory")?;
    let temp_path = temp_dir.path();

    create_init_lua(temp_path)?;
    create_themes_lua(temp_path, url, colorscheme, setup, appearance)?;
    copy_extract_theme_lua(temp_path)?;

    run_nvim_extraction(temp_path, colorscheme)?;

    let json_path = temp_path.join(format!("{colorscheme}.json"));
    let json_content = fs::read_to_string(&json_path).context(format!(
        "Failed to read generated JSON at {}",
        json_path.display()
    ))?;
    let json_content = with_detected_license(temp_path, url, &json_content)?;

    if let Some(output_path) = output {
        fs::write(output_path, &json_content)
            .context(format!("Failed to write output to {output_path}"))?;
        eprintln!("Theme saved to {output_path}");
    } else {
        println!("{json_content}");
    }

    Ok(())
}

/// Records the license of the plugin the theme came from, whatever it is:
/// a theme generated here is the user's to redistribute or not.
fn with_detected_license(temp_path: &Path, url: &str, json: &str) -> Result<String> {
    let dir = license::license_dir(&temp_path.join("licenses"), url)
        .with_context(|| format!("Invalid theme URL {url}"))?;
    let detected = license::LicenseDetector::new()
        .context("Failed to load the SPDX license list")?
        .detect_dir(&dir)
        .with_context(|| format!("Failed to read license files in {}", dir.display()))?;
    license::set_theme_license(json, &detected).context("Generated theme JSON is invalid")
}

fn create_init_lua(temp_path: &std::path::Path) -> Result<()> {
    let init_content = r#"vim.env.XDG_DATA_HOME = "nvim/data"
vim.opt.termguicolors = true
vim.opt.runtimepath:prepend(vim.fn.stdpath("data") .. "/site")
"#;

    fs::write(temp_path.join("init.lua"), init_content).context("Failed to write init.lua")?;

    Ok(())
}

fn create_themes_lua(
    temp_path: &std::path::Path,
    url: &str,
    colorscheme: &str,
    setup: Option<&str>,
    appearance: Option<&str>,
) -> Result<()> {
    let appearance = appearance.unwrap_or("dark");

    let config_fn = if let Some(setup_code) = setup {
        format!(
            r#"config = function()
			vim.o.background = "{appearance}"
			{setup_code}
			vim.cmd([[colorscheme {colorscheme}]])
		end,"#
        )
    } else {
        format!(
            r#"config = function()
			vim.o.background = "{appearance}"
			vim.cmd([[colorscheme {colorscheme}]])
		end,"#
        )
    };

    let themes_content = format!(
        r#"return {{
	{{
		url = "{url}",
		name = "{colorscheme}",
		{config_fn}
	}},
}}
"#
    );

    fs::write(temp_path.join("themes.lua"), themes_content)
        .context("Failed to write themes.lua")?;

    Ok(())
}

fn copy_extract_theme_lua(temp_path: &std::path::Path) -> Result<()> {
    fs::write(temp_path.join("extract_theme.lua"), EXTRACT_THEME_LUA)
        .context("Failed to write extract_theme.lua to temp directory")?;

    Ok(())
}

fn run_nvim_extraction(temp_path: &std::path::Path, colorscheme: &str) -> Result<()> {
    let output = Command::new("nvim")
        .arg("--clean")
        .arg("--headless")
        .arg("-V3")
        .arg("-u")
        .arg("init.lua")
        .arg("-l")
        .arg("extract_theme.lua")
        .arg(colorscheme)
        .current_dir(temp_path)
        .output()
        .context("Failed to execute nvim")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        anyhow::bail!("Neovim theme extraction failed:\nstdout: {stdout}\nstderr: {stderr}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORKSPACE_EXTRACT_THEME_LUA: &str = include_str!("../../../themes/extract_theme.lua");

    #[test]
    fn embedded_extract_theme_script_matches_workspace_source() {
        assert_eq!(EXTRACT_THEME_LUA, WORKSPACE_EXTRACT_THEME_LUA);
    }

    #[test]
    fn copy_extract_theme_lua_writes_embedded_content() {
        let tmp = tempfile::tempdir().unwrap();

        copy_extract_theme_lua(tmp.path()).unwrap();

        let copied = fs::read_to_string(tmp.path().join("extract_theme.lua")).unwrap();

        assert_eq!(copied, EXTRACT_THEME_LUA);
    }
}
