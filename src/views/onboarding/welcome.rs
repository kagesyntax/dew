use crate::config::{APP_NAME, APP_TAGLINE};
use crate::ui::Floaters;
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, GraduationCap, Link2};

/// Welcome: a district-style hero — floating spheres, a display title, then three feature
/// rows separated by hairlines. No artwork, no gradients behind the type.
#[component]
pub fn WelcomeStep() -> Element {
    rsx! {
        div { class: "hero-block flex flex-1 flex-col justify-center pb-4",
            Floaters {}
            div { class: "relative z-10 pt-2",
                img {
                    src: crate::views::APP_ICON,
                    class: "fade-in mb-7 h-16 w-16 rounded-[16px] border border-separator",
                    alt: "",
                }
                h1 { class: "display-lg grad rv", "Welcome to {APP_NAME}" }
                p { class: "lead mt-3 max-w-[28ch] rv d1", "{APP_TAGLINE}" }
                div { class: "mt-9 rv d2",
                    Feature {
                        title: "The Bible, offline".to_string(),
                        text: "King James, World English, Berean and American Standard, ready without a connection.".to_string(),
                        BookOpen { size: 24, stroke_width: "1.7" }
                    }
                    Feature {
                        title: "This week's lesson".to_string(),
                        text: "Memory verse, discussion questions and Daily Dew Drops from the manual.".to_string(),
                        GraduationCap { size: 24, stroke_width: "1.7" }
                    }
                    Feature {
                        title: "Verses one tap away".to_string(),
                        text: "Every reference in a lesson opens right where you are.".to_string(),
                        Link2 { size: 24, stroke_width: "1.7" }
                    }
                }
            }
        }
    }
}

#[component]
fn Feature(title: String, text: String, children: Element) -> Element {
    rsx! {
        div { class: "flex items-start gap-4 border-t border-separator py-5",
            div { class: "mt-0.5 shrink-0 text-label-2", {children} }
            div {
                p { class: "text-[16px] font-semibold tracking-tight", "{title}" }
                p { class: "caption mt-1 leading-snug text-label-2", "{text}" }
            }
        }
    }
}
