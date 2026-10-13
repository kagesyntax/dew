//! Inset grouped table views (Settings-style lists).

use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;

#[component]
pub fn GroupedList(header: Option<String>, footer: Option<String>, children: Element) -> Element {
    rsx! {
        section { class: "mx-auto mb-7 max-w-2xl px-4",
            if let Some(h) = header {
                h2 { class: "mb-1.5 px-4 text-[13px] uppercase tracking-wide text-label-2", "{h}" }
            }
            div { class: "overflow-hidden rounded-card bg-surface [&>*:not(:last-child)]:border-b [&>*:not(:last-child)]:border-separator",
                {children}
            }
            if let Some(f) = footer {
                p { class: "mt-1.5 px-4 text-[13px] leading-snug text-label-2", "{f}" }
            }
        }
    }
}

/// One row: optional coloured icon tile, title, subtitle, detail text, trailing accessory.
#[component]
pub fn ListRow(
    title: String,
    subtitle: Option<String>,
    detail: Option<String>,
    icon: Option<Element>,
    /// Background colour of the icon tile (iOS Settings style).
    #[props(default = "#0a7aff".to_string())]
    tint: String,
    trailing: Option<Element>,
    #[props(default)] chevron: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let clickable = onclick.is_some();
    rsx! {
        div {
            class: if clickable { "press flex min-h-11 cursor-pointer items-center gap-3 py-2.5 pr-4 pl-4 active:bg-fill" } else { "flex min-h-11 items-center gap-3 py-2.5 pr-4 pl-4" },
            onclick: move |e| if let Some(h) = &onclick { h.call(e) },
            if let Some(i) = icon {
                div { class: "flex h-[29px] w-[29px] shrink-0 items-center justify-center rounded-[7px] text-white", style: "background: {tint}", {i} }
            }
            div { class: "min-w-0 flex-1",
                p { class: "truncate text-[17px]", "{title}" }
                if let Some(s) = subtitle {
                    p { class: "text-[13px] leading-snug text-label-2", "{s}" }
                }
            }
            if let Some(d) = detail {
                span { class: "shrink-0 text-[17px] text-label-2", "{d}" }
            }
            {trailing}
            if chevron {
                ChevronRight { size: 18, class: "shrink-0 text-label-3", stroke_width: "2.5" }
            }
        }
    }
}
