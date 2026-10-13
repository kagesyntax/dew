//! Bible tab: book/chapter picker and the reader.

mod books;
mod reader;
mod reader_bar;
mod selection_bar;
mod translation_picker;

pub use books::BibleHome;
pub use reader::Reader;
pub use translation_picker::{TranslationHost, TranslationSheet};
