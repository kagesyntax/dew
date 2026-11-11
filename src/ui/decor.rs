//! Floating monochrome spheres and rings — the district site's `.floaters`.
//! Purely decorative: `data-depth` is read by assets/motion.js for pointer parallax.

use dioxus::prelude::*;

#[component]
pub fn Sphere(
    kind: &'static str,
    size: &'static str,
    pos: &'static str,
    depth: &'static str,
    drift: &'static str,
) -> Element {
    rsx! {
        div {
            class: "sphere {kind} drift {drift}",
            "data-depth": depth,
            style: "width:{size};aspect-ratio:1;{pos}",
        }
    }
}

#[component]
pub fn Ring(size: &'static str, pos: &'static str, depth: &'static str) -> Element {
    rsx! { div { class: "ringo drift b", "data-depth": depth, style: "width:{size};aspect-ratio:1;{pos}" } }
}

/// A hero's worth of spheres, positioned for a phone-width page.
#[component]
pub fn Floaters() -> Element {
    rsx! {
        div { class: "floaters", aria_hidden: "true",
            // Kept to the top-right corner and the bottom-left edge so they never sit
            // behind the title or the lead paragraph.
            Sphere { kind: "", size: "clamp(58px,17vw,96px)", pos: "right:-8%;top:-16%", depth: ".35", drift: "" }
            Ring { size: "clamp(74px,20vw,124px)", pos: "right:-13%;top:30%", depth: ".45" }
            Sphere { kind: "glass", size: "clamp(30px,9vw,48px)", pos: "left:-6%;bottom:-10%", depth: ".6", drift: "b" }
            Sphere { kind: "ink", size: "clamp(14px,4vw,20px)", pos: "right:34%;top:-8%", depth: ".25", drift: "c" }
        }
    }
}
