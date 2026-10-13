//! Every screen and its URL. Layouts nest: `Shell` gates launch/onboarding, `TabLayout` adds
//! the iOS-style tab bar.

use crate::views::{
    bible::{BibleHome, Reader},
    manual::{LessonView, ManualHome},
    NotFound, SettingsView, Shell, TabLayout, Today,
};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(Shell)]
        #[layout(TabLayout)]
            #[route("/")]
            Today {},
            #[route("/bible")]
            BibleHome {},
            /// `focus` is a verse or range to highlight and scroll to (`"16"`, `"16-18"`).
            #[route("/bible/:book/:chapter?:focus")]
            Reader { book: String, chapter: u16, focus: String },
            #[route("/manual")]
            ManualHome {},
            #[route("/manual/:id")]
            LessonView { id: String },
            #[route("/settings")]
            SettingsView {},
        #[end_layout]
    #[end_layout]
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

impl Route {
    pub fn reader(book: &str, chapter: u16) -> Self {
        Route::Reader { book: book.to_string(), chapter, focus: String::new() }
    }

    pub fn reader_focus(passage: &church_core::scripture::Passage) -> Self {
        Route::Reader { book: passage.book.id.to_string(), chapter: passage.chapter, focus: passage.focus_query() }
    }
}
