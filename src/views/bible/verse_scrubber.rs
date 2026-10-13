//! Verse scrubber: the fast-scroll handle iOS shows in Photos, Files and Contacts, applied to a
//! chapter. Long chapters only (Psalm 119 has 176 verses): it fades in on the right edge while you
//! scroll and fades out when you stop. Drag it and the text follows live under your thumb, a bubble
//! names the verse ("Psalms 119:88"), and letting go leaves that verse briefly lit.
//!
//! Dragging runs in JS (pointer capture at 60fps); Rust only renders the parts and the data.

use dioxus::prelude::*;

/// Chapters shorter than this fit in a few flicks; a scrubber would only add noise.
pub const MIN_VERSES: usize = 25;

#[component]
pub fn VerseScrubber(label: String, count: usize) -> Element {
    use_hook(install);
    rsx! {
        div {
            class: "verse-scrubber",
            "data-count": "{count}",
            "data-label": "{label}",
            role: "slider",
            aria_label: "Scrub to a verse",
            aria_valuemin: "1",
            aria_valuemax: "{count}",
            div { class: "vs-thumb" }
            div { class: "vs-bubble", span { class: "vs-ref" } }
        }
    }
}

fn install() {
    document::eval(
        r#"
        if (!window.__dewScrubber) {
            window.__dewScrubber = true;
            let hideTimer = 0, dragging = false, lastVerse = 0;
            const el = () => document.querySelector('.verse-scrubber');
            const geom = (s) => {
                const r = s.getBoundingClientRect();
                return { top: r.top + 22, height: Math.max(1, r.height - 44) };
            };
            const show = () => {
                const s = el(); if (!s) return;
                s.classList.add('is-visible');
                clearTimeout(hideTimer);
                if (!dragging) hideTimer = setTimeout(() => s.classList.remove('is-visible'), 1400);
            };
            // Thumb follows the reading position while scrolling normally.
            const sync = () => {
                const s = el(); if (!s || dragging) return;
                const n = +s.dataset.count;
                const first = document.getElementById('v1'), last = document.getElementById('v' + n);
                if (!first || !last) return;
                const a = first.getBoundingClientRect().top + scrollY - 120;
                const b = last.getBoundingClientRect().top + scrollY - 120;
                const f = Math.min(1, Math.max(0, (scrollY - a) / Math.max(1, b - a)));
                const g = geom(s);
                s.style.setProperty('--vs-y', (g.top - s.getBoundingClientRect().top + f * g.height) + 'px');
            };
            addEventListener('scroll', () => { if (!el()) return; sync(); if (scrollY > 40) show(); }, { passive: true });

            const scrubTo = (s, clientY) => {
                const g = geom(s), n = +s.dataset.count;
                const f = Math.min(1, Math.max(0, (clientY - g.top) / g.height));
                const v = 1 + Math.round(f * (n - 1));
                s.style.setProperty('--vs-y', (g.top - s.getBoundingClientRect().top + f * g.height) + 'px');
                s.querySelector('.vs-ref').textContent = s.dataset.label + ':' + v;
                s.setAttribute('aria-valuenow', v);
                if (v !== lastVerse) {
                    lastVerse = v;
                    const t = document.getElementById('v' + v);
                    if (t) scrollTo({ top: t.getBoundingClientRect().top + scrollY - 110, behavior: 'instant' });
                    if (v === 1 || v === n || v % 10 === 0) navigator.vibrate && navigator.vibrate(4);
                }
            };
            addEventListener('pointerdown', (e) => {
                const s = e.target.closest && e.target.closest('.verse-scrubber');
                if (!s) return;
                e.preventDefault();
                dragging = true; lastVerse = 0;
                s.setPointerCapture(e.pointerId);
                s.classList.add('is-visible', 'is-dragging');
                clearTimeout(hideTimer);
                scrubTo(s, e.clientY);
                const move = (ev) => scrubTo(s, ev.clientY);
                const up = () => {
                    dragging = false;
                    s.classList.remove('is-dragging');
                    s.removeEventListener('pointermove', move);
                    s.removeEventListener('pointerup', up);
                    s.removeEventListener('pointercancel', up);
                    const t = document.getElementById('v' + lastVerse);
                    if (t) { t.classList.remove('verse-flash'); void t.offsetWidth; t.classList.add('verse-flash'); }
                    show();
                };
                s.addEventListener('pointermove', move);
                s.addEventListener('pointerup', up);
                s.addEventListener('pointercancel', up);
            });
        }
        "#,
    );
}
