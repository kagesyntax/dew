//! "What's New" sheet, shown once after the app updates to a new version.

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
        div { class: "fade-in fixed inset-0 z-[60] flex items-end justify-center bg-black/40 sm:items-center",
            div { class: "sheet-up safe-bottom flex max-h-[92vh] w-full max-w-md flex-col rounded-t-[28px] border border-separator bg-surface sm:rounded-[28px]",
                div { class: "mx-auto mt-2.5 h-1 w-9 rounded-full bg-[var(--app-separator)] sm:hidden" }
                div { class: "overflow-y-auto px-7 pt-8",
                    p { class: "eyebrow", "Just updated" }
                    h2 { class: "display mt-2",
                        "What's new in"
                        br {}
                        span { class: "text-accent", "{APP_NAME} {APP_VERSION}" }
                    }
                    ul { class: "mt-7 mb-2",
                        for (i, n) in notes.iter().enumerate() {
                            li { key: "{i}", class: "flex gap-3 border-t border-separator py-4",
                                span { class: "mt-0.5 shrink-0 text-label-2", Sparkles { size: 18, stroke_width: "1.9" } }
                                span { class: "text-[16px] leading-snug", "{n}" }
                            }
                        }
                    }
                }
                div { class: "px-7 pt-4 pb-5",
                    button {
                        class: "btn btn-primary btn-block",
                        onclick: move |_| app.settings.write().last_seen_version = APP_VERSION.to_string(),
                        "Continue"
                    }
                }
            }
        }
    }
}
