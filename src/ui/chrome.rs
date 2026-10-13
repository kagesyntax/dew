//! Navigation chrome: large titles, back bars, cards, the "Back to lesson" pill.

use crate::state::{navigation, use_app};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronLeft, CornerUpLeft};

/// iOS large title header (34pt bold) with optional subtitle and trailing accessory.
#[component]
pub fn LargeTitle(title: String, subtitle: Option<String>, trailing: Option<Element>) -> Element {
    rsx! {
        header { class: "safe-top px-5 pt-12 pb-3",
            div { class: "flex items-end justify-between gap-3",
                div { class: "min-w-0",
                    if let Some(sub) = subtitle {
                        p { class: "text-[13px] font-semibold uppercase tracking-wide text-label-2", "{sub}" }
                    }
                    h1 { class: "truncate text-[34px] font-bold leading-tight tracking-tight", "{title}" }
                }
                if let Some(t) = trailing {
                    div { class: "pb-1.5", {t} }
                }
            }
        }
    }
}

/// Compact sticky bar for pushed screens: back chevron, centered title, trailing slot.
#[component]
pub fn BackBar(
    back_label: String,
    onback: EventHandler<()>,
    title: Option<String>,
    trailing: Option<Element>,
) -> Element {
    rsx! {
        div { class: "bar-blur safe-top sticky top-0 z-30 border-b border-separator",
            div { class: "relative mx-auto flex h-11 max-w-2xl items-center justify-between px-2",
                button {
                    class: "press flex items-center text-[17px] text-accent",
                    onclick: move |_| onback.call(()),
                    ChevronLeft { size: 28, stroke_width: "2.2" }
                    span { class: "-ml-1", "{back_label}" }
                }
                if let Some(t) = title {
                    span { class: "pointer-events-none absolute inset-x-28 truncate text-center text-[17px] font-semibold", "{t}" }
                }
                div { class: "flex items-center gap-1 pr-2", {trailing} }
            }
        }
    }
}

#[component]
pub fn Card(children: Element, #[props(default)] class: String) -> Element {
    rsx! {
        div { class: "rounded-card bg-surface p-4 shadow-[0_1px_2px_rgb(0_0_0/0.04)] {class}", {children} }
    }
}

#[component]
pub fn Spinner() -> Element {
    rsx! {
        div { class: "flex justify-center py-16",
            div { class: "h-7 w-7 animate-spin rounded-full border-[3px] border-fill border-t-label-2" }
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
                class: "press fade-in fixed left-1/2 z-40 flex -translate-x-1/2 items-center gap-2 rounded-full bg-accent px-4 py-2.5 text-[15px] font-semibold text-white shadow-lg shadow-black/20",
                style: "bottom: calc(env(safe-area-inset-bottom) + 76px);",
                onclick: move |_| navigation::go_back(app),
                CornerUpLeft { size: 18, stroke_width: "2.4" }
                "Back to {p.label}"
            }
        }
    }
}
