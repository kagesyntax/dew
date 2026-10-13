//! Manual content store. Order of precedence:
//! 1. Lessons synced from Turso (cached in local storage, so they work offline)
//! 2. The manual bundled with the app (`assets/manual/manual.json`)

use crate::platform::storage;
use crate::services::turso::{Turso, TursoError};
use church_core::manual::{Lesson, Manual};
use dioxus::prelude::*;

const BUNDLED: &str = include_str!("../../assets/manual/manual.json");
const CACHE_KEY: &str = "manual.cache";

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Synced(usize),
    Offline(String),
}

#[derive(Clone, Copy)]
pub struct ManualStore {
    pub manual: Signal<Manual>,
    pub sync: Signal<SyncStatus>,
}

pub fn use_manual() -> ManualStore {
    use_context::<ManualStore>()
}

pub fn bundled() -> Manual {
    serde_json::from_str(BUNDLED).unwrap_or_default()
}

pub fn use_manual_provider() -> ManualStore {
    let store =
        use_context_provider(|| ManualStore { manual: Signal::new(bundled()), sync: Signal::new(SyncStatus::Idle) });
    use_hook(move || {
        let mut store = store;
        spawn(async move {
            if let Some(cached) = storage::load::<Manual>(CACHE_KEY).await {
                if !cached.lessons.is_empty() {
                    store.manual.set(cached);
                }
            }
            // Quietly refresh from Turso on launch when configured.
            if Turso::from_config().is_ok() {
                sync(store).await;
            }
        });
    });
    store
}

/// Pull the latest lessons from Turso and cache them.
pub async fn sync(mut store: ManualStore) {
    store.sync.set(SyncStatus::Syncing);
    match fetch_remote().await {
        Ok(lessons) if !lessons.is_empty() => {
            let mut manual = store.manual.peek().clone();
            let count = lessons.len();
            manual.lessons = lessons;
            storage::save(CACHE_KEY, &manual);
            store.manual.set(manual);
            store.sync.set(SyncStatus::Synced(count));
        }
        Ok(_) => store.sync.set(SyncStatus::Synced(0)),
        Err(e) => store.sync.set(SyncStatus::Offline(e.to_string())),
    }
}

async fn fetch_remote() -> Result<Vec<Lesson>, TursoError> {
    let db = Turso::from_config()?;
    let rows = db.query("SELECT payload FROM lessons WHERE published = 1 ORDER BY lesson_date ASC", &[]).await?;
    Ok(rows
        .into_iter()
        .filter_map(|r| r.into_iter().next())
        .filter_map(|v| v.as_str().and_then(|s| serde_json::from_str::<Lesson>(s).ok()))
        .collect())
}
