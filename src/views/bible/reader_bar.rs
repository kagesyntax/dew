//! The reader toolbar, docked on top of the tab bar so the two read as one piece of bottom chrome
//! (like Safari's toolbar or Apple Books' reading controls). Always visible.
//!
//!   ‹   [ John 3 | Verse 16 ]   ›
//!
//! - ‹ › turn chapters (also ← → on a keyboard).
//! - "John 3" opens Go To: any book and chapter.
//! - "Verse 16" follows your reading position live; tap it for every verse in the chapter, with
//!   first words and a filter (a number, or words like "loved the world"), and jump straight there.
//!   Shown when Settings → Find a Verse is Verse List; the Scrubber setting hides it.

use super::books::BookChooser;
use crate::components::sheet::{Sheet, SheetTitle};
use crate::platform::js;
use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronDown, ChevronLeft, ChevronRight, Search, X};

#[allow(clippy::too_many_arguments)]
#[component]
pub fn ReaderBar(
    book: String,
    book_name: String,
    chapter: u16,
    prev: Option<Route>,
    next: Option<Route>,
    verses: Vec<String>,
    start_verse: u16,
    show_verse: bool,
) -> Element {
    let mut picker = use_signal(|| false);
    let mut verse_sheet = use_signal(|| false);
    let mut current = use_signal(|| start_verse);
    let nav = navigator();
    let count = verses.len();

    // Keep "Verse N" in step with what's at the top of the screen.
    let key = format!("{book}-{chapter}");
    use_effect(use_reactive!(|key, start_verse| {
        let _ = &key;
        current.set(start_verse);
        spawn(async move {
            let mut ev = document::eval(
                r#"
                if (window.__dewVerseTrack) removeEventListener('scroll', window.__dewVerseTrack);
                let last = 0, ticking = false;
                window.__dewVerseTrack = () => {
                    if (ticking) return; ticking = true;
                    requestAnimationFrame(() => {
                        ticking = false;
                        const spans = document.querySelectorAll('.reader-text .verse');
                        if (!spans.length) return;
                        const line = 120;
                        let lo = 0, hi = spans.length - 1;
                        while (lo < hi) {
                            const mid = (lo + hi + 1) >> 1;
                            if (spans[mid].getBoundingClientRect().top <= line) lo = mid; else hi = mid - 1;
                        }
                        const atEnd = innerHeight + scrollY >= document.documentElement.scrollHeight - 8;
                        const n = atEnd && scrollY > 0 ? spans.length : lo + 1;
                        if (n !== last) { last = n; dioxus.send(n); }
                    });
                };
                addEventListener('scroll', window.__dewVerseTrack, { passive: true });
                "#,
            );
            while let Ok(n) = ev.recv::<u16>().await {
                current.set(n);
            }
        });
    }));

    // ← / → turn chapters on a keyboard.
    let (p2, n2) = (prev.clone(), next.clone());
    use_effect(use_reactive!(|p2, n2| {
        spawn(async move {
            let mut keys = document::eval(
                r#"
                if (window.__dewKeys) window.removeEventListener('keydown', window.__dewKeys);
                window.__dewKeys = (e) => {
                    const t = e.target;
                    if (e.metaKey || e.ctrlKey || e.altKey || (t && (t.isContentEditable || /INPUT|TEXTAREA|SELECT/.test(t.tagName)))) return;
                    if (document.querySelector('.dx-sheet-root[data-state="open"]')) return;
                    if (e.key === 'ArrowLeft') dioxus.send('prev');
                    if (e.key === 'ArrowRight') dioxus.send('next');
                };
                window.addEventListener('keydown', window.__dewKeys);
                "#,
            );
            while let Ok(dir) = keys.recv::<String>().await {
                let target = if dir == "prev" { p2.clone() } else { n2.clone() };
                if let Some(r) = target {
                    navigator().push(r);
                }
            }
        });
    }));

    let here = Some((book.clone(), chapter));
    let arrow =
        "press flex h-11 w-11 shrink-0 items-center justify-center rounded-full text-accent disabled:text-label-3";
    let seg = "press flex h-full min-w-0 items-center justify-center gap-1 px-3.5";

    rsx! {
        div { class: "reader-bar bar-blur fixed inset-x-0 z-30",
            div { class: "mx-auto flex h-[50px] max-w-2xl items-center gap-2 px-2",
                button {
                    class: arrow,
                    aria_label: "Previous chapter",
                    disabled: prev.is_none(),
                    onclick: move |_| { if let Some(r) = prev.clone() { js::haptic(); nav.push(r); } },
                    ChevronLeft { size: 26, stroke_width: "2.4" }
                }
                div { class: "flex h-9 min-w-0 flex-1 items-stretch justify-center",
                    div { class: "flex min-w-0 max-w-full items-stretch overflow-hidden rounded-full bg-fill",
                        button {
                            class: "{seg} text-[15px] font-semibold",
                            aria_label: "Go to book and chapter",
                            onclick: move |_| picker.set(true),
                            span { class: "truncate", "{book_name} {chapter}" }
                            if !show_verse || count == 0 {
                                ChevronDown { size: 14, stroke_width: "3", class: "shrink-0 text-label-2" }
                            }
                        }
                        if show_verse && count > 0 {
                            span { class: "my-2 w-px shrink-0 bg-separator" }
                            button {
                                class: "{seg} shrink-0 text-[15px] tabular-nums",
                                aria_label: "Choose a verse, at verse {current}",
                                onclick: move |_| verse_sheet.set(true),
                                span { class: "text-label-2", "Verse" }
                                span { class: "font-semibold", "{current}" }
                                ChevronDown { size: 14, stroke_width: "3", class: "shrink-0 text-label-2" }
                            }
                        }
                    }
                }
                button {
                    class: arrow,
                    aria_label: "Next chapter",
                    disabled: next.is_none(),
                    onclick: move |_| { if let Some(r) = next.clone() { js::haptic(); nav.push(r); } },
                    ChevronRight { size: 26, stroke_width: "2.4" }
                }
            }
        }

        Sheet { open: picker(), on_open_change: move |v| picker.set(v), "data-side": "bottom", class: "passage-sheet",
            SheetHeader { title: "Go To".to_string(), subtitle: None, onclose: move |_| picker.set(false) }
            div { class: "px-4",
                if picker() {
                    BookChooser {
                        current: here.clone(),
                        reveal_current: true,
                        sticky_search: true,
                        on_pick: move |r: Route| {
                            picker.set(false);
                            nav.push(r);
                        },
                    }
                }
            }
        }

        Sheet { open: verse_sheet(), on_open_change: move |v| verse_sheet.set(v), "data-side": "bottom", class: "passage-sheet",
            if verse_sheet() {
                VerseList {
                    title: format!("{book_name} {chapter}"),
                    onclose: move |_| verse_sheet.set(false),
                    verses: verses.clone(),
                    current: current(),
                    on_pick: move |n: u16| {
                        verse_sheet.set(false);
                        current.set(n);
                        jump_to_verse(n);
                    },
                }
            }
        }
    }
}

