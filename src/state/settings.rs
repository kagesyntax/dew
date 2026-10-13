//! User preferences. Every field has a serde default so old saved settings keep loading after
//! new options are added.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

impl ThemeMode {
    pub const ALL: [ThemeMode; 3] = [ThemeMode::System, ThemeMode::Light, ThemeMode::Dark];

    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::System => "System",
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
        }
    }
}

/// What tapping a scripture reference inside the manual does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LinkMode {
    /// Bottom sheet preview with "Open in Bible".
    #[default]
    Sheet,
    /// Expand the verses inline, right under the paragraph.
    Inline,
    /// Jump straight to the Bible reader (with a "Back to lesson" pill).
    Navigate,
}

impl LinkMode {
    pub const ALL: [LinkMode; 3] = [LinkMode::Sheet, LinkMode::Inline, LinkMode::Navigate];

    pub fn label(self) -> &'static str {
        match self {
            LinkMode::Sheet => "Preview",
            LinkMode::Inline => "Inline",
            LinkMode::Navigate => "Open Bible",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            LinkMode::Sheet => "Show verses in a sheet without leaving the lesson.",
            LinkMode::Inline => "Expand verses right inside the lesson text.",
            LinkMode::Navigate => "Jump to the Bible. A pill brings you back to the lesson.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub onboarded: bool,
    pub theme: ThemeMode,
    /// Translation id (`kjv`, `bsb`, `asv`).
    pub translation: String,
    /// 0..=4, maps to html[data-text].
    pub text_size: u8,
    pub verse_numbers: bool,
    pub link_mode: LinkMode,
    pub auto_update_check: bool,
    /// Teacher mode: show the manual's bracketed [Note:]/[Hint:] guidance.
    pub leader_notes: bool,
    pub display_name: String,
    /// Last version whose What's New sheet was shown.
    pub last_seen_version: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            onboarded: false,
            theme: ThemeMode::System,
            translation: "kjv".into(),
            text_size: 2,
            verse_numbers: true,
            link_mode: LinkMode::Sheet,
            auto_update_check: true,
            leader_notes: false,
            display_name: String::new(),
            last_seen_version: String::new(),
        }
    }
}
