//! Inset lists in the district design language: an eyebrow, one tile, hairline rows.

use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;

#[component]
pub fn GroupedList(header: Option<String>, footer: Option<String>, children: Element) -> Element {
    rsx! {
        section { class: "mx-auto mb-8 max-w-[780px] px-4",
            if let Some(h) = header {
                h2 { class: "eyebrow mb-2 px-2", "{h}" }
            }
            div { class: "tile flush rows", {children} }
            if let Some(f) = footer {
                p { class: "caption mt-2 px-2 leading-snug text-label-2", "{f}" }
            }
        }
    }
}

/// One row: title, optional subtitle, right-aligned detail, trailing control, chevron.
/// `icon` is rendered as a quiet monochrome glyph, not a colour tile.
#[component]
pub fn ListRow(
    title: String,
    subtitle: Option<String>,
    detail: Option<String>,
    icon: Option<Element>,
    trailing: Option<Element>,
    #[props(default)] chevron: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let clickable = onclick.is_some();
    rsx! {
        div {
            class: if clickable { "row press cursor-pointer" } else { "row" },
            onclick: move |e| if let Some(h) = &onclick { h.call(e) },
            if let Some(i) = icon {
                div { class: "shrink-0 text-label-2", {i} }
            }
            div { class: "min-w-0 flex-1",
                p { class: "truncate text-[16px]", "{title}" }
                if let Some(s) = subtitle {
                    p { class: "caption leading-snug text-label-2", "{s}" }
                }
            }
            if let Some(d) = detail {
                span { class: "min-w-0 shrink truncate text-right text-[15px] text-label-2", "{d}" }
            }
            {trailing}
            if chevron {
                ChevronRight { size: 17, class: "shrink-0 text-label-3", stroke_width: "2.2" }
            }
        }
    }
}
