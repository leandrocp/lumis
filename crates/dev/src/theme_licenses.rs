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
    pub(crate) sources: BTreeMap<String, SourceLicense>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceLicense {
    pub(crate) license: String,
    pub(crate) license_file: Option<String>,
    pub(crate) license_revision: Option<String>,
    #[serde(default)]
    copyright: Vec<String>,
    note: Option<String>,
}

#[derive(Clone)]
pub(crate) struct ResolvedTheme<'a> {
    pub(crate) name: &'a str,
    pub(crate) url: &'a str,
    pub(crate) revision: Option<String>,
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

pub(crate) fn sync_license_texts(
    themes: &[ResolvedTheme<'_>],
    definitions: &[ThemeDefinition],
    licenses: &ThemeLicenses,
    check: bool,
    fetch: bool,
) -> Result<()> {
    let mut sources = BTreeSet::new();
    for theme in themes {
        if theme.license.license_file.is_some() {
            let revision = theme
                .revision
                .as_deref()
                .context("theme revision missing")?;
            sources.insert((theme.url, revision));
        }
    }
    for url in dependency_users(definitions).keys() {
        let source = &licenses.sources[*url];
        if source.license_file.is_some() {
            let revision = source.license_revision.as_deref().with_context(|| {
                format!("{url} needs a license_revision for its extraction dependency license")
            })?;
            sources.insert((*url, revision));
        }
    }
    for (url, revision) in sources {
        sync_license_text(url, revision, &licenses.sources[url], check, fetch)?;
    }
    Ok(())
}

fn sync_license_text(
    url: &str,
    revision: &str,
    source: &SourceLicense,
    _check: bool,
    fetch: bool,
) -> Result<()> {
    ensure!(
        revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "{url} needs a full commit hash, got {revision}"
    );
    let file = source
        .license_file
        .as_deref()
        .context("license file missing")?;
    let repo = url
        .strip_prefix("https://github.com/")
        .context("expected GitHub source")?;
    let relative = Path::new(repo).join(revision).join(file);
    let path = Path::new("themes/licenses").join(&relative);
    if fetch {
        let remote = format!("https://raw.githubusercontent.com/{repo}/{revision}/{file}");
        let bytes = ureq::get(&remote).call()?.body_mut().read_to_vec()?;
        ensure!(!bytes.is_empty(), "empty license at {remote}");
        fs::create_dir_all(path.parent().context("license parent missing")?)?;
        fs::write(&path, bytes)?;
    }
    let bytes = fs::read(&path).with_context(|| {
        format!(
            "missing {}; run `mise run themes-fetch-licenses`",
            path.display()
        )
    })?;
    ensure!(!bytes.is_empty(), "empty license: {}", path.display());
    Ok(())
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
        "Lumis themes are color values extracted from the Neovim colorscheme plugins below, at the commit listed for each theme. Each plugin keeps its own license, which is not the license of Lumis. The license links point to the full upstream texts and copyright notices shipped in `licenses/`.".to_string(),
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
            "Some themes are loaded through another plugin during extraction. These plugins are not pinned during extraction; the bundled license texts use the commits recorded in `themes/licenses.toml`.".to_string(),
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
                license_link(
                    url,
                    license.license_revision.as_deref().unwrap_or("HEAD"),
                    license
                ),
                copyright_cell(license),
            ));
        }
    }

    let notes: Vec<String> = licenses
        .sources
        .iter()
        .filter(|(url, _)| {
            themes.iter().any(|theme| theme.url == url.as_str())
                || dependencies.contains_key(url.as_str())
        })
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

pub(crate) fn dependency_users(definitions: &[ThemeDefinition]) -> BTreeMap<&str, BTreeSet<&str>> {
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
        Some(file) => {
            let repo = url.strip_prefix("https://github.com/").unwrap_or(url);
            format!("[{label}](licenses/{repo}/{revision}/{file})")
        }
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
            builtin: false,
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
