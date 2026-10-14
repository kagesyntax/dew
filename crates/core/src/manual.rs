//! Sunday School manual data model, shaped after the church's printed manual
//! ("Radiant Life" layout):
//!
//! ```text
//! October 18, 2026
//! Unit 4: ANCIENT FAITH/FUTURE FAITH      Study 4: Applying Your Confidence
//! Text: Romans 10:17; Heb 11:1-8,29-30,32-40
//! Memory Verse: Hebrews 11:1 — "Now faith is ..."
//! >>Getting Started            (paragraphs)
//! >>Activity Option            (see Teacher Guide + note)
//! >>Study Overview             (bullets)
//! >>Discussion Questions       a. b. c. sections -> numbered questions [Note:/Hint: for leaders]
//! >>Invitation Option          (closing call to action, after the questions)
//! >>Act                        (application)
//! >>Reach Out                   (a service to someone else)
//! >>Daily Dew Drops            Mon..Sat devotional readings
//! ```
//!
//! The same JSON shape is used for the bundled manual (`assets/manual/manual.json`), the
//! Turso `lessons.payload` column and the offline cache.

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
    pub reference: String,
    pub text: String,
    /// Translation the manual quotes (e.g. "KJV").
    #[serde(default)]
    pub version: Option<String>,
}

/// "Activity Option" / "Illustration Option" box.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LessonOption {
    pub label: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

/// A named sub-point under a question ("Abel – What are some sincere ways...").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bullet {
    #[serde(default)]
    pub title: Option<String>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    /// Bracketed leader guidance: `[Note: ...]` / `[Hint: ...]`.
    #[serde(default)]
    pub leader_note: Option<String>,
    #[serde(default)]
    pub bullets: Vec<Bullet>,
}

