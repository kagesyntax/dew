//! The individual onboarding pages.

use super::AppIcon;
use crate::bible::TRANSLATIONS;
use crate::components::switch::Switch;
use crate::config::{APP_NAME, CHURCH_NAME};
use crate::state::settings::{LinkMode, ThemeMode};
use crate::state::use_app;
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, Check, GraduationCap, Link2, Monitor, Moon, Sun};

#[component]
fn StepHeader(title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "pt-8 pb-6 text-center",
            h1 { class: "text-[30px] font-bold leading-tight tracking-tight", "{title}" }
            p { class: "mt-2 text-[17px] leading-snug text-label-2", "{subtitle}" }
        }
    }
}

#[component]
pub fn WelcomeStep() -> Element {
    rsx! {
        div { class: "flex flex-1 flex-col items-center justify-center pb-6 text-center",
            AppIcon { size: 104 }
            h1 { class: "mt-7 text-[34px] font-bold tracking-tight", "{APP_NAME}" }
            p { class: "mt-1 text-[17px] text-label-2", "{CHURCH_NAME}" }
            div { class: "mt-10 w-full space-y-6 text-left",
                Feature { title: "The Bible, offline", text: "Four free translations, fast and beautiful to read.",
                    BookOpen { size: 26, stroke_width: "2" }
                }
                Feature { title: "Your Sunday School manual", text: "Every lesson, memory verse and Daily Dew Drop in your pocket.",
                    GraduationCap { size: 26, stroke_width: "2" }
                }
                Feature { title: "Tap any verse", text: "Read it in place, or jump to the Bible and come right back to your lesson.",
                    Link2 { size: 26, stroke_width: "2" }
                }
            }
        }
    }
}

#[component]
fn Feature(title: String, text: String, children: Element) -> Element {
    rsx! {
        div { class: "flex items-start gap-4",
            div { class: "mt-0.5 text-accent", {children} }
            div {
                p { class: "text-[17px] font-semibold", "{title}" }
                p { class: "text-[15px] leading-snug text-label-2", "{text}" }
            }
        }
    }
}

#[component]
pub fn TranslationStep() -> Element {
    let mut app = use_app();
    let selected = app.settings.read().translation.clone();
    rsx! {
        StepHeader { title: "Choose your Bible", subtitle: "You can switch any time while reading." }
        div { class: "space-y-3",
            for t in TRANSLATIONS.iter() {
                button {
                    key: "{t.id}",
                    class: if selected == t.id { "press flex w-full items-center gap-4 rounded-2xl bg-surface-2 p-4 text-left ring-2 ring-accent" } else { "press flex w-full items-center gap-4 rounded-2xl bg-surface-2 p-4 text-left" },
                    onclick: move |_| app.settings.write().translation = t.id.to_string(),
                    div { class: "flex h-11 w-14 shrink-0 items-center justify-center rounded-lg bg-surface text-[15px] font-bold", "{t.abbreviation}" }
                    div { class: "min-w-0 flex-1",
                        p { class: "text-[17px] font-semibold", "{t.name}" }
                        p { class: "text-[14px] leading-snug text-label-2", "{t.description}" }
                    }
                    if selected == t.id {
                        Check { size: 22, class: "shrink-0 text-accent", stroke_width: "3" }
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
        StepHeader { title: "Make it yours", subtitle: "Pick a look and a comfortable reading size." }
        div { class: "grid grid-cols-3 gap-3",
            for mode in ThemeMode::ALL {
                button {
                    key: "{mode.label()}",
                    class: "press flex flex-col items-center gap-2",
                    onclick: move |_| app.settings.write().theme = mode,
                    div {
                        class: if theme == mode { "flex aspect-[3/4] w-full items-center justify-center rounded-2xl ring-[2.5px] ring-accent" } else { "flex aspect-[3/4] w-full items-center justify-center rounded-2xl ring-1 ring-separator" },
                        style: match mode {
                            ThemeMode::Light => "background:#f2f2f7;color:#000",
                            ThemeMode::Dark => "background:#1c1c1e;color:#fff",
                            ThemeMode::System => "background:linear-gradient(135deg,#f2f2f7 50%,#1c1c1e 50%);color:#0a7aff",
                        },
                        match mode {
                            ThemeMode::Light => rsx! { Sun { size: 30 } },
                            ThemeMode::Dark => rsx! { Moon { size: 30 } },
                            ThemeMode::System => rsx! { Monitor { size: 30 } },
                        }
                    }
                    span { class: if theme == mode { "text-[15px] font-semibold text-accent" } else { "text-[15px] text-label-2" }, "{mode.label()}" }
                }
            }
        }
        div { class: "mt-8 rounded-2xl bg-surface-2 p-5",
            p { class: "reader-text", "In the beginning was the Word, and the Word was with God, and the Word was God." }
            div { class: "mt-4 flex items-center gap-3",
                span { class: "text-[14px] text-label-2", "A" }
                input {
                    r#type: "range",
                    min: "0",
                    max: "4",
                    value: "{size}",
                    class: "flex-1 accent-[var(--app-accent)]",
                    aria_label: "Text size",
                    oninput: move |e| if let Ok(v) = e.value().parse::<u8>() { app.settings.write().text_size = v },
                }
                span { class: "text-[22px] text-label-2", "A" }
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
        StepHeader { title: "Verses in lessons", subtitle: "When you tap a reference like Heb 11:1-3 in the manual…" }
        div { class: "space-y-3",
            for m in LinkMode::ALL {
                button {
                    key: "{m.label()}",
                    class: if mode == m { "press flex w-full items-center gap-4 rounded-2xl bg-surface-2 p-4 text-left ring-2 ring-accent" } else { "press flex w-full items-center gap-4 rounded-2xl bg-surface-2 p-4 text-left" },
                    onclick: move |_| app.settings.write().link_mode = m,
                    div { class: "min-w-0 flex-1",
                        p { class: "text-[17px] font-semibold", "{m.label()}" }
                        p { class: "text-[14px] leading-snug text-label-2", "{m.description()}" }
                    }
                    if mode == m {
                        Check { size: 22, class: "shrink-0 text-accent", stroke_width: "3" }
                    }
                }
            }
        }
        div { class: "mt-6 flex items-center gap-4 rounded-2xl bg-surface-2 p-4",
            div { class: "flex-1",
                p { class: "text-[17px] font-semibold", "I teach Sunday School" }
                p { class: "text-[14px] leading-snug text-label-2", "Show the manual's leader notes and hints." }
            }
            Switch { checked: teacher, on_checked_change: move |v| app.settings.write().leader_notes = v }
        }
    }
}
