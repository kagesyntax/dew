//! Compile-time configuration. Everything here can be overridden with environment variables at
//! build time (CI sets them from repository secrets / variables), so no secrets live in git.

/// Display name of the church shown on the welcome screen and Today tab.
pub const CHURCH_NAME: &str = match option_env!("CHURCH_NAME") {
    Some(v) => v,
    None => "Grace Community Church",
};

pub const APP_NAME: &str = "Bible & Manual";

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
    None => "https://github.com/your-church/church-app",
};
