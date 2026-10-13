//! Today tab: greeting, Verse of the Day, this week's lesson, today's Daily Dew Drop and
//! "continue reading".

use crate::bible::votd::verse_of_the_day;
use crate::bible::{passage_verses, translation};
use crate::config::CHURCH_NAME;
use crate::manual::use_manual;
use crate::platform::{clock, js};
use crate::routes::Route;
use crate::services::updater::{self, UpdateStatus};
use crate::state::use_app;
use crate::ui::{Card, LargeTitle, PassageText, ScriptureText};
use church_core::scripture::{books, parse};
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, ChevronRight, Droplets, Share, Sparkles};

#[component]
pub fn Today() -> Element {
    let app = use_app();
    let store = use_manual();
    let manual = store.manual.read().clone();
    let today = clock::today_iso();
    let weekday = clock::weekday_from_monday();
    let lesson = manual.current_lesson(&today).cloned();
    let devotional = manual.devotional_lesson(&today).cloned();
    let dew = devotional.as_ref().and_then(|l| l.dew_drop_for(weekday).cloned().map(|d| (l.id.clone(), d)));
    let last_read = app.progress.read().last_read.clone();
    let initial = CHURCH_NAME.chars().next().unwrap_or('C');

    // Background update check (Settings → Updates → Check Automatically).
    let update = use_resource(move || async move {
        if app.settings.read().auto_update_check {
            updater::check().await
        } else {
            UpdateStatus::Idle
        }
    });

    rsx! {
        LargeTitle {
            title: clock::greeting().to_string(),
            subtitle: clock::header_date(),
            trailing: rsx! {
                Link { to: Route::SettingsView {}, class: "press flex h-9 w-9 items-center justify-center rounded-full bg-gradient-to-br from-indigo-500 to-blue-500 text-[15px] font-semibold text-white", "{initial}" }
            },
        }

        div { class: "mx-auto max-w-2xl space-y-4 px-4",
            if let Some(UpdateStatus::Available(m)) = update.read().clone() {
                button {
                    class: "press fade-in flex w-full items-center gap-3 rounded-card bg-accent/10 p-3.5 text-left",
                    onclick: move |_| updater::apply(&m),
                    Sparkles { size: 20, class: "text-accent" }
                    span { class: "flex-1 text-[15px]", "Version {m.version} is available." }
                    span { class: "text-[15px] font-semibold text-accent", "Update" }
                }
            }

            VerseOfTheDay {}

            if let Some(l) = lesson {
                Link { to: Route::LessonView { id: l.id.clone() }, class: "press block",
                    div { class: "relative overflow-hidden rounded-card bg-gradient-to-br from-indigo-600 to-blue-500 p-5 text-white shadow-lg shadow-blue-900/15",
                        p { class: "text-[13px] font-semibold uppercase tracking-wide text-white/75", "Sunday School · {clock::short_date(&l.date)}" }
                        p { class: "mt-1 text-[24px] font-bold leading-tight", "{l.title}" }
                        p { class: "mt-0.5 text-[15px] text-white/80", "{l.kicker()} · {l.unit_title}" }
                        div { class: "mt-4 rounded-xl bg-white/15 p-3 backdrop-blur",
                            p { class: "font-serif text-[16px] italic leading-snug", "“{l.memory_verse.text}”" }
                            p { class: "mt-1 text-[13px] font-semibold text-white/80", "{l.memory_verse.reference}" }
                        }
                        div { class: "mt-4 flex items-center justify-between text-[15px] font-semibold",
                            span { "Open lesson" }
                            ChevronRight { size: 20, stroke_width: "2.5" }
                        }
                    }
                }
            }

            if let Some((lesson_id, d)) = dew {
                Card {
                    div { class: "flex items-center gap-2 text-[13px] font-semibold uppercase tracking-wide text-teal-600 dark:text-teal-400",
                        Droplets { size: 16, stroke_width: "2.4" }
                        "Daily Dew Drop · {d.day}"
                    }
                    p { class: "mt-1.5 text-[20px] font-bold", "{d.title}" }
                    ScriptureText { text: d.reference.clone(), class: "text-[17px]" }
                    Link { to: Route::LessonView { id: lesson_id }, class: "mt-2 inline-block text-[15px] text-label-2", "From this week's lesson ›" }
                }
            }

            if let Some(pos) = last_read {
                if let Some(book) = books::by_id(&pos.book) {
                    Link { to: Route::reader(&pos.book, pos.chapter), class: "press block",
                        Card { class: "flex items-center gap-3",
                            div { class: "flex h-11 w-11 items-center justify-center rounded-xl bg-accent/10 text-accent",
                                BookOpen { size: 22, stroke_width: "2.2" }
                            }
                            div { class: "flex-1",
                                p { class: "text-[13px] text-label-2", "Continue reading" }
                                p { class: "text-[17px] font-semibold", "{book.name} {pos.chapter}" }
                            }
                            ChevronRight { size: 20, class: "text-label-3", stroke_width: "2.5" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VerseOfTheDay() -> Element {
    let app = use_app();
    let reference = parse(verse_of_the_day(clock::day_of_year())).expect("curated verse parses");
    let tr = app.settings.read().translation.clone();
    let label = reference.label();
    let first = reference.first().clone();
    let share_ref = reference.clone();

    rsx! {
        Card { class: "p-5",
            p { class: "text-[13px] font-semibold uppercase tracking-wide text-label-2", "Verse of the Day" }
            div { class: "mt-3", PassageText { key: "{label}-{tr}", reference: reference.clone() } }
            div { class: "mt-3 flex items-center justify-between",
                Link { to: Route::reader_focus(&first), class: "text-[15px] font-semibold text-accent", "{label} · {translation(&tr).abbreviation}" }
                button {
                    class: "press rounded-full p-2 text-accent",
                    aria_label: "Share verse",
                    onclick: move |_| {
                        let r = share_ref.clone();
                        let tr = tr.clone();
                        spawn(async move {
                            if let Ok(v) = passage_verses(&tr, &r).await {
                                let body = v.iter().map(|x| x.text.as_str()).collect::<Vec<_>>().join(" ");
                                js::share_text(&r.label(), &format!("“{body}” — {}", r.label()));
                            }
                        });
                    },
                    Share { size: 20, stroke_width: "2.2" }
                }
            }
        }
    }
}
