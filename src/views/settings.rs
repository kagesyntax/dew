//! Settings: simple on top, powerful underneath. Eyebrow + tile groups, filter chips for the
//! either/or choices, hairline rows for everything else.

use crate::bible::translation;
use crate::components::switch::Switch;
use crate::components::toggle_group::{ToggleGroup, ToggleItem};
use crate::config::{APP_NAME, APP_VERSION, BUILD_SHA, CHURCH_NAME, REPO_URL};
use crate::manual::{self, use_manual, SyncStatus};
use crate::platform::{js, storage};
use crate::routes::Route;
use crate::services::turso::Turso;
use crate::services::updater::UpdateStatus;
use crate::state::settings::{LinkMode, Settings, ThemeMode, VerseNav};
use crate::state::{progress::Progress, use_app};
use crate::ui::{GroupedList, LargeTitle, ListRow};
use crate::views::bible::TranslationSheet;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    BookOpen, Church, CloudDownload, Compass, Database, Hash, History, Lightbulb, Link2, ListOrdered, MessageSquare,
    NotebookPen, Palette, RotateCcw, ScrollText, Share, ShieldCheck, Type,
};
use std::collections::HashSet;

#[component]
pub fn SettingsView() -> Element {
    let mut app = use_app();
    let store = use_manual();
    let s = app.settings.read().clone();
    let mut translation_open = use_signal(|| false);
    let tr = translation(&s.translation);
    let theme_idx = ThemeMode::ALL.iter().position(|m| *m == s.theme).unwrap_or(0);
    let nav_idx = VerseNav::ALL.iter().position(|m| *m == s.verse_nav).unwrap_or(0);
    let link_idx = LinkMode::ALL.iter().position(|m| *m == s.link_mode).unwrap_or(0);
    let turso_ready = Turso::from_config().is_ok();

    let update_available = match &*app.update.read() {
        UpdateStatus::Available(m) => Some(m.version.clone()),
        _ => None,
    };

    let sync_detail = match store.sync.read().clone() {
        SyncStatus::Idle => None,
        SyncStatus::Syncing => Some("Syncing…".to_string()),
        SyncStatus::Synced(n) => Some(format!("{n} lessons")),
        SyncStatus::Offline(_) => Some("Offline".to_string()),
    };

    let bible_footer = if s.resume_reading {
        "The Bible tab opens on the chapter you were reading, with any verse you searched for highlighted."
    } else {
        "The Bible tab opens on the list of books. Continue Reading on Today still remembers your place."
    }
    .to_string();
    rsx! {
        LargeTitle {
            title: "Settings".to_string(),
            subtitle: APP_NAME.to_string(),
            lead: Some(format!("{CHURCH_NAME} · on this device only.")),
        }

        div { class: "pt-4" }
        GroupedList { header: "Appearance",
            div { class: "px-[18px] py-4",
                div { class: "mb-3 flex items-center gap-2",
                    Palette { size: 17, class: "text-label-2", stroke_width: "1.9" }
                    span { class: "text-[16px]", "Theme" }
                }
                ToggleGroup {
                    horizontal: true,
                    pressed: HashSet::from([theme_idx]),
                    on_pressed_change: move |set: HashSet<usize>| if let Some(i) = set.into_iter().next() { app.settings.write().theme = ThemeMode::ALL[i] },
                    for (i, m) in ThemeMode::ALL.iter().enumerate() {
                        ToggleItem { key: "{i}", index: i, "{m.label()}" }
                    }
                }
            }
            div { class: "px-[18px] py-4",
                div { class: "mb-1 flex items-center gap-2",
                    Type { size: 17, class: "text-label-2", stroke_width: "1.9" }
                    span { class: "flex-1 text-[16px]", "Reading size" }
                    span { class: "micro tabular-nums text-label-3", "{[16, 18, 20, 23, 26][s.text_size.min(4) as usize]}pt" }
                }
                div { class: "mt-2 flex items-center gap-3",
                    span { class: "text-[13px] text-label-3", "A" }
                    input {
                        r#type: "range", min: "0", max: "4", value: "{s.text_size}",
                        class: "flex-1 accent-[var(--app-accent)]",
                        aria_label: "Text size",
                        oninput: move |e| if let Ok(v) = e.value().parse::<u8>() { app.settings.write().text_size = v },
                    }
                    span { class: "text-[22px] text-label-3", "A" }
                }
            }
        }

        GroupedList { header: "Bible", footer: bible_footer,
            ListRow {
                title: "Translation".to_string(),
                detail: Some(format!("{} · {}", tr.abbreviation, tr.name)),
                chevron: true,
                icon: rsx! { BookOpen { size: 18, stroke_width: "1.9" } },
                onclick: move |_| translation_open.set(true),
            }
            ListRow {
                title: "Verse Numbers".to_string(),
                icon: rsx! { Hash { size: 18, stroke_width: "1.9" } },
                trailing: rsx! { Switch { checked: s.verse_numbers, on_checked_change: move |v| app.settings.write().verse_numbers = v } },
            }
            ListRow {
                title: "Open where I left off".to_string(),
                icon: rsx! { History { size: 18, stroke_width: "1.9" } },
                trailing: rsx! { Switch { checked: s.resume_reading, on_checked_change: move |v| app.settings.write().resume_reading = v } },
            }
        }

        GroupedList { header: "Find a verse", footer: s.verse_nav.description().to_string(),
            div { class: "px-[18px] py-4",
                div { class: "mb-3 flex items-center gap-2",
                    ListOrdered { size: 17, class: "text-label-2", stroke_width: "1.9" }
                    span { class: "text-[16px]", "Verse navigation" }
                }
                ToggleGroup {
                    horizontal: true,
                    pressed: HashSet::from([nav_idx]),
                    on_pressed_change: move |set: HashSet<usize>| if let Some(i) = set.into_iter().next() { app.settings.write().verse_nav = VerseNav::ALL[i] },
                    for (i, m) in VerseNav::ALL.iter().enumerate() {
                        ToggleItem { key: "{i}", index: i, "{m.label()}" }
                    }
                }
            }
        }

        GroupedList {
            header: "Sunday School",
            footer: s.link_mode.description().to_string(),
            div { class: "px-[18px] py-4",
                div { class: "mb-3 flex items-center gap-2",
                    Link2 { size: 17, class: "text-label-2", stroke_width: "1.9" }
                    span { class: "text-[16px]", "Scripture links" }
                }
                ToggleGroup {
                    horizontal: true,
                    pressed: HashSet::from([link_idx]),
                    on_pressed_change: move |set: HashSet<usize>| if let Some(i) = set.into_iter().next() { app.settings.write().link_mode = LinkMode::ALL[i] },
                    for (i, m) in LinkMode::ALL.iter().enumerate() {
                        ToggleItem { key: "{i}", index: i, "{m.label()}" }
                    }
                }
            }
            ListRow {
                title: "Teacher mode".to_string(),
                subtitle: Some("Show leader notes and hints".to_string()),
                icon: rsx! { NotebookPen { size: 18, stroke_width: "1.9" } },
                trailing: rsx! { Switch { checked: s.leader_notes, on_checked_change: move |v| app.settings.write().leader_notes = v } },
            }
            ListRow {
                title: "Sync lessons".to_string(),
                detail: sync_detail,
                subtitle: if turso_ready { None } else { Some("Church database not configured — using built-in lessons".to_string()) },
                icon: rsx! { Database { size: 18, stroke_width: "1.9" } },
                onclick: move |_| if turso_ready { spawn(manual::sync(store)); },
            }
        }

        GroupedList { header: "Help",
            ListRow {
                title: "Feature tour".to_string(),
                chevron: true,
                icon: rsx! { Compass { size: 18, stroke_width: "1.9" } },
                onclick: move |_| { navigator().push(Route::TourReplay {}); },
            }
            ListRow {
                title: "Show tips again".to_string(),
                icon: rsx! { Lightbulb { size: 18, stroke_width: "1.9" } },
                onclick: move |_| app.progress.write().seen_tips.clear(),
            }
        }

        GroupedList { header: "About",
            ListRow {
                title: "Software update".to_string(),
                chevron: true,
                detail: if update_available.is_some() { None } else { Some(format!("Version {APP_VERSION}")) },
                icon: rsx! { CloudDownload { size: 18, stroke_width: "1.9" } },
                trailing: update_available.as_ref().map(|_| rsx! { span { class: "badge on", "New" } }),
                onclick: move |_| { navigator().push(Route::SoftwareUpdate {}); },
            }
            ListRow { title: "Church".to_string(), detail: Some(CHURCH_NAME.to_string()), icon: rsx! { Church { size: 18, stroke_width: "1.9" } } }
            ListRow {
                title: "Share app".to_string(),
                chevron: true,
                icon: rsx! { Share { size: 18, stroke_width: "1.9" } },
                onclick: move |_| js::share_text(APP_NAME, &format!("{APP_NAME}: the Bible and this week's Sunday School lesson. {}", crate::config::APP_URL.trim_end_matches("app/"))),
            }
            ListRow {
                title: "Send feedback".to_string(),
                chevron: true,
                icon: rsx! { MessageSquare { size: 18, stroke_width: "1.9" } },
                onclick: move |_| js::open_url(&format!("{REPO_URL}/issues/new?template=feedback.yml&title=Feedback%20(v{APP_VERSION})")),
            }
            ListRow {
                title: "Credits".to_string(),
                chevron: true,
                icon: rsx! { ScrollText { size: 18, stroke_width: "1.9" } },
                onclick: move |_| { navigator().push(Route::Acknowledgements {}); },
            }
        }

        GroupedList {
            footer: "No account, no tracking. Your highlights, bookmarks and settings stay on this device.".to_string(),
            ListRow {
                title: "Privacy".to_string(),
                detail: Some("On device".to_string()),
                icon: rsx! { ShieldCheck { size: 18, stroke_width: "1.9" } },
            }
        }

        GroupedList { footer: "Clears highlights, bookmarks, lesson progress and preferences on this device.".to_string(),
            ListRow {
                title: "Reset app".to_string(),
                icon: rsx! { RotateCcw { size: 18, stroke_width: "1.9" } },
                onclick: move |_| {
                    storage::clear_all();
                    app.progress.set(Progress::default());
                    app.settings.set(Settings::default());
                },
            }
        }
        p { class: "micro pb-6 text-center text-label-3", "{APP_NAME} {APP_VERSION} ({BUILD_SHA})" }

        TranslationSheet { open: translation_open }
    }
}
