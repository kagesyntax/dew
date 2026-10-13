use dioxus::prelude::*;

/// The app mark: an open book with a cross, on an indigo→blue squircle.
#[component]
pub fn AppIcon(#[props(default = 72)] size: u32) -> Element {
    let radius = size as f32 * 0.225;
    rsx! {
        div {
            class: "flex items-center justify-center shadow-xl shadow-blue-900/25",
            style: "width:{size}px;height:{size}px;border-radius:{radius}px;background:linear-gradient(145deg,#4f46e5 0%,#0a7aff 100%);",
            svg {
                width: "{size * 62 / 100}",
                height: "{size * 62 / 100}",
                view_box: "0 0 64 64",
                fill: "none",
                stroke: "white",
                stroke_width: "3.6",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M32 18v34" }
                path { d: "M32 18c-6-4-14-5-22-4v34c8-1 16 0 22 4" }
                path { d: "M32 18c6-4 14-5 22-4v34c-8-1-16 0-22 4" }
                path { d: "M32 4v10M27 8.5h10", stroke_width: "3.2" }
            }
        }
    }
}
