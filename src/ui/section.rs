//! Section headings, the district site's eyebrow-over-title pattern (`Heading` in agn-lagos).

use dioxus::prelude::*;

#[component]
pub fn SectionHeading(eyebrow: String, title: String, #[props(default)] class: String) -> Element {
    rsx! {
        div { class: "mb-3 {class}",
            if !eyebrow.is_empty() {
                p { class: "eyebrow rv", "{eyebrow}" }
            }
            h2 { class: "h2 mt-1 rv d1", "{title}" }
        }
    }
}
