use crate::ThemeDefinition;
use anyhow::{ensure, Context, Result};
use lumis_build::license::{is_copyleft, license_dir, LicenseDetector, NO_ASSERTION, NO_LICENSE};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Where theme extraction copies each plugin's license files.
pub(crate) const LICENSES_DIR: &str = "themes/licenses";

pub(crate) struct ResolvedTheme<'a> {
    pub(crate) name: &'a str,
    pub(crate) url: &'a str,
    revision: Option<String>,
    pub(crate) license: String,
    license_files: Vec<String>,
}

/// Detects the license of a plugin from the files extraction copied for it.
pub(crate) fn detect(detector: &LicenseDetector, licenses_dir: &Path, url: &str) -> Result<String> {
    let dir = license_dir(licenses_dir, url).with_context(|| format!("invalid theme URL {url}"))?;
    detector
        .detect_dir(&dir)
        .with_context(|| format!("failed to read {}", dir.display()))
}

/// Pairs every theme with its recorded license and pinned revision, and fails
/// when a recorded license is not the one its license files hold, when a
/// built-in theme is copyleft or has no license, or when a theme JSON or a
/// license directory belongs to no theme.
pub(crate) fn resolve<'a>(
    definitions: &'a [ThemeDefinition],
    themes_dir: &Path,
    licenses_dir: &Path,
) -> Result<Vec<ResolvedTheme<'a>>> {
    let detector = LicenseDetector::new().context("failed to load the SPDX license list")?;
    let mut detected: BTreeMap<&str, String> = BTreeMap::new();
    let mut problems = Vec::new();
    let mut themes = Vec::new();

    for definition in definitions {
        let path = themes_dir.join(format!("{}.json", definition.name));
        let (revision, recorded) = read_theme_fields(&path)?;
        let expected = if let Some(license) = detected.get(definition.url.as_str()) {
            license.clone()
        } else {
            let license = detect(&detector, licenses_dir, &definition.url)?;
            detected.insert(&definition.url, license.clone());
            license
        };
        if let Some(problem) = check_license(&definition.name, recorded.as_deref(), &expected) {
            problems.push(problem);
        }
        themes.push(ResolvedTheme {
            name: &definition.name,
            url: &definition.url,
            revision,
            license: recorded.unwrap_or(expected),
            license_files: license_files(&license_dir(licenses_dir, &definition.url)?)?,
        });
    }

    problems.extend(unlisted_themes(definitions, themes_dir)?);
    problems.extend(orphaned_license_dirs(definitions, licenses_dir)?);

    ensure!(
        problems.is_empty(),
        "theme licenses are not valid:\n- {}",
        problems.join("\n- ")
    );
    Ok(themes)
}

/// The built-in theme set accepts only a license its files hold that is
/// neither copyleft nor missing. `lumis gen-theme` applies none of this.
fn check_license(name: &str, recorded: Option<&str>, detected: &str) -> Option<String> {
    let Some(recorded) = recorded else {
        return Some(format!(
            "{name} has no license; run `mise run themes-gen {name}`"
        ));
    };
    if recorded != detected {
        return Some(format!(
            "{name} records {recorded}, but its license files hold {detected}; run `mise run themes-gen {name}`"
        ));
    }
    if recorded == NO_LICENSE {
        return Some(format!("{name} comes from a plugin without a license"));
    }
    if recorded == NO_ASSERTION {
        return Some(format!(
            "{name} has a license file that matches no known license"
        ));
    }
    if is_copyleft(recorded) {
        return Some(format!(
            "{name} is {recorded}, a copyleft license Lumis cannot bundle"
        ));
    }
    None
}

fn read_theme_fields(path: &Path) -> Result<(Option<String>, Option<String>)> {
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let theme: serde_json::Value = serde_json::from_str(&source)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    let field = |key: &str| {
        theme
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
    };
    Ok((field("revision"), field("license")))
}

fn unlisted_themes(definitions: &[ThemeDefinition], themes_dir: &Path) -> Result<Vec<String>> {
    let listed: BTreeSet<&str> = definitions.iter().map(|d| d.name.as_str()).collect();
    let mut problems = Vec::new();
    for entry in fs::read_dir(themes_dir)? {
        let path = entry?.path();
        if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            let name = path.file_stem().unwrap_or_default().to_string_lossy();
            if !listed.contains(name.as_ref()) {
                problems.push(format!(
                    "{} has no entry in themes/themes.lua",
                    path.display()
                ));
            }
        }
    }
    problems.sort();
    Ok(problems)
}

fn orphaned_license_dirs(
    definitions: &[ThemeDefinition],
    licenses_dir: &Path,
) -> Result<Vec<String>> {
    let expected: BTreeSet<PathBuf> = definitions
        .iter()
        .map(|definition| license_dir(licenses_dir, &definition.url))
        .collect::<Result<_, _>>()?;
    let mut found = BTreeSet::new();
    collect_file_dirs(licenses_dir, &mut found)?;
    Ok(found
        .difference(&expected)
        .map(|dir| format!("{} belongs to no theme", dir.display()))
        .collect())
}

