//! Settings → Acknowledgements: where every text in the app comes from and under what terms.

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
        div { class: "pt-6",
            GroupedList { header: "Bible Translations",
                for (name, terms, url) in ITEMS.iter().copied() {
                    ListRow { key: "{name}", title: name, subtitle: terms.to_string(), chevron: true, onclick: move |_| js::open_url(url) }
                }
            }
            GroupedList { header: "Sunday School Manual",
                ListRow { title: "Radiant Life Teen Manual", subtitle: "© its publisher. Used for the church's own classes".to_string() }
            }
            GroupedList { header: "Open Source",
                for (name, terms, url) in SOFTWARE.iter().copied() {
                    ListRow { key: "{name}", title: name, detail: terms.to_string(), chevron: true, onclick: move |_| js::open_url(url) }
                }
            }
        }
    }
}
