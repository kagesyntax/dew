//! A single lesson, laid out from the printed manual's structure. Provides a
//! `LessonContext` so every scripture link knows how to bring the reader back here.

use super::sections::{DewDrops, Discussion, SectionTitle};
use crate::manual::use_manual;
use crate::platform::{clock, js};
use crate::routes::Route;
use crate::state::use_app;
use crate::ui::scripture::LessonContext;
use crate::ui::{tips, BackBar, Card, ScriptureText, Tip};
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    ChevronLeft, ChevronRight, CircleCheck, Flag, Lightbulb, Link2, ListChecks, Quote, Rocket, Share,
};

/// Route entry: keyed so each lesson mounts fresh (its own LessonContext and scroll restore).
#[component]
fn OptionBox(opt: church_core::manual::LessonOption, icon_class: String) -> Element {
    rsx! {
        div { class: "mt-4 rounded-card border border-dashed border-separator p-4",
            div { class: "flex items-center gap-2 text-[15px] font-semibold",
                Lightbulb { size: 18, class: "{icon_class}", stroke_width: "2.3" }
                "{opt.label}"
                if let Some(t) = &opt.text {
                    span { class: "font-normal text-label-2", "· {t}" }
                }
            }
            if let Some(n) = &opt.note {
                p { class: "mt-1.5 text-[15px] leading-snug text-label-2", strong { class: "text-label", "Note: " } "{n}" }
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
                button { class: "press p-1.5 text-accent", aria_label: "Share lesson", onclick: move |_| js::share_text("Sunday School", &share_text), Share { size: 21, stroke_width: "2.2" } }
            },
        }

        div { class: "mx-auto max-w-2xl px-5 pt-5",
            // Header
            p { class: "text-[13px] font-semibold uppercase tracking-wide text-accent", "{clock::pretty_date(&lesson.date)}" }
            h1 { class: "mt-1.5 font-serif text-[32px] font-bold leading-[1.15] tracking-tight", "{lesson.title}" }
            p { class: "mt-1.5 text-[15px] text-label-2", "Unit {lesson.unit_number} · Study {lesson.study_number} — {lesson.unit_title}" }
            div { class: "mt-4 border-y border-separator py-3",
                p { class: "text-[13px] font-semibold uppercase tracking-wide text-label-2", "Bible Text" }
                ScriptureText { text: lesson.text.clone(), class: "mt-0.5 text-[17px] leading-relaxed" }
            }

            Tip {
                id: tips::LESSON_LINKS,
                title: "Tap a blue reference",
                message: "Read the verses without leaving the lesson. Change how this works in Settings.",
                icon: rsx! { Link2 { size: 22, stroke_width: "2" } },
                class: "mt-4",
            }

            // Memory verse
            Card { class: "mt-5 p-5",
                div { class: "flex items-center gap-2 text-[13px] font-semibold uppercase tracking-wide text-label-2",
                    Quote { size: 15, stroke_width: "2.4" }
                    "Memory Verse"
                }
                p { class: "mt-2 font-serif text-[21px] leading-snug italic", "“{lesson.memory_verse.text}”" }
                ScriptureText {
                    text: lesson.memory_verse.reference.clone(),
                    suffix: lesson.memory_verse.version.as_ref().map(|v| format!("({v})")),
                    class: "mt-2 text-[15px]",
                }
            }

            if !lesson.getting_started.is_empty() {
                SectionTitle { title: "Getting Started", Flag { size: 18, stroke_width: "2.3" } }
                for (i, p) in lesson.getting_started.iter().enumerate() {
                    ScriptureText { key: "{i}", text: p.clone(), class: "mb-3 text-[17px] leading-relaxed" }
                }
            }

            for box_ in [&lesson.option, &lesson.illustration_option] {
                if let Some(opt) = box_ {
                    OptionBox { opt: opt.clone(), icon_class: "text-amber-500", }
                }
            }

            if !lesson.overview.is_empty() {
                SectionTitle { title: "Study Overview", ListChecks { size: 18, stroke_width: "2.3" } }
                p { class: "mb-2 text-[15px] text-label-2", "Today's study considers…" }
                ul { class: "space-y-2",
                    for (i, o) in lesson.overview.iter().enumerate() {
                        li { key: "{i}", class: "flex gap-3 text-[17px] leading-snug",
                            span { class: "mt-2.5 h-1.5 w-1.5 shrink-0 rounded-full bg-accent" }
                            ScriptureText { text: o.clone(), class: "flex-1" }
                        }
                    }
                }
            }

            Discussion { sections: lesson.discussion.clone() }

            if let Some(act) = &lesson.act {
                div { class: "mt-8 rounded-card bg-accent/10 p-5",
                    div { class: "flex items-center gap-2 text-[13px] font-bold uppercase tracking-wide text-accent",
                        Rocket { size: 16, stroke_width: "2.4" }
                        "Act"
                    }
                    ScriptureText { text: act.clone(), class: "mt-2 text-[17px] leading-relaxed" }
                }
            }

            if let Some(inv) = &lesson.invitation_option {
                OptionBox { opt: inv.clone(), icon_class: "text-accent" }
            }

            if let Some(ro) = &lesson.reach_out {
                OptionBox { opt: ro.clone(), icon_class: "text-teal-600 dark:text-teal-400" }
            }

            DewDrops { drops: lesson.daily_dew_drops.clone(), lesson_date: lesson.date.clone() }

            // Complete + prev/next
            button {
                class: if done { "press mt-8 flex h-[52px] w-full items-center justify-center gap-2 rounded-[14px] bg-green-500/15 text-[17px] font-semibold text-green-600 dark:text-green-400" } else { "press mt-8 flex h-[52px] w-full items-center justify-center gap-2 rounded-[14px] bg-accent text-[17px] font-semibold text-white" },
                onclick: move |_| { js::haptic(); app.progress.write().toggle_completed(&id_done); },
                CircleCheck { size: 20, stroke_width: "2.4" }
                if done { "Completed" } else { "Mark as Completed" }
            }
            div { class: "mt-4 grid grid-cols-2 gap-3",
                if let Some(p) = prev {
                    Link { to: Route::LessonView { id: p.id.clone() }, class: "press flex items-center gap-2 rounded-card bg-surface p-3",
                        ChevronLeft { size: 18, class: "text-label-3", stroke_width: "2.5" }
                        div { class: "min-w-0",
                            p { class: "text-[12px] text-label-2", "Previous" }
                            p { class: "truncate text-[15px] font-medium", "{p.title}" }
                        }
                    }
                } else { div {} }
                if let Some(n) = next {
                    Link { to: Route::LessonView { id: n.id.clone() }, class: "press flex items-center justify-end gap-2 rounded-card bg-surface p-3 text-right",
                        div { class: "min-w-0",
                            p { class: "text-[12px] text-label-2", "Next" }
                            p { class: "truncate text-[15px] font-medium", "{n.title}" }
                        }
                        ChevronRight { size: 18, class: "text-label-3", stroke_width: "2.5" }
                    }
                }
            }
            if let Some(pages) = &lesson.pages {
                p { class: "mt-6 text-center text-[13px] text-label-3", "Manual pages {pages}" }
            }
        }
    }
}
