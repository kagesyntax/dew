//! Chapter reader in the style of YouVersion / Apple Books: serif text, verse numbers,
//! tap-to-select with a highlight/copy/bookmark bar, a floating reader toolbar (previous, passage
//! picker, translation, next), and the "Back to lesson" pill when you arrived from the manual.

use super::reader_bar::ReaderBar;
use super::selection_bar::SelectionBar;
use super::verse_scrubber::{VerseScrubber, MIN_VERSES};
use crate::bible::load_book;
use crate::platform::js;
use crate::routes::Route;
use crate::state::progress::ReadingPosition;
use crate::state::use_app;
use crate::ui::{tips, BackBar, Spinner, Tip};
use church_core::scripture::{books, verse_key, BOOKS};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronRight, Highlighter};
use std::collections::BTreeSet;

fn parse_focus(focus: &str) -> Option<(u16, u16)> {
    if focus.is_empty() {
        return None;
    }
    let (a, b) = focus.split_once('-').unwrap_or((focus, focus));
    let s = a.trim().parse().ok()?;
    Some((s, b.trim().parse().unwrap_or(s)))
}

fn neighbour(book_id: &str, chapter: u16, forward: bool) -> Option<Route> {
    let book = books::by_id(book_id)?;
    let idx = books::index_of(book);
    if forward {
        if chapter < book.chapters {
            Some(Route::reader(book.id, chapter + 1))
        } else {
            BOOKS.get(idx + 1).map(|b| Route::reader(b.id, 1))
        }
    } else if chapter > 1 {
        Some(Route::reader(book.id, chapter - 1))
    } else {
        idx.checked_sub(1).and_then(|i| BOOKS.get(i)).map(|b| Route::reader(b.id, b.chapters))
    }
}

fn route_label(r: &Route) -> String {
    match r {
        Route::Reader { book, chapter, .. } => {
            format!("{} {}", books::by_id(book).map(|b| b.name).unwrap_or(book), chapter)
        }
        _ => String::new(),
    }
}

#[component]
pub fn Reader(book: String, chapter: u16, focus: String) -> Element {
    let mut app = use_app();
    let Some(meta) = books::by_id(&book) else {
        return rsx! { p { class: "p-8 text-center text-label-2", "Unknown book “{book}”." } };
    };
    let chapter = chapter.clamp(1, meta.chapters);
    let focus_range = parse_focus(&focus);
    let mut selected: Signal<BTreeSet<u16>> = use_signal(BTreeSet::new);

    // Remember position for the Bible tab and "Continue reading".
    use_effect(use_reactive!(|book, chapter, focus| {
        app.progress.write().last_read = Some(ReadingPosition { book: book.clone(), chapter, focus: focus.clone() });
        selected.write().clear();
        js::show_chrome();
    }));

    let mut book_sig = use_reactive!(|book| book);
    let text = use_resource(move || {
        let tr = app.settings.read().translation.clone();
        let b = book_sig();
        async move { load_book(&tr, &b).await }
    });

    // Scroll focused verses into view once text is on screen.
    use_effect(move || {
        if matches!(&*text.read(), Some(Ok(_))) {
            if let Some((s, _)) = focus_range {
                js::scroll_into_view(&format!("v{s}"));
            }
        }
    });

    let prev = neighbour(meta.id, chapter, false);
    let next = neighbour(meta.id, chapter, true);
    let show_numbers = app.settings.read().verse_numbers;
    let highlights = app.progress.read().highlights.clone();
    let nav = navigator();

    rsx! {
        BackBar {
            back_label: "Books",
            onback: move |_| { nav.push(Route::BibleHome {}); },
            title: format!("{} {}", meta.name, chapter),
        }

        article { class: "mx-auto max-w-2xl bg-bg px-6 pt-6 pb-24 sm:rounded-b-card",
            header { class: "mb-6 text-center",
                p { class: "text-[15px] font-semibold uppercase tracking-[0.18em] text-label-2", "{meta.name}" }
                p { class: "font-serif text-[64px] leading-none font-light", "{chapter}" }
            }
            Tip {
                id: tips::READER_SELECT,
                title: "Tap a verse",
                message: "Select verses to highlight, bookmark, copy or share them.",
                icon: rsx! { Highlighter { size: 22, stroke_width: "2" } },
                class: "mb-6 font-sans",
            }
            match &*text.read() {
                None => rsx! { Spinner {} },
                Some(Err(e)) => rsx! { p { class: "py-10 text-center text-label-2", "{e}" } },
                Some(Ok(data)) => match data.get(chapter as usize - 1) {
                    None => rsx! { p { class: "py-10 text-center text-label-2", "This chapter isn't available." } },
                    Some(verses) => rsx! {
                        if verses.len() >= MIN_VERSES {
                            VerseScrubber { key: "{meta.id}-{chapter}", label: format!("{} {}", meta.name, chapter), count: verses.len() }
                        }
                        p { class: "reader-text",
                            for (i, v) in verses.iter().enumerate() {
                                {
                                    let n = i as u16 + 1;
                                    let key = verse_key(meta.id, chapter, n);
                                    let hl = highlights.get(&key).map(|h| h.as_str()).unwrap_or("");
                                    let focused = focus_range.is_some_and(|(s, e)| n >= s && n <= e);
                                    let is_sel = selected.read().contains(&n);
                                    rsx! {
                                        span {
                                            key: "{n}",
                                            id: "v{n}",
                                            class: "verse",
                                            "data-hl": "{hl}",
                                            "data-focus": if focused && hl.is_empty() { "true" } else { "false" },
                                            "data-selected": if is_sel { "true" } else { "false" },
                                            onclick: move |_| {
                                                let mut s = selected.write();
                                                if !s.remove(&n) { s.insert(n); }
                                            },
                                            if show_numbers {
                                                sup { class: "mr-1 font-sans text-[0.58em] font-semibold text-label-2 select-none", "{n}" }
                                            }
                                            "{v} "
                                        }
                                    }
                                }
                            }
                        }
                    },
                },
            }

            // End of chapter: carry straight on, Apple Books style.
            if let Some(n) = next.clone() {
                Link { to: n.clone(), class: "press mt-10 flex items-center justify-between rounded-card bg-surface px-4 py-3.5 font-sans",
                    span {
                        span { class: "block text-[13px] text-label-2", "Next Chapter" }
                        span { class: "text-[17px] font-semibold", "{route_label(&n)}" }
                    }
                    ChevronRight { size: 22, class: "text-label-3", stroke_width: "2.4" }
                }
            }
        }

        if !selected.read().is_empty() {
            SelectionBar { book: meta.id.to_string(), chapter, selected }
        } else {
            ReaderBar { book: meta.id.to_string(), book_name: meta.name.to_string(), chapter, prev: prev.clone(), next: next.clone() }
        }
    }
}
