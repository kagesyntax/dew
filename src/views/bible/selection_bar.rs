//! Action bar for selected verses: highlight colours, copy, share, bookmark.

use crate::bible::{load_book, translation};
use crate::platform::js;
use crate::state::progress::Highlight;
use crate::state::use_app;
use church_core::scripture::{books, verse_key};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Bookmark, Copy, Eraser, Share, X};
use std::collections::BTreeSet;

fn label(book: &str, chapter: u16, verses: &BTreeSet<u16>) -> String {
    let name = books::by_id(book).map(|b| b.name).unwrap_or(book);
    let list: Vec<u16> = verses.iter().copied().collect();
    let contiguous = list.windows(2).all(|w| w[1] == w[0] + 1);
    match (list.first(), list.last()) {
        (Some(a), Some(b)) if a == b => format!("{name} {chapter}:{a}"),
        (Some(a), Some(b)) if contiguous => format!("{name} {chapter}:{a}–{b}"),
        _ => format!("{name} {chapter}:{}", list.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")),
    }
}

#[component]
pub fn SelectionBar(book: String, chapter: u16, selected: Signal<BTreeSet<u16>>) -> Element {
    let mut app = use_app();
    let sel = selected.read().clone();
    let title = label(&book, chapter, &sel);

    let text_of = {
        let book = book.clone();
        move |share: bool| {
            let book = book.clone();
            let sel = selected.peek().clone();
            let tr = app.settings.peek().translation.clone();
            spawn(async move {
                if let Ok(data) = load_book(&tr, &book).await {
                    let ch = &data[chapter as usize - 1];
                    let body =
                        sel.iter().filter_map(|v| ch.get(*v as usize - 1)).cloned().collect::<Vec<_>>().join(" ");
                    let lbl = label(&book, chapter, &sel);
                    let out = format!("“{body}” — {lbl} ({})", translation(&tr).abbreviation);
                    if share {
                        js::share_text(&lbl, &out)
                    } else {
                        js::copy_text(&out)
                    }
                }
            });
        }
    };
    let copy = text_of.clone();
    let share = text_of;

    let set_hl = {
        let book = book.clone();
        move |h: Option<Highlight>| {
            let mut p = app.progress.write();
            for v in selected.peek().iter() {
                let k = verse_key(&book, chapter, *v);
                match h {
                    Some(c) => {
                        p.highlights.insert(k, c);
                    }
                    None => {
                        p.highlights.remove(&k);
                    }
                }
            }
            drop(p);
            selected.write().clear();
        }
    };
    let erase = set_hl.clone();
    let first_key = sel.iter().next().map(|v| verse_key(&book, chapter, *v)).unwrap_or_default();
    let is_bookmarked = app.progress.read().bookmarks.contains(&first_key);

    rsx! {
        div {
            class: "fade-in fixed inset-x-0 z-40 mx-auto max-w-md px-3",
            style: "bottom: calc(max(var(--safe-bottom), 8px) + 60px);",
            div { class: "bar-blur rounded-2xl border border-separator p-3 shadow-xl shadow-black/15",
                div { class: "mb-2 flex items-center justify-between",
                    span { class: "text-[15px] font-semibold", "{title}" }
                    button { class: "press rounded-full bg-fill p-1 text-label-2", aria_label: "Clear selection", onclick: move |_| selected.write().clear(), X { size: 16, stroke_width: "2.6" } }
                }
                div { class: "flex items-center gap-2",
                    for h in Highlight::ALL {
                        button {
                            key: "{h.as_str()}",
                            class: "press h-8 w-8 rounded-full ring-2 ring-black/5",
                            style: "background: {h.swatch()}",
                            aria_label: "Highlight {h.as_str()}",
                            onclick: { let f = set_hl.clone(); move |_| { let mut f = f.clone(); f(Some(h)) } },
                        }
                    }
                    button { class: "press flex h-8 w-8 items-center justify-center rounded-full bg-fill text-label-2", aria_label: "Remove highlight", onclick: { let f = erase.clone(); move |_| { let mut f = f.clone(); f(None) } }, Eraser { size: 16 } }
                    div { class: "flex-1" }
                    button { class: "press p-1.5 text-accent", aria_label: "Bookmark",
                        onclick: move |_| app.progress.write().toggle_bookmark(&first_key),
                        Bookmark { size: 21, stroke_width: "2.2", fill: if is_bookmarked { "currentColor" } else { "none" } }
                    }
                    button { class: "press p-1.5 text-accent", aria_label: "Copy", onclick: { let c = copy.clone(); move |_| { let c = c.clone(); c(false) } }, Copy { size: 21, stroke_width: "2.2" } }
                    button { class: "press p-1.5 text-accent", aria_label: "Share", onclick: { let s = share.clone(); move |_| { let s = s.clone(); s(true) } }, Share { size: 21, stroke_width: "2.2" } }
                }
            }
        }
    }
}
