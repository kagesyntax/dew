use crate::bible::{translation::LSB_READER_URL, TRANSLATIONS};
use crate::components::sheet::{Sheet, SheetTitle};
use crate::platform::js;
use crate::state::use_app;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ExternalLink};

/// Small pill that shows the active translation and opens the picker sheet.
#[component]
pub fn TranslationPill() -> Element {
    let app = use_app();
    let mut open = use_signal(|| false);
    let abbr = crate::bible::translation(&app.settings.read().translation).abbreviation;
    rsx! {
        button {
            class: "press rounded-full bg-fill px-3 py-1 text-[13px] font-bold tracking-wide",
            onclick: move |_| open.set(true),
            "{abbr}"
        }
        TranslationSheet { open }
    }
}

#[component]
pub fn TranslationSheet(open: Signal<bool>) -> Element {
    let mut app = use_app();
    let selected = app.settings.read().translation.clone();
    rsx! {
        Sheet { open: open(), on_open_change: move |v| open.set(v), "data-side": "bottom",
            div { class: "px-5 pt-1",
                SheetTitle { class: "text-[22px] font-bold", "Translation" }
            }
            div { class: "mx-4 overflow-hidden rounded-card bg-surface-2 [&>*:not(:last-child)]:border-b [&>*:not(:last-child)]:border-separator",
                for t in TRANSLATIONS.iter() {
                    button {
                        key: "{t.id}",
                        class: "press flex w-full items-center gap-3 px-4 py-3 text-left",
                        onclick: move |_| {
                            app.settings.write().translation = t.id.to_string();
                            open.set(false);
                        },
                        span { class: "w-12 text-[15px] font-bold", "{t.abbreviation}" }
                        span { class: "flex-1 text-[16px]", "{t.name}" }
                        if selected == t.id {
                            Check { size: 20, class: "text-accent", stroke_width: "3" }
                        }
                    }
                }
                button {
                    class: "press flex w-full items-center gap-3 px-4 py-3 text-left",
                    onclick: move |_| js::open_url(LSB_READER_URL),
                    span { class: "w-12 text-[15px] font-bold text-label-2", "LSB" }
                    span { class: "flex-1 text-[16px] text-label-2", "Legacy Standard Bible" }
                    ExternalLink { size: 18, class: "text-label-3" }
                }
            }
            p { class: "px-6 text-[13px] leading-snug text-label-2",
                "KJV, WEB, BSB and ASV are public domain and work offline. The LSB is copyrighted (© The Lockman Foundation) and opens on LSBible.org."
            }
        }
    }
}
