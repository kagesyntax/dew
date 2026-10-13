//! Book and chapter chooser, used by the Bible tab and by the reader's passage picker sheet.
//! A search field filters books and understands references like "jn 3:16" or "1 cor 13";
//! tapping a book reveals its chapter grid in place, with the chapter you're on filled in.

use super::translation_picker::TranslationPill;
use crate::routes::Route;
use crate::state::use_app;
use crate::ui::{tips, LargeTitle, Tip};
use church_core::scripture::{parse, parser::parse_book, Testament, BOOKS};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronDown, CircleX, Search};

#[component]
pub fn BibleHome() -> Element {
    let app = use_app();
    // With "Open Where I Left Off" switched off the tab is a plain list of books.
    let current = if app.settings.peek().resume_reading {
        app.progress.peek().last_read.as_ref().map(|p| (p.book.clone(), p.chapter))
    } else {
        None
    };
    let nav = navigator();
    rsx! {
        LargeTitle { title: "Bible", trailing: rsx! { TranslationPill {} } }
        div { class: "mx-auto max-w-2xl px-4",
            Tip {
                id: tips::BIBLE_GOTO,
                title: "Go straight to a verse",
                message: "Type a reference like jn 3:16 or 1 cor 13 and tap Go.",
                icon: rsx! { Search { size: 22, stroke_width: "2" } },
                class: "mb-4",
            }
            BookChooser { current, on_pick: move |r: Route| { nav.push(r); } }
        }
    }
}

/// Search field + Old/New Testament book lists with inline chapter grids.
/// `current` is the (book id, chapter) the reader is on; that book starts expanded.
#[component]
pub fn BookChooser(
    current: Option<(String, u16)>,
    on_pick: EventHandler<Route>,
    #[props(default)] autofocus: bool,
    #[props(default)] reveal_current: bool,
    #[props(default)] sticky_search: bool,
) -> Element {
    let mut query = use_signal(String::new);
    let cur = current.clone();
    let mut expanded: Signal<Option<&'static str>> =
        use_signal(move || cur.as_ref().and_then(|(b, _)| church_core::scripture::books::by_id(b)).map(|b| b.id));

    // In the picker sheet, open on the book you're reading.
    let reveal = current.as_ref().filter(|_| reveal_current).map(|(b, _)| format!("book-{b}"));
    use_effect(move || {
        if let Some(id) = &reveal {
            crate::platform::js::scroll_into_view(id);
        }
    });

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
    // Typing a single book name opens its chapters straight away.
    let only = (filtered.len() == 1 && !q.is_empty()).then(|| filtered[0].id);
    let (cur_book, cur_ch) = current.clone().unwrap_or_default();

    let go = move |_| {
        let text = query.read().clone();
        if let Some(r) = parse(&text) {
            on_pick.call(Route::reader_focus(r.first()));
        } else if let Some(b) = parse_book(&text) {
            on_pick.call(Route::reader(b.id, 1));
        }
    };

    rsx! {
        form {
            class: if sticky_search { "sticky top-0 z-10 mb-4 flex items-center gap-2 rounded-[10px] bg-surface-2 px-2.5 py-2 shadow-[0_-10px_0_10px_var(--app-surface)]" } else { "mb-4 flex items-center gap-2 rounded-[10px] bg-fill px-2.5 py-2" },
            onsubmit: go,
            Search { size: 18, class: "shrink-0 text-label-2" }
            input {
                class: "min-w-0 flex-1 bg-transparent text-[17px] outline-none placeholder:text-label-2",
                r#type: "search",
                enterkeyhint: "go",
                autofocus,
                placeholder: "Book or reference, like John 3:16",
                value: "{query}",
                oninput: move |e| query.set(e.value()),
            }
            if !q.is_empty() {
                button { r#type: "button", class: "press text-label-3", aria_label: "Clear", onclick: move |_| query.set(String::new()),
                    CircleX { size: 18, stroke_width: "2.2" }
                }
            }
        }
        if let Some(r) = jump {
            button {
                class: "press mb-4 w-full rounded-card bg-accent px-4 py-3 text-left text-[17px] font-semibold text-white",
                onclick: move |_| on_pick.call(Route::reader_focus(r.first())),
                "Go to {r.label()}"
            }
        }
        if filtered.is_empty() {
            p { class: "py-10 text-center text-[15px] text-label-2", "No books match “{query}”." }
        }
        for testament in [Testament::Old, Testament::New] {
            if filtered.iter().any(|b| b.testament == testament) {
                h2 { class: "mt-2 mb-1.5 px-4 text-[13px] uppercase tracking-wide text-label-2",
                    if testament == Testament::Old { "Old Testament" } else { "New Testament" }
                }
                div { class: "mb-6 overflow-hidden rounded-card bg-surface [&>*:not(:last-child)]:border-b [&>*:not(:last-child)]:border-separator",
                    for book in filtered.iter().copied().filter(|b| b.testament == testament) {
                        {
                            let open = expanded() == Some(book.id) || only == Some(book.id);
                            let is_cur = cur_book == book.id;
                            rsx! {
                                div { key: "{book.id}", id: "book-{book.id}",
                                    button {
                                        class: "press flex w-full items-center justify-between px-4 py-3 text-left",
                                        "aria-expanded": if open { "true" } else { "false" },
                                        onclick: move |_| {
                                            let next = if *expanded.peek() == Some(book.id) { None } else { Some(book.id) };
                                            expanded.set(next);
                                        },
                                        span { class: if is_cur { "text-[17px] font-semibold text-accent" } else { "text-[17px]" }, "{book.name}" }
                                        ChevronDown {
                                            size: 18,
                                            stroke_width: "2.5",
                                            class: if open { "rotate-180 text-label-3 transition-transform" } else { "text-label-3 transition-transform" },
                                        }
                                    }
                                    if open {
                                        div { class: "fade-in grid grid-cols-6 gap-2 px-4 pb-4 sm:grid-cols-8",
                                            for ch in 1..=book.chapters {
                                                button {
                                                    key: "{ch}",
                                                    class: if is_cur && ch == cur_ch { "press flex aspect-square items-center justify-center rounded-lg bg-accent text-[16px] font-semibold text-white" } else { "press flex aspect-square items-center justify-center rounded-lg bg-fill text-[16px] font-medium" },
                                                    "aria-current": if is_cur && ch == cur_ch { "true" } else { "false" },
                                                    onclick: move |_| on_pick.call(Route::reader(book.id, ch)),
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
}
