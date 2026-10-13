//! Turns written references ("Jn 3:16-18; Rom. 8:28, 31") into [`Reference`]s and finds them
//! inside free text so the manual can render them as tappable links.

use super::books::{self, Book, BOOKS};
use super::reference::{Passage, Reference};
use regex::Regex;
use std::sync::OnceLock;

/// A reference found in a block of text, with byte offsets into the original string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    pub start: usize,
    pub end: usize,
    pub reference: Reference,
}

fn book_alternation() -> String {
    // Every spelling we accept, stripped of any leading number; numbered books get the
    // number from the optional prefix group in the main regex.
    let mut words: Vec<String> = Vec::new();
    for b in BOOKS {
        let mut push = |s: &str| {
            let s = s.trim_start_matches(|c: char| c.is_ascii_digit()).trim().to_lowercase();
            if !s.is_empty() && !words.contains(&s) {
                words.push(s);
            }
        };
        push(b.name);
        for a in b.aliases {
            push(a);
        }
    }
    words.push("psalm".into());
    words.push("song of songs".into());
    // Longest first so "philemon" wins over "phil".
    words.sort_by_key(|w| std::cmp::Reverse(w.len()));
    words
        .iter()
        .map(|w| regex::escape(w).replace(' ', r"\s+"))
        .collect::<Vec<_>>()
        .join("|")
}

fn regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        let num = r"\d+(?::\d+)?(?:\s*[-–—]\s*\d+(?::\d+)?)?";
        let pattern = format!(
            r"(?i)\b(?P<prefix>[123]\s*|(?:iii|ii|i|first|second|third)\s+)?(?P<book>{books})\b\.?\s*(?P<body>{num}(?:\s*[,;]\s*{num})*)",
            books = book_alternation(),
            num = num
        );
        Regex::new(&pattern).expect("valid scripture regex")
    })
}

fn resolve_book(prefix: Option<&str>, word: &str) -> Option<&'static Book> {
    let word = word.split_whitespace().collect::<Vec<_>>().join(" ");
    match prefix.map(str::trim).filter(|p| !p.is_empty()) {
        Some(p) => books::lookup(&format!("{p} {word}")),
        None => books::lookup(&word),
    }
}

/// Parse the numeric body ("3:16-18, 20; 4:1") of a reference for `book`.
fn parse_body(book: &'static Book, body: &str) -> Option<Vec<Passage>> {
    let mut passages = Vec::new();
    let mut chapter: Option<u16> = None;
    let mut verse_context: bool;

    for segment in body.split(';') {
        // A ';' always starts a new chapter context.
        verse_context = false;
        for item in segment.split(',') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            let (lhs, rhs) = match item.split_once(['-', '–', '—']) {
                Some((a, b)) => (a.trim(), Some(b.trim())),
                None => (item, None),
            };
            let parse_cv = |s: &str| -> Option<(Option<u16>, u16)> {
                match s.split_once(':') {
                    Some((c, v)) => Some((Some(c.trim().parse().ok()?), v.trim().parse().ok()?)),
                    None => Some((None, s.parse().ok()?)),
                }
            };
            let (lc, ln) = parse_cv(lhs)?;
            let passage = match (lc, verse_context) {
                // "3:16" or "3:16-18" or "3:16-4:2"
                (Some(c), _) => {
                    chapter = Some(c);
                    verse_context = true;
                    match rhs.map(parse_cv) {
                        Some(Some((Some(c2), v2))) => Passage {
                            book,
                            chapter: c,
                            end_chapter: c2.max(c),
                            start_verse: Some(ln),
                            end_verse: Some(v2),
                        },
                        Some(Some((None, v2))) => Passage::verses(book, c, ln, v2),
                        Some(None) => return None,
                        None => Passage::verses(book, c, ln, ln),
                    }
                }
                // "20" after a verse in the same chapter -> verse 20
                (None, true) => {
                    let c = chapter?;
                    match rhs.map(parse_cv) {
                        Some(Some((None, v2))) => Passage::verses(book, c, ln, v2),
                        Some(Some((Some(c2), v2))) => Passage {
                            book,
                            chapter: c,
                            end_chapter: c2.max(c),
                            start_verse: Some(ln),
                            end_verse: Some(v2),
                        },
                        Some(None) => return None,
                        None => Passage::verses(book, c, ln, ln),
                    }
                }
                // "3" or "3-4" -> whole chapter(s)
                (None, false) => {
                    chapter = Some(ln);
                    match rhs.map(parse_cv) {
                        Some(Some((None, c2))) => Passage { book, chapter: ln, end_chapter: c2.max(ln), start_verse: None, end_verse: None },
                        Some(Some((Some(c2), v2))) => Passage {
                            book,
                            chapter: ln,
                            end_chapter: c2.max(ln),
                            start_verse: Some(1),
                            end_verse: Some(v2),
                        },
                        Some(None) => return None,
                        None => Passage::chapter(book, ln),
                    }
                }
            };
            if passage.chapter == 0 || passage.end_chapter > book.chapters {
                return None;
            }
            passages.push(passage);
        }
    }
    (!passages.is_empty()).then_some(passages)
}

