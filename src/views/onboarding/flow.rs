use super::steps::{AppearanceStep, LinksStep, TranslationStep};
use super::tour::{PageDots, TourPage, TOUR_PAGES};
use super::welcome::WelcomeStep;
use crate::state::use_app;
use dioxus::prelude::*;

/// Welcome → Tour (3) → Setup (3).
#[derive(Clone, Copy, PartialEq)]
enum Step {
    Welcome,
    Tour(usize),
    Setup(usize),
}

const SETUP_STEPS: usize = 3;

impl Step {
    fn next(self) -> Option<Step> {
        match self {
            Step::Welcome => Some(Step::Tour(0)),
            Step::Tour(i) if i + 1 < TOUR_PAGES => Some(Step::Tour(i + 1)),
            Step::Tour(_) => Some(Step::Setup(0)),
            Step::Setup(i) if i + 1 < SETUP_STEPS => Some(Step::Setup(i + 1)),
            Step::Setup(_) => None,
        }
    }
    fn prev(self) -> Step {
        match self {
            Step::Welcome | Step::Tour(0) => Step::Welcome,
            Step::Tour(i) => Step::Tour(i - 1),
            Step::Setup(0) => Step::Tour(TOUR_PAGES - 1),
            Step::Setup(i) => Step::Setup(i - 1),
        }
    }
}

#[component]
pub fn Onboarding() -> Element {
    let mut app = use_app();
    let mut step = use_signal(|| Step::Welcome);
    let current = step();
    let mut finish = move || app.settings.write().onboarded = true;

    let primary = match current {
        Step::Welcome => "Continue",
        Step::Setup(i) if i + 1 == SETUP_STEPS => "Start Reading",
        _ => "Continue",
    };

    rsx! {
        div { class: "flex min-h-screen flex-col bg-bg",
            div { class: "safe-top mx-auto flex h-14 w-full max-w-md items-center justify-between px-5 pt-2",
                if current != Step::Welcome {
                    button { class: "press text-[17px] text-accent", onclick: move |_| step.set(current.prev()), "Back" }
                } else { span {} }
                if let Step::Tour(_) = current {
                    button { class: "press text-[17px] text-accent", onclick: move |_| step.set(Step::Setup(0)), "Skip Tour" }
                }
            }

            div { class: "mx-auto flex w-full max-w-md flex-1 flex-col px-7",
                match current {
                    Step::Welcome => rsx! { div { class: "fade-in flex flex-1 flex-col", WelcomeStep {} } },
                    Step::Tour(i) => rsx! { TourPage { index: i } },
                    Step::Setup(i) => rsx! {
                        div { key: "setup-{i}", class: "page-in flex flex-1 flex-col",
                            match i {
                                0 => rsx! { TranslationStep {} },
                                1 => rsx! { AppearanceStep {} },
                                _ => rsx! { LinksStep {} },
                            }
                        }
                    },
                }
            }

            div { class: "safe-bottom mx-auto w-full max-w-md space-y-5 px-7 pt-3 pb-4",
                match current {
                    Step::Tour(i) => rsx! { PageDots { count: TOUR_PAGES, current: i } },
                    Step::Setup(i) => rsx! { PageDots { count: SETUP_STEPS, current: i } },
                    Step::Welcome => rsx! { p { class: "text-center text-[13px] leading-snug text-label-2", "Works offline. No account needed." } },
                }
                button {
                    class: "press h-[52px] w-full rounded-[14px] bg-accent text-[17px] font-semibold text-white",
                    onclick: move |_| match current.next() {
                        Some(n) => step.set(n),
                        None => finish(),
                    },
                    "{primary}"
                }
            }
        }
    }
}
