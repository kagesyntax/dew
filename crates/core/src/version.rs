//! Update manifest shared by the release pipeline and the in-app updater.

use serde::{Deserialize, Serialize};

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
    fn compares_versions() {
        assert!(is_newer("0.1.0", "0.2.0"));
        assert!(is_newer("0.1.0", "v0.1.1"));
        assert!(!is_newer("0.2.0", "0.2.0"));
        assert!(!is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("1.0.0", "garbage"));
    }
}