/// Lettered discussion section: "a. God is worthy of our faith."
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscussionSection {
    pub letter: String,
    pub heading: String,
    /// Optional lead-in paragraph ("Read Php 1:3-6. Note that while ...").
    #[serde(default)]
    pub intro: Option<String>,
    #[serde(default)]
    pub questions: Vec<Question>,
    /// A closing "Note:" printed after the questions.
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DewDrop {
    /// "Mon".."Sat"
    pub day: String,
    pub title: String,
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    /// ISO date of the Sunday the lesson is taught (`2026-10-18`).
    pub date: String,
    pub unit_number: u32,
    pub unit_title: String,
    pub study_number: u32,
    pub title: String,
    /// Main text, e.g. `"Romans 10:17; Heb 11:1-8,29-30,32-40"`.
    pub text: String,
    pub memory_verse: MemoryVerse,
    #[serde(default)]
    pub getting_started: Vec<String>,
    #[serde(default)]
    pub option: Option<LessonOption>,
    /// The second call-to-action box, printed after the discussion questions
    /// (">>Invitation Option: ... See pg2 (AC.CONFIRMATION) for guidance"). Lessons that have no
    /// closing invitation leave this unset.
    #[serde(default)]
    pub invitation_option: Option<LessonOption>,
    /// A second ">>Illustration Option" box. A few lessons print both an Activity Option and an
    /// Illustration Option; the Activity Option stays in `option`.
    #[serde(default)]
    pub illustration_option: Option<LessonOption>,
    /// The closing service box, printed after the Act (">>Reach Out: Is the example you set
    /// every day pleasing to God...").
    #[serde(default)]
    pub reach_out: Option<LessonOption>,
    #[serde(default)]
    pub overview: Vec<String>,
    #[serde(default)]
    pub discussion: Vec<DiscussionSection>,
    #[serde(default)]
    pub act: Option<String>,
    #[serde(default)]
    pub daily_dew_drops: Vec<DewDrop>,
    /// Printed page range, e.g. "39–40".
    #[serde(default)]
    pub pages: Option<String>,
}

impl Lesson {
    /// "Unit 4 · Study 4"
    pub fn kicker(&self) -> String {
        format!("Unit {} · Study {}", self.unit_number, self.study_number)
    }

    /// Every scripture reference mentioned anywhere in the lesson, de-duplicated, in order.
    pub fn references(&self) -> Vec<Reference> {
        let mut texts: Vec<&str> = vec![&self.text, &self.memory_verse.reference];
        texts.extend(self.getting_started.iter().map(String::as_str));
        for s in &self.discussion {
            texts.extend(s.intro.iter().map(String::as_str));
            for q in &s.questions {
                texts.push(&q.text);
                texts.extend(q.leader_note.iter().map(String::as_str));
                texts.extend(q.bullets.iter().map(|b| b.text.as_str()));
            }
            texts.extend(s.note.iter().map(String::as_str));
        }
        texts.extend(self.act.iter().map(String::as_str));
        texts.extend(self.daily_dew_drops.iter().map(|d| d.reference.as_str()));

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

    pub fn matches(&self, query: &str) -> bool {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return true;
        }
        let hay = format!(
            "{} {} {} {} {} {}",
            self.unit_title,
            self.title,
            self.text,
            self.memory_verse.reference,
            self.memory_verse.text,
            self.discussion.iter().map(|s| s.heading.as_str()).collect::<Vec<_>>().join(" ")
        )
        .to_lowercase();
        hay.contains(&q)
    }

    /// Dew drop for a weekday index (0 = Monday .. 5 = Saturday).
    pub fn dew_drop_for(&self, weekday_from_monday: u32) -> Option<&DewDrop> {
        const DAYS: [&str; 6] = ["mon", "tue", "wed", "thu", "fri", "sat"];
        let key = DAYS.get(weekday_from_monday as usize)?;
        self.daily_dew_drops.iter().find(|d| d.day.to_lowercase().starts_with(key))
    }
}

impl Manual {
    pub fn lesson(&self, id: &str) -> Option<&Lesson> {
        self.lessons.iter().find(|l| l.id == id)
    }

    fn sorted(&self) -> Vec<&Lesson> {
        let mut v: Vec<&Lesson> = self.lessons.iter().collect();
        v.sort_by(|a, b| a.date.cmp(&b.date));
        v
    }

    /// The lesson for the coming Sunday (first lesson dated today or later), falling back to the
    /// most recent one. ISO dates compare correctly as strings.
    pub fn current_lesson(&self, today: &str) -> Option<&Lesson> {
        let sorted = self.sorted();
        sorted.iter().find(|l| l.date.as_str() >= today).copied().or_else(|| sorted.last().copied())
    }

    /// The lesson whose week (the Sunday it was taught through Saturday) contains `today`.
    /// Daily Dew Drops printed under a lesson are read in the days after it is taught.
    pub fn devotional_lesson(&self, today: &str) -> Option<&Lesson> {
        self.sorted().into_iter().rev().find(|l| l.date.as_str() <= today)
    }

    pub fn neighbours(&self, id: &str) -> (Option<&Lesson>, Option<&Lesson>) {
        let sorted = self.sorted();
        match sorted.iter().position(|l| l.id == id) {
            Some(i) => (i.checked_sub(1).map(|p| sorted[p]), sorted.get(i + 1).copied()),
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
            let text = crate::scripture::parse(&l.text).unwrap_or_else(|| panic!("bad text in {}", l.id));
            assert!(!text.passages.is_empty());
            assert!(crate::scripture::parse(&l.memory_verse.reference).is_some(), "bad memory verse in {}", l.id);
            for d in &l.daily_dew_drops {
                assert!(!find_references(&d.reference).is_empty(), "bad dew drop {} in {}", d.reference, l.id);
            }
        }
    }

    #[test]
    fn multi_book_text_is_detected() {
        let refs = find_references("Romans 10:17; Heb 11:1-8,29-30,32-40");
        let labels: Vec<String> = refs.iter().map(|f| f.reference.label()).collect();
        assert_eq!(labels, vec!["Romans 10:17", "Hebrews 11:1–8, 29–30, 32–40"]);
    }

    #[test]
    fn finds_current_and_devotional_lesson() {
        let m = sample();
        assert_eq!(m.current_lesson("2026-10-13").unwrap().id, "2026-10-18");
        assert_eq!(m.current_lesson("2999-01-01").unwrap().id, m.sorted().last().unwrap().id);
        assert_eq!(m.devotional_lesson("2026-10-20").unwrap().id, "2026-10-18");
    }

    #[test]
    fn collects_references_and_dew_drops() {
        let m = sample();
        let l = m.lesson("2026-10-18").unwrap();
        assert!(l.references().len() >= 8);
        assert_eq!(l.dew_drop_for(0).unwrap().title, "Unfathomable");
    }
}
