//! Root component: global styles, document head, providers and the router.

use crate::config::APP_NAME;
use crate::manual::use_manual_provider;
use crate::platform::js;
use crate::routes::Route;
use crate::state::use_app_state_provider;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/public/icons/icon.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const APP_CSS: Asset = asset!("/assets/app.css");
const COMPONENTS_THEME: Asset = asset!("/assets/dx-components-theme.css");

#[component]
pub fn App() -> Element {
    use_app_state_provider();
    use_manual_provider();
    use_hook(js::register_service_worker);
    // Android reports no usable CSS safe-area insets, so the bars are measured natively and
    // re-measured on rotation. Compiled out everywhere else.
    #[cfg(target_os = "android")]
    use_hook(crate::platform::android_chrome::watch_insets);

    rsx! {
        document::Title { "{APP_NAME}" }
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1, viewport-fit=cover" }
        document::Meta { name: "apple-mobile-web-app-capable", content: "yes" }
        document::Meta { name: "apple-mobile-web-app-status-bar-style", content: "default" }
        document::Meta { name: "apple-mobile-web-app-title", content: "{APP_NAME}" }
        document::Meta { name: "description", content: "Dew: the Bible and the teens' Sunday School lesson for AGC Oshodi." }
        document::Link { rel: "icon", href: FAVICON, r#type: "image/svg+xml" }
        document::Link { rel: "alternate icon", href: "/favicon.ico" }
        document::Meta { name: "theme-color", content: "#f2f2f7" }
        document::Link { rel: "manifest", href: "/manifest.webmanifest" }
        document::Link { rel: "apple-touch-icon", href: "/icons/icon-180.png" }
        document::Stylesheet { href: COMPONENTS_THEME }
        document::Stylesheet { href: APP_CSS }
        document::Stylesheet { href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
