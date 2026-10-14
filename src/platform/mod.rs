//! Thin bridges to the host (browser / webview). Everything platform-specific lives here so the
//! rest of the app stays declarative. All bridges go through `document::eval`, which works on
//! web, desktop and mobile webviews alike.

pub mod clock;
pub mod js;
pub mod storage;
pub mod theme;

#[cfg(target_os = "android")]
pub mod android_chrome;
