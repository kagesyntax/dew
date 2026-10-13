//! The walkthrough pages, used in first run and replayable from Settings.

use super::illustrations::{HighlightArt, LinksArt, ReturnArt};
use crate::routes::Route;
use dioxus::prelude::*;

pub const TOUR_PAGES: usize = 3;

#[component]
pub fn TourPage(index: usize) -> Element {
    let (title, body) = match index {
        0 => (
            "Tap a verse, stay in your lesson",
            "References in the manual are links. A sheet shows the verses so you never lose your place.",
        ),
        1 => (
            "Jump to the Bible and back",
            "Open the full chapter, then tap Back to Study to return to the exact spot in your lesson.",
        ),
        _ => (
            "Highlight, copy and share",
            "Tap verses in the reader to mark them in four colours or send them to a friend.",
        ),
    };
    rsx! {
        div { key: "{index}", class: "page-in flex flex-1 flex-col justify-center pb-4",
            match index {
                0 => rsx! { LinksArt {} },
                1 => rsx! { ReturnArt {} },
                _ => rsx! { HighlightArt {} },
            }
            h2 { class: "mt-10 text-center text-[28px] font-bold leading-tight tracking-tight", "{title}" }
            p { class: "mx-auto mt-2.5 max-w-[22rem] text-center text-[17px] leading-snug text-label-2", "{body}" }
        }
    }
}

#[component]
pub fn PageDots(count: usize, current: usize) -> Element {
    rsx! {
        div { class: "flex justify-center gap-2", role: "tablist",
            for i in 0..count {
                span { key: "{i}", class: if i == current { "h-2 w-2 rounded-full bg-label" } else { "h-2 w-2 rounded-full bg-label-3/50" } }
            }
        }
    }
}

/// Settings → Feature Tour.
#[component]
pub fn TourReplay() -> Element {
    let mut page = use_signal(|| 0usize);
    let nav = navigator();
    let p = page();
    rsx! {
        div { class: "flex min-h-screen flex-col bg-bg",
            div { class: "safe-top mx-auto flex w-full max-w-md justify-end px-5 pt-4",
                button { class: "press text-[17px] text-accent", onclick: move |_| { nav.push(Route::SettingsView {}); }, "Done" }
            }
            div { class: "mx-auto flex w-full max-w-md flex-1 flex-col px-6", TourPage { index: p } }
            div { class: "safe-bottom mx-auto w-full max-w-md space-y-5 px-6 pt-3 pb-4",
                PageDots { count: TOUR_PAGES, current: p }
                button {
                    class: "press h-[52px] w-full rounded-[14px] bg-accent text-[17px] font-semibold text-white",
                    onclick: move |_| if p + 1 < TOUR_PAGES { page.set(p + 1) } else { nav.push(Route::SettingsView {}); },
                    if p + 1 < TOUR_PAGES { "Continue" } else { "Done" }
                }
            }
        }
    }
}
