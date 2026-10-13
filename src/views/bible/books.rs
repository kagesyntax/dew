//! Book picker with an inline chapter grid and a "Go to" box that understands references
//! like "jn 3:16" or "1 cor 13".

use super::translation_picker::TranslationPill;
use crate::routes::Route;
use crate::state::use_app;
use crate::ui::{tips, LargeTitle, Tip};
use church_core::scripture::{parse, parser::parse_book, Testament, BOOKS};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronDown, Search};

#[component]
pub fn BibleHome() -> Element {
    let app = use_app();
    let mut query = use_signal(String::new);
    let mut expanded: Signal<Option<&'static str>> = use_signal(|| {
        app.progress.peek().last_read.as_ref().and_then(|p| church_core::scripture::books::by_id(&p.book)).map(|b| b.id)
    });
    let nav = navigator();

    let q = query.read().trim().to_lowercase();
    let jump = parse(&q);
    let filtered: Vec<_> = BOOKS
        .iter()
        .filter(|b| {
            q.is_empty()
                || jump.is_some()
                || b.name.to_lowercase().contains(&q)
                || parse_book(&q).is_some_and(|x| x.id == b.id)
        })
        .collect();

    let go = move |_| {
        let text = query.read().clone();
        if let Some(r) = parse(&text) {
            nav.push(Route::reader_focus(r.first()));
        } else if let Some(b) = parse_book(&text) {
            nav.push(Route::reader(b.id, 1));
        }
    };

    rsx! {
        LargeTitle { title: "Bible", trailing: rsx! { TranslationPill {} } }
        div { class: "mx-auto max-w-2xl px-4",
            form {
                class: "mb-4 flex items-center gap-2 rounded-[10px] bg-fill px-2.5 py-2",
                onsubmit: go,
                Search { size: 18, class: "text-label-2" }
                input {
                    class: "flex-1 bg-transparent text-[17px] outline-none placeholder:text-label-2",
                    placeholder: "Search or go to “John 3:16”",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                }
            }
            Tip {
                id: tips::BIBLE_GOTO,
                title: "Go straight to a verse",
                message: "Type a reference like jn 3:16 or 1 cor 13 and tap Go.",
                icon: rsx! { Search { size: 22, stroke_width: "2" } },
                class: "mb-4",
            }
            if let Some(r) = jump {
                button {
                    class: "press mb-4 w-full rounded-card bg-accent px-4 py-3 text-left text-[17px] font-semibold text-white",
                    onclick: move |_| { nav.push(Route::reader_focus(r.first())); },
                    "Go to {r.label()}"
                }
            }
            for testament in [Testament::Old, Testament::New] {
                if filtered.iter().any(|b| b.testament == testament) {
                    h2 { class: "mt-2 mb-1.5 px-4 text-[13px] uppercase tracking-wide text-label-2",
                        if testament == Testament::Old { "Old Testament" } else { "New Testament" }
                    }
                    div { class: "mb-6 overflow-hidden rounded-card bg-surface [&>*:not(:last-child)]:border-b [&>*:not(:last-child)]:border-separator",
                        for book in filtered.iter().copied().filter(|b| b.testament == testament) {
                            div { key: "{book.id}",
                                button {
                                    class: "press flex w-full items-center justify-between px-4 py-3 text-left",
                                    onclick: move |_| {
                                        let next = if *expanded.peek() == Some(book.id) { None } else { Some(book.id) };
                                        expanded.set(next);
                                    },
                                    span { class: "text-[17px]", "{book.name}" }
                                    ChevronDown {
                                        size: 18,
                                        stroke_width: "2.5",
                                        class: if expanded() == Some(book.id) { "rotate-180 text-label-3 transition-transform" } else { "text-label-3 transition-transform" },
                                    }
                                }
                                if expanded() == Some(book.id) {
                                    div { class: "fade-in grid grid-cols-6 gap-2 px-4 pb-4 sm:grid-cols-8",
                                        for ch in 1..=book.chapters {
                                            Link {
                                                key: "{ch}",
                                                to: Route::reader(book.id, ch),
                                                class: "press flex aspect-square items-center justify-center rounded-lg bg-fill text-[16px] font-medium",
                                                "{ch}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
