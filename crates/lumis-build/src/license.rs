//! Identifies the license of a theme plugin from its license files.
//!
//! Detection compares the text with every license on the SPDX License List and
//! reports one identifier per file, so it never errors. These choices are made
//! on top of the raw scores, each because the score alone gives a wrong answer:
//!
//! - **The closest candidates are scored again by the part of their text that
//!   matches best.** A license file often carries less than SPDX's text: an
//!   Apache-2.0 file without its appendix scores higher against Pixar's license
//!   as a whole, and an LGPL file scores 0.3 because SPDX's LGPL text embeds the
//!   GPL it only refers to.
//! - **A file is identified by the license it mostly holds.** The closest
//!   license locates its region of the file before the comparison above, so a
//!   file holding Apache-2.0 followed by the shorter Vim license is Apache-2.0.
//! - **Near ties prefer an OSI-approved, non-deprecated license, then the one
//!   whose whole text matches best**, so an MIT file is MIT and not one of the
//!   longer licenses that contain the MIT text.
//! - **GNU licenses are named by version only**, as GitHub does: `GPL-3.0`, not
//!   `GPL-3.0-only` or `GPL-3.0-or-later`. Both variants share one text, and
//!   which one applies is stated outside the license file.
//! - **A text that resembles no license is [`NO_ASSERTION`]**, and a plugin
//!   without license files is [`NO_LICENSE`], which are SPDX's values for those
//!   cases.

use spdx::detection::{LicenseEntry, Store, TextData};
use std::fs;
use std::io;
use std::path::Path;

/// SPDX's value for a plugin without a license file.
pub const NO_LICENSE: &str = "NONE";

/// SPDX's value for a license file that matches no known license.
pub const NO_ASSERTION: &str = "NOASSERTION";

/// Licenses whose whole text scores within this distance of the best are
/// scored again by the part of their text that matches best.
const CANDIDATE_WINDOW: f32 = 0.1;

/// A file whose best match scores below this resembles no license.
const MIN_SCORE: f32 = 0.8;

/// Candidates within this distance of the best score are a tie.
const TIE_MARGIN: f32 = 0.02;

/// Identifies licenses from license file texts against the SPDX License List.
pub struct LicenseDetector {
    store: Store,
}

impl LicenseDetector {
    /// Loads the SPDX License List bundled with this crate.
    pub fn new() -> Result<Self, io::Error> {
        let store = Store::load_inline().map_err(io::Error::other)?;
        Ok(Self { store })
    }

    /// Returns the SPDX expression for a plugin's license file texts:
    /// [`NO_LICENSE`] for none, and the identifiers of distinct licenses joined
    /// with `AND`.
    pub fn detect(&self, texts: &[&str]) -> String {
        let mut licenses: Vec<String> = texts.iter().map(|text| self.identify(text)).collect();
        licenses.sort();
        licenses.dedup();
        if licenses.is_empty() {
            NO_LICENSE.to_string()
        } else {
            licenses.join(" AND ")
        }
    }

    /// Detects the license of the license files in `dir`, the layout theme
    /// extraction writes. A missing directory is a plugin without a license.
    pub fn detect_dir(&self, dir: &Path) -> io::Result<String> {
        let texts = read_license_files(dir)?;
        let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
        Ok(self.detect(&texts))
    }

    fn identify(&self, text: &str) -> String {
        let file = TextData::from(text);
        let Some((_, _, closest)) = self.ranked(&file).into_iter().next() else {
            return NO_ASSERTION.to_string();
        };
        let (region, region_score) = file.optimize_bounds(&closest.original);
        let data = if region_score > file.match_score(&closest.original) {
            region
        } else {
            file
        };

        let whole = self.ranked(&data);
        let Some(&(best_whole, _, _)) = whole.first() else {
            return NO_ASSERTION.to_string();
        };

        let mut candidates: Vec<(f32, f32, &str)> = whole
            .iter()
            .take_while(|(score, _, _)| best_whole - score <= CANDIDATE_WINDOW)
            .map(|&(whole, name, entry)| {
                let contained = best_score(entry, |known| known.optimize_bounds(&data).1);
                (whole.max(contained), whole, name)
            })
            .collect();
        candidates.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.2.cmp(b.2)));

        let Some(&(best, _, _)) = candidates.first() else {
            return NO_ASSERTION.to_string();
        };
        if best < MIN_SCORE {
            return NO_ASSERTION.to_string();
        }

        candidates
            .iter()
            .take_while(|(score, _, _)| best - score <= TIE_MARGIN)
            .max_by(|a, b| {
                is_preferred(a.2)
                    .cmp(&is_preferred(b.2))
                    .then_with(|| a.1.total_cmp(&b.1))
            })
            .map_or_else(|| NO_ASSERTION.to_string(), |(_, _, name)| spdx_name(name))
    }

    fn ranked(&self, data: &TextData) -> Vec<(f32, &str, &LicenseEntry)> {
        let mut ranked: Vec<(f32, &str, &LicenseEntry)> = self
            .store
            .iter()
            .map(|(name, entry)| {
                let score = best_score(entry, |known| data.match_score(known));
                (score, name.as_str(), entry)
            })
            .collect();
        ranked.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        ranked
    }
}

