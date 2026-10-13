use crate::config::APP_NAME;
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, GraduationCap, Link2};

/// Apple-style welcome: big two-line title, feature rows with tinted symbols, no artwork.
#[component]
pub fn WelcomeStep() -> Element {
    rsx! {
        div { class: "flex flex-1 flex-col justify-center pb-4",
            h1 { class: "text-[40px] font-bold leading-[1.08] tracking-tight",
                "Welcome to"
                br {}
                span { class: "text-accent", "{APP_NAME}" }
            }
            div { class: "mt-12 space-y-8",
                Feature { title: "The Bible, offline", text: "King James, World English, Berean and American Standard, ready without a connection.",
                    BookOpen { size: 30, stroke_width: "1.8" }
                }
                Feature { title: "This week's lesson", text: "Memory verse, discussion questions and Daily Dew Drops from the manual.",
                    GraduationCap { size: 30, stroke_width: "1.8" }
                }
                Feature { title: "Verses one tap away", text: "Every reference in a lesson opens right where you are.",
                    Link2 { size: 30, stroke_width: "1.8" }
                }
            }
        }
    }
}

#[component]
fn Feature(title: String, text: String, children: Element) -> Element {
    rsx! {
        div { class: "flex items-start gap-5",
            div { class: "flex w-9 shrink-0 justify-center text-accent", {children} }
            div {
                p { class: "text-[17px] font-semibold", "{title}" }
                p { class: "mt-0.5 text-[16px] leading-snug text-label-2", "{text}" }
            }
        }
    }
}
