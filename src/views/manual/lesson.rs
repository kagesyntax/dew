//! A single lesson, laid out from the printed manual's structure, in the district design
//! language: eyebrow, display title, hairline blocks, tiles and accordions. Provides a
//! `LessonContext` so every scripture link knows how to bring the reader back here.

use super::sections::{DewDrops, Discussion, SectionTitle};
use crate::manual::use_manual;
use crate::platform::{clock, js};
use crate::routes::Route;
use crate::state::use_app;
use crate::ui::scripture::LessonContext;
use crate::ui::{tips, BackBar, ScriptureText, Tip};
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    ChevronLeft, ChevronRight, CircleCheck, Flag, Lightbulb, Link2, ListChecks, Quote, Rocket, Share,
};

#[component]
fn OptionBox(opt: church_core::manual::LessonOption) -> Element {
    rsx! {
        div { class: "tile soft rv",
            div { class: "flex items-center gap-2",
                Lightbulb { size: 17, class: "shrink-0 text-label-2", stroke_width: "2" }
                p { class: "text-[15px] font-semibold tracking-tight", "{opt.label}" }
                if let Some(t) = &opt.text {
                    span { class: "text-[14px] font-normal text-label-2", "· {t}" }
                }
            }
            if let Some(n) = &opt.note {
                p { class: "caption mt-2 leading-snug text-label-2", strong { class: "text-label", "Note: " } "{n}" }
            }
        }
    }
}

/// Route entry: keyed so each lesson mounts fresh (its own LessonContext and scroll restore).
#[component]
pub fn LessonView(id: String) -> Element {
    rsx! { LessonPage { key: "{id}", id } }
}

