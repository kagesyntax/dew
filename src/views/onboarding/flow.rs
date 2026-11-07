use super::steps::{AppearanceStep, LinksStep, TranslationStep};
use super::tour::{PageDots, TourPage, TOUR_PAGES};
use super::welcome::WelcomeStep;
use crate::state::use_app;
use crate::ui::use_motion;
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
    use_motion();
    let mut app = use_app();
    let mut step = use_signal(|| Step::Welcome);
    let current = step();
    let mut finish = move || {
        let mut s = app.settings.write();
        s.onboarded = true;
        s.last_seen_version = crate::config::APP_VERSION.to_string();
    };

    let primary = match current {
        Step::Welcome => "Continue",
        Step::Setup(i) if i + 1 == SETUP_STEPS => "Start reading",
        _ => "Continue",
    };

    rsx! {
        div { class: "flex min-h-screen flex-col bg-bg",
            div { class: "safe-top mx-auto flex h-14 w-full max-w-[780px] items-center justify-between px-5 pt-2",
                if current != Step::Welcome {
                    button { class: "press text-[16px] font-medium text-accent", onclick: move |_| step.set(current.prev()), "Back" }
                } else { span {} }
                if let Step::Tour(_) = current {
                    button { class: "press text-[16px] font-medium text-label-2", onclick: move |_| step.set(Step::Setup(0)), "Skip tour" }
                }
            }

            div { class: "mx-auto flex w-full max-w-[780px] flex-1 flex-col px-6",
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

            div { class: "safe-bottom mx-auto w-full max-w-[780px] space-y-5 px-6 pt-3 pb-5",
                match current {
                    Step::Tour(i) => rsx! { PageDots { count: TOUR_PAGES, current: i } },
                    Step::Setup(i) => rsx! { PageDots { count: SETUP_STEPS, current: i } },
                    Step::Welcome => rsx! { p { class: "micro text-center text-label-3", "Works offline. No account needed." } },
                }
                button {
                    class: "btn btn-primary btn-block",
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
