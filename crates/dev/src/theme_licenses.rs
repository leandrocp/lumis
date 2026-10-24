use crate::ThemeDefinition;
use anyhow::{ensure, Context, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// SPDX's value for a source that grants no license at all.
const NO_LICENSE: &str = "NONE";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ThemeLicenses {
    sources: BTreeMap<String, SourceLicense>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceLicense {
    pub(crate) license: String,
    license_file: Option<String>,
    #[serde(default)]
    copyright: Vec<String>,
    note: Option<String>,
}

pub(crate) struct ResolvedTheme<'a> {
    pub(crate) name: &'a str,
    pub(crate) url: &'a str,
    revision: Option<String>,
    pub(crate) license: &'a SourceLicense,
}

pub(crate) fn read(path: &str) -> Result<ThemeLicenses> {
    let source = fs::read_to_string(path).with_context(|| format!("failed to read {path}"))?;
    toml::from_str(&source).with_context(|| format!("failed to parse {path}"))
}

/// Pairs every theme with its source's license and pinned revision, and fails
/// on a source without a license entry or an entry no theme uses.
pub(crate) fn resolve<'a>(
    themes: &'a [ThemeDefinition],
    licenses: &'a ThemeLicenses,
    themes_dir: &Path,
) -> Result<Vec<ResolvedTheme<'a>>> {
    validate(themes, licenses)?;

    themes
        .iter()
        .map(|theme| {
            Ok(ResolvedTheme {
                name: &theme.name,
                url: &theme.url,
                revision: read_revision(&themes_dir.join(format!("{}.json", theme.name)))?,
                license: &licenses.sources[&theme.url],
            })
        })
        .collect()
}

fn validate(themes: &[ThemeDefinition], licenses: &ThemeLicenses) -> Result<()> {
    let required: BTreeSet<&str> = themes
        .iter()
        .flat_map(|theme| std::iter::once(&theme.url).chain(&theme.dependencies))
        .map(String::as_str)
        .collect();
    let recorded: BTreeSet<&str> = licenses.sources.keys().map(String::as_str).collect();

    let missing: Vec<&str> = required.difference(&recorded).copied().collect();
    ensure!(
        missing.is_empty(),
        "themes/licenses.toml has no entry for: {}",
        missing.join(", ")
    );

    let unused: Vec<&str> = recorded.difference(&required).copied().collect();
    ensure!(
        unused.is_empty(),
        "themes/licenses.toml has entries no theme uses: {}",
        unused.join(", ")
    );

    let inconsistent: Vec<&str> = licenses
        .sources
        .iter()
        .filter(|(_, source)| {
            source.license.is_empty()
                || (source.license == NO_LICENSE) != source.license_file.is_none()
        })
        .map(|(url, _)| url.as_str())
        .collect();
    ensure!(
        inconsistent.is_empty(),
        "themes/licenses.toml entries need a license and a license_file, or license = \"{NO_LICENSE}\" and no file: {}",
        inconsistent.join(", ")
    );

    let excluded: Vec<&str> = licenses
        .sources
        .iter()
        .filter(|(_, source)| {
            !matches!(
                source.license.as_str(),
                "MIT" | "Apache-2.0" | "BSD-3-Clause" | "CC0-1.0" | "ISC"
            )
        })
        .map(|(url, _)| url.as_str())
        .collect();
    ensure!(
        excluded.is_empty(),
        "bundled themes require a reviewed permissive license: {}",
        excluded.join(", ")
    );

    Ok(())
}