#[component]
fn LessonPage(id: String) -> Element {
    let mut app = use_app();
    let store = use_manual();
    let manual = store.manual.read().clone();
    let nav = navigator();

    let Some(lesson) = manual.lesson(&id).cloned() else {
        return rsx! {
            BackBar { back_label: "Manual", onback: move |_| { nav.push(Route::ManualHome {}); } }
            p { class: "p-10 text-center text-label-2", "This lesson isn't available." }
        };
    };

    let ctx =
        LessonContext { label: format!("Study {}", lesson.study_number), route: Route::LessonView { id: id.clone() } };
    use_context_provider(|| ctx.clone());

    // Remember the lesson and restore scroll when coming back from the Bible.
    let effect_id = id.clone();
    use_effect(move || {
        app.progress.write().last_lesson = Some(effect_id.clone());
        if let Some(y) = app.pending_scroll.write().take() {
            js::scroll_to(y);
        }
    });

    let (prev, next) = manual.neighbours(&id);
    let (prev, next) = (prev.cloned(), next.cloned());
    let done = app.progress.read().is_completed(&id);
    let share_text = format!(
        "{} — {} ({})\nMemory verse: {} {}",
        lesson.title,
        lesson.kicker(),
        clock::pretty_date(&lesson.date),
        lesson.memory_verse.reference,
        lesson.memory_verse.text
    );
    let id_done = id.clone();

    rsx! {
        BackBar {
            back_label: "Manual",
            onback: move |_| { nav.push(Route::ManualHome {}); },
            title: ctx.label.clone(),
            trailing: rsx! {
                button {
                    class: "press flex h-9 w-9 items-center justify-center rounded-full text-label-2",
                    aria_label: "Share lesson",
                    onclick: move |_| js::share_text("Sunday School", &share_text),
                    Share { size: 18, stroke_width: "2" }
                }
            },
        }

        article { class: "wrap stack pb-12", id: "article",
            header { class: "pt-8",
                p { class: "eyebrow rv", "{clock::pretty_date(&lesson.date)}" }
                h1 { class: "display mt-2 rv d1", "{lesson.title}" }
                p { class: "lead mt-2 rv d2",
                    "Unit {lesson.unit_number} · Study {lesson.study_number} — {lesson.unit_title}"
                }
            }

            div { class: "tile ring rv",
                p { class: "eyebrow", "Bible Text" }
                ScriptureText { text: lesson.text.clone(), class: "mt-2 text-[16px] leading-relaxed" }
            }

            Tip {
                id: tips::LESSON_LINKS,
                title: "Tap a reference",
                message: "Read the verses without leaving the lesson. Change how this works in Settings.",
                icon: rsx! { Link2 { size: 20, stroke_width: "1.9" } },
            }

            div { class: "tile ink rv",
                div { class: "flex items-center gap-2",
                    Quote { size: 14, stroke_width: "2.2", class: "opacity-60" }
                    p { class: "eyebrow", "Memory Verse" }
                }
                p { class: "serif mt-3 text-[20px] leading-[1.42]", "“{lesson.memory_verse.text}”" }
                ScriptureText {
                    text: lesson.memory_verse.reference.clone(),
                    suffix: lesson.memory_verse.version.as_ref().map(|v| format!("({v})")),
                    class: "mt-3 text-[14px] opacity-70",
                }
            }

            if !lesson.getting_started.is_empty() {
                section { class: "rv",
                    SectionTitle { title: "Getting Started".to_string(), Flag { size: 16, stroke_width: "2.1" } }
                    for (i, p) in lesson.getting_started.iter().enumerate() {
                        ScriptureText { key: "{i}", text: p.clone(), class: "mb-3 text-[16px] leading-relaxed" }
                    }
                }
            }

            for box_ in [&lesson.option, &lesson.illustration_option] {
                if let Some(opt) = box_ {
                    OptionBox { opt: opt.clone() }
                }
            }

            if !lesson.overview.is_empty() {
                section { class: "rv",
                    SectionTitle { title: "Study Overview".to_string(), ListChecks { size: 16, stroke_width: "2.1" } }
                    p { class: "caption mb-3 text-label-2", "Today's study considers…" }
                    div { class: "rows tile flush",
                        for (i, o) in lesson.overview.iter().enumerate() {
                            div { key: "{i}", class: "flex gap-4 px-[18px] py-3.5",
                                span { class: "micro w-5 shrink-0 pt-0.5 font-semibold tabular-nums text-label-3", "{i + 1:02}" }
                                ScriptureText { text: o.clone(), class: "flex-1 text-[16px] leading-snug" }
                            }
                        }
                    }
                }
            }

            Discussion { sections: lesson.discussion.clone() }

            if let Some(act) = &lesson.act {
                div { class: "tile tint rv",
                    div { class: "flex items-center gap-2",
                        Rocket { size: 15, stroke_width: "2.2", class: "text-accent" }
                        p { class: "eyebrow text-accent", "Act" }
                    }
                    ScriptureText { text: act.clone(), class: "mt-2 text-[16px] leading-relaxed" }
                }
            }

            if let Some(inv) = &lesson.invitation_option {
                OptionBox { opt: inv.clone() }
            }

            if let Some(ro) = &lesson.reach_out {
                OptionBox { opt: ro.clone() }
            }

            DewDrops { drops: lesson.daily_dew_drops.clone(), lesson_date: lesson.date.clone() }

            // Complete + prev/next
            button {
                class: if done { "btn btn-secondary btn-block mt-2" } else { "btn btn-primary btn-block mt-2" },
                onclick: move |_| { js::haptic(); app.progress.write().toggle_completed(&id_done); },
                CircleCheck { size: 18, stroke_width: "2.2" }
                if done { "Completed" } else { "Mark as completed" }
            }
            div { class: "grid grid-cols-2 gap-3",
                if let Some(p) = prev {
                    Link { to: Route::LessonView { id: p.id.clone() }, class: "tile link press block",
                        div { class: "flex items-center gap-2",
                            ChevronLeft { size: 16, class: "shrink-0 text-label-3", stroke_width: "2.2" }
                            div { class: "min-w-0",
                                p { class: "micro text-label-3", "Previous" }
                                p { class: "mt-0.5 truncate text-[15px] font-semibold", "{p.title}" }
                            }
                        }
                    }
                } else { div {} }
                if let Some(n) = next {
                    Link { to: Route::LessonView { id: n.id.clone() }, class: "tile link press block text-right",
                        div { class: "flex items-center justify-end gap-2",
                            div { class: "min-w-0",
                                p { class: "micro text-label-3", "Next" }
                                p { class: "mt-0.5 truncate text-[15px] font-semibold", "{n.title}" }
                            }
                            ChevronRight { size: 16, class: "shrink-0 text-label-3", stroke_width: "2.2" }
                        }
                    }
                }
            }
            if let Some(pages) = &lesson.pages {
                p { class: "micro pb-2 text-center text-label-3", "Manual pages {pages}" }
            }
        }
    }
}
