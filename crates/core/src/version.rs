//! Update manifest shared by the release pipeline and the in-app updater.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Published by CI next to the web build as `version.json` (see .github/workflows/release.yml).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub version: String,
    #[serde(default)]
    pub notes: String,
    /// Where native builds download the new version from (GitHub Release page).
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub published_at: Option<String>,
    /// Release-note bullets for the "What's New" list (from CHANGELOG.md).
    #[serde(default)]
    pub highlights: Vec<String>,
    /// Native installers by platform key (`macos`, `windows`, `linux`, `android`).
    #[serde(default)]
    pub downloads: BTreeMap<String, String>,
    /// Builds older than this must update (shown as a required update).
    #[serde(default)]
    pub minimum_version: Option<String>,
}

impl UpdateManifest {
    /// True when the running build is below `minimum_version`.
    pub fn is_required_for(&self, current: &str) -> bool {
        self.minimum_version.as_deref().is_some_and(|min| is_newer(current, min))
    }
}

/// Bullet points of one version's section in a Keep-a-Changelog file
/// (`## [0.2.0] - 2026-11-01` followed by `- item` lines, across `###` subsections).
pub fn release_notes(changelog: &str, version: &str) -> Vec<String> {
    let version = version.trim_start_matches('v');
    let mut out = Vec::new();
    let mut inside = false;
    for line in changelog.lines() {
        let t = line.trim();
        if let Some(h) = t.strip_prefix("## ") {
            if inside {
                break;
            }
            let h = h.trim_start_matches('[');
            inside =
                h.starts_with(version) && !h[version.len()..].starts_with(|c: char| c.is_ascii_digit() || c == '.');
            continue;
        }
        if inside {
            if let Some(item) = t.strip_prefix("- ").or_else(|| t.strip_prefix("* ")) {
                out.push(item.replace("**", ""));
            }
        }
    }
    out
}

/// True when `remote` is a strictly newer semver than `current`.
pub fn is_newer(current: &str, remote: &str) -> bool {
    match (
        semver::Version::parse(current.trim_start_matches('v')),
        semver::Version::parse(remote.trim_start_matches('v')),
    ) {
        (Ok(c), Ok(r)) => r > c,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_release_notes() {
        let md = "# Changelog\n\n## [Unreleased]\n- wip\n\n## [0.1.10] - x\n- ten\n\n## [0.1.1] - 2026-10-14\n### Added\n- **Tips** on first use\n### Fixed\n* parser\n\n## [0.1.0]\n- first\n";
        assert_eq!(release_notes(md, "v0.1.1"), vec!["Tips on first use", "parser"]);
        assert_eq!(release_notes(md, "0.1.0"), vec!["first"]);
        assert!(release_notes(md, "9.9.9").is_empty());
    }

    #[test]
    fn required_updates() {
        let m = UpdateManifest {
            version: "0.3.0".into(),
            notes: String::new(),
            url: None,
            published_at: None,
            highlights: vec![],
            downloads: Default::default(),
            minimum_version: Some("0.2.0".into()),
        };
        assert!(m.is_required_for("0.1.5"));
        assert!(!m.is_required_for("0.2.0"));
    }

    #[test]
    fn compares_versions() {
        assert!(is_newer("0.1.0", "0.2.0"));
        assert!(is_newer("0.1.0", "v0.1.1"));
        assert!(!is_newer("0.2.0", "0.2.0"));
        assert!(!is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("1.0.0", "garbage"));
    }
}
