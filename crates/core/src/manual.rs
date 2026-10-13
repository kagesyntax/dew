//! Sunday School manual data model.
//!
//! The same JSON shape is used for the bundled manual (`assets/manual/manual.json`), the
//! Turso `lessons.payload` column and the offline cache, so content can be published from
//! the database without shipping a new app version.

use crate::scripture::{find_references, Reference};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Manual {
    pub title: String,
    #[serde(default)]
    pub edition: String,
    pub lessons: Vec<Lesson>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryVerse {
    pub text: String,
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Section {
    pub heading: String,
    #[serde(default)]
    pub body: Vec<String>,
    /// Numbered teaching points; any scripture inside becomes a link.
    #[serde(default)]
    pub points: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub number: u32,
    /// ISO date of the Sunday the lesson is taught (`2026-10-18`).
    pub date: String,
    pub title: String,
    pub memory_verse: MemoryVerse,
    /// Main passage, e.g. `"Romans 12:1-8"`.
    pub bible_passage: String,
    #[serde(default)]
    pub introduction: Vec<String>,
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(default)]
    pub conclusion: Vec<String>,
    #[serde(default)]
    pub questions: Vec<String>,
    #[serde(default)]
    pub prayer: Option<String>,
    #[serde(default)]
    pub assignment: Option<String>,
}

impl Lesson {
    /// Every scripture reference mentioned anywhere in the lesson, de-duplicated, in order.
    pub fn references(&self) -> Vec<Reference> {
        let mut texts: Vec<&str> = vec![&self.bible_passage, &self.memory_verse.reference];
        texts.extend(self.introduction.iter().map(String::as_str));
        for s in &self.sections {
            texts.push(&s.heading);
            texts.extend(s.body.iter().map(String::as_str));
            texts.extend(s.points.iter().map(String::as_str));
        }
        texts.extend(self.conclusion.iter().map(String::as_str));
        texts.extend(self.questions.iter().map(String::as_str));
        texts.extend(self.assignment.iter().map(String::as_str));

        let mut out: Vec<Reference> = Vec::new();
        for t in texts {
            for f in find_references(t) {
                if !out.iter().any(|r| r.label() == f.reference.label()) {
                    out.push(f.reference);
                }
            }
        }
        out
    }

    /// Plain-text search haystack.
    pub fn matches(&self, query: &str) -> bool {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return true;
        }
        let hay = format!(
            "{} {} {} {} {}",
            self.number,
            self.title,
            self.bible_passage,
            self.memory_verse.text,
            self.sections.iter().map(|s| s.heading.as_str()).collect::<Vec<_>>().join(" ")
        )
        .to_lowercase();
        hay.contains(&q)
    }
}

impl Manual {
    pub fn lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.iter().find(|l| l.id == id)
    }

    /// The lesson for the coming Sunday (first lesson dated today or later), falling back to the
    /// most recent one. `today` is an ISO date string; ISO dates compare correctly as strings.
    pub fn current_lesson(&self, today: &str) -> Option<&Lesson> {
        let mut sorted: Vec<&Lesson> = self.lessons.iter().collect();
        sorted.sort_by(|a, b| a.date.cmp(&b.date));
        sorted.iter().find(|l| l.date.as_str() >= today).copied().or_else(|| sorted.last().copied())
    }

    /// Lessons before/after `id` in teaching order.
    pub fn neighbours(&self, id: &str) -> (Option<&Lesson>, Option<&Lesson>) {
        let i = self.lessons.iter().position(|l| l.id == id);
        match i {
            Some(i) => (i.checked_sub(1).and_then(|p| self.lessons.get(p)), self.lessons.get(i + 1)),
            None => (None, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Manual {
        serde_json::from_str(include_str!("../../../assets/manual/manual.json")).expect("bundled manual parses")
    }

    #[test]
    fn bundled_manual_is_valid() {
        let m = sample();
        assert!(!m.lessons.is_empty());
        for l in &m.lessons {
            assert!(crate::scripture::parse(&l.bible_passage).is_some(), "bad passage in {}", l.id);
            assert!(crate::scripture::parse(&l.memory_verse.reference).is_some(), "bad memory verse in {}", l.id);
        }
    }

    #[test]
    fn finds_current_lesson() {
        let m = sample();
        let first = &m.lessons[0];
        assert_eq!(m.current_lesson("1900-01-01").unwrap().id, first.id);
        assert_eq!(m.current_lesson("2999-01-01").unwrap().id, m.lessons.last().unwrap().id);
    }

    #[test]
    fn collects_references() {
        let m = sample();
        assert!(m.lessons[0].references().len() >= 3);
    }
}
