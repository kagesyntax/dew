use crate::manual::{use_manual, SyncStatus};
use crate::platform::clock;
use crate::routes::Route;
use crate::state::use_app;
use crate::ui::{Floaters, LargeTitle, SectionHeading};
use church_core::manual::Lesson;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronRight, CircleCheck, Search, X};

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Open,
    Done,
}

#[component]
pub fn ManualHome() -> Element {
    let app = use_app();
    let store = use_manual();
    let manual = store.manual.read().clone();
    let mut query = use_signal(String::new);
    let mut filter = use_signal(|| Filter::All);
    let today = clock::today_iso();
    let current = manual.current_lesson(&today).cloned();
    let last = app.progress.read().last_lesson.clone().and_then(|id| manual.lesson(&id).cloned());
    let q = query.read().clone();

    let mut lessons: Vec<Lesson> = manual.lessons.iter().filter(|l| l.matches(&q)).cloned().collect();
    lessons.sort_by(|a, b| b.date.cmp(&a.date));
    let total = manual.lessons.len();
    let completed = app.progress.read().completed_lessons.len();
    let sync_note = match store.sync.read().clone() {
        SyncStatus::Synced(n) if n > 0 => Some(format!("Updated from the church database · {n} lessons")),
        SyncStatus::Offline(_) => Some("Offline · showing saved lessons".to_string()),
        _ => None,
    };

    rsx! {
        div { class: "hero-block",
            Floaters {}
            LargeTitle {
                title: "Sunday School".to_string(),
                subtitle: manual.title.clone(),
                lead: Some(format!("{completed} of {total} lessons finished this year.")),
            }
            div { class: "wrap relative z-10 stack pb-3",
                div { class: "searchbar rv",
                    Search { size: 17, stroke_width: "2.2" }
                    input {
                        r#type: "search",
                        placeholder: "Search lessons, verses, topics",
                        value: "{query}",
                        "aria-label": "Search lessons",
                        oninput: move |e| query.set(e.value()),
                    }
                    if !q.is_empty() {
                        button {
                            class: "press absolute top-1/2 right-3 -translate-y-1/2 text-label-3",
                            aria_label: "Clear search",
                            onclick: move |_| query.set(String::new()),
                            X { size: 17, stroke_width: "2.2" }
                        }
                    }
                }
                div { class: "chips rv d1",
                    for (f, label) in [(Filter::All, "All lessons"), (Filter::Open, "To do"), (Filter::Done, "Finished")] {
                        button {
                            key: "{label}",
                            class: if filter() == f { "chip on" } else { "chip" },
                            onclick: move |_| filter.set(f),
                            "{label}"
                        }
                    }
                }
            }
        }

        div { class: "wrap stack pt-4 pb-10",
            if q.is_empty() && filter() == Filter::All {
                if let Some(l) = current.clone() {
                    Link { to: Route::LessonView { id: l.id.clone() }, class: "tile ink link press block rv",
                        div { class: "flex items-start justify-between gap-3",
                            p { class: "eyebrow", "This Sunday" }
                            span { class: "micro tabular-nums opacity-60", "{clock::short_date(&l.date)}" }
                        }
                        h2 { class: "h2 mt-2", "{l.title}" }
                        p { class: "caption mt-1 opacity-70", "Unit {l.unit_number} · {l.unit_title}" }
                        div { class: "mt-4 flex items-center justify-between gap-3",
                            p { class: "caption opacity-60", "{l.text}" }
                            ChevronRight { size: 18, stroke_width: "2.4", class: "shrink-0 opacity-60" }
                        }
                    }
                }
                if let Some(l) = last.filter(|l| Some(&l.id) != current.as_ref().map(|c| &c.id)) {
                    Link { to: Route::LessonView { id: l.id.clone() }, class: "tile link press block rv d1",
                        div { class: "flex items-center gap-3",
                            div { class: "min-w-0 flex-1",
                                p { class: "eyebrow", "Pick up where you left off" }
                                p { class: "mt-1 truncate text-[17px] font-semibold tracking-tight", "{l.title}" }
                            }
                            ChevronRight { size: 18, class: "shrink-0 text-label-3", stroke_width: "2.2" }
                        }
                    }
                }
            }

            div { class: "flex items-end justify-between gap-3",
                SectionHeading { eyebrow: "Library".to_string(), title: "All lessons".to_string() }
                p { class: "micro pb-3 text-label-3", "{completed} of {total} done" }
            }

            {
                let shown: Vec<Lesson> = lessons
                    .into_iter()
                    .filter(|l| match filter() {
                        Filter::All => true,
                        Filter::Open => !app.progress.read().is_completed(&l.id),
                        Filter::Done => app.progress.read().is_completed(&l.id),
                    })
                    .collect();
                if shown.is_empty() {
                    rsx! {
                        div { class: "tile soft loose text-center",
                            p { class: "text-[17px] font-semibold", "Nothing here yet" }
                            p { class: "caption mt-1 text-label-2",
                                if q.is_empty() { "Finish a lesson and it will show up here." } else { "No lessons match “{q}”." }
                            }
                        }
                    }
                } else {
                    rsx! {
                        div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2",
                            for l in shown {
                                {
                                    let done = app.progress.read().is_completed(&l.id);
                                    rsx! {
                                        Link { key: "{l.id}", to: Route::LessonView { id: l.id.clone() }, class: "tile link press block rv",
                                            div { class: "flex items-start justify-between gap-2",
                                                p { class: "eyebrow", "{clock::short_date(&l.date)}" }
                                                if done {
                                                    CircleCheck { size: 17, class: "shrink-0 text-accent", stroke_width: "2.2" }
                                                } else {
                                                    span { class: "micro text-label-3", "Study {l.study_number}" }
                                                }
                                            }
                                            h3 { class: "h3 mt-1.5", "{l.title}" }
                                            p { class: "caption mt-1 line-clamp-2 text-label-2", "{l.kicker()}" }
                                            p { class: "micro mt-3 text-label-3", "{l.text}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            p { class: "micro text-center text-label-3", "{manual.edition}" }
            if let Some(n) = sync_note {
                p { class: "micro text-center text-label-3", "{n}" }
            }
        }
    }
}
