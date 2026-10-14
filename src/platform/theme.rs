//! Applies Light / Dark / System appearance and reader text size to the document root.
//!
//! The two theme colours live here as the single source of truth. They are interpolated into the
//! JavaScript below (for `theme-color` and `html[data-theme]`) and converted to ARGB for the
//! Android system bars, so the web theme colour and the native bars can never disagree, and both
//! update together whenever the theme changes.

use crate::state::settings::ThemeMode;
use dioxus::prelude::*;

/// Page background in light mode. Matches `--app-bg-grouped` in `assets/app.css`.
pub const LIGHT_BG: &str = "#f2f2f7";
/// Page background in dark mode. Matches `--app-bg-grouped` in `assets/app.css`.
pub const DARK_BG: &str = "#000000";

/// `"#rrggbb"` to an opaque Android ARGB colour.
#[cfg(target_os = "android")]
fn argb(hex: &str) -> u32 {
    let hex = hex.trim_start_matches('#');
    let rgb = u32::from_str_radix(hex, 16).unwrap_or(0);
    0xFF00_0000 | (rgb & 0x00FF_FFFF)
}

/// Sets `html[data-theme]`, `color-scheme` and the `theme-color` meta, and keeps the Android system
/// bars in step. In System mode a `prefers-color-scheme` listener keeps the app in sync when the
/// OS flips.
pub fn apply(mode: ThemeMode, text_size: u8) {
    let mode = match mode {
        ThemeMode::System => "system",
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    };
    // `dioxus.send` pushes into the channel of the very eval that contains it, so the setup and the
    // Rust side that reads it have to come from the same `document::eval` below.
    let js = format!(
        r#"
        const mode = "{mode}";
        const LIGHT = "{LIGHT_BG}";
        const DARK = "{DARK_BG}";
        const root = document.documentElement;
        root.dataset.text = "{text_size}";
        const mq = window.matchMedia('(prefers-color-scheme: dark)');
        const apply = () => {{
            const dark = mode === 'dark' || (mode === 'system' && mq.matches);
            const bg = dark ? DARK : LIGHT;
            root.dataset.theme = dark ? 'dark' : 'light';
            root.style.colorScheme = dark ? 'dark' : 'light';
            let meta = document.querySelector('meta[name="theme-color"]');
            if (!meta) {{ meta = document.createElement('meta'); meta.name = 'theme-color'; document.head.appendChild(meta); }}
            meta.content = bg;
            // Report the resolved theme so the native system bars use the same colour.
            dioxus.send(dark);
        }};
        apply();
        if (window.__themeListener) mq.removeEventListener('change', window.__themeListener);
        window.__themeListener = apply;
        mq.addEventListener('change', apply);
        "#
    );

    // System mode can flip while the app is open, so the listener keeps reporting and the native
    // bars follow. Other platforms compile this out and only need the DOM side.
    #[cfg(target_os = "android")]
    spawn(async move {
        let mut ev = document::eval(&js);
        while let Ok(dark) = ev.recv::<bool>().await {
            let bg = if dark { DARK_BG } else { LIGHT_BG };
            crate::platform::android_chrome::apply_system_bars(argb(bg), !dark);
            crate::platform::android_chrome::push_insets();
        }
    });

    #[cfg(not(target_os = "android"))]
    document::eval(&js);
}
