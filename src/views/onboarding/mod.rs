//! First-run experience: launch screen + a four-step setup flow.

mod app_icon;
mod flow;
mod steps;

pub use app_icon::AppIcon;
pub use flow::Onboarding;

use dioxus::prelude::*;

/// Launch screen shown while persisted state loads (usually a few milliseconds).
#[component]
pub fn Splash() -> Element {
    rsx! {
        div { class: "flex min-h-screen items-center justify-center bg-bg",
            div { class: "fade-in", AppIcon { size: 96 } }
        }
    }
}
