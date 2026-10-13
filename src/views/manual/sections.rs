//! Lesson building blocks: section titles, discussion questions with leader notes, and the
//! Daily Dew Drops list.

use crate::platform::clock;
use crate::state::use_app;
use crate::ui::ScriptureText;
use church_core::manual::{DewDrop, DiscussionSection};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Droplets, MessagesSquare, NotebookPen};

#[component]
pub fn SectionTitle(title: String, children: Element) -> Element {
    rsx! {
        h2 { class: "mt-8 mb-3 flex items-center gap-2 text-[20px] font-bold tracking-tight",
            span { class: "text-accent", {children} }
            "{title}"
        }
    }
}

#[component]
pub fn Discussion(sections: Vec<DiscussionSection>) -> Element {
    let app = use_app();
    let teacher = app.settings.read().leader_notes;
    if sections.is_empty() {
        return rsx! {};
    }
    rsx! {
        SectionTitle { title: "Discussion Questions", MessagesSquare { size: 18, stroke_width: "2.3" } }
        div { class: "space-y-4",
            for (si, s) in sections.into_iter().enumerate() {
                div { key: "{si}", class: "rounded-card bg-surface p-4",
                    div { class: "mb-2 flex items-center gap-2.5",
                        span { class: "flex h-7 w-7 shrink-0 items-center justify-center rounded-full bg-accent text-[14px] font-bold uppercase text-white", "{s.letter}" }
                        p { class: "text-[17px] font-semibold italic", "{s.heading}" }
                    }
                    if let Some(intro) = &s.intro {
                        ScriptureText { text: intro.clone(), class: "mb-3 text-[16px] leading-relaxed text-label-2" }
                    }
                    ol { class: "space-y-3",
                        for (qi, q) in s.questions.iter().enumerate() {
                            li { key: "{qi}", class: "flex gap-3",
                                span { class: "w-5 shrink-0 pt-px text-right text-[16px] font-semibold text-label-2", "{qi + 1}." }
                                div { class: "min-w-0 flex-1",
                                    ScriptureText { text: q.text.clone(), class: "text-[17px] leading-relaxed" }
                                    if !q.bullets.is_empty() {
                                        ul { class: "mt-2 space-y-2",
                                            for (bi, b) in q.bullets.iter().enumerate() {
                                                li { key: "{bi}", class: "rounded-xl bg-surface-2 px-3 py-2.5 text-[16px] leading-snug",
                                                    ScriptureText { text: b.text.clone(), prefix: b.title.clone() }
                                                }
                                            }
                                        }
                                    }
                                    if teacher {
                                        if let Some(n) = &q.leader_note {
                                            div { class: "mt-2 flex gap-2 rounded-xl bg-amber-400/15 px-3 py-2 text-[15px] leading-snug",
                                                NotebookPen { size: 16, class: "mt-0.5 shrink-0 text-amber-600 dark:text-amber-400", stroke_width: "2.3" }
                                                ScriptureText { text: n.clone(), class: "flex-1" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if let Some(n) = &s.note {
                        p { class: "mt-3 border-t border-separator pt-3 text-[16px] leading-relaxed",
                            strong { "Note: " }
                            "{n}"
                        }
                    }
                }
            }
        }
    }
}

const DAY_NAMES: [&str; 6] = ["mon", "tue", "wed", "thu", "fri", "sat"];

#[component]
pub fn DewDrops(drops: Vec<DewDrop>, lesson_date: String) -> Element {
    if drops.is_empty() {
        return rsx! {};
    }
    // Highlight today's reading during the week after this lesson.
    let today = clock::today();
    let start = chrono::NaiveDate::parse_from_str(&lesson_date, "%Y-%m-%d").ok();
    let today_idx = start.and_then(|s| {
        let diff = (today - s).num_days();
        (1..=6).contains(&diff).then_some(diff as usize - 1)
    });
    rsx! {
        SectionTitle { title: "Daily Dew Drops", Droplets { size: 18, stroke_width: "2.3" } }
        div { class: "overflow-hidden rounded-card bg-surface [&>*:not(:last-child)]:border-b [&>*:not(:last-child)]:border-separator",
            for (i, d) in drops.into_iter().enumerate() {
                {
                    let is_today = today_idx.is_some_and(|t| d.day.to_lowercase().starts_with(DAY_NAMES[t]));
                    rsx! {
                        div { key: "{i}", class: if is_today { "flex items-start gap-3 bg-teal-500/10 px-4 py-3" } else { "flex items-start gap-3 px-4 py-3" },
                            span { class: if is_today { "w-10 shrink-0 pt-1 text-[13px] font-bold uppercase text-teal-600 dark:text-teal-400" } else { "w-10 shrink-0 pt-1 text-[13px] font-semibold uppercase text-label-2" }, "{d.day}" }
                            div { class: "min-w-0 flex-1",
                                p { class: "text-[16px] font-medium", "{d.title}" }
                                ScriptureText { text: d.reference.clone(), class: "text-[15px]" }
                            }
                            if is_today {
                                span { class: "mt-0.5 rounded-full bg-teal-500 px-2 py-0.5 text-[11px] font-bold uppercase text-white", "Today" }
                            }
                        }
                    }
                }
            }
        }
    }
}
