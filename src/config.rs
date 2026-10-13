//! Compile-time configuration. Everything here can be overridden with environment variables at
//! build time (CI sets them from repository secrets / variables), so no secrets live in git.

/// Display name of the church shown in Settings and on the Today tab
/// (Assemblies of God Church, Oshodi, Lagos).
pub const CHURCH_NAME: &str = match option_env!("CHURCH_NAME") {
    Some(v) => v,
    None => "AGC Oshodi",
};

/// The product name, Apple-style: one warm word, after the dew-drop icon and the manual's
/// Daily Dew Drops.
pub const APP_NAME: &str = "Dew";

/// One-line description used on the welcome screen, stores and share sheets.
pub const APP_TAGLINE: &str = "Fresh Word every morning.";

/// App version, taken from Cargo.toml (bumped by the release workflow).
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Turso (libSQL) database used to publish manual lessons. Use a **read-only** token here:
/// it ships inside the client. Leave unset to run fully offline on the bundled manual.
pub const TURSO_DATABASE_URL: Option<&str> = option_env!("TURSO_DATABASE_URL");
pub const TURSO_AUTH_TOKEN: Option<&str> = option_env!("TURSO_AUTH_TOKEN");

/// Where the updater looks for `version.json`. On the web the default is the copy CI deploys
/// next to the app; native builds should point at the GitHub Pages URL.
pub const UPDATE_MANIFEST_URL: &str = match option_env!("UPDATE_MANIFEST_URL") {
    Some(v) => v,
    None => "/version.json",
};

/// Repository page, used for "What's new" and native downloads.
pub const REPO_URL: &str = match option_env!("REPO_URL") {
    Some(v) => v,
    None => "https://github.com/agc-oshodi/dew",
};

/// Commit and date the build was made from (see build.rs).
pub const BUILD_SHA: &str = env!("BUILD_SHA");
pub const BUILD_DATE: &str = env!("BUILD_DATE");

/// "Stable" for tagged releases, "Preview" for develop builds (set by CI).
pub const RELEASE_CHANNEL: &str = match option_env!("RELEASE_CHANNEL") {
    Some(v) => v,
    None => "Development",
};

/// The bundled changelog, for the What's New sheet when no network is available.
pub const CHANGELOG: &str = include_str!("../CHANGELOG.md");

/// Public address of the web app, used by "Share App".
pub const APP_URL: &str = match option_env!("APP_URL") {
    Some(v) => v,
    None => "https://agc-oshodi.github.io/dew/",
};
