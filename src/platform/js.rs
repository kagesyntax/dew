//! Small one-off host helpers.

use dioxus::prelude::*;

/// Current vertical scroll offset of the window.
pub async fn scroll_y() -> f64 {
    document::eval("return window.scrollY;").join::<f64>().await.unwrap_or(0.0)
}

/// Restore a scroll offset after the next paint (content must exist first).
pub fn scroll_to(y: f64) {
    document::eval(&format!(
        "requestAnimationFrame(() => requestAnimationFrame(() => window.scrollTo({{ top: {y}, behavior: 'instant' }})));"
    ));
}

pub fn scroll_top() {
    document::eval("window.scrollTo({ top: 0, behavior: 'instant' });");
}

/// Smoothly bring an element into the middle of the viewport.
pub fn scroll_into_view(id: &str) {
    let id = serde_json::to_string(id).unwrap_or_default();
    document::eval(&format!(
        "requestAnimationFrame(() => requestAnimationFrame(() => document.getElementById({id})?.scrollIntoView({{ block: 'center', behavior: 'smooth' }})));"
    ));
}

pub fn copy_text(text: &str) {
    let t = serde_json::to_string(text).unwrap_or_default();
    document::eval(&format!("navigator.clipboard && navigator.clipboard.writeText({t});"));
}

/// Native share sheet where available, clipboard otherwise.
pub fn share_text(title: &str, text: &str) {
    let (t, x) = (serde_json::to_string(title).unwrap_or_default(), serde_json::to_string(text).unwrap_or_default());
    document::eval(&format!(
        "if (navigator.share) {{ navigator.share({{ title: {t}, text: {x} }}).catch(() => {{}}); }} else if (navigator.clipboard) {{ navigator.clipboard.writeText({x}); }}"
    ));
}

pub fn open_url(url: &str) {
    let u = serde_json::to_string(url).unwrap_or_default();
    document::eval(&format!("window.open({u}, '_blank', 'noopener');"));
}

/// Drop service-worker caches and reload so the newest deployed build is served.
pub fn reload_fresh() {
    document::eval(
        r#"
        (async () => {
            try {
                if ('caches' in window) { for (const k of await caches.keys()) await caches.delete(k); }
                if (navigator.serviceWorker) { for (const r of await navigator.serviceWorker.getRegistrations()) await r.update(); }
            } catch (e) {}
            location.reload();
        })();
        "#,
    );
}

/// Register the offline service worker (web, https only).
pub fn register_service_worker() {
    document::eval(
        "if ('serviceWorker' in navigator && location.protocol === 'https:') { navigator.serviceWorker.register('sw.js').catch(() => {}); }",
    );
}

/// Haptic tick on devices that support it.
pub fn haptic() {
    document::eval("navigator.vibrate && navigator.vibrate(8);");
}

/// Track scroll direction on `<html data-chrome="shown|hidden">` so reading toolbars can tuck
/// away while you read down the page and return when you scroll up or reach the end.
pub fn install_scroll_chrome() {
    document::eval(
        r#"
        if (!window.__dewChrome) {
            window.__dewChrome = true;
            let last = window.scrollY, ticking = false;
            const root = document.documentElement;
            root.dataset.chrome = 'shown';
            window.addEventListener('scroll', () => {
                if (ticking) return; ticking = true;
                requestAnimationFrame(() => {
                    const y = window.scrollY, dy = y - last;
                    const atEnd = window.innerHeight + y >= document.documentElement.scrollHeight - 48;
                    if (y < 80 || atEnd || dy < -6) root.dataset.chrome = 'shown';
                    else if (dy > 6) root.dataset.chrome = 'hidden';
                    if (Math.abs(dy) > 6 || y < 80) last = y;
                    ticking = false;
                });
            }, { passive: true });
        }
        "#,
    );
}

/// Bring the reading toolbars back (after turning a chapter).
pub fn show_chrome() {
    document::eval("document.documentElement.dataset.chrome = 'shown';");
}
