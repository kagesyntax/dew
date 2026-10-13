//! Settings → Software Update, modelled on iOS: the app, its version and build, one button,
//! and the release notes for whatever is available. Releases come from GitHub Actions
//! (`.github/workflows/release.yml` publishes `version.json`).

use crate::components::switch::Switch;
use crate::config::{APP_NAME, APP_VERSION, BUILD_DATE, BUILD_SHA, CHANGELOG, RELEASE_CHANNEL};
use crate::services::updater::{self, UpdateStatus};
use crate::state::use_app;
use crate::ui::{BackBar, GroupedList, ListRow, Spinner};
use church_core::version::release_notes;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowDownToLine, CircleCheck};

pub const APP_ICON: Asset = asset!("/assets/app-icon.svg");

#[component]
pub fn SoftwareUpdate() -> Element {
    let mut app = use_app();
    let status = app.update.read().clone();
    let auto = app.settings.read().auto_update_check;

    let mut check = move || {
        app.update.set(UpdateStatus::Checking);
        spawn(async move { app.update.set(updater::check().await) });
    };
    // Check when the screen opens, like iOS.
    use_hook(move || {
        if matches!(*app.update.peek(), UpdateStatus::Idle | UpdateStatus::Failed(_) | UpdateStatus::UpToDate) {
            check();
        }
    });

    let build = if BUILD_DATE.is_empty() { BUILD_SHA.to_string() } else { format!("{BUILD_SHA} · {BUILD_DATE}") };

    rsx! {
        BackBar { back_label: "Settings", title: "Software Update", onback: move |_| { navigator().push(crate::routes::Route::SettingsView {}); } }
        div { class: "mx-auto max-w-2xl px-4 pt-4",
            match status.clone() {
                UpdateStatus::Available(m) => {
                    let notes = if m.highlights.is_empty() { release_notes(CHANGELOG, &m.version) } else { m.highlights.clone() };
                    let required = m.is_required_for(APP_VERSION);
                    rsx! {
                        div { class: "rounded-card bg-surface p-5",
                            div { class: "flex items-center gap-4",
                                img { src: APP_ICON, class: "h-14 w-14 rounded-[13px]", alt: "" }
                                div {
                                    p { class: "text-[17px] font-semibold", "{APP_NAME} {m.version}" }
                                    p { class: "text-[14px] text-label-2",
                                        if required { "Required update" } else { "{RELEASE_CHANNEL} release" }
                                        if let Some(d) = m.published_at.as_deref() { " · {d.get(..10).unwrap_or(d)}" }
                                    }
                                }
                            }
                            if !m.notes.is_empty() && notes.is_empty() {
                                p { class: "mt-4 text-[15px] leading-snug", "{m.notes}" }
                            }
                            if !notes.is_empty() {
                                ul { class: "mt-4 space-y-2",
                                    for (i, n) in notes.iter().enumerate() {
                                        li { key: "{i}", class: "flex gap-2.5 text-[15px] leading-snug",
                                            span { class: "mt-[7px] h-1.5 w-1.5 shrink-0 rounded-full bg-label-3" }
                                            "{n}"
                                        }
                                    }
                                }
                            }
                            button {
                                class: "press mt-5 h-[50px] w-full rounded-[12px] bg-accent text-[17px] font-semibold text-white",
                                onclick: move |_| {
                                    app.update.set(UpdateStatus::Installing);
                                    updater::apply(&m);
                                },
                                if cfg!(feature = "web") { "Update Now" } else { "Download Update" }
                            }
                        }
                    }
                }
                UpdateStatus::Checking | UpdateStatus::Idle => rsx! {
                    div { class: "flex flex-col items-center py-16 text-label-2",
                        Spinner {}
                        p { class: "mt-3 text-[15px]", "Checking for Update…" }
                    }
                },
                UpdateStatus::Installing => rsx! {
                    div { class: "flex flex-col items-center py-16 text-label-2",
                        Spinner {}
                        p { class: "mt-3 text-[15px]", "Installing…" }
                    }
                },
                UpdateStatus::UpToDate => rsx! {
                    div { class: "flex flex-col items-center py-12 text-center",
                        img { src: APP_ICON, class: "h-20 w-20 rounded-[18px] shadow-sm", alt: "" }
                        p { class: "mt-4 text-[20px] font-semibold", "{APP_NAME} {APP_VERSION}" }
                        p { class: "mt-1 flex items-center gap-1.5 text-[15px] text-label-2",
                            span { class: "text-[#34c759]", CircleCheck { size: 17, stroke_width: "2.4" } }
                            "{APP_NAME} is up to date"
                        }
                    }
                },
                UpdateStatus::Failed(e) => rsx! {
                    div { class: "flex flex-col items-center py-12 text-center",
                        img { src: APP_ICON, class: "h-20 w-20 rounded-[18px] shadow-sm", alt: "" }
                        p { class: "mt-4 text-[20px] font-semibold", "{APP_NAME} {APP_VERSION}" }
                        p { class: "mt-1 max-w-xs text-[15px] text-label-2", "Unable to check for update. {e}" }
                        button { class: "press mt-4 text-[17px] text-accent", onclick: move |_| check(), "Try Again" }
                    }
                },
            }

        }
        div { class: "mt-6",
                GroupedList {
                    footer: "Updates are published by the church's GitHub releases. The web app installs them in place; phone and desktop builds download the new installer.".to_string(),
                    ListRow {
                        title: "Automatic Updates", tint: "#34c759",
                        subtitle: "Check every time the app opens".to_string(),
                        icon: rsx! { ArrowDownToLine { size: 17, stroke_width: "2.2" } },
                        trailing: rsx! { Switch { checked: auto, on_checked_change: move |v| app.settings.write().auto_update_check = v } },
                    }
                }
                GroupedList {
                    ListRow { title: "Version", detail: APP_VERSION.to_string() }
                    ListRow { title: "Build", detail: build }
                    ListRow { title: "Channel", detail: RELEASE_CHANNEL.to_string() }
                    if !matches!(status, UpdateStatus::Checking | UpdateStatus::Installing) {
                        ListRow { title: "Check for Update", onclick: move |_| check() }
                    }
                }
            }
    }
}