fn best_score(entry: &LicenseEntry, score: impl Fn(&TextData) -> f32) -> f32 {
    entry
        .alternates
        .iter()
        .fold(score(&entry.original), |best, alternate| {
            best.max(score(alternate))
        })
}

/// Whether any license in an SPDX expression is copyleft. An expression that
/// does not parse, including [`NO_LICENSE`] and [`NO_ASSERTION`], is not.
pub fn is_copyleft(expression: &str) -> bool {
    spdx::Expression::parse_mode(expression, spdx::ParseMode::LAX).is_ok_and(|expression| {
        expression.requirements().any(|requirement| {
            requirement
                .req
                .license
                .id()
                .is_some_and(spdx::LicenseId::is_copyleft)
        })
    })
}

fn read_license_files(dir: &Path) -> io::Result<Vec<String>> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut paths = entries
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<io::Result<Vec<_>>>()?;
    paths.sort();
    paths
        .iter()
        .filter(|path| path.is_file())
        .map(|path| fs::read(path).map(|bytes| String::from_utf8_lossy(&bytes).into_owned()))
        .collect()
}

fn is_preferred(name: &str) -> bool {
    spdx::license_id(name).is_some_and(|id| id.is_osi_approved() && !id.is_deprecated())
}

fn spdx_name(name: &str) -> String {
    match spdx::license_id(name) {
        Some(id) if id.is_gnu() => match id.version() {
            Some(version) => format!("{}-{version}", id.base()),
            None => id.name.to_string(),
        },
        Some(id) => id.name.to_string(),
        None => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIT: &str = "MIT License

Copyright (c) 2024 Someone

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
";

    fn detector() -> LicenseDetector {
        LicenseDetector::new().unwrap()
    }

    fn license_text(id: &str) -> &'static str {
        spdx::license_id(id).unwrap().text()
    }

    #[test]
    fn identifies_a_permissive_license() {
        assert_eq!(detector().detect(&[MIT]), "MIT");
    }

    #[test]
    fn no_license_file_is_none() {
        assert_eq!(detector().detect(&[]), NO_LICENSE);
    }

    #[test]
    fn text_that_is_not_a_license_is_no_assertion() {
        let readme = "# my-theme.nvim\n\nA dark colorscheme for Neovim with Tree-sitter support.\n\nInstall it with your plugin manager and run :colorscheme my-theme.\n";
        assert_eq!(detector().detect(&[readme]), NO_ASSERTION);
    }

    #[test]
    fn apache_without_its_appendix_is_apache_not_pixar() {
        let apache = license_text("Apache-2.0");
        let without_appendix = &apache[..apache.find("APPENDIX").unwrap()];
        assert_eq!(detector().detect(&[without_appendix]), "Apache-2.0");
    }

    #[test]
    fn a_file_is_identified_by_the_license_it_mostly_holds() {
        let apache = license_text("Apache-2.0");
        let without_appendix = &apache[..apache.find("APPENDIX").unwrap()];
        let with_vim = format!("{without_appendix}\n\n{}", license_text("Vim"));
        assert_eq!(detector().detect(&[&with_vim]), "Apache-2.0");
    }

    #[test]
    fn lgpl_without_the_gpl_it_refers_to_is_lgpl() {
        let lgpl = license_text("LGPL-3.0-only");
        let lgpl_only = &lgpl[..lgpl.find("GNU GENERAL PUBLIC LICENSE").unwrap()];
        assert_eq!(detector().detect(&[lgpl_only]), "LGPL-3.0");
    }

    #[test]
    fn gnu_licenses_are_named_by_version() {
        let detector = detector();
        assert_eq!(detector.detect(&[license_text("GPL-3.0-only")]), "GPL-3.0");
        assert_eq!(detector.detect(&[license_text("GPL-2.0-only")]), "GPL-2.0");
    }

    #[test]
    fn distinct_licenses_are_joined_and_duplicates_collapse() {
        let detector = detector();
        let apache = license_text("Apache-2.0");
        assert_eq!(detector.detect(&[apache, MIT]), "Apache-2.0 AND MIT");
        assert_eq!(detector.detect(&[MIT, MIT]), "MIT");
    }

    #[test]
    fn copyleft_is_read_from_every_license_in_the_expression() {
        assert!(is_copyleft("GPL-3.0"));
        assert!(is_copyleft("LGPL-3.0"));
        assert!(is_copyleft("MIT AND GPL-2.0"));
        assert!(!is_copyleft("Apache-2.0"));
        assert!(!is_copyleft("Apache-2.0 AND MIT"));
        assert!(!is_copyleft(NO_LICENSE));
        assert!(!is_copyleft(NO_ASSERTION));
    }
}
