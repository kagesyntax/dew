use crate::bible::{translation::LSB_READER_URL, TRANSLATIONS};
use crate::components::sheet::{Sheet, SheetTitle};
use crate::platform::js;
use crate::state::use_app;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ExternalLink};

/// One translation picker for the whole app, mounted at the root of the tab layout.
/// Sheets are `position: fixed`, and a fixed element inside a blurred bar (backdrop-filter) is
/// positioned against that bar instead of the screen, so the sheet must never live inside one.
#[derive(Clone, Copy)]
pub struct TranslationPicker(pub Signal<bool>);

impl TranslationPicker {
    pub fn open(mut self) {
        self.0.set(true);
    }
}

/// Mount once (TabLayout); provides `TranslationPicker` to everything below.
#[component]
pub fn TranslationHost(children: Element) -> Element {
    let picker = use_context_provider(|| TranslationPicker(Signal::new(false)));
    rsx! {
        {children}
        TranslationSheet { open: picker.0 }
    }
}

/// Small pill that shows the active translation and opens the picker sheet.
#[component]
pub fn TranslationPill() -> Element {
    let app = use_app();
    let picker = use_context::<TranslationPicker>();
    let abbr = crate::bible::translation(&app.settings.read().translation).abbreviation;
    rsx! {
        button {
            class: "press badge px-2.5",
            aria_label: "Translation {abbr}",
            onclick: move |_| picker.open(),
            "{abbr}"
        }
    }
}

#[component]
pub fn TranslationSheet(open: Signal<bool>) -> Element {
    let mut app = use_app();
    let selected = app.settings.read().translation.clone();
    rsx! {
        Sheet { open: open(), on_open_change: move |v| open.set(v), "data-side": "bottom",
            div { class: "px-5 pt-1",
                SheetTitle { class: "text-[24px] font-bold tracking-tight", "Translation" }
            }
            div { class: "tile flush rows mx-4",
                for t in TRANSLATIONS.iter() {
                    button {
                        key: "{t.id}",
                        class: "row press",
                        onclick: move |_| {
                            app.settings.write().translation = t.id.to_string();
                            open.set(false);
                        },
                        span { class: "w-12 text-[13px] font-semibold tracking-[0.04em]", "{t.abbreviation}" }
                        span { class: "flex-1 text-[16px]", "{t.name}" }
                        if selected == t.id {
                            Check { size: 18, class: "text-accent", stroke_width: "2.6" }
                        }
                    }
                }
                button {
                    class: "row press",
                    onclick: move |_| js::open_url(LSB_READER_URL),
                    span { class: "w-12 text-[13px] font-semibold tracking-[0.04em] text-label-3", "LSB" }
                    span { class: "flex-1 text-[16px] text-label-2", "Legacy Standard Bible" }
                    ExternalLink { size: 17, class: "text-label-3" }
                }
            }
            p { class: "caption px-6 leading-snug text-label-2",
                "KJV, WEB, BSB and ASV are public domain and work offline. The LSB is copyrighted (© The Lockman Foundation) and opens on LSBible.org."
            }
        }
    }
}
