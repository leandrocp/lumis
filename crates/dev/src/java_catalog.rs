use crate::ParserInfo;
use anyhow::{ensure, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

pub(crate) const LANG: &str = "packages/java/lumis/src/main/java/sh/lumis/Lang.java";
pub(crate) const THEME: &str = "packages/java/lumis/src/main/java/sh/lumis/Theme.java";

// Plaintext is declared in `crates/lumis-core/build.rs`, not in `languages.toml`.
const PLAINTEXT_ALIASES: [&str; 4] = ["plaintext", "text", "txt", "plain"];

/// Renders the Java `Lang` enum: a constant per language id and a lookup entry per alias.
pub(crate) fn render_lang(parsers: &BTreeMap<String, ParserInfo>) -> Result<String> {
    let mut constants = vec![r#"PLAINTEXT("")"#.to_string()];
    let mut aliases: Vec<(&str, String)> = PLAINTEXT_ALIASES
        .iter()
        .map(|alias| (*alias, "PLAINTEXT".to_string()))
        .collect();
    for (id, info) in parsers {
        let constant = constant(id)?;
        constants.push(format!(r#"{constant}("{id}")"#));
        for alias in &info.aliases {
            aliases.push((alias, constant.clone()));
        }
    }
    check_aliases(parsers, &aliases)?;
    let mut lookups = String::new();
    for (alias, constant) in &aliases {
        writeln!(lookups, r#"        langs.put("{alias}", {constant});"#)?;
    }
    Ok(render_enum(
        "languages.toml",
        "langs-gen-catalog",
        "Lang",
        &constants,
        &lookups,
    ))
}

/// Renders the Java `Theme` enum: a constant per theme name.
pub(crate) fn render_theme<'a>(names: impl IntoIterator<Item = &'a str>) -> Result<String> {
    let constants = names
        .into_iter()
        .map(|name| Ok(format!(r#"{}("{name}")"#, constant(name)?)))
        .collect::<Result<Vec<_>>>()?;
    Ok(render_enum(
        "themes/themes.lua",
        "docs-gen-themes-md",
        "Theme",
        &constants,
        "",
    ))
}

fn constant(id: &str) -> Result<String> {
    let mut chars = id.chars();
    let valid = chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
    ensure!(valid, "`{id}` cannot be turned into a Java constant");
    Ok(id.to_ascii_uppercase())
}

fn check_aliases(parsers: &BTreeMap<String, ParserInfo>, aliases: &[(&str, String)]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for (alias, _) in aliases {
        ensure!(
            !alias.contains(['"', '\\']),
            "alias `{alias}` needs escaping in a Java string"
        );
        ensure!(
            !parsers.contains_key(*alias),
            "alias `{alias}` is also a language id"
        );
        ensure!(seen.insert(*alias), "alias `{alias}` is declared twice");
    }
    Ok(())
}

fn render_enum(
    source: &str,
    task: &str,
    name: &str,
    constants: &[String],
    lookups: &str,
) -> String {
    let variable = name.to_lowercase();
    let constants = constants
        .iter()
        .map(|constant| format!("    {constant}"))
        .collect::<Vec<_>>()
        .join(",\n");
    format!(
        r#"// Generated from {source} by `mise run {task}`. Do not edit.
package sh.lumis;

import java.util.HashMap;
import java.util.Map;

public enum {name} {{
{constants};

    private final String value;

    {name}(String value) {{
        this.value = value;
    }}

    public String value() {{
        return value;
    }}

    private static final Map<String, {name}> {variable}s;

    static {{
        {variable}s = new HashMap<>();
        for ({name} {variable} : {name}.values()) {{
            {variable}s.put({variable}.value(), {variable});
        }}
{lookups}    }}

    public static {name} fromString(String {variable}) {{
        if (!{variable}s.containsKey({variable})) {{
            throw new IllegalArgumentException("{name} '" + {variable} + "' not found.");
        }}
        return {variable}s.get({variable});
    }}
}}
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsers(entries: &[(&str, &[&str])]) -> BTreeMap<String, ParserInfo> {
        entries
            .iter()
            .map(|(id, aliases)| {
                let info = ParserInfo {
                    aliases: aliases.iter().map(ToString::to_string).collect(),
                    ..ParserInfo::default()
                };
                ((*id).to_string(), info)
            })
            .collect()
    }

    #[test]
    fn lang_has_a_constant_per_id_and_a_lookup_per_alias() {
        let output = render_lang(&parsers(&[("csharp", &["c#"]), ("asm", &[])])).unwrap();

        assert!(
            output.contains("    PLAINTEXT(\"\"),\n    ASM(\"asm\"),\n    CSHARP(\"csharp\");\n")
        );
        assert!(output.contains("        langs.put(\"text\", PLAINTEXT);\n"));
        assert!(output.contains("        langs.put(\"c#\", CSHARP);\n"));
    }

    #[test]
    fn theme_has_a_constant_per_name() {
        let output = render_theme(["dracula", "github_dark"]).unwrap();

        assert!(output.contains("    DRACULA(\"dracula\"),\n    GITHUB_DARK(\"github_dark\");\n"));
    }

    #[test]
    fn a_name_that_is_not_an_identifier_is_rejected() {
        assert!(render_theme(["tokyo-night"]).is_err());
        assert!(render_theme(["1337"]).is_err());
    }

    #[test]
    fn an_alias_that_would_shadow_a_lookup_is_rejected() {
        assert!(render_lang(&parsers(&[("bash", &["zsh"]), ("zsh", &[])])).is_err());
        assert!(render_lang(&parsers(&[("bash", &["sh"]), ("zsh", &["sh"])])).is_err());
    }
}
