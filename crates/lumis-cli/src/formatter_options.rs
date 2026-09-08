//! Which `lumis highlight` flags each formatter accepts.
//!
//! `fixtures/formatter-options.json` says which formatter accepts which option.
//! `build.rs` turns that into [`FORMATTER_OPTIONS`]. This module adds the part
//! the manifest does not carry: how the flags are grouped in `--help`, and
//! which CLI flag spellings belong to each group.
//!
//! Which formatters accept a group is *derived* from the manifest rather than
//! declared here, so there is no second copy to keep in step. Adding an option
//! to a formatter in the manifest moves `--help`, the validation and
//! `lumis formatters show` together.

use crate::Formatter;

include!(concat!(env!("OUT_DIR"), "/formatter_options.rs"));

pub(crate) const OPTSET_GLOBAL: &str = "Global options";
pub(crate) const OPTSET_TERMINAL: &str = "Options for --formatter terminal";
pub(crate) const OPTSET_HTML: &str =
    "Options for --formatter html-inline, html-linked, html-multi-themes";
pub(crate) const OPTSET_STYLED: &str = "Options for --formatter html-inline, html-multi-themes";
pub(crate) const OPTSET_MULTI_THEME: &str = "Options for --formatter html-multi-themes";

/// A set of flags that the same formatters accept.
pub(crate) struct OptionGroup {
    /// How the group is named in an error. Reads after "are not accepted by".
    pub label: &'static str,
    /// Every CLI flag in the group, in `--help` order.
    pub flags: &'static [&'static str],
    /// The manifest options this group covers. A formatter accepts the group
    /// when it accepts all of them.
    pub manifest_options: &'static [&'static str],
}

impl OptionGroup {
    pub(crate) fn accepts(&self, formatter: Formatter) -> bool {
        let accepted = manifest_options_for(formatter);
        self.manifest_options
            .iter()
            .all(|option| accepted.contains(option))
    }

    pub(crate) fn accepted_by(&self) -> Vec<Formatter> {
        Formatter::ALL
            .into_iter()
            .filter(|formatter| self.accepts(*formatter))
            .collect()
    }
}

/// `--highlight-lines-style` has no manifest entry of its own. The manifest
/// carries one `highlight_lines` option covering the whole struct, and only the
/// inline-style formatters have a `style` field on theirs, so the flag sits in
/// the styled group and inherits the formatters `italic` and
/// `include_highlights` derive.
pub(crate) const OPTION_GROUPS: &[OptionGroup] = &[
    OptionGroup {
        label: "`--theme`",
        flags: &["--theme"],
        manifest_options: &["theme"],
    },
    OptionGroup {
        label: "terminal options",
        flags: &["--background", "--width"],
        manifest_options: &["background", "width"],
    },
    OptionGroup {
        label: "HTML options",
        flags: &[
            "--pre-class",
            "--header-open",
            "--header-close",
            "--highlight-lines",
            "--highlight-lines-class",
        ],
        manifest_options: &["pre_class", "header", "highlight_lines"],
    },
    OptionGroup {
        label: "inline-style options",
        flags: &[
            "--italic",
            "--include-highlights",
            "--highlight-lines-style",
        ],
        manifest_options: &["italic", "include_highlights"],
    },
    OptionGroup {
        label: "multi-theme options",
        flags: &["--themes", "--default-theme", "--css-variable-prefix"],
        manifest_options: &["themes", "default_theme", "css_variable_prefix"],
    },
];

fn manifest_options_for(formatter: Formatter) -> &'static [&'static str] {
    let slug = formatter.slug();
    FORMATTER_OPTIONS
        .iter()
        .find(|(name, _)| *name == slug)
        .map_or_else(
            || panic!("formatter-options.json has no entry for `{slug}`"),
            |(_, options)| *options,
        )
}