pub(crate) fn verify_inventory(themes: &[ThemeDefinition]) -> Result<()> {
    let expected: BTreeSet<&str> = themes.iter().map(|theme| theme.name.as_str()).collect();
    ensure!(expected.len() > 200, "incomplete bundled theme catalog");
    for (directory, extension) in [
        ("themes", "json"),
        ("crates/lumis-core/themes", "json"),
        ("css", "css"),
        ("crates/lumis/css", "css"),
        ("packages/elixir/lumis/priv/static/css", "css"),
        ("packages/javascript/themes/themes", "ts"),
    ] {
        let files = fs::read_dir(directory)?.collect::<std::io::Result<Vec<_>>>()?;
        let actual: BTreeSet<String> = files
            .iter()
            .map(std::fs::DirEntry::path)
            .filter(|path| path.extension().is_some_and(|ext| ext == extension))
            .map(|path| path.file_stem().unwrap().to_string_lossy().into_owned())
            .collect();
        ensure!(
            expected == actual.iter().map(String::as_str).collect(),
            "theme inventory differs from themes/themes.lua in {directory}"
        );
    }
    Ok(())
}

fn read_revision(path: &Path) -> Result<Option<String>> {
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let theme: serde_json::Value = serde_json::from_str(&source)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(theme
        .get("revision")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string))
}

pub(crate) fn license_label(license: &str) -> &str {
    if license == NO_LICENSE {
        "None found"
    } else {
        license
    }
}

