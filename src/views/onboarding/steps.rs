//! The individual onboarding pages: translation, appearance, verse-link behaviour.

use crate::bible::TRANSLATIONS;
use crate::components::switch::Switch;
use crate::state::settings::{LinkMode, ThemeMode};
use crate::state::use_app;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, Monitor, Moon, Sun};

#[component]
fn StepHeader(title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "pt-4 pb-7",
            h1 { class: "h2", "{title}" }
            p { class: "lead mt-2 max-w-[34ch]", "{subtitle}" }
        }
    }
}

#[component]
pub fn TranslationStep() -> Element {
    let mut app = use_app();
    let selected = app.settings.read().translation.clone();
    rsx! {
        StepHeader {
            title: "Choose your Bible".to_string(),
            subtitle: "You can switch any time while reading.".to_string(),
        }
        div { class: "tile flush rows",
            for t in TRANSLATIONS.iter() {
                button {
                    key: "{t.id}",
                    class: if selected == t.id { "row press bg-[var(--app-accent-soft)]" } else { "row press" },
                    onclick: move |_| app.settings.write().translation = t.id.to_string(),
                    span { class: "w-11 shrink-0 text-[12px] font-semibold tracking-[0.04em] text-label-2", "{t.abbreviation}" }
                    div { class: "min-w-0 flex-1 text-left",
                        p { class: "text-[16px] font-medium", "{t.name}" }
                        p { class: "caption leading-snug text-label-2", "{t.description}" }
                    }
                    if selected == t.id {
                        Check { size: 18, class: "shrink-0 text-accent", stroke_width: "2.6" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AppearanceStep() -> Element {
    let mut app = use_app();
    let theme = app.settings.read().theme;
    let size = app.settings.read().text_size;
    rsx! {
        StepHeader {
            title: "Make it yours".to_string(),
            subtitle: "Pick a look and a comfortable reading size.".to_string(),
        }
        div { class: "grid grid-cols-3 gap-3",
            for mode in ThemeMode::ALL {
                button {
                    key: "{mode.label()}",
                    class: "press flex flex-col items-center gap-2",
                    onclick: move |_| app.settings.write().theme = mode,
                    div {
                        class: if theme == mode { "flex aspect-[3/4] w-full items-center justify-center rounded-[18px] ring-2 ring-[var(--app-accent)]" } else { "flex aspect-[3/4] w-full items-center justify-center rounded-[18px] ring-1 ring-[var(--app-separator)]" },
                        style: match mode {
                            ThemeMode::Light => "background:#fbfbfd;color:#1d1d1f",
                            ThemeMode::Dark => "background:#000000;color:#f5f5f7",
                            ThemeMode::System => "background:linear-gradient(135deg,#fbfbfd 50%,#1d1d1f 50%);color:#ffffff",
                        },
                        match mode {
                            ThemeMode::Light => rsx! { Sun { size: 26, stroke_width: "1.8" } },
                            ThemeMode::Dark => rsx! { Moon { size: 26, stroke_width: "1.8" } },
                            ThemeMode::System => rsx! { Monitor { size: 26, stroke_width: "1.8" } },
                        }
                    }
                    span { class: if theme == mode { "text-[14px] font-semibold text-accent" } else { "text-[14px] text-label-2" }, "{mode.label()}" }
                }
            }
        }
        div { class: "tile soft mt-7",
            p { class: "reader-text", "In the beginning was the Word, and the Word was with God, and the Word was God." }
            div { class: "mt-4 flex items-center gap-3",
                span { class: "text-[13px] text-label-3", "A" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "4",
                    value: "{size}",
                    class: "flex-1 accent-[var(--app-accent)]",
                    aria_label: "Text size",
                    oninput: move |e| if let Ok(v) = e.value().parse::<u8>() { app.settings.write().text_size = v },
                }
                span { class: "text-[22px] text-label-3", "A" }
            }
        }
    }
}

#[component]
pub fn LinksStep() -> Element {
    let mut app = use_app();
    let mode = app.settings.read().link_mode;
    let teacher = app.settings.read().leader_notes;
    rsx! {
        StepHeader {
            title: "Verses in lessons".to_string(),
            subtitle: "When you tap a reference like Heb 11:1-3 in the manual…".to_string(),
        }
        div { class: "tile flush rows",
            for m in LinkMode::ALL {
                button {
                    key: "{m.label()}",
                    class: if mode == m { "row press bg-[var(--app-accent-soft)]" } else { "row press" },
                    onclick: move |_| app.settings.write().link_mode = m,
                    div { class: "min-w-0 flex-1 text-left",
                        p { class: "text-[16px] font-medium", "{m.label()}" }
                        p { class: "caption leading-snug text-label-2", "{m.description()}" }
                    }
                    if mode == m {
                        Check { size: 18, class: "shrink-0 text-accent", stroke_width: "2.6" }
                    }
                }
            }
        }
        div { class: "tile mt-4",
            div { class: "flex items-center gap-4",
                div { class: "flex-1",
                    p { class: "text-[16px] font-medium", "I teach Sunday School" }
                    p { class: "caption leading-snug text-label-2", "Show the manual's leader notes and hints." }
                }
                Switch { checked: teacher, on_checked_change: move |v| app.settings.write().leader_notes = v }
            }
        }
    }
}
