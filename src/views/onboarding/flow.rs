use super::steps::{AppearanceStep, LinksStep, TranslationStep, WelcomeStep};
use crate::state::use_app;
use dioxus::prelude::*;

const STEPS: usize = 4;

/// Paged setup: Welcome → Translation → Appearance → Scripture links.
#[component]
pub fn Onboarding() -> Element {
    let mut app = use_app();
    let mut step = use_signal(|| 0usize);
    let current = step();

    let mut finish = move |_| {
        app.settings.write().onboarded = true;
    };

    rsx! {
        div { class: "flex min-h-screen flex-col bg-bg",
            // Top bar: back, progress dots, skip
            div { class: "safe-top mx-auto flex w-full max-w-md items-center justify-between px-5 pt-4",
                button {
                    class: if current == 0 { "invisible w-14 text-left text-[17px] text-accent" } else { "press w-14 text-left text-[17px] text-accent" },
                    onclick: move |_| step.set(current.saturating_sub(1)),
                    "Back"
                }
                div { class: "flex gap-1.5",
                    for i in 0..STEPS {
                        span {
                            key: "{i}",
                            class: if i == current { "h-1.5 w-5 rounded-full bg-accent transition-all" } else { "h-1.5 w-1.5 rounded-full bg-label-3 transition-all" },
                        }
                    }
                }
                button {
                    class: if current == STEPS - 1 { "invisible w-14 text-right text-[17px] text-label-2" } else { "press w-14 text-right text-[17px] text-label-2" },
                    onclick: finish,
                    "Skip"
                }
            }

            div { key: "{current}", class: "fade-in mx-auto flex w-full max-w-md flex-1 flex-col px-6",
                match current {
                    0 => rsx! { WelcomeStep {} },
                    1 => rsx! { TranslationStep {} },
                    2 => rsx! { AppearanceStep {} },
                    _ => rsx! { LinksStep {} },
                }
            }

            div { class: "safe-bottom mx-auto w-full max-w-md px-6 pt-3 pb-4",
                button {
                    class: "press h-[52px] w-full rounded-[14px] bg-accent text-[17px] font-semibold text-white shadow-sm",
                    onclick: move |e| if current + 1 < STEPS { step.set(current + 1) } else { finish(e) },
                    if current == 0 { "Get Started" } else if current + 1 < STEPS { "Continue" } else { "Start Reading" }
                }
            }
        }
    }
}
