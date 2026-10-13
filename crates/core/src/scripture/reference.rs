//! Value types for parsed scripture references.

use super::books::{self, Book};
use std::fmt;

/// One contiguous run of scripture inside a single book.
///
/// * `John 3`          -> chapter 3..=3, verses None
/// * `John 3:16`       -> chapter 3..=3, verses 16..=16
/// * `John 3:16-4:2`   -> chapter 3..=4, start 16, end 2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passage {
    pub book: &'static Book,
    pub chapter: u16,
    pub end_chapter: u16,
    pub start_verse: Option<u16>,
    pub end_verse: Option<u16>,
}

impl Passage {
    pub fn chapter(book: &'static Book, chapter: u16) -> Self {
        Self { book, chapter, end_chapter: chapter, start_verse: None, end_verse: None }
    }

    pub fn verses(book: &'static Book, chapter: u16, start: u16, end: u16) -> Self {
        Self { book, chapter, end_chapter: chapter, start_verse: Some(start), end_verse: Some(end.max(start)) }
    }

    /// Does this passage include `chapter:verse`?
    pub fn contains(&self, chapter: u16, verse: u16) -> bool {
        if chapter < self.chapter || chapter > self.end_chapter {
            return false;
        }
        let after_start = chapter > self.chapter || self.start_verse.is_none_or(|s| verse >= s);
        let before_end = chapter < self.end_chapter || self.end_verse.is_none_or(|e| verse <= e);
        after_start && before_end
    }

    /// Compact focus string used in reader URLs: `"16-18"` (same chapter) or `""`.
    pub fn focus_query(&self) -> String {
        match (self.start_verse, self.end_verse) {
            (Some(s), Some(e)) if self.chapter == self.end_chapter && e != s => format!("{s}-{e}"),
            (Some(s), _) => s.to_string(),
            _ => String::new(),
        }
    }

    /// Label without the book name, e.g. `3:16–18`.
    pub fn short_label(&self) -> String {
        match (self.start_verse, self.end_verse) {
            (None, _) if self.chapter == self.end_chapter => format!("{}", self.chapter),
            (None, _) => format!("{}–{}", self.chapter, self.end_chapter),
            (Some(s), Some(e)) if self.chapter == self.end_chapter && s == e => format!("{}:{s}", self.chapter),
            (Some(s), Some(e)) if self.chapter == self.end_chapter => format!("{}:{s}–{e}", self.chapter),
            (Some(s), Some(e)) => format!("{}:{s}–{}:{e}", self.chapter, self.end_chapter),
            (Some(s), None) => format!("{}:{s}", self.chapter),
        }
    }
}

impl fmt::Display for Passage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.book.name, self.short_label())
    }
}

/// A full reference as written in a manual, possibly spanning several passages:
/// `Romans 12:1-2, 9; 13:8`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub passages: Vec<Passage>,
}

impl Reference {
    pub fn first(&self) -> &Passage {
        &self.passages[0]
    }

    pub fn book(&self) -> &'static Book {
        self.passages[0].book
    }

    /// Canonical label: `Romans 12:1–2, 9; 13:8`.
    pub fn label(&self) -> String {
        let mut out = String::new();
        let mut prev: Option<&Passage> = None;
        for p in &self.passages {
            match prev {
                None => out.push_str(&p.to_string()),
                Some(q) if q.book.id != p.book.id => {
                    out.push_str("; ");
                    out.push_str(&p.to_string());
                }
                Some(q) if q.chapter == p.chapter && q.end_chapter == p.chapter && p.start_verse.is_some() => {
                    out.push_str(", ");
                    match (p.start_verse, p.end_verse) {
                        (Some(s), Some(e)) if s != e => out.push_str(&format!("{s}–{e}")),
                        (Some(s), _) => out.push_str(&s.to_string()),
                        _ => {}
                    }
                }
                Some(_) => {
                    out.push_str("; ");
                    out.push_str(&p.short_label());
                }
            }
            prev = Some(p);
        }
        out
    }

    /// Stable key for storage (`"JHN.3.16-18"`).
    pub fn key(&self) -> String {
        let p = self.first();
        format!("{}.{}.{}", p.book.id, p.chapter, p.focus_query())
    }
}

/// Key for a single verse, used by highlights/bookmarks (`"JHN.3.16"`).
pub fn verse_key(book: &str, chapter: u16, verse: u16) -> String {
    format!("{book}.{chapter}.{verse}")
}

/// Parse a verse key back into parts.
pub fn parse_verse_key(key: &str) -> Option<(&'static Book, u16, u16)> {
    let mut it = key.split('.');
    let book = books::by_id(it.next()?)?;
    let ch = it.next()?.parse().ok()?;
    let v = it.next()?.parse().ok()?;
    Some((book, ch, v))
}
