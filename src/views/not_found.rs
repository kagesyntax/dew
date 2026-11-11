use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = segments.join("/");
    rsx! {
        div { class: "wrap flex min-h-screen flex-col items-start justify-center gap-3",
            p { class: "eyebrow", "404" }
            h1 { class: "display grad", "Page not found" }
            p { class: "lead max-w-[28ch]", "There's nothing at /{path}. The Word is still one tap away." }
            Link { to: Route::Today {}, class: "btn btn-primary mt-3", "Go to Today" }
        }
    }
}
