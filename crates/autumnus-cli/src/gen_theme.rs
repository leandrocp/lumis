use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn workspace_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn generate_theme(
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

    let json_path = temp_path.join(format!("{}.json", colorscheme));
    let json_content = fs::read_to_string(&json_path)
        .context(format!("Failed to read generated JSON at {:?}", json_path))?;

    if let Some(output_path) = output {
        fs::write(output_path, &json_content)
            .context(format!("Failed to write output to {}", output_path))?;
        eprintln!("✓ Theme saved to {}", output_path);
    } else {
        println!("{}", json_content);
    }

    Ok(())
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
			vim.o.background = "{}"
			{}
			vim.cmd([[colorscheme {}]])
		end,"#,
            appearance, setup_code, colorscheme
        )
    } else {
        format!(
            r#"config = function()
			vim.o.background = "{}"
			vim.cmd([[colorscheme {}]])
		end,"#,
            appearance, colorscheme
        )
    };

    let themes_content = format!(
        r#"return {{
	{{
		url = "{}",
		name = "{}",
		{}
	}},
}}
"#,
        url, colorscheme, config_fn
    );

    fs::write(temp_path.join("themes.lua"), themes_content)
        .context("Failed to write themes.lua")?;

    Ok(())
}

fn copy_extract_theme_lua(temp_path: &std::path::Path) -> Result<()> {
    let extract_theme_path = workspace_root().join("themes").join("extract_theme.lua");

    let content = fs::read_to_string(&extract_theme_path)
        .context("Failed to read themes/extract_theme.lua")?;

    fs::write(temp_path.join("extract_theme.lua"), content)
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

        anyhow::bail!(
            "Neovim theme extraction failed:\nstdout: {}\nstderr: {}",
            stdout,
            stderr
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_init_lua() {
        let temp_dir = TempDir::new().unwrap();
        let result = create_init_lua(temp_dir.path());

        assert!(result.is_ok());
        let init_path = temp_dir.path().join("init.lua");
        assert!(init_path.exists());

        let content = fs::read_to_string(init_path).unwrap();
        assert!(content.contains("vim.env.XDG_DATA_HOME"));
        assert!(content.contains("vim.opt.termguicolors = true"));
    }

    #[test]
    fn test_create_themes_lua_without_setup() {
        let temp_dir = TempDir::new().unwrap();
        let result = create_themes_lua(
            temp_dir.path(),
            "https://github.com/test/theme",
            "test_theme",
            None,
            None,
        );

        assert!(result.is_ok());
        let themes_path = temp_dir.path().join("themes.lua");
        assert!(themes_path.exists());

        let content = fs::read_to_string(themes_path).unwrap();
        assert!(content.contains("url = \"https://github.com/test/theme\""));
        assert!(content.contains("name = \"test_theme\""));
        assert!(content.contains("vim.o.background = \"dark\""));
        assert!(content.contains("colorscheme test_theme"));
    }

    #[test]
    fn test_create_themes_lua_with_setup() {
        let temp_dir = TempDir::new().unwrap();
        let result = create_themes_lua(
            temp_dir.path(),
            "https://github.com/test/theme",
            "test_theme",
            Some("require('test').setup()"),
            Some("light"),
        );

        assert!(result.is_ok());
        let themes_path = temp_dir.path().join("themes.lua");
        let content = fs::read_to_string(themes_path).unwrap();

        assert!(content.contains("vim.o.background = \"light\""));
        assert!(content.contains("require('test').setup()"));
    }

    #[test]
    fn test_copy_extract_theme_lua() {
        let temp_dir = TempDir::new().unwrap();
        let result = copy_extract_theme_lua(temp_dir.path());

        assert!(result.is_ok());
        let extract_path = temp_dir.path().join("extract_theme.lua");
        assert!(extract_path.exists());

        let content = fs::read_to_string(extract_path).unwrap();
        assert!(!content.is_empty());
    }
}
