//! In-context tips modelled on Apple's TipKit: a small card with a symbol, a bold title and
//! one sentence, shown once where the feature lives and dismissed with ×. This is the
//! "product tour" after onboarding: it teaches features at the moment they are useful
//! instead of front-loading them.

use crate::state::use_app;
use dioxus::prelude::*;
use dioxus_icons::lucide::X;

/// Stable tip ids (persisted in `Progress.seen_tips`).
pub mod tips {
    pub const TODAY: &str = "today.week";
    pub const LESSON_LINKS: &str = "lesson.links";
    pub const READER_SELECT: &str = "reader.select";
    pub const BIBLE_GOTO: &str = "bible.goto";
}

#[component]
pub fn Tip(id: String, title: String, message: String, icon: Element, #[props(default)] class: String) -> Element {
    let mut app = use_app();
    if app.progress.read().has_seen(&id) {
        return rsx! {};
    }
    rsx! {
        div { class: "tip-in flex gap-3 rounded-card bg-surface p-4 shadow-sm {class}", role: "note",
            div { class: "mt-0.5 shrink-0 text-accent", {icon} }
            div { class: "min-w-0 flex-1",
                p { class: "text-[15px] font-semibold", "{title}" }
                p { class: "mt-0.5 text-[15px] leading-snug text-label-2", "{message}" }
            }
            button {
                class: "press -mt-1 -mr-1 h-7 w-7 shrink-0 rounded-full text-label-3",
                aria_label: "Dismiss tip",
                onclick: move |_| app.progress.write().mark_seen(&id),
                X { size: 17, stroke_width: "2.4" }
            }
        }
    }
}
