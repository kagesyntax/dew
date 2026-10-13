//! Apple-style "What's New" sheet, shown once after the app updates to a new version.

use crate::config::{APP_NAME, APP_VERSION, CHANGELOG};
use crate::state::use_app;
use church_core::version::release_notes;
use dioxus::prelude::*;
use dioxus_icons::lucide::Sparkles;

#[component]
pub fn WhatsNew() -> Element {
    let mut app = use_app();
    let seen = app.settings.read().last_seen_version.clone();
    if seen == APP_VERSION {
        return rsx! {};
    }
    let notes = release_notes(CHANGELOG, APP_VERSION);
    if notes.is_empty() {
        // Nothing to say: record the version silently.
        spawn(async move { app.settings.write().last_seen_version = APP_VERSION.to_string() });
        return rsx! {};
    }
    rsx! {
        div { class: "fade-in fixed inset-0 z-[60] flex items-end justify-center bg-black/30 sm:items-center",
            div { class: "sheet-up safe-bottom flex max-h-[92vh] w-full max-w-md flex-col rounded-t-[22px] bg-bg sm:rounded-[22px]",
                div { class: "overflow-y-auto px-7 pt-10",
                    h2 { class: "text-[34px] font-bold leading-[1.1] tracking-tight",
                        "What's New in"
                        br {}
                        span { class: "text-accent", "{APP_NAME} {APP_VERSION}" }
                    }
                    ul { class: "mt-8 space-y-5 pb-4",
                        for (i, n) in notes.iter().enumerate() {
                            li { key: "{i}", class: "flex gap-4",
                                span { class: "mt-0.5 shrink-0 text-accent", Sparkles { size: 22, stroke_width: "2" } }
                                span { class: "text-[16px] leading-snug", "{n}" }
                            }
                        }
                    }
                }
                div { class: "px-7 pt-3 pb-5",
                    button {
                        class: "press h-[52px] w-full rounded-[14px] bg-accent text-[17px] font-semibold text-white",
                        onclick: move |_| app.settings.write().last_seen_version = APP_VERSION.to_string(),
                        "Continue"
                    }
                }
            }
        }
    }
}
