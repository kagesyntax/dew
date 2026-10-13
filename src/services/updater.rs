//! In-app updates. CI publishes `version.json` (see `church_core::version::UpdateManifest`)
//! with every release; Settings compares it with the running version.
//! * Web / PWA: "Update" clears the offline cache and reloads the newest deployed build.
//! * Desktop / mobile: "Update" opens the release download page.

use church_core::version::{is_newer, UpdateManifest};

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpToDate,
    Available(UpdateManifest),
    Installing,
    Failed(String),
}

/// Platform key used in `UpdateManifest.downloads`.
pub fn platform_key() -> &'static str {
    if cfg!(feature = "web") {
        "web"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "android") {
        "android"
    } else if cfg!(target_os = "ios") {
        "ios"
    } else {
        "linux"
    }
}

fn manifest_url() -> String {
    let url = crate::config::UPDATE_MANIFEST_URL;
    // Cache-bust so a CDN never hides a fresh release.
    let sep = if url.contains('?') { '&' } else { '?' };
    format!("{url}{sep}t={}", chrono::Utc::now().timestamp())
}

/// Resolve a relative manifest path against the app's base URL (works under a GitHub Pages sub-path) (reqwest needs absolute URLs).
async fn absolute(url: String) -> String {
    if url.starts_with("http") {
        return url;
    }
    let js = format!("return new URL({}, document.baseURI).href;", serde_json::to_string(&url).unwrap_or_default());
    dioxus::prelude::document::eval(&js).join::<String>().await.unwrap_or(url)
}

pub async fn check() -> UpdateStatus {
    let url = absolute(manifest_url()).await;
    let resp = match reqwest::get(&url).await {
        Ok(r) if r.status().is_success() => r,
        Ok(r) => return UpdateStatus::Failed(format!("Update server returned {}", r.status())),
        Err(e) => return UpdateStatus::Failed(format!("Couldn't reach update server ({e})")),
    };
    match resp.json::<UpdateManifest>().await {
        Ok(m) if is_newer(crate::config::APP_VERSION, &m.version) => UpdateStatus::Available(m),
        Ok(_) => UpdateStatus::UpToDate,
        Err(e) => UpdateStatus::Failed(format!("Invalid update manifest ({e})")),
    }
}

/// Apply an available update for the current platform.
pub fn apply(manifest: &UpdateManifest) {
    if cfg!(feature = "web") {
        crate::platform::js::reload_fresh();
    } else if let Some(url) = manifest.downloads.get(platform_key()) {
        crate::platform::js::open_url(url);
    } else {
        let url = manifest.url.clone().unwrap_or_else(|| format!("{}/releases/latest", crate::config::REPO_URL));
        crate::platform::js::open_url(&url);
    }
}
