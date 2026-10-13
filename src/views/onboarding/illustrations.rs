//! Tour artwork drawn with the app's own UI pieces (no stock images, no gradients), so the
//! walkthrough shows exactly what the user will see.

use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, Copy, CornerUpLeft, Share};

#[component]
fn Frame(children: Element) -> Element {
    rsx! {
        div { class: "relative mx-auto h-[300px] w-full max-w-[320px] overflow-hidden rounded-[28px] bg-surface shadow-[0_8px_30px_rgba(0,0,0,0.08)] ring-1 ring-separator",
            {children}
        }
    }
}

/// A lesson paragraph with a tapped reference and the preview sheet rising over it.
#[component]
pub fn LinksArt() -> Element {
    rsx! {
        Frame {
            div { class: "p-5",
                p { class: "text-[11px] font-semibold uppercase tracking-wide text-label-2", "Discussion · a" }
                p { class: "mt-2 text-[15px] leading-relaxed",
                    "Read "
                    span { class: "rounded bg-accent/12 px-0.5 font-semibold text-accent", "Heb 11:1-3,6" }
                    ". Describe some ways people exercise faith."
                }
            }
            div { class: "absolute inset-x-0 bottom-0 rounded-t-[22px] bg-surface-2 px-5 pt-2.5 pb-5 shadow-[0_-6px_24px_rgba(0,0,0,0.10)]",
                div { class: "mx-auto mb-3 h-1 w-9 rounded-full bg-label-3/50" }
                p { class: "text-[17px] font-bold", "Hebrews 11:1–3, 6" }
                p { class: "mt-1.5 font-serif text-[14px] leading-relaxed text-label-2",
                    sup { class: "mr-0.5 font-sans text-[9px] font-semibold", "1" }
                    "Now faith is the substance of things hoped for, the evidence of things not seen."
                }
                div { class: "mt-3 flex h-9 items-center justify-center gap-1.5 rounded-[10px] bg-accent text-[14px] font-semibold text-white",
                    BookOpen { size: 15, stroke_width: "2.3" }
                    "Open in Bible"
                }
            }
        }
    }
}

/// The reader with the "Back to Study 4" pill.
#[component]
pub fn ReturnArt() -> Element {
    rsx! {
        Frame {
            div { class: "px-6 pt-6 text-center",
                p { class: "text-[11px] font-semibold uppercase tracking-[0.18em] text-label-2", "Hebrews" }
                p { class: "font-serif text-[44px] leading-none font-light", "11" }
            }
            div { class: "space-y-2.5 px-6 pt-5",
                for w in ["92%", "100%", "84%", "96%", "70%"] {
                    div { key: "{w}", class: "h-2.5 rounded-full bg-fill", style: "width:{w}" }
                }
            }
            div { class: "absolute inset-x-0 bottom-6 flex justify-center",
                div { class: "flex items-center gap-1.5 rounded-full bg-accent px-4 py-2 text-[14px] font-semibold text-white shadow-lg shadow-black/15",
                    CornerUpLeft { size: 15, stroke_width: "2.5" }
                    "Back to Study 4"
                }
            }
        }
    }
}

/// A highlighted verse and the selection bar.
#[component]
pub fn HighlightArt() -> Element {
    rsx! {
        Frame {
            div { class: "p-6",
                p { class: "font-serif text-[16px] leading-[1.8]",
                    sup { class: "mr-0.5 font-sans text-[9px] font-semibold text-label-2", "16" }
                    span { style: "background: var(--hl-yellow); border-radius: 3px;",
                        "For God so loved the world, that he gave his only begotten Son, "
                    }
                    span { style: "background: var(--hl-yellow); border-radius: 3px;",
                        "that whosoever believeth in him should not perish, but have everlasting life."
                    }
                }
            }
            div { class: "absolute inset-x-4 bottom-5 rounded-2xl bg-surface-2 p-3 ring-1 ring-separator",
                p { class: "mb-2 text-[13px] font-semibold", "John 3:16" }
                div { class: "flex items-center gap-2",
                    for c in ["#ffd60a", "#30d158", "#64d2ff", "#ff6fae"] {
                        span { key: "{c}", class: "h-6 w-6 rounded-full", style: "background:{c}" }
                    }
                    div { class: "flex-1" }
                    span { class: "text-accent", Copy { size: 18, stroke_width: "2.2" } }
                    span { class: "text-accent", Share { size: 18, stroke_width: "2.2" } }
                }
            }
        }
    }
}