fn collect_file_dirs(dir: &Path, found: &mut BTreeSet<PathBuf>) -> Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            collect_file_dirs(&entry.path(), found)?;
        } else {
            found.insert(dir.to_path_buf());
        }
    }
    Ok(())
}

fn license_files(dir: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                files.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
    }
    files.sort();
    Ok(files)
}

pub(crate) fn render_notice(themes: &[ResolvedTheme<'_>]) -> String {
    let mut lines = vec![
        "# Theme Notices".to_string(),
        String::new(),
        "> This file is auto-generated by `mise run docs-gen-themes-md` from `themes/themes.lua` and the revision and license recorded in each theme. Do not edit.".to_string(),
        String::new(),
        "Lumis themes are color values extracted from the Neovim colorscheme plugins below, at the commit listed for each theme. Each plugin keeps its own license, which is not the license of Lumis. The license is detected from the plugin's license file during extraction; follow the license link for the full terms and copyright notice.".to_string(),
        String::new(),
        "## Themes".to_string(),
        String::new(),
        "| Theme | Source | Commit | License |".to_string(),
        "|-------|--------|--------|---------|".to_string(),
    ];

    for theme in themes {
        let revision = theme.revision.as_deref();
        lines.push(format!(
            "| `{}` | {} | {} | {} |",
            theme.name,
            repo_link(theme.url),
            revision.map_or_else(
                || "Not recorded".to_string(),
                |rev| format!(
                    "[`{}`]({}/tree/{rev})",
                    &rev[..rev.len().min(12)],
                    theme.url
                )
            ),
            license_link(theme, revision.unwrap_or("HEAD")),
        ));
    }

    lines.push(String::new());
    lines.join("\n")
}

fn repo_link(url: &str) -> String {
    let name = url.strip_prefix("https://github.com/").unwrap_or(url);
    format!("[{name}]({url})")
}

fn license_link(theme: &ResolvedTheme<'_>, revision: &str) -> String {
    match theme.license_files.first() {
        Some(file) => format!("[{}]({}/blob/{revision}/{file})", theme.license, theme.url),
        None => theme.license.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_a_permissive_license_its_files_hold() {
        assert_eq!(check_license("t", Some("MIT"), "MIT"), None);
        assert_eq!(
            check_license("t", Some("Apache-2.0 AND MIT"), "Apache-2.0 AND MIT"),
            None
        );
    }

    #[test]
    fn rejects_a_theme_without_a_license_field() {
        assert!(check_license("t", None, "MIT")
            .unwrap()
            .contains("has no license"));
    }

    #[test]
    fn rejects_a_license_its_files_do_not_hold() {
        assert!(check_license("t", Some("MIT"), "Apache-2.0")
            .unwrap()
            .contains("records MIT, but its license files hold Apache-2.0"));
    }

    #[test]
    fn rejects_copyleft_and_missing_licenses() {
        for license in ["GPL-3.0", "LGPL-3.0", "MIT AND GPL-2.0"] {
            assert!(check_license("t", Some(license), license)
                .unwrap()
                .contains("copyleft"));
        }
        assert!(check_license("t", Some(NO_LICENSE), NO_LICENSE)
            .unwrap()
            .contains("without a license"));
        assert!(check_license("t", Some(NO_ASSERTION), NO_ASSERTION)
            .unwrap()
            .contains("matches no known license"));
    }

    #[test]
    fn reports_license_dirs_and_theme_files_no_theme_uses() {
        let root =
            std::env::temp_dir().join(format!("lumis-theme-licenses-{}", std::process::id()));
        let licenses = root.join("licenses");
        fs::create_dir_all(licenses.join("a").join("a")).unwrap();
        fs::create_dir_all(licenses.join("gone").join("gone")).unwrap();
        fs::write(licenses.join("a").join("a").join("LICENSE"), "x").unwrap();
        fs::write(licenses.join("gone").join("gone").join("LICENSE"), "x").unwrap();
        fs::write(root.join("a.json"), "{}").unwrap();
        fs::write(root.join("stray.json"), "{}").unwrap();
        let definitions = [ThemeDefinition {
            name: "a".to_string(),
            url: "https://github.com/a/a".to_string(),
        }];

        let orphans = orphaned_license_dirs(&definitions, &licenses).unwrap();
        let unlisted = unlisted_themes(&definitions, &root).unwrap();
        fs::remove_dir_all(&root).unwrap();

        assert_eq!(orphans.len(), 1);
        assert!(orphans[0].contains("gone"));
        assert_eq!(unlisted.len(), 1);
        assert!(unlisted[0].contains("stray.json"));
    }
}