#[component]
fn SheetHeader(title: String, subtitle: Option<String>, onclose: EventHandler<()>) -> Element {
    rsx! {
        div { class: "flex items-start justify-between px-5 pt-1 pb-3",
            div {
                SheetTitle { class: "text-[22px] font-bold tracking-tight", "{title}" }
                if let Some(s) = subtitle {
                    p { class: "text-[15px] text-label-2", "{s}" }
                }
            }
            button {
                class: "press mt-0.5 rounded-full bg-fill p-1.5 text-label-2",
                aria_label: "Close",
                onclick: move |_| onclose.call(()),
                X { size: 18, stroke_width: "2.6" }
            }
        }
    }
}

/// Every verse in the chapter: number and opening words. Type a number to go straight to it, or
/// words to find the verse that says them.
#[component]
fn VerseList(
    title: String,
    onclose: EventHandler<()>,
    verses: Vec<String>,
    current: u16,
    on_pick: EventHandler<u16>,
) -> Element {
    let count = verses.len();
    let mut query = use_signal(String::new);
    let q = query.read().trim().to_lowercase();

    let matches: Vec<(u16, &String)> = verses
        .iter()
        .enumerate()
        .map(|(i, v)| (i as u16 + 1, v))
        .filter(|(n, v)| {
            if q.is_empty() {
                true
            } else if q.chars().all(|c| c.is_ascii_digit()) {
                n.to_string().starts_with(&q)
            } else {
                v.to_lowercase().contains(&q)
            }
        })
        .collect();
    let first = matches.first().map(|(n, _)| *n);

    // Open with the verse you're on in view.
    use_effect(move || {
        document::eval(&format!(
            "requestAnimationFrame(() => {{ const r = document.getElementById('vl-{current}'); if (!r) return; let s = r.parentElement; while (s && !(s.scrollHeight > s.clientHeight && /auto|scroll/.test(getComputedStyle(s).overflowY))) s = s.parentElement; if (s) s.scrollTop += r.getBoundingClientRect().top - s.getBoundingClientRect().top - (s.clientHeight + 130) / 2 + r.offsetHeight / 2; }});"
        ));
    });

    rsx! {
        // Title and filter stay put while the verses scroll under them.
        div { class: "sticky top-0 z-10 bg-surface pb-3",
            SheetHeader {
                title,
                subtitle: Some(if count == 1 { "1 verse".to_string() } else { format!("{count} verses") }),
                onclose: move |_| onclose.call(()),
            }
            form {
                class: "mx-4 flex items-center gap-2 rounded-[10px] bg-surface-2 px-2.5 py-2",
                onsubmit: move |e| { e.prevent_default(); if let Some(n) = first { on_pick.call(n); } },
                Search { size: 18, class: "shrink-0 text-label-2" }
                input {
                    class: "min-w-0 flex-1 bg-transparent text-[17px] outline-none placeholder:text-label-2",
                    r#type: "search",
                    enterkeyhint: "go",
                    inputmode: "search",
                    placeholder: "Verse number or words",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                }
            }
        }
        div { class: "px-4 pb-8",
            if matches.is_empty() {
                p { class: "py-10 text-center text-[15px] text-label-2", "No verse in this chapter says “{query}”." }
            }
            div { class: "overflow-hidden rounded-card bg-surface-2",
                for (n, v) in matches {
                    Fragment { key: "{n}",
                        if let (true, Some((letter, name, _))) = (q.is_empty(), stanza(v)) {
                            div { class: "flex items-baseline gap-2 bg-surface px-4 pt-4 pb-1.5 text-[13px] font-semibold uppercase tracking-wide text-label-2",
                                span { class: "text-[17px] font-normal normal-case text-label", "{letter}" }
                                "{name}"
                            }
                        }
                        button {
                            id: "vl-{n}",
                            class: if n == current { "verse-row press flex w-full items-baseline gap-3 bg-fill px-4 py-3 text-left" } else { "verse-row press flex w-full items-baseline gap-3 px-4 py-3 text-left" },
                            onclick: move |_| on_pick.call(n),
                            span { class: if n == current { "w-8 shrink-0 text-right text-[15px] font-bold tabular-nums text-accent" } else { "w-8 shrink-0 text-right text-[15px] font-semibold tabular-nums text-label-2" }, "{n}" }
                            span { class: "line-clamp-2 min-w-0 font-serif text-[16px] leading-snug", "{stanza(v).map(|(_, _, rest)| rest).unwrap_or(v)}" }
                        }
                    }
                }
            }
        }
    }
}