/// Every flag a formatter accepts, for `lumis formatters show`. The universal
/// flags come first, then each group in `--help` order. `--formatter` is not
/// among them: it selects the formatter rather than configuring one.
pub(crate) fn accepted_flags(formatter: Formatter) -> Vec<&'static str> {
    let mut flags = vec!["--language", "--rainbow-brackets"];
    for group in OPTION_GROUPS.iter().filter(|g| g.accepts(formatter)) {
        flags.extend_from_slice(group.flags);
    }
    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    /// The headings live on the arg structs in `main.rs` and the groups live
    /// here. Nothing declares the mapping twice, so this derives it: every flag
    /// in a group must sit under one heading, and that heading must hold
    /// exactly the group's flags.
    #[test]
    fn each_group_is_exactly_one_help_section() {
        let cli = crate::Cli::command();
        let highlight = cli
            .find_subcommand("highlight")
            .expect("the highlight subcommand exists");

        let heading_of = |flag: &str| {
            let long = flag.trim_start_matches("--");
            highlight
                .get_arguments()
                .find(|arg| arg.get_long() == Some(long))
                .unwrap_or_else(|| panic!("`{flag}` is not an argument of `lumis highlight`"))
                .get_help_heading()
        };

        let mut seen: Vec<Option<&str>> = Vec::new();
        for group in OPTION_GROUPS {
            let heading = heading_of(group.flags[0]);
            for flag in group.flags {
                assert_eq!(
                    heading_of(flag),
                    heading,
                    "`{flag}` renders under {:?}, but `{}` in the same group renders under {heading:?}",
                    heading_of(flag),
                    group.flags[0],
                );
            }

            // Flags with no heading share the main block with `--language` and
            // friends, so only a named section can be compared as a set.
            if let Some(named) = heading {
                assert!(
                    !seen.contains(&heading),
                    "two groups render under {named:?}; give one its own heading"
                );

                let mut rendered: Vec<&str> = highlight
                    .get_arguments()
                    .filter(|arg| arg.get_help_heading() == heading)
                    .filter_map(clap::Arg::get_long)
                    .collect();
                let mut declared: Vec<&str> = group
                    .flags
                    .iter()
                    .map(|f| f.trim_start_matches("--"))
                    .collect();
                rendered.sort_unstable();
                declared.sort_unstable();
                assert_eq!(
                    rendered, declared,
                    "{named:?} renders {rendered:?} but its group declares {declared:?}"
                );
            }
            seen.push(heading);
        }
    }

    #[test]
    fn the_manifest_covers_every_formatter() {
        assert_eq!(
            FORMATTER_OPTIONS.len(),
            Formatter::ALL.len(),
            "the manifest describes {} formatters but the CLI offers {}",
            FORMATTER_OPTIONS.len(),
            Formatter::ALL.len()
        );
    }

    #[test]
    fn every_manifest_option_is_reachable() {
        let grouped: Vec<&str> = OPTION_GROUPS
            .iter()
            .flat_map(|group| group.manifest_options.iter().copied())
            .collect();

        for (formatter, options) in FORMATTER_OPTIONS {
            for option in *options {
                assert!(
                    grouped.contains(option) || matches!(*option, "language"),
                    "manifest option `{option}` on `{formatter}` belongs to no group; \
                     add it to OPTION_GROUPS or to the universal list"
                );
            }
        }
    }

    /// The reachability check above only reads manifest to group. A group naming
    /// an option no formatter has would fail no assertion there: `accepts` would
    /// simply return false everywhere, and the group would silently apply to
    /// nothing.
    #[test]
    fn every_group_option_exists_and_reaches_a_formatter() {
        let known: Vec<&str> = FORMATTER_OPTIONS
            .iter()
            .flat_map(|(_, options)| options.iter().copied())
            .collect();

        for group in OPTION_GROUPS {
            assert!(!group.flags.is_empty(), "{} declares no flags", group.label);

            for option in group.manifest_options {
                assert!(
                    known.contains(option),
                    "{} names manifest option `{option}`, which no formatter declares",
                    group.label
                );
            }

            assert!(
                !group.accepted_by().is_empty(),
                "{} is accepted by no formatter, so its flags can never be used",
                group.label
            );
        }
    }
}
