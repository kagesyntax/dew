//! Today tab: greeting, Verse of the Day, this week's lesson, today's Daily Dew Drop,
//! "continue reading" and a quiet progress strip. Laid out in the district site's language:
//! an eyebrow, a display greeting, and a stack of tiles.

use crate::bible::votd::verse_of_the_day;
use crate::bible::{passage_verses, translation};
use crate::config::CHURCH_NAME;
use crate::manual::use_manual;
use crate::platform::{clock, js};
use crate::routes::Route;
use crate::services::updater::{self, UpdateStatus};
use crate::state::use_app;
use crate::ui::{tips, Floaters, LargeTitle, PassageText, ScriptureText, Tip};
use church_core::scripture::{books, parse};
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, CalendarDays, ChevronRight, Share, Sparkles};

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
    let done = app.progress.read().completed_lessons.len();
    let highlights = app.progress.read().highlights.len();
    let bookmarks = app.progress.read().bookmarks.len();

    // Background update check (Settings → Updates → Check Automatically).
    let update = use_resource(move || async move {
        if app.settings.read().auto_update_check {
            updater::check().await
        } else {
            UpdateStatus::Idle
        }
    });

    rsx! {
        div { class: "hero-block",
            Floaters {}
            LargeTitle {
                title: clock::greeting().to_string(),
                subtitle: clock::header_date(),
                lead: Some("Fresh Word every morning, from AGC Oshodi.".to_string()),
                trailing: rsx! {
                    Link {
                        to: Route::SettingsView {},
                        class: "press flex h-10 w-10 items-center justify-center rounded-full border border-separator bg-surface text-[14px] font-semibold text-label-2",
                        aria_label: "Settings",
                        "{initial}"
                    }
                },
            }
        }

        div { class: "wrap stack pb-10 pt-3",
            if let Some(UpdateStatus::Available(m)) = update.read().clone() {
                button {
                    class: "tile tint press flex w-full items-center gap-3 text-left",
                    onclick: move |_| updater::apply(&m),
                    Sparkles { size: 19, class: "shrink-0 text-accent" }
                    span { class: "flex-1 text-[15px]", "Version {m.version} is available." }
                    span { class: "text-[14px] font-semibold text-accent", "Update" }
                }
            }

            VerseOfTheDay {}

            Tip {
                id: tips::TODAY,
                title: "Your week at a glance",
                message: "This Sunday's lesson and today's Daily Dew Drop always live here.",
                icon: rsx! { CalendarDays { size: 20, stroke_width: "1.9" } },
            }

            if let Some(l) = lesson {
                Link { to: Route::LessonView { id: l.id.clone() }, class: "tile ink link press block rv",
                    div { class: "flex items-start justify-between gap-3",
                        p { class: "eyebrow", "This Sunday" }
                        span { class: "micro tabular-nums opacity-60", "{clock::short_date(&l.date)}" }
                    }
                    h2 { class: "h2 mt-2", "{l.title}" }
                    p { class: "caption mt-1 opacity-70", "{l.kicker()}" }
                    blockquote { class: "serif mt-5 text-[18px] leading-[1.5]", "“{l.memory_verse.text}”" }
                    p { class: "micro mt-2 opacity-60", "{l.memory_verse.reference}" }
                    div { class: "mt-5 flex items-center justify-between",
                        span { class: "lnk", "Open lesson" }
                        ChevronRight { size: 18, stroke_width: "2.4", class: "opacity-60" }
                    }
                }
            }

            if let Some((lesson_id, d)) = dew {
                div { class: "tile rv",
                    div { class: "flex items-center justify-between gap-3",
                        p { class: "eyebrow", "Daily Dew Drop" }
                        span { class: "badge", "{d.day}" }
                    }
                    h3 { class: "h3 mt-2", "{d.title}" }
                    ScriptureText { text: d.reference.clone(), class: "mt-1 text-[15px]" }
                    Link {
                        to: Route::LessonView { id: lesson_id },
                        class: "lnk mt-3 inline-flex",
                        "From this week's lesson"
                    }
                }
            }

            if let Some(pos) = last_read {
                if let Some(book) = books::by_id(&pos.book) {
                    Link { to: Route::reader(&pos.book, pos.chapter), class: "tile link press block rv",
                        div { class: "flex items-center gap-3",
                            div { class: "flex h-10 w-10 shrink-0 items-center justify-center rounded-full border border-separator text-label-2",
                                BookOpen { size: 19, stroke_width: "1.9" }
                            }
                            div { class: "min-w-0 flex-1",
                                p { class: "eyebrow", "Continue reading" }
                                p { class: "mt-1 truncate text-[17px] font-semibold tracking-tight", "{book.name} {pos.chapter}" }
                            }
                            ChevronRight { size: 18, class: "shrink-0 text-label-3", stroke_width: "2.2" }
                        }
                    }
                }
            }

            div { class: "stats rv",
                div { class: "st",
                    div { class: "num", span { "data-count": "{done}", "{done}" } }
                    p { class: "lbl", "Lessons finished" }
                }
                div { class: "st",
                    div { class: "num", span { "data-count": "{highlights}", "{highlights}" } }
                    p { class: "lbl", "Verses highlighted" }
                }
                div { class: "st",
                    div { class: "num", span { "data-count": "{bookmarks}", "{bookmarks}" } }
                    p { class: "lbl", "Bookmarks kept" }
                }
            }

            p { class: "micro text-center text-label-3", "{manual.title}" }
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
    let abbr = translation(&tr).abbreviation.to_string();

    rsx! {
        div { class: "tile rv",
            div { class: "flex items-center justify-between",
                p { class: "eyebrow", "Verse of the Day" }
                span { class: "badge", "{abbr}" }
            }
            div { class: "mt-3", PassageText { key: "{label}-{tr}", reference: reference.clone() } }
            hr { class: "hr my-4" }
            div { class: "flex items-center justify-between gap-3",
                Link { to: Route::reader_focus(&first), class: "lnk", "{label}" }
                button {
                    class: "press flex h-9 w-9 items-center justify-center rounded-full border border-separator text-label-2",
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
                    Share { size: 17, stroke_width: "2" }
                }
            }
        }
    }
}
