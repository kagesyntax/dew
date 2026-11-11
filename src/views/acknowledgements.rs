//! Settings → Credits: where every text in the app comes from and under what terms.

use crate::platform::js;
use crate::ui::{BackBar, GroupedList, ListRow};
use dioxus::prelude::*;

const ITEMS: &[(&str, &str, &str)] = &[
    ("King James Version", "Public domain (Crown copyright in the UK)", "https://www.kingjamesbibleonline.org"),
    ("World English Bible", "Public domain, eBible.org", "https://ebible.org/web/"),
    ("Berean Standard Bible", "Public domain since 2023", "https://berean.bible/terms.htm"),
    ("American Standard Version", "Public domain", "https://en.wikipedia.org/wiki/American_Standard_Version"),
    (
        "Legacy Standard Bible",
        "© The Lockman Foundation. Opened on read.lsbible.org, not stored in the app",
        "https://read.lsbible.org",
    ),
];

const SOFTWARE: &[(&str, &str, &str)] = &[
    ("Dioxus", "MIT / Apache-2.0", "https://dioxuslabs.com"),
    ("Lucide icons", "ISC", "https://lucide.dev"),
    ("Tailwind CSS", "MIT", "https://tailwindcss.com"),
    ("Turso / libSQL", "MIT", "https://turso.tech"),
];

#[component]
pub fn Acknowledgements() -> Element {
    rsx! {
        BackBar { back_label: "Settings", title: "Credits", onback: move |_| { navigator().push(crate::routes::Route::SettingsView {}); } }
        div { class: "wrap pt-8",
            h1 { class: "display grad", "Credits" }
            p { class: "lead mt-2 max-w-[32ch]", "Every text in Dew, and the open-source work it stands on." }
        }
        div { class: "pt-6",
            GroupedList { header: "Bible translations",
                for (name, terms, url) in ITEMS.iter().copied() {
                    ListRow {
                        key: "{name}",
                        title: name.to_string(),
                        subtitle: Some(terms.to_string()),
                        chevron: true,
                        onclick: move |_| js::open_url(url),
                    }
                }
            }
            GroupedList { header: "Sunday School manual",
                ListRow {
                    title: "Radiant Life Teen Manual".to_string(),
                    subtitle: Some("© its publisher. Used for the church's own classes".to_string()),
                }
            }
            GroupedList { header: "Open source",
                for (name, terms, url) in SOFTWARE.iter().copied() {
                    ListRow {
                        key: "{name}",
                        title: name.to_string(),
                        detail: Some(terms.to_string()),
                        chevron: true,
                        onclick: move |_| js::open_url(url),
                    }
                }
            }
        }
    }
}