/// Bring verse `n` to the top of the screen and light it briefly.
fn jump_to_verse(n: u16) {
    document::eval(&format!(
        r#"setTimeout(() => {{
            const t = document.getElementById('v{n}');
            if (!t) return;
            scrollTo({{ top: t.getBoundingClientRect().top + scrollY - 110, behavior: 'instant' }});
            t.classList.remove('verse-flash'); void t.offsetWidth; t.classList.add('verse-flash');
        }}, 60);"#
    ));
}

/// Psalm 119's acrostic stanzas open with a Hebrew letter and its name ("א ALEPH. Blessed…").
/// Returns (letter, Name, rest of the verse) so the list can show them as section headers.
fn stanza(v: &str) -> Option<(char, String, &str)> {
    let mut chars = v.chars();
    let letter = chars.next().filter(|c| ('\u{05D0}'..='\u{05EA}').contains(c))?;
    let after = v[letter.len_utf8()..].trim_start();
    let (name, rest) = after.split_once('.')?;
    if name.is_empty() || name.len() > 8 || !name.chars().all(|c| c.is_ascii_uppercase()) {
        return None;
    }
    let mut pretty = name.to_lowercase();
    pretty[..1].make_ascii_uppercase();
    Some((letter, pretty, rest.trim_start()))
}
