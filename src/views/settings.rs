//! Settings: simple on top, powerful underneath. Grouped like iOS Settings.

use crate::bible::translation;
use crate::components::switch::Switch;
use crate::components::toggle_group::{ToggleGroup, ToggleItem};
use crate::config::{APP_VERSION, CHURCH_NAME, REPO_URL};
use crate::manual::{self, use_manual, SyncStatus};
use crate::platform::{js, storage};
use crate::services::turso::Turso;
use crate::services::updater::{self, UpdateStatus};
use crate::state::settings::{LinkMode, Settings, ThemeMode};
use crate::state::{progress::Progress, use_app};
use crate::ui::{GroupedList, LargeTitle, ListRow};
use crate::views::bible::TranslationSheet;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    ArrowDownToLine, BookOpen, CloudDownload, Compass, Database, Hash, Info, Lightbulb, Link2, NotebookPen, Palette,
    RotateCcw, Type,
};
use std::collections::HashSet;

#[component]
pub fn SettingsView() -> Element {
    let mut app = use_app();
    let store = use_manual();
    let s = app.settings.read().clone();
    let mut translation_open = use_signal(|| false);
    let mut update = use_signal(|| UpdateStatus::Idle);
    let tr = translation(&s.translation);
    let theme_idx = ThemeMode::ALL.iter().position(|m| *m == s.theme).unwrap_or(0);
    let link_idx = LinkMode::ALL.iter().position(|m| *m == s.link_mode).unwrap_or(0);
    let turso_ready = Turso::from_config().is_ok();

    let mut check_updates = move |_| {
        update.set(UpdateStatus::Checking);
        spawn(async move { update.set(updater::check().await) });
    };

    let (update_title, update_detail) = match update.read().clone() {
        UpdateStatus::Idle => ("Check for Updates".to_string(), None),
        UpdateStatus::Checking => ("Checking…".to_string(), None),
        UpdateStatus::UpToDate => ("Check for Updates".to_string(), Some("Up to date".to_string())),
        UpdateStatus::Available(m) => (format!("Update to {}", m.version), Some("New".to_string())),
        UpdateStatus::Failed(_) => ("Check for Updates".to_string(), Some("Unavailable".to_string())),
    };
    let update_footer = match update.read().clone() {
        UpdateStatus::Available(m) if !m.notes.is_empty() => format!("What's new: {}", m.notes),
        UpdateStatus::Failed(e) => e,
        _ => "Updates install from the church's release channel. The web app refreshes itself; phone and desktop builds open the download page.".into(),
    };

    let sync_detail = match store.sync.read().clone() {
        SyncStatus::Idle => None,
        SyncStatus::Syncing => Some("Syncing…".to_string()),
        SyncStatus::Synced(n) => Some(format!("{n} lessons")),
        SyncStatus::Offline(_) => Some("Offline".to_string()),
    };

    rsx! {
        LargeTitle { title: "Settings" }

        GroupedList { header: "Appearance",
            div { class: "px-4 py-3",
                div { class: "mb-2.5 flex items-center gap-3",
                    div { class: "flex h-[29px] w-[29px] items-center justify-center rounded-[7px] bg-indigo-500 text-white", Palette { size: 17, stroke_width: "2.2" } }
                    span { class: "text-[17px]", "Theme" }
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
            div { class: "px-4 py-3",
                div { class: "mb-2 flex items-center gap-3",
                    div { class: "flex h-[29px] w-[29px] items-center justify-center rounded-[7px] bg-orange-500 text-white", Type { size: 17, stroke_width: "2.2" } }
                    span { class: "flex-1 text-[17px]", "Text Size" }
                }
                div { class: "flex items-center gap-3",
                    span { class: "text-[13px] text-label-2", "A" }
                    input {
                        r#type: "range", min: "0", max: "4", value: "{s.text_size}",
                        class: "flex-1 accent-[var(--app-accent)]",
                        aria_label: "Text size",
                        oninput: move |e| if let Ok(v) = e.value().parse::<u8>() { app.settings.write().text_size = v },
                    }
                    span { class: "text-[22px] text-label-2", "A" }
                }
            }
        }

        GroupedList { header: "Bible",
            ListRow {
                title: "Translation", detail: tr.abbreviation.to_string(), chevron: true, tint: "#0a7aff",
                icon: rsx! { BookOpen { size: 17, stroke_width: "2.2" } },
                onclick: move |_| translation_open.set(true),
            }
            ListRow {
                title: "Verse Numbers", tint: "#8e8e93",
                icon: rsx! { Hash { size: 17, stroke_width: "2.2" } },
                trailing: rsx! { Switch { checked: s.verse_numbers, on_checked_change: move |v| app.settings.write().verse_numbers = v } },
            }
        }

        GroupedList {
            header: "Sunday School",
            footer: s.link_mode.description().to_string(),
            div { class: "px-4 py-3",
                div { class: "mb-2.5 flex items-center gap-3",
                    div { class: "flex h-[29px] w-[29px] items-center justify-center rounded-[7px] bg-teal-500 text-white", Link2 { size: 17, stroke_width: "2.2" } }
                    span { class: "text-[17px]", "Scripture Links" }
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
                title: "Teacher Mode", subtitle: "Show leader notes and hints".to_string(), tint: "#ff9f0a",
                icon: rsx! { NotebookPen { size: 17, stroke_width: "2.2" } },
                trailing: rsx! { Switch { checked: s.leader_notes, on_checked_change: move |v| app.settings.write().leader_notes = v } },
            }
            ListRow {
                title: "Sync Lessons", detail: sync_detail, tint: "#5856d6",
                subtitle: if turso_ready { None } else { Some("Church database not configured — using built-in lessons".to_string()) },
                icon: rsx! { Database { size: 17, stroke_width: "2.2" } },
                onclick: move |_| if turso_ready { spawn(manual::sync(store)); },
            }
        }

        GroupedList { header: "Updates", footer: update_footer,
            ListRow {
                title: update_title, detail: update_detail, tint: "#34c759",
                icon: rsx! { CloudDownload { size: 17, stroke_width: "2.2" } },
                onclick: move |e| match update.peek().clone() {
                    UpdateStatus::Available(m) => updater::apply(&m),
                    UpdateStatus::Checking => {}
                    _ => check_updates(e),
                },
            }
            ListRow {
                title: "Check Automatically", tint: "#30b0c7",
                icon: rsx! { ArrowDownToLine { size: 17, stroke_width: "2.2" } },
                trailing: rsx! { Switch { checked: s.auto_update_check, on_checked_change: move |v| app.settings.write().auto_update_check = v } },
            }
        }

        GroupedList { header: "Help",
            ListRow {
                title: "Feature Tour", chevron: true, tint: "#0a7aff",
                icon: rsx! { Compass { size: 17, stroke_width: "2.2" } },
                onclick: move |_| { navigator().push(crate::routes::Route::TourReplay {}); },
            }
            ListRow {
                title: "Show Tips Again", tint: "#ffcc00",
                icon: rsx! { Lightbulb { size: 17, stroke_width: "2.2" } },
                onclick: move |_| app.progress.write().seen_tips.clear(),
            }
        }

        GroupedList { header: "About",
            ListRow { title: "Version", detail: APP_VERSION.to_string(), tint: "#8e8e93", icon: rsx! { Info { size: 17, stroke_width: "2.2" } } }
            ListRow { title: "Church", detail: CHURCH_NAME.to_string(), tint: "#af52de", icon: rsx! { BookOpen { size: 17, stroke_width: "2.2" } } }
            ListRow { title: "Source & Release Notes", chevron: true, tint: "#1c1c1e", icon: rsx! { Info { size: 17, stroke_width: "2.2" } }, onclick: move |_| js::open_url(REPO_URL) }
        }

        GroupedList { footer: "Clears highlights, bookmarks, lesson progress and preferences on this device.".to_string(),
            ListRow {
                title: "Reset App", tint: "#ff3b30",
                icon: rsx! { RotateCcw { size: 17, stroke_width: "2.2" } },
                onclick: move |_| {
                    storage::clear_all();
                    app.progress.set(Progress::default());
                    app.settings.set(Settings::default());
                },
            }
        }
        p { class: "pb-4 text-center text-[13px] text-label-3", "Made with Dioxus · v{APP_VERSION}" }

        TranslationSheet { open: translation_open }
    }
}
