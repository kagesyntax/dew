use crate::routes::Route;
use crate::state::use_app;
use crate::ui::{PreviewHost, ReturnPill, TabBar};
use crate::views::onboarding::{Onboarding, Splash};
use dioxus::prelude::*;

/// Gate: launch screen until state is loaded, onboarding on first run, then the app.
#[component]
pub fn Shell() -> Element {
    let app = use_app();
    if !*app.ready.read() {
        return rsx! { Splash {} };
    }
    if !app.settings.read().onboarded {
        return rsx! { Onboarding {} };
    }
    rsx! { Outlet::<Route> {} }
}

/// Tab bar, the shared passage preview sheet and the return pill.
#[component]
pub fn TabLayout() -> Element {
    let route = use_route::<Route>();
    let app = use_app();

    // New screens start at the top unless we're returning to a saved position.
    use_effect(use_reactive!(|route| {
        let _ = &route;
        if app.pending_scroll.peek().is_none() {
            crate::platform::js::scroll_top();
        }
        // Re-bind the scroll/pointer engine to the freshly rendered page.
        document::eval(
            "(() => { let n = 24; const go = () => { if (window.AGN) window.AGN.remount(); else if (n-- > 0) setTimeout(go, 50); }; go(); })();",
        );
    }));

    rsx! {
        PreviewHost {
            crate::views::bible::TranslationHost {
                main { class: "min-h-screen pb-12", Outlet::<Route> {} }
                ReturnPill {}
                TabBar {}
                crate::views::WhatsNew {}
            }
        }
    }
}
