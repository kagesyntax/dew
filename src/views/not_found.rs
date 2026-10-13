use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = segments.join("/");
    rsx! {
        div { class: "flex min-h-screen flex-col items-center justify-center gap-3 p-8 text-center",
            p { class: "text-[22px] font-bold", "Page not found" }
            p { class: "text-label-2", "/{path}" }
            Link { to: Route::Today {}, class: "font-semibold text-accent", "Go to Today" }
        }
    }
}
