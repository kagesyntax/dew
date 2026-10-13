//! Settings: simple on top, powerful underneath. Grouped like iOS Settings.

use crate::bible::translation;
use crate::components::switch::Switch;
use crate::components::toggle_group::{ToggleGroup, ToggleItem};
use crate::config::{APP_NAME, APP_VERSION, BUILD_SHA, CHURCH_NAME, REPO_URL};
use crate::manual::{self, use_manual, SyncStatus};
use crate::platform::{js, storage};
use crate::routes::Route;
use crate::services::turso::Turso;
use crate::services::updater::UpdateStatus;
use crate::state::settings::{LinkMode, Settings, ThemeMode};
use crate::state::{progress::Progress, use_app};
use crate::ui::{GroupedList, LargeTitle, ListRow};
use crate::views::bible::TranslationSheet;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    BookOpen, Church, CloudDownload, Compass, Database, Hash, Lightbulb, Link2, MessageSquare, NotebookPen, Palette,
    RotateCcw, ScrollText, Share, ShieldCheck, Type,
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

        GroupedList { header: "Help",
            ListRow {
                title: "Feature Tour", chevron: true, tint: "#0a7aff",
                icon: rsx! { Compass { size: 17, stroke_width: "2.2" } },
                onclick: move |_| { navigator().push(Route::TourReplay {}); },
            }
            ListRow {
                title: "Show Tips Again", tint: "#ffcc00",
                icon: rsx! { Lightbulb { size: 17, stroke_width: "2.2" } },
                onclick: move |_| app.progress.write().seen_tips.clear(),
            }
        }

        GroupedList { header: "About",
            ListRow {
                title: "Software Update", chevron: true, tint: "#8e8e93",
                detail: if update_available.is_some() { None } else { Some(format!("Version {APP_VERSION}")) },
                icon: rsx! { CloudDownload { size: 17, stroke_width: "2.2" } },
                trailing: update_available.as_ref().map(|_| rsx! { span { class: "flex h-[22px] min-w-[22px] items-center justify-center rounded-full bg-[#ff3b30] px-1.5 text-[13px] font-semibold text-white", "1" } }),
                onclick: move |_| { navigator().push(Route::SoftwareUpdate {}); },
            }
            ListRow { title: "Church", detail: CHURCH_NAME.to_string(), tint: "#af52de", icon: rsx! { Church { size: 17, stroke_width: "2.2" } } }
            ListRow {
                title: "Share App", tint: "#0a7aff", chevron: true,
                icon: rsx! { Share { size: 17, stroke_width: "2.2" } },
                onclick: move |_| js::share_text(APP_NAME, &format!("{APP_NAME}: the Bible and this week's Sunday School lesson. {}", crate::config::APP_URL)),
            }
            ListRow {
                title: "Send Feedback", tint: "#34c759", chevron: true,
                icon: rsx! { MessageSquare { size: 17, stroke_width: "2.2" } },
                onclick: move |_| js::open_url(&format!("{REPO_URL}/issues/new?template=feedback.yml&title=Feedback%20(v{APP_VERSION})")),
            }
            ListRow {
                title: "Credits", tint: "#ff9500", chevron: true,
                icon: rsx! { ScrollText { size: 17, stroke_width: "2.2" } },
                onclick: move |_| { navigator().push(Route::Acknowledgements {}); },
            }
        }

        GroupedList {
            footer: "No account, no tracking. Your highlights, bookmarks and settings stay on this device.".to_string(),
            ListRow {
                title: "Privacy", tint: "#0a7aff",
                icon: rsx! { ShieldCheck { size: 17, stroke_width: "2.2" } },
                detail: "On Device".to_string(),
            }
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
        p { class: "pb-4 text-center text-[13px] text-label-3", "{APP_NAME} {APP_VERSION} ({BUILD_SHA})" }

        TranslationSheet { open: translation_open }
    }
}
