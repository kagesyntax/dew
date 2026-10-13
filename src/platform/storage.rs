//! Key/value persistence on top of `localStorage` (available in every Dioxus renderer that uses
//! a webview). Values are stored as JSON under a versioned namespace.

use dioxus::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

const NAMESPACE: &str = "church-app.v1";

fn full_key(key: &str) -> String {
    format!("{NAMESPACE}.{key}")
}

/// Read and deserialize a value. Returns `None` when missing or unreadable.
pub async fn load<T: DeserializeOwned>(key: &str) -> Option<T> {
    let js = format!("return localStorage.getItem({});", serde_json::to_string(&full_key(key)).ok()?);
    let raw: Option<String> = document::eval(&js).join().await.ok()?;
    serde_json::from_str(&raw?).ok()
}

/// Serialize and store a value (fire-and-forget).
pub fn save<T: Serialize>(key: &str, value: &T) {
    let (Ok(k), Ok(json)) = (serde_json::to_string(&full_key(key)), serde_json::to_string(value)) else {
        return;
    };
    let Ok(v) = serde_json::to_string(&json) else { return };
    document::eval(&format!("localStorage.setItem({k}, {v});"));
}

/// Remove every key written by the app (used by "Reset app" in Settings).
pub fn clear_all() {
    let prefix = serde_json::to_string(NAMESPACE).unwrap_or_default();
    document::eval(&format!(
        "Object.keys(localStorage).filter(k => k.startsWith({prefix})).forEach(k => localStorage.removeItem(k));"
    ));
}
