//! App-wide reactive state, provided once at the root and read anywhere with [`use_app`].

pub mod navigation;
pub mod progress;
pub mod settings;

use crate::platform::{storage, theme};
use dioxus::prelude::*;
use navigation::ReturnPoint;
use progress::Progress;
use settings::Settings;

const SETTINGS_KEY: &str = "settings";
const PROGRESS_KEY: &str = "progress";

#[derive(Clone, Copy)]
pub struct AppState {
    pub settings: Signal<Settings>,
    pub progress: Signal<Progress>,
    /// Where "Back to lesson" returns to after jumping into the Bible from the manual.
    pub return_point: Signal<Option<ReturnPoint>>,
    /// Scroll offset to restore once the destination screen has rendered.
    pub pending_scroll: Signal<Option<f64>>,
    /// True once persisted state has been read (the launch screen shows until then).
    pub ready: Signal<bool>,
}

pub fn use_app() -> AppState {
    use_context::<AppState>()
}

/// Creates the state, loads it from storage and keeps storage + theme in sync.
pub fn use_app_state_provider() -> AppState {
    let state = use_context_provider(|| AppState {
        settings: Signal::new(Settings::default()),
        progress: Signal::new(Progress::default()),
        return_point: Signal::new(None),
        pending_scroll: Signal::new(None),
        ready: Signal::new(false),
    });

    // Load once.
    use_hook(move || {
        let mut s = state;
        spawn(async move {
            if let Some(saved) = storage::load::<Settings>(SETTINGS_KEY).await {
                s.settings.set(saved);
            }
            if let Some(saved) = storage::load::<Progress>(PROGRESS_KEY).await {
                s.progress.set(saved);
            }
            s.ready.set(true);
        });
    });

    // Persist + apply appearance whenever settings change (after the initial load).
    use_effect(move || {
        let settings = state.settings.read().clone();
        theme::apply(settings.theme, settings.text_size);
        if *state.ready.peek() {
            storage::save(SETTINGS_KEY, &settings);
        }
    });

    use_effect(move || {
        let progress = state.progress.read().clone();
        if *state.ready.peek() {
            storage::save(PROGRESS_KEY, &progress);
        }
    });

    state
}
