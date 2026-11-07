//! Lesson building blocks: section titles, discussion questions in an accordion, and the
//! Daily Dew Drops list.

use crate::platform::clock;
use crate::state::use_app;
use crate::ui::ScriptureText;
use church_core::manual::{DewDrop, DiscussionSection};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Droplets, MessagesSquare, NotebookPen};

/// A small blue glyph and an uppercase label — the district site's eyebrow, used as a section
/// divider inside long pages.
#[component]
pub fn SectionTitle(title: String, children: Element) -> Element {
    rsx! {
        h2 { class: "mt-9 mb-3 flex items-center gap-2 text-[13px] font-semibold tracking-[0.06em] uppercase text-label-2",
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
    // The first section opens by default; the rest stay folded so a long lesson stays scannable.
    let mut open = use_signal(|| vec![0usize]);

    rsx! {
        section { class: "rv",
            SectionTitle { title: "Discussion Questions".to_string(), MessagesSquare { size: 16, stroke_width: "2.1" } }
            div { class: "acc",
                for (si, s) in sections.into_iter().enumerate() {
                    {
                        let is_open = open.read().contains(&si);
                        rsx! {
                            div { key: "{si}", class: if is_open { "item open" } else { "item" },
                                button {
                                    class: "head press",
                                    "aria-expanded": if is_open { "true" } else { "false" },
                                    onclick: move |_| {
                                        let mut o = open.write();
                                        if let Some(i) = o.iter().position(|x| *x == si) { o.remove(i); } else { o.push(si); }
                                    },
                                    span { class: "n", "{s.letter}" }
                                    span { class: "ttl", "{s.heading}" }
                                    span { class: "ic", aria_hidden: "true" }
                                }
                                div { class: "body",
                                    div {
                                        div { class: "pb-5",
                                            if let Some(intro) = &s.intro {
                                                ScriptureText { text: intro.clone(), class: "mb-3 text-[15px] leading-relaxed text-label-2" }
                                            }
                                            ol { class: "space-y-4",
                                                for (qi, q) in s.questions.iter().enumerate() {
                                                    li { key: "{qi}", class: "flex gap-3",
                                                        span { class: "w-4 shrink-0 pt-0.5 text-right text-[15px] font-semibold tabular-nums text-label-3", "{qi + 1}." }
                                                        div { class: "min-w-0 flex-1",
                                                            ScriptureText { text: q.text.clone(), class: "text-[16px] leading-relaxed" }
                                                            if !q.bullets.is_empty() {
                                                                ul { class: "mt-2.5 space-y-2",
                                                                    for (bi, b) in q.bullets.iter().enumerate() {
                                                                        li { key: "{bi}", class: "tile soft snug text-[15px] leading-snug",
                                                                            ScriptureText { text: b.text.clone(), prefix: b.title.clone() }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if teacher {
                                                                if let Some(n) = &q.leader_note {
                                                                    div { class: "mt-2.5 flex gap-2 rounded-[12px] bg-[var(--app-accent-soft)] px-3 py-2 text-[14px] leading-snug",
                                                                        NotebookPen { size: 15, class: "mt-0.5 shrink-0 text-accent", stroke_width: "2.1" }
                                                                        ScriptureText { text: n.clone(), class: "flex-1" }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if let Some(n) = &s.note {
                                                p { class: "caption mt-4 border-t border-separator pt-3 leading-relaxed",
                                                    strong { "Note: " }
                                                    "{n}"
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
        section { class: "rv",
            SectionTitle { title: "Daily Dew Drops".to_string(), Droplets { size: 16, stroke_width: "2.1" } }
            div { class: "tile flush rows overflow-hidden",
                for (i, d) in drops.into_iter().enumerate() {
                    {
                        let is_today = today_idx.is_some_and(|t| d.day.to_lowercase().starts_with(DAY_NAMES[t]));
                        rsx! {
                            div {
                                key: "{i}",
                                class: if is_today { "flex items-start gap-3 bg-[var(--app-accent-soft)] px-[18px] py-3.5" } else { "flex items-start gap-3 px-[18px] py-3.5" },
                                span {
                                    class: if is_today { "w-8 shrink-0 pt-0.5 text-[11px] font-semibold tracking-[0.06em] uppercase text-accent" } else { "w-8 shrink-0 pt-0.5 text-[11px] font-semibold tracking-[0.06em] uppercase text-label-3" },
                                    "{d.day}"
                                }
                                div { class: "min-w-0 flex-1",
                                    p { class: "text-[15px] font-medium leading-snug", "{d.title}" }
                                    ScriptureText { text: d.reference.clone(), class: "mt-0.5 text-[14px]" }
                                }
                                if is_today {
                                    span { class: "badge on mt-0.5", "Today" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
