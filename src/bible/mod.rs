//! Bible text: available translations and chapter loading.

pub mod loader;
pub mod translation;
pub mod votd;

pub use loader::{load_book, passage_verses, PassageVerse};
pub use translation::{translation, TRANSLATIONS};