/// Parse a single reference typed by a user ("jn 3:16", "Psalm 23", "1 cor 13:4-7").
/// Lenient: any known abbreviation is accepted.
pub fn parse(input: &str) -> Option<Reference> {
    let caps = regex().captures(input.trim())?;
    let book = resolve_book(caps.name("prefix").map(|m| m.as_str()), caps.name("book")?.as_str())?;
    let passages = parse_body(book, caps.name("body")?.as_str())?;
    Some(Reference { passages })
}

/// Parse just a book name ("Romans", "1 jn") with no numbers.
pub fn parse_book(input: &str) -> Option<&'static Book> {
    books::lookup(input)
}

/// Find every reference inside free text. Stricter than [`parse`] to avoid false positives in
/// prose ("it is 5 days"): short abbreviations need a `chapter:verse` form, full names don't.
pub fn find_references(text: &str) -> Vec<Found> {
    let mut found = Vec::new();
    for caps in regex().captures_iter(text) {
        let (Some(all), Some(book_m), Some(body_m)) = (caps.get(0), caps.name("book"), caps.name("body")) else {
            continue;
        };
        let prefix = caps.name("prefix").map(|m| m.as_str());
        let Some(book) = resolve_book(prefix, book_m.as_str()) else { continue };
        let body = body_m.as_str().trim_end_matches([',', ';', ' ', '-', '–']);
        let is_full_name = {
            let typed = books::normalize(&format!("{} {}", prefix.unwrap_or(""), book_m.as_str()));
            typed == books::normalize(book.name) || book_m.as_str().len() >= 4
        };
        if !is_full_name && !body.contains(':') {
            continue;
        }
        let Some(passages) = parse_body(book, body) else { continue };
        let end = body_m.start() + body.len();
        found.push(Found { start: all.start(), end, reference: Reference { passages } });
    }
    found
}

/// Text split into plain runs and reference runs, ready for rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Text(String),
    Ref { text: String, reference: Reference },
}

pub fn segment(text: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut cursor = 0;
    for f in find_references(text) {
        if f.start > cursor {
            out.push(Segment::Text(text[cursor..f.start].to_string()));
        }
        out.push(Segment::Ref { text: text[f.start..f.end].to_string(), reference: f.reference });
        cursor = f.end;
    }
    if cursor < text.len() {
        out.push(Segment::Text(text[cursor..].to_string()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn label(s: &str) -> String {
        parse(s).map(|r| r.label()).unwrap_or_else(|| "NONE".into())
    }

    #[test]
    fn parses_common_forms() {
        assert_eq!(label("John 3:16"), "John 3:16");
        assert_eq!(label("jn 3:16-18"), "John 3:16–18");
        assert_eq!(label("Psalm 23"), "Psalms 23");
        assert_eq!(label("1 Cor 13:4-7"), "1 Corinthians 13:4–7");
        assert_eq!(label("II Tim. 3:16"), "2 Timothy 3:16");
        assert_eq!(label("Romans 12:1-2, 9; 13:8"), "Romans 12:1–2, 9; 13:8");
        assert_eq!(label("Matt 5-7"), "Matthew 5–7");
        assert_eq!(label("Gen 1:1-2:3"), "Genesis 1:1–2:3");
        assert_eq!(label("Philemon 1:6"), "Philemon 1:6");
        assert_eq!(label("Song of Solomon 2:4"), "Song of Solomon 2:4");
        assert_eq!(label("Isa 59:1-2"), "Isaiah 59:1–2");
        assert_eq!(label("2 Chron 20:14-21"), "2 Chronicles 20:14–21");
        assert_eq!(label("Php 1:3-6, 12-30"), "Philippians 1:3–6, 12–30");
    }

    #[test]
    fn rejects_out_of_range() {
        assert_eq!(label("Jude 2:1"), "NONE");
        assert_eq!(label("Genesis 51:1"), "NONE");
    }

    #[test]
    fn finds_in_prose_without_false_positives() {
        let text = "Read John 3:16-18 and Rom. 8:28; it is 5 days until Acts 2. See also Heb 11:1, 6.";
        let refs: Vec<String> = find_references(text).iter().map(|f| f.reference.label()).collect();
        assert_eq!(refs, vec!["John 3:16–18", "Romans 8:28", "Acts 2", "Hebrews 11:1, 6"]);
    }

    #[test]
    fn segments_keep_text_intact() {
        let text = "Memory verse: Phil 4:13.";
        let joined: String = segment(text)
            .into_iter()
            .map(|s| match s {
                Segment::Text(t) => t,
                Segment::Ref { text, .. } => text,
            })
            .collect();
        assert_eq!(joined, text);
    }

    #[test]
    fn contains_logic() {
        let r = parse("John 3:16-4:2").unwrap();
        let p = r.first();
        assert!(p.contains(3, 16) && p.contains(3, 36) && p.contains(4, 1));
        assert!(!p.contains(3, 15) && !p.contains(4, 3));
    }
}
