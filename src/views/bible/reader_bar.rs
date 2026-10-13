//! The reader's floating toolbar, sitting just above the tab bar like Apple Books' reading
//! controls: previous chapter, the passage ("Genesis 2", tap to jump anywhere), the translation,
//! next chapter. It tucks away while you scroll down to read and comes back when you scroll up
//! or reach the end. Arrow keys turn chapters on a keyboard.

use super::books::BookChooser;
use super::translation_picker::TranslationPicker;
use crate::components::sheet::{Sheet, SheetTitle};
use crate::platform::js;
use crate::routes::Route;
use crate::state::use_app;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronDown, ChevronLeft, ChevronRight, X};

#[component]
pub fn ReaderBar(book: String, book_name: String, chapter: u16, prev: Option<Route>, next: Option<Route>) -> Element {
    let app = use_app();
    let mut picker = use_signal(|| false);
    let translations = use_context::<TranslationPicker>();
    let abbr = crate::bible::translation(&app.settings.read().translation).abbreviation;
    let nav = navigator();

    // Show/hide on scroll direction (sets <html data-chrome>), installed once per page load.
    use_hook(js::install_scroll_chrome);

    // ← / → turn chapters on a keyboard.
    let (p2, n2) = (prev.clone(), next.clone());
    use_effect(use_reactive!(|p2, n2| {
        spawn(async move {
            let mut keys = document::eval(
                r#"
                if (window.__dewKeys) window.removeEventListener('keydown', window.__dewKeys);
                window.__dewKeys = (e) => {
                    const t = e.target;
                    if (e.metaKey || e.ctrlKey || e.altKey || (t && (t.isContentEditable || /INPUT|TEXTAREA|SELECT/.test(t.tagName)))) return;
                    if (document.querySelector('.dx-sheet-root[data-state="open"]')) return;
                    if (e.key === 'ArrowLeft') dioxus.send('prev');
                    if (e.key === 'ArrowRight') dioxus.send('next');
                };
                window.addEventListener('keydown', window.__dewKeys);
                "#,
            );
            while let Ok(dir) = keys.recv::<String>().await {
                let target = if dir == "prev" { p2.clone() } else { n2.clone() };
                if let Some(r) = target {
                    navigator().push(r);
                }
            }
        });
    }));

    let current = Some((book.clone(), chapter));
    let btn = "press flex h-11 w-11 shrink-0 items-center justify-center rounded-full text-label disabled:text-label-3";

    rsx! {
        div {
            class: "reader-bar pointer-events-none fixed inset-x-0 z-30 flex justify-center px-4",
            style: "bottom: calc(max(env(safe-area-inset-bottom), 8px) + 62px);",
            div { class: "reader-capsule pointer-events-auto flex h-[52px] w-full max-w-[22rem] items-center gap-0.5 rounded-full border border-separator px-1 shadow-lg shadow-black/10",
                button {
                    class: btn,
                    aria_label: "Previous chapter",
                    disabled: prev.is_none(),
                    onclick: move |_| { if let Some(r) = prev.clone() { js::haptic(); nav.push(r); } },
                    ChevronLeft { size: 24, stroke_width: "2.3" }
                }
                button {
                    class: "press flex h-11 min-w-0 flex-1 items-center justify-center gap-1 rounded-full px-2 text-[16px] font-semibold",
                    aria_label: "Choose book and chapter",
                    onclick: move |_| picker.set(true),
                    span { class: "truncate", "{book_name} {chapter}" }
                    ChevronDown { size: 15, stroke_width: "2.8", class: "shrink-0 text-label-2" }
                }
                span { class: "h-5 w-px bg-separator" }
                button {
                    class: "press flex h-11 shrink-0 items-center rounded-full px-3 text-[13px] font-bold tracking-wide text-label-2",
                    aria_label: "Translation {abbr}",
                    onclick: move |_| translations.open(),
                    "{abbr}"
                }
                button {
                    class: btn,
                    aria_label: "Next chapter",
                    disabled: next.is_none(),
                    onclick: move |_| { if let Some(r) = next.clone() { js::haptic(); nav.push(r); } },
                    ChevronRight { size: 24, stroke_width: "2.3" }
                }
            }
        }

        Sheet { open: picker(), on_open_change: move |v| picker.set(v), "data-side": "bottom", class: "passage-sheet",
            div { class: "flex items-center justify-between px-5 pt-1",
                SheetTitle { class: "text-[22px] font-bold tracking-tight", "Go To" }
                button {
                    class: "press rounded-full bg-fill p-1.5 text-label-2",
                    aria_label: "Close",
                    onclick: move |_| picker.set(false),
                    X { size: 18, stroke_width: "2.6" }
                }
            }
            div { class: "px-4",
                if picker() {
                    BookChooser {
                        current: current.clone(),
                        reveal_current: true,
                        sticky_search: true,
                        on_pick: move |r: Route| {
                            picker.set(false);
                            nav.push(r);
                        },
                    }
                }
            }
        }
    }
}
