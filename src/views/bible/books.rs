//! Book and chapter chooser, used by the Bible tab and by the reader's passage picker sheet.
//! A search field filters books and understands references like "jn 3:16" or "1 cor 13";
//! tapping a book reveals its chapter chips in place, with the chapter you're on filled in.

use super::translation_picker::TranslationPill;
use crate::routes::Route;
use crate::state::use_app;
use crate::ui::{tips, Floaters, LargeTitle, Tip};
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
    let reading =
        current.as_ref().and_then(|(b, c)| church_core::scripture::books::by_id(b).map(|m| (m.id, m.name, *c)));
    rsx! {
        div { class: "hero-block",
            Floaters {}
            LargeTitle {
                title: "Bible".to_string(),
                subtitle: "Read".to_string(),
                lead: Some("Four public-domain translations, offline.".to_string()),
                trailing: rsx! { TranslationPill {} },
            }
        }
        div { class: "wrap stack pb-10",
            if let Some((id, name, chapter)) = reading {
                Link { to: Route::reader(id, chapter), class: "tile ink link press block rv",
                    div { class: "flex items-center justify-between",
                        p { class: "eyebrow", "Continue reading" }
                        span { class: "lnk", "Open" }
                    }
                    p { class: "mt-2 text-[22px] font-semibold tracking-tight", "{name} {chapter}" }
                }
            }
            Tip {
                id: tips::BIBLE_GOTO,
                title: "Go straight to a verse",
                message: "Type a reference like jn 3:16 or 1 cor 13 and tap Go.",
                icon: rsx! { Search { size: 20, stroke_width: "1.9" } },
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
            class: if sticky_search {
                "searchbar sticky top-0 z-10 mb-4"
            } else {
                "searchbar mb-1"
            },
            onsubmit: go,
            Search { size: 17, stroke_width: "2.2" }
            input {
                r#type: "search",
                enterkeyhint: "go",
                autofocus,
                placeholder: "Book or reference, like John 3:16",
                value: "{query}",
                "aria-label": "Search books or type a reference",
                oninput: move |e| query.set(e.value()),
            }
            if !q.is_empty() {
                button {
                    r#type: "button",
                    class: "press absolute top-1/2 right-3 -translate-y-1/2 text-label-3",
                    aria_label: "Clear",
                    onclick: move |_| query.set(String::new()),
                    CircleX { size: 17, stroke_width: "2.2" }
                }
            }
        }
        if let Some(r) = jump {
            button {
                class: "tile tint press mb-3 flex w-full items-center justify-between text-left",
                onclick: move |_| on_pick.call(Route::reader_focus(r.first())),
                span { class: "text-[16px] font-semibold", "Go to {r.label()}" }
                span { class: "lnk", "Open" }
            }
        }
        if filtered.is_empty() {
            p { class: "py-10 text-center text-[15px] text-label-2", "No books match “{query}”." }
        }
        for testament in [Testament::Old, Testament::New] {
            if filtered.iter().any(|b| b.testament == testament) {
                h2 { class: "eyebrow mt-5 mb-2 px-1",
                    if testament == Testament::Old { "Old Testament" } else { "New Testament" }
                }
                div { class: "tile flush rows",
                    for book in filtered.iter().copied().filter(|b| b.testament == testament) {
                        {
                            let open = expanded() == Some(book.id) || only == Some(book.id);
                            let is_cur = cur_book == book.id;
                            rsx! {
                                div { key: "{book.id}", id: "book-{book.id}",
                                    button {
                                        class: "row press",
                                        "aria-expanded": if open { "true" } else { "false" },
                                        onclick: move |_| {
                                            let next = if *expanded.peek() == Some(book.id) { None } else { Some(book.id) };
                                            expanded.set(next);
                                        },
                                        span { class: if is_cur { "flex-1 truncate text-[16px] font-semibold text-accent" } else { "flex-1 truncate text-[16px]" }, "{book.name}" }
                                        span { class: "micro text-label-3", "{book.chapters}" }
                                        ChevronDown {
                                            size: 16,
                                            stroke_width: "2.2",
                                            class: if open { "rotate-180 text-label-3 transition-transform" } else { "text-label-3 transition-transform" },
                                        }
                                    }
                                    if open {
                                        div { class: "fade-in grid grid-cols-6 gap-2 px-[18px] pb-4 sm:grid-cols-8",
                                            for ch in 1..=book.chapters {
                                                button {
                                                    key: "{ch}",
                                                    class: if is_cur && ch == cur_ch { "chip on mini justify-center" } else { "chip mini justify-center" },
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
        div { class: "h-2" }
    }
}
