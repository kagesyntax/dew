//! Navigation chrome in the district design language: display titles, the sticky local bar,
//! tiles, and the "Back to lesson" pill.

use crate::state::{navigation, use_app};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronLeft, CornerUpLeft};

/// Page header: a small uppercase eyebrow over a bold, tight display title.
/// Mirrors `.route-head` on the district site (`HeaderIntro` in agn-lagos).
#[component]
pub fn LargeTitle(
    title: String,
    subtitle: Option<String>,
    trailing: Option<Element>,
    /// Optional one-line lead under the title.
    lead: Option<String>,
) -> Element {
    rsx! {
        header { class: "wrap pt-10 pb-2",
            div { class: "flex items-start justify-between gap-4",
                div { class: "min-w-0",
                    if let Some(sub) = subtitle {
                        p { class: "eyebrow rv", "{sub}" }
                    }
                    h1 { class: "display grad rv d1", "{title}" }
                }
                if let Some(t) = trailing {
                    div { class: "shrink-0 pt-1", {t} }
                }
            }
            if let Some(l) = lead {
                p { class: "lead mt-2 max-w-[34ch] rv d2", "{l}" }
            }
        }
    }
}

/// Compact sticky bar for pushed screens: a back control, a centred title, a trailing slot.
/// The district site's `.lnav`, with the title centred like an app bar.
#[component]
pub fn BackBar(
    back_label: String,
    onback: EventHandler<()>,
    title: Option<String>,
    trailing: Option<Element>,
) -> Element {
    rsx! {
        div { class: "lnav safe-top",
            div { class: "wrap in relative",
                button {
                    class: "press flex shrink-0 items-center gap-0.5 text-[16px] font-medium text-accent",
                    onclick: move |_| onback.call(()),
                    ChevronLeft { size: 22, stroke_width: "2.4" }
                    span { class: "-ml-0.5", "{back_label}" }
                }
                if let Some(t) = title {
                    span { class: "pointer-events-none absolute inset-x-24 truncate text-center text-[15px] font-semibold tracking-tight", "{t}" }
                }
                div { class: "flex shrink-0 items-center gap-1", {trailing} }
            }
        }
    }
}

#[component]
pub fn Spinner() -> Element {
    rsx! {
        div { class: "flex justify-center py-16",
            div { class: "h-6 w-6 animate-spin rounded-full border-2 border-separator border-t-label-2" }
        }
    }
}

/// Floating pill shown in the Bible after jumping from a lesson. One tap returns to the exact
/// lesson and scroll position.
#[component]
pub fn ReturnPill() -> Element {
    let app = use_app();
    let point = app.return_point.read().clone();
    rsx! {
        if let Some(p) = point {
            button {
                class: "return-pill press fade-in fixed left-1/2 z-40 flex -translate-x-1/2 items-center gap-2 rounded-full bg-[var(--app-accent-solid)] px-5 py-3 text-[15px] font-medium text-white",
                style: "bottom: calc(max(var(--safe-bottom), 8px) + 76px); box-shadow: 0 12px 30px -12px rgb(0 0 0 / .45);",
                onclick: move |_| navigation::go_back(app),
                CornerUpLeft { size: 17, stroke_width: "2.4" }
                "Back to {p.label}"
            }
        }
    }
}
