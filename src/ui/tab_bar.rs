use crate::routes::Route;
use crate::state::use_app;
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, GraduationCap, House, Settings};

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Today,
    Bible,
    Manual,
    Settings,
}

fn active_tab(route: &Route) -> Tab {
    match route {
        Route::BibleHome {} | Route::Reader { .. } => Tab::Bible,
        Route::ManualHome {} | Route::LessonView { .. } => Tab::Manual,
        Route::SettingsView {} | Route::SoftwareUpdate {} | Route::Acknowledgements {} => Tab::Settings,
        _ => Tab::Today,
    }
}

/// Bottom tab bar with translucent material, like UITabBar.
#[component]
pub fn TabBar() -> Element {
    let route = use_route::<Route>();
    let active = active_tab(&route);
    let app = use_app();
    // The Bible tab reopens the chapter you were reading, like YouVersion.
    // Settings → Bible → Open Where I Left Off decides whether the tab resumes reading.
    let resume = app.settings.read().resume_reading;
    let bible_target = match app.progress.read().last_read.clone() {
        Some(pos) if resume => Route::Reader { book: pos.book, chapter: pos.chapter, focus: pos.focus },
        _ => Route::BibleHome {},
    };

    let update_ready = matches!(*app.update.read(), crate::services::updater::UpdateStatus::Available(_));
    rsx! {
        nav { class: "bar-blur safe-bottom fixed inset-x-0 bottom-0 z-40 border-t border-separator",
            div { class: "mx-auto grid max-w-2xl grid-cols-4 pt-1.5",
                TabItem { to: Route::Today {}, label: "Today", active: active == Tab::Today,
                    House { size: 25, stroke_width: if active == Tab::Today { "2.3" } else { "1.8" } }
                }
                TabItem { to: bible_target, label: "Bible", active: active == Tab::Bible,
                    BookOpen { size: 25, stroke_width: if active == Tab::Bible { "2.3" } else { "1.8" } }
                }
                TabItem { to: Route::ManualHome {}, label: "Manual", active: active == Tab::Manual,
                    GraduationCap { size: 25, stroke_width: if active == Tab::Manual { "2.3" } else { "1.8" } }
                }
                TabItem { to: Route::SettingsView {}, label: "Settings", active: active == Tab::Settings,
                    span { class: "relative",
                        Settings { size: 25, stroke_width: if active == Tab::Settings { "2.3" } else { "1.8" } }
                        if update_ready {
                            span { class: "absolute -top-1 -right-1.5 h-[11px] w-[11px] rounded-full border-2 border-bg bg-[#ff3b30]", aria_label: "Update available" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TabItem(to: Route, label: &'static str, active: bool, children: Element) -> Element {
    let color = if active { "text-accent" } else { "text-label-2" };
    rsx! {
        Link {
            to,
            class: "press flex flex-col items-center gap-0.5 pb-0.5 {color}",
            aria_label: label,
            {children}
            span { class: "text-[10px] font-medium tracking-tight", "{label}" }
        }
    }
}
