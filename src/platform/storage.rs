//! Key/value persistence on top of `localStorage` (available in every Dioxus renderer that uses
//! a webview). Values are stored as JSON under a versioned namespace.

use dioxus::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

/// Storage namespace, chosen in JS: the live demo embedded on the website (an iframe) keeps its
/// own sandbox so trying Dew on the site never touches a visitor's real app data.
const NS_JS: &str = "((window.self !== window.top) ? 'dew.demo.v1.' : 'dew.v1.')";

/// Read and deserialize a value. Returns `None` when missing or unreadable.
pub async fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let js = format!("return localStorage.getItem({NS_JS} + {});", serde_json::to_string(key).ok()?);
    let raw: Option<String> = document::eval(&js).join().await.ok()?;
    serde_json::from_str(&raw?).ok()
}

/// Serialize and store a value (fire-and-forget).
pub fn save<T: Serialize>(key: &str, value: &T) {
    let (Ok(k), Ok(json)) = (serde_json::to_string(key), serde_json::to_string(value)) else {
        return;
    };
    let Ok(v) = serde_json::to_string(&json) else { return };
    document::eval(&format!("localStorage.setItem({NS_JS} + {k}, {v});"));
}

/// Remove every key written by the app (used by "Reset app" in Settings).
pub fn clear_all() {
    document::eval(&format!(
        "const p = {NS_JS}; Object.keys(localStorage).filter(k => k.startsWith(p)).forEach(k => localStorage.removeItem(k));"
    ));
}
