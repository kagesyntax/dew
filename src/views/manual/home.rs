use crate::manual::{use_manual, SyncStatus};
use crate::platform::clock;
use crate::routes::Route;
use crate::state::use_app;
use crate::ui::LargeTitle;
use church_core::manual::Lesson;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronRight, CircleCheck, Search};

#[component]
pub fn ManualHome() -> Element {
    let app = use_app();
    let store = use_manual();
    let manual = store.manual.read().clone();
    let mut query = use_signal(String::new);
    let today = clock::today_iso();
    let current = manual.current_lesson(&today).cloned();
    let last = app.progress.read().last_lesson.clone().and_then(|id| manual.lesson(&id).cloned());
    let q = query.read().clone();

    let mut lessons: Vec<Lesson> = manual.lessons.iter().filter(|l| l.matches(&q)).cloned().collect();
    lessons.sort_by(|a, b| a.date.cmp(&b.date));
    let completed = app.progress.read().completed_lessons.len();
    let total = manual.lessons.len();
    let sync_note = match store.sync.read().clone() {
        SyncStatus::Synced(n) if n > 0 => Some(format!("Updated from the church database · {n} lessons")),
        SyncStatus::Offline(_) => Some("Offline · showing saved lessons".to_string()),
        _ => None,
    };

    rsx! {
        LargeTitle { title: manual.title.clone(), subtitle: "Sunday School".to_string() }
        div { class: "mx-auto max-w-2xl px-4",
            div { class: "mb-4 flex items-center gap-2 rounded-[10px] bg-fill px-2.5 py-2",
                Search { size: 18, class: "text-label-2" }
                input {
                    class: "flex-1 bg-transparent text-[17px] outline-none placeholder:text-label-2",
                    placeholder: "Search lessons, verses, topics",
                    value: "{query}",
                    oninput: move |e| query.set(e.value()),
                }
            }

            if q.is_empty() {
                if let Some(l) = current.clone() {
                    p { class: "mb-1.5 px-4 text-[13px] uppercase tracking-wide text-label-2", "This Sunday" }
                    Link { to: Route::LessonView { id: l.id.clone() }, class: "press mb-6 block",
                        div { class: "rounded-card bg-surface p-5",
                            p { class: "text-[13px] font-semibold uppercase tracking-wide text-accent", "{clock::pretty_date(&l.date)}" }
                            p { class: "mt-1 text-[24px] font-bold leading-tight", "{l.title}" }
                            p { class: "mt-0.5 text-[15px] text-label-2", "Unit {l.unit_number} · {l.unit_title}" }
                            div { class: "mt-4 flex items-center justify-between border-t border-separator pt-3",
                                p { class: "text-[15px] text-label-2", "{l.text}" }
                                ChevronRight { size: 18, class: "text-label-3", stroke_width: "2.5" }
                            }
                        }
                    }
                }
                if let Some(l) = last.filter(|l| Some(&l.id) != current.as_ref().map(|c| &c.id)) {
                    Link { to: Route::LessonView { id: l.id.clone() }, class: "press mb-6 flex items-center gap-3 rounded-card bg-surface px-4 py-3",
                        div { class: "flex-1",
                            p { class: "text-[13px] text-label-2", "Pick up where you left off" }
                            p { class: "text-[17px] font-semibold", "{l.title}" }
                        }
                        ChevronRight { size: 20, class: "text-label-3", stroke_width: "2.5" }
                    }
                }
            }

            div { class: "mb-1.5 flex items-baseline justify-between px-4",
                p { class: "text-[13px] uppercase tracking-wide text-label-2", "All lessons" }
                p { class: "text-[13px] text-label-2", "{completed} of {total} done" }
            }
            div { class: "overflow-hidden rounded-card bg-surface [&>*:not(:last-child)]:border-b [&>*:not(:last-child)]:border-separator",
                if lessons.is_empty() {
                    p { class: "px-4 py-6 text-center text-label-2", "No lessons match “{q}”." }
                }
                for l in lessons {
                    {
                        let done = app.progress.read().is_completed(&l.id);
                        rsx! {
                            Link { key: "{l.id}", to: Route::LessonView { id: l.id.clone() }, class: "press flex items-center gap-3 px-4 py-3 active:bg-fill",
                                div { class: "flex w-11 shrink-0 flex-col items-center rounded-lg bg-fill py-1",
                                    span { class: "text-[11px] font-semibold uppercase text-red-500", "{clock::short_date(&l.date).split(' ').next().unwrap_or_default()}" }
                                    span { class: "text-[19px] font-semibold leading-tight", "{clock::short_date(&l.date).split(' ').nth(1).unwrap_or_default()}" }
                                }
                                div { class: "min-w-0 flex-1",
                                    p { class: "truncate text-[17px] font-medium", "{l.title}" }
                                    p { class: "truncate text-[14px] text-label-2", "{l.kicker()} · {l.text}" }
                                }
                                if done {
                                    CircleCheck { size: 22, class: "shrink-0 text-green-500", stroke_width: "2.2" }
                                } else {
                                    ChevronRight { size: 18, class: "shrink-0 text-label-3", stroke_width: "2.5" }
                                }
                            }
                        }
                    }
                }
            }
            p { class: "mt-2 px-4 text-[13px] text-label-2", "{manual.edition}" }
            if let Some(n) = sync_note {
                p { class: "px-4 text-[13px] text-label-2", "{n}" }
            }
        }
    }
}
