//! First-run experience, in Apple's order:
//! 1. Welcome — "Welcome to Dew" with three feature rows (like Apple's own apps)
//! 2. Tour — three illustrated walkthrough pages built from real UI, skippable
//! 3. Setup — translation, appearance, verse-link behaviour / teacher mode
//!
//! After that, TipKit-style tips (`ui::tip`) teach the rest in context. The tour can be
//! replayed from Settings → Feature Tour.

mod flow;
mod illustrations;
mod steps;
mod tour;
mod welcome;

pub use flow::Onboarding;
pub use tour::TourReplay;

use dioxus::prelude::*;

/// Launch screen: like iOS, just the app's background while state loads.
#[component]
pub fn Splash() -> Element {
    // Launch screen: just the icon, like iOS.
    rsx! {
        div { class: "flex min-h-screen items-center justify-center bg-bg",
            img { src: crate::views::APP_ICON, class: "h-24 w-24 rounded-[22px]", alt: "" }
        }
    }
}
