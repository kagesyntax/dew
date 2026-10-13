//! Reading position, lesson progress, highlights and bookmarks.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Highlight {
    Yellow,
    Green,
    Blue,
    Pink,
}

impl Highlight {
    pub const ALL: [Highlight; 4] = [Highlight::Yellow, Highlight::Green, Highlight::Blue, Highlight::Pink];

    pub fn as_str(self) -> &'static str {
        match self {
            Highlight::Yellow => "yellow",
            Highlight::Green => "green",
            Highlight::Blue => "blue",
            Highlight::Pink => "pink",
        }
    }

    pub fn swatch(self) -> &'static str {
        match self {
            Highlight::Yellow => "#ffd60a",
            Highlight::Green => "#34c759",
            Highlight::Blue => "#0a84ff",
            Highlight::Pink => "#ff375f",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadingPosition {
    pub book: String,
    pub chapter: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Progress {
    pub last_read: Option<ReadingPosition>,
    pub last_lesson: Option<String>,
    pub completed_lessons: Vec<String>,
    /// verse key (`JHN.3.16`) -> colour
    pub highlights: BTreeMap<String, Highlight>,
    /// verse keys, newest first
    pub bookmarks: Vec<String>,
    /// TipKit-style tips the user has dismissed or acted on.
    pub seen_tips: Vec<String>,
}

impl Progress {
    pub fn is_completed(&self, lesson_id: &str) -> bool {
        self.completed_lessons.iter().any(|l| l == lesson_id)
    }

    pub fn toggle_completed(&mut self, lesson_id: &str) {
        if let Some(i) = self.completed_lessons.iter().position(|l| l == lesson_id) {
            self.completed_lessons.remove(i);
        } else {
            self.completed_lessons.push(lesson_id.to_string());
        }
    }

    pub fn has_seen(&self, tip: &str) -> bool {
        self.seen_tips.iter().any(|t| t == tip)
    }

    pub fn mark_seen(&mut self, tip: &str) {
        if !self.has_seen(tip) {
            self.seen_tips.push(tip.to_string());
        }
    }

    pub fn toggle_bookmark(&mut self, key: &str) {
        if let Some(i) = self.bookmarks.iter().position(|k| k == key) {
            self.bookmarks.remove(i);
        } else {
            self.bookmarks.insert(0, key.to_string());
        }
    }
}
