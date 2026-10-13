//! Applies Light / Dark / System appearance and reader text size to the document root.

use crate::state::settings::ThemeMode;
use dioxus::prelude::*;

/// Sets `html[data-theme]`, `color-scheme` and the `theme-color` meta. In System mode a
/// `prefers-color-scheme` listener keeps the app in sync when the OS flips.
pub fn apply(mode: ThemeMode, text_size: u8) {
    let mode = match mode {
        ThemeMode::System => "system",
        ThemeMode::Light => "light",
        ThemeMode::Dark => "dark",
    };
    document::eval(&format!(
        r#"
        const mode = "{mode}";
        const root = document.documentElement;
        root.dataset.text = "{text_size}";
        const mq = window.matchMedia('(prefers-color-scheme: dark)');
        const apply = () => {{
            const dark = mode === 'dark' || (mode === 'system' && mq.matches);
            root.dataset.theme = dark ? 'dark' : 'light';
            root.style.colorScheme = dark ? 'dark' : 'light';
            let meta = document.querySelector('meta[name="theme-color"]');
            if (!meta) {{ meta = document.createElement('meta'); meta.name = 'theme-color'; document.head.appendChild(meta); }}
            meta.content = dark ? '#000000' : '#f2f2f7';
        }};
        apply();
        if (window.__themeListener) mq.removeEventListener('change', window.__themeListener);
        window.__themeListener = apply;
        mq.addEventListener('change', apply);
        "#
    ));
}