pub(crate) fn render_notice(
    themes: &[ResolvedTheme<'_>],
    definitions: &[ThemeDefinition],
    licenses: &ThemeLicenses,
) -> String {
    let mut lines = vec![
        "# Theme Notices".to_string(),
        String::new(),
        "> This file is auto-generated by `mise run docs-gen-themes-md` from `themes/themes.lua`, `themes/licenses.toml` and the revision recorded in each theme. Do not edit.".to_string(),
        String::new(),
        "Lumis themes are color values extracted from the Neovim colorscheme plugins below, at the commit listed for each theme. Each plugin keeps its own license, which is not the license of Lumis. Copyright lines are reproduced from each plugin's license file; follow the license link for the full terms.".to_string(),
        String::new(),
        "## Themes".to_string(),
        String::new(),
        "| Theme | Source | Commit | License | Copyright |".to_string(),
        "|-------|--------|--------|---------|-----------|".to_string(),
    ];

    for theme in themes {
        let revision = theme.revision.as_deref();
        lines.push(format!(
            "| `{}` | {} | {} | {} | {} |",
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
            license_link(theme.url, revision.unwrap_or("HEAD"), theme.license),
            copyright_cell(theme.license),
        ));
    }

    let dependencies = dependency_users(definitions);
    if !dependencies.is_empty() {
        lines.extend([
            String::new(),
            "## Extraction dependencies".to_string(),
            String::new(),
            "Some themes are loaded through another plugin during extraction. These plugins are not pinned, so their licenses link to the default branch.".to_string(),
            String::new(),
            "| Plugin | Used by | License | Copyright |".to_string(),
            "|--------|---------|---------|-----------|".to_string(),
        ]);
        for (url, users) in &dependencies {
            let license = &licenses.sources[*url];
            let users: Vec<String> = users.iter().map(|name| format!("`{name}`")).collect();
            lines.push(format!(
                "| {} | {} | {} | {} |",
                repo_link(url),
                users.join(", "),
                license_link(url, "HEAD", license),
                copyright_cell(license),
            ));
        }
    }

    let notes: Vec<String> = licenses
        .sources
        .iter()
        .filter_map(|(url, source)| {
            let note = source.note.as_deref()?;
            Some(format!("- {}: {note}", repo_link(url)))
        })
        .collect();
    if !notes.is_empty() {
        lines.extend([String::new(), "## Notes".to_string(), String::new()]);
        lines.extend(notes);
    }

    lines.push(String::new());
    lines.join("\n")
}

fn dependency_users(definitions: &[ThemeDefinition]) -> BTreeMap<&str, BTreeSet<&str>> {
    let mut users: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for theme in definitions {
        for dependency in &theme.dependencies {
            users
                .entry(dependency.as_str())
                .or_default()
                .insert(&theme.name);
        }
    }
    users
}

fn repo_link(url: &str) -> String {
    let name = url.strip_prefix("https://github.com/").unwrap_or(url);
    format!("[{name}]({url})")
}

fn license_link(url: &str, revision: &str, license: &SourceLicense) -> String {
    let label = license_label(&license.license);
    match &license.license_file {
        Some(file) => format!("[{label}]({url}/blob/{revision}/{file})"),
        None => label.to_string(),
    }
}

fn copyright_cell(license: &SourceLicense) -> String {
    if license.license == NO_LICENSE {
        return "No license file".to_string();
    }
    if license.copyright.is_empty() {
        return "Not stated in the license file".to_string();
    }
    license
        .copyright
        .iter()
        .map(|line| line.replace('|', "\\|"))
        .collect::<Vec<_>>()
        .join("<br>")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme(url: &str, dependencies: &[&str]) -> ThemeDefinition {
        ThemeDefinition {
            name: "theme".to_string(),
            url: url.to_string(),
            dependencies: dependencies.iter().map(ToString::to_string).collect(),
        }
    }

    fn licenses(source: &str) -> ThemeLicenses {
        toml::from_str(source).unwrap()
    }

    const MIT_A: &str = r#"
        [sources."https://github.com/a/a"]
        license = "MIT"
        license_file = "LICENSE"
    "#;

    #[test]
    fn accepts_a_licensed_source() {
        validate(&[theme("https://github.com/a/a", &[])], &licenses(MIT_A)).unwrap();
    }

    #[test]
    fn rejects_nonpermissive_and_unreviewed_theme_licenses() {
        for license in [
            "GPL-2.0",
            "GPL-3.0",
            "LGPL-3.0",
            "Apache-2.0 AND Vim",
            "NONE",
            "Custom",
        ] {
            let source = format!(
                "[sources.\"https://github.com/a/a\"]\nlicense = \"{license}\"\nlicense_file = \"LICENSE\"\n"
            );
            assert!(
                validate(&[theme("https://github.com/a/a", &[])], &licenses(&source)).is_err(),
                "accepted {license} in the bundled theme catalog"
            );
        }
    }

    #[test]
    fn rejects_a_theme_without_an_entry() {
        let err = validate(&[theme("https://github.com/b/b", &[])], &licenses(MIT_A)).unwrap_err();
        assert!(err
            .to_string()
            .contains("no entry for: https://github.com/b/b"));
    }

    #[test]
    fn rejects_a_dependency_without_an_entry() {
        let themes = [theme(
            "https://github.com/a/a",
            &["https://github.com/dep/dep"],
        )];
        let err = validate(&themes, &licenses(MIT_A)).unwrap_err();
        assert!(err
            .to_string()
            .contains("no entry for: https://github.com/dep/dep"));
    }

    #[test]
    fn rejects_an_entry_no_theme_uses() {
        let source = format!(
            "{MIT_A}\n[sources.\"https://github.com/gone/gone\"]\nlicense = \"MIT\"\nlicense_file = \"LICENSE\"\n"
        );
        let err =
            validate(&[theme("https://github.com/a/a", &[])], &licenses(&source)).unwrap_err();
        assert!(err
            .to_string()
            .contains("no theme uses: https://github.com/gone/gone"));
    }

    #[test]
    fn rejects_a_license_without_a_file() {
        let source = "[sources.\"https://github.com/a/a\"]\nlicense = \"MIT\"\n";
        let err = validate(&[theme("https://github.com/a/a", &[])], &licenses(source)).unwrap_err();
        assert!(err.to_string().contains("https://github.com/a/a"));
    }

    #[test]
    fn copyright_cell_tells_a_missing_license_from_a_missing_copyright_line() {
        let sources = licenses(
            "[sources.\"https://github.com/a/a\"]\nlicense = \"MIT\"\nlicense_file = \"LICENSE\"\n\
             [sources.\"https://github.com/b/b\"]\nlicense = \"NONE\"\n",
        );
        assert_eq!(
            copyright_cell(&sources.sources["https://github.com/a/a"]),
            "Not stated in the license file"
        );
        assert_eq!(
            copyright_cell(&sources.sources["https://github.com/b/b"]),
            "No license file"
        );
    }
}
