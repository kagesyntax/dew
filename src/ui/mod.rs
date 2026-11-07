//! App-specific building blocks in the district design language.
//!
//! The visual system itself lives in `assets/agn.css` (tokens, type scale, tiles, pills,
//! reveals); these modules only compose it.

use dioxus::prelude::*;

pub mod chrome;
pub mod decor;
pub mod list;
pub mod scripture;
pub mod section;
pub mod tab_bar;
pub mod tip;

pub use chrome::{BackBar, LargeTitle, ReturnPill, Spinner};
pub use decor::Floaters;
pub use list::{GroupedList, ListRow};
pub use scripture::{PassageText, PreviewHost, ScriptureText};
pub use section::SectionHeading;
pub use tab_bar::TabBar;
pub use tip::{tips, Tip};

/// Re-binds the JS motion engine to the freshly rendered page.
/// Call once per layout or page (TabLayout does it for every tab route).
pub fn use_motion() {
    use_effect(|| {
        document::eval(
            "(() => { let n = 24; const go = () => { if (window.AGN) window.AGN.remount(); else if (n-- > 0) setTimeout(go, 50); }; go(); })();",
        );
    });
}
