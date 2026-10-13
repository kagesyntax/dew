//! Canonical list of the 66 books with the aliases the reference parser understands.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Testament {
    Old,
    New,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Book {
    /// USFM-style id, also used as the asset file name (`assets/bible/<tr>/<id>.json`).
    pub id: &'static str,
    pub name: &'static str,
    pub testament: Testament,
    pub chapters: u16,
    /// Lower-case abbreviations without spaces or dots (`"1cor"`, `"jn"`).
    pub aliases: &'static [&'static str],
}

pub static BOOKS: &[Book] = &[
    Book { id: "GEN", name: "Genesis", testament: Testament::Old, chapters: 50, aliases: &["gen", "ge", "gn"] },
    Book { id: "EXO", name: "Exodus", testament: Testament::Old, chapters: 40, aliases: &["exod", "exo", "ex"] },
    Book { id: "LEV", name: "Leviticus", testament: Testament::Old, chapters: 27, aliases: &["lev", "le", "lv"] },
    Book { id: "NUM", name: "Numbers", testament: Testament::Old, chapters: 36, aliases: &["num", "nu", "nm", "nb"] },
    Book { id: "DEU", name: "Deuteronomy", testament: Testament::Old, chapters: 34, aliases: &["deut", "deu", "dt", "de"] },
    Book { id: "JOS", name: "Joshua", testament: Testament::Old, chapters: 24, aliases: &["josh", "jos", "jsh"] },
    Book { id: "JDG", name: "Judges", testament: Testament::Old, chapters: 21, aliases: &["judg", "jdg", "jg", "jdgs"] },
    Book { id: "RUT", name: "Ruth", testament: Testament::Old, chapters: 4, aliases: &["ruth", "rth", "ru"] },
    Book { id: "1SA", name: "1 Samuel", testament: Testament::Old, chapters: 31, aliases: &["1sam", "1sa", "1sm", "1s"] },
    Book { id: "2SA", name: "2 Samuel", testament: Testament::Old, chapters: 24, aliases: &["2sam", "2sa", "2sm", "2s"] },
    Book { id: "1KI", name: "1 Kings", testament: Testament::Old, chapters: 22, aliases: &["1kgs", "1ki", "1kg", "1k", "1kings"] },
    Book { id: "2KI", name: "2 Kings", testament: Testament::Old, chapters: 25, aliases: &["2kgs", "2ki", "2kg", "2k"] },
    Book { id: "1CH", name: "1 Chronicles", testament: Testament::Old, chapters: 29, aliases: &["1chr", "1ch", "1chron"] },
    Book { id: "2CH", name: "2 Chronicles", testament: Testament::Old, chapters: 36, aliases: &["2chr", "2ch", "2chron"] },
    Book { id: "EZR", name: "Ezra", testament: Testament::Old, chapters: 10, aliases: &["ezra", "ezr"] },
    Book { id: "NEH", name: "Nehemiah", testament: Testament::Old, chapters: 13, aliases: &["neh", "ne"] },
    Book { id: "EST", name: "Esther", testament: Testament::Old, chapters: 10, aliases: &["esth", "est", "es"] },
    Book { id: "JOB", name: "Job", testament: Testament::Old, chapters: 42, aliases: &["job", "jb"] },
    Book { id: "PSA", name: "Psalms", testament: Testament::Old, chapters: 150, aliases: &["ps", "psa", "pss", "psalm", "pslm", "psm"] },
    Book { id: "PRO", name: "Proverbs", testament: Testament::Old, chapters: 31, aliases: &["prov", "pro", "prv", "pr"] },
    Book { id: "ECC", name: "Ecclesiastes", testament: Testament::Old, chapters: 12, aliases: &["eccl", "ecc", "ec", "eccles", "qoh"] },
    Book { id: "SNG", name: "Song of Solomon", testament: Testament::Old, chapters: 8, aliases: &["song", "sos", "so", "sng", "songofsongs", "canticles", "cant"] },
    Book { id: "ISA", name: "Isaiah", testament: Testament::Old, chapters: 66, aliases: &["isa", "is"] },
    Book { id: "JER", name: "Jeremiah", testament: Testament::Old, chapters: 52, aliases: &["jer", "je", "jr"] },
    Book { id: "LAM", name: "Lamentations", testament: Testament::Old, chapters: 5, aliases: &["lam", "la"] },
    Book { id: "EZK", name: "Ezekiel", testament: Testament::Old, chapters: 48, aliases: &["ezek", "eze", "ezk"] },
    Book { id: "DAN", name: "Daniel", testament: Testament::Old, chapters: 12, aliases: &["dan", "da", "dn"] },
    Book { id: "HOS", name: "Hosea", testament: Testament::Old, chapters: 14, aliases: &["hos", "ho"] },
    Book { id: "JOL", name: "Joel", testament: Testament::Old, chapters: 3, aliases: &["joel", "jl", "joe"] },
    Book { id: "AMO", name: "Amos", testament: Testament::Old, chapters: 9, aliases: &["amos", "am"] },
    Book { id: "OBA", name: "Obadiah", testament: Testament::Old, chapters: 1, aliases: &["obad", "ob", "oba"] },
    Book { id: "JON", name: "Jonah", testament: Testament::Old, chapters: 4, aliases: &["jonah", "jon", "jnh"] },
    Book { id: "MIC", name: "Micah", testament: Testament::Old, chapters: 7, aliases: &["mic", "mc"] },
    Book { id: "NAM", name: "Nahum", testament: Testament::Old, chapters: 3, aliases: &["nah", "na"] },
    Book { id: "HAB", name: "Habakkuk", testament: Testament::Old, chapters: 3, aliases: &["hab", "hb"] },
    Book { id: "ZEP", name: "Zephaniah", testament: Testament::Old, chapters: 3, aliases: &["zeph", "zep", "zp"] },
    Book { id: "HAG", name: "Haggai", testament: Testament::Old, chapters: 2, aliases: &["hag", "hg"] },
    Book { id: "ZEC", name: "Zechariah", testament: Testament::Old, chapters: 14, aliases: &["zech", "zec", "zc"] },
    Book { id: "MAL", name: "Malachi", testament: Testament::Old, chapters: 4, aliases: &["mal", "ml"] },
    Book { id: "MAT", name: "Matthew", testament: Testament::New, chapters: 28, aliases: &["matt", "mat", "mt"] },
    Book { id: "MRK", name: "Mark", testament: Testament::New, chapters: 16, aliases: &["mark", "mrk", "mk", "mr"] },
    Book { id: "LUK", name: "Luke", testament: Testament::New, chapters: 24, aliases: &["luke", "luk", "lk"] },
    Book { id: "JHN", name: "John", testament: Testament::New, chapters: 21, aliases: &["john", "jhn", "jn", "joh"] },
    Book { id: "ACT", name: "Acts", testament: Testament::New, chapters: 28, aliases: &["acts", "act", "ac"] },
    Book { id: "ROM", name: "Romans", testament: Testament::New, chapters: 16, aliases: &["rom", "ro", "rm"] },
    Book { id: "1CO", name: "1 Corinthians", testament: Testament::New, chapters: 16, aliases: &["1cor", "1co"] },
    Book { id: "2CO", name: "2 Corinthians", testament: Testament::New, chapters: 13, aliases: &["2cor", "2co"] },
    Book { id: "GAL", name: "Galatians", testament: Testament::New, chapters: 6, aliases: &["gal", "ga"] },
    Book { id: "EPH", name: "Ephesians", testament: Testament::New, chapters: 6, aliases: &["eph", "ephes"] },
    Book { id: "PHP", name: "Philippians", testament: Testament::New, chapters: 4, aliases: &["phil", "php", "pp"] },
    Book { id: "COL", name: "Colossians", testament: Testament::New, chapters: 4, aliases: &["col"] },
    Book { id: "1TH", name: "1 Thessalonians", testament: Testament::New, chapters: 5, aliases: &["1thess", "1th", "1thes"] },
    Book { id: "2TH", name: "2 Thessalonians", testament: Testament::New, chapters: 3, aliases: &["2thess", "2th", "2thes"] },
    Book { id: "1TI", name: "1 Timothy", testament: Testament::New, chapters: 6, aliases: &["1tim", "1ti", "1tm"] },
    Book { id: "2TI", name: "2 Timothy", testament: Testament::New, chapters: 4, aliases: &["2tim", "2ti", "2tm"] },
    Book { id: "TIT", name: "Titus", testament: Testament::New, chapters: 3, aliases: &["titus", "tit"] },
    Book { id: "PHM", name: "Philemon", testament: Testament::New, chapters: 1, aliases: &["phlm", "phm", "philem", "pm"] },
    Book { id: "HEB", name: "Hebrews", testament: Testament::New, chapters: 13, aliases: &["heb"] },
    Book { id: "JAS", name: "James", testament: Testament::New, chapters: 5, aliases: &["jas", "jm", "jam", "james"] },
    Book { id: "1PE", name: "1 Peter", testament: Testament::New, chapters: 5, aliases: &["1pet", "1pe", "1pt", "1p"] },
    Book { id: "2PE", name: "2 Peter", testament: Testament::New, chapters: 3, aliases: &["2pet", "2pe", "2pt", "2p"] },
    Book { id: "1JN", name: "1 John", testament: Testament::New, chapters: 5, aliases: &["1john", "1jn", "1jo", "1joh"] },
    Book { id: "2JN", name: "2 John", testament: Testament::New, chapters: 1, aliases: &["2john", "2jn", "2jo", "2joh"] },
    Book { id: "3JN", name: "3 John", testament: Testament::New, chapters: 1, aliases: &["3john", "3jn", "3jo", "3joh"] },
    Book { id: "JUD", name: "Jude", testament: Testament::New, chapters: 1, aliases: &["jude", "jud", "jd"] },
    Book { id: "REV", name: "Revelation", testament: Testament::New, chapters: 22, aliases: &["rev", "re", "rv", "revelation", "revelations", "apocalypse"] },
];

/// Look a book up by its id (`"JHN"`).
pub fn by_id(id: &str) -> Option<&'static Book> {
    BOOKS.iter().find(|b| b.id.eq_ignore_ascii_case(id))
}

/// Position of the book in canon order (0-based).
pub fn index_of(book: &Book) -> usize {
    BOOKS.iter().position(|b| b.id == book.id).unwrap_or(0)
}

/// Normalise a written book name into a lookup key: lower-case, no spaces/dots,
/// leading roman numerals or ordinal words turned into digits ("II Cor." -> "2cor").
pub fn normalize(raw: &str) -> String {
    let lower = raw.trim().to_lowercase().replace('.', "");
    let mut parts = lower.split_whitespace().peekable();
    let mut out = String::new();
    if let Some(first) = parts.peek().copied() {
        let numeral = match first {
            "i" | "1st" | "first" => Some("1"),
            "ii" | "2nd" | "second" => Some("2"),
            "iii" | "3rd" | "third" => Some("3"),
            _ => None,
        };
        // Only treat "i" as a numeral when something follows ("I John").
        if let Some(n) = numeral {
            let mut probe = parts.clone();
            probe.next();
            if probe.peek().is_some() {
                out.push_str(n);
                parts.next();
            }
        }
    }
    for p in parts {
        out.push_str(p);
    }
    out
}

/// Resolve a written book name or abbreviation.
pub fn lookup(raw: &str) -> Option<&'static Book> {
    let key = normalize(raw);
    if key.is_empty() {
        return None;
    }
    BOOKS.iter().find(|b| {
        normalize(b.name) == key || b.aliases.iter().any(|a| *a == key) || (key == "psalm" && b.id == "PSA")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_66_books() {
        assert_eq!(BOOKS.len(), 66);
        assert_eq!(BOOKS.iter().filter(|b| b.testament == Testament::New).count(), 27);
    }

    #[test]
    fn looks_up_variants() {
        assert_eq!(lookup("John").unwrap().id, "JHN");
        assert_eq!(lookup("1 Cor.").unwrap().id, "1CO");
        assert_eq!(lookup("II Kings").unwrap().id, "2KI");
        assert_eq!(lookup("First John").unwrap().id, "1JN");
        assert_eq!(lookup("Song of Solomon").unwrap().id, "SNG");
        assert_eq!(lookup("Ps").unwrap().id, "PSA");
        assert_eq!(lookup("Psalm").unwrap().id, "PSA");
        assert!(lookup("Hezekiah").is_none());
    }

    #[test]
    fn aliases_are_unique() {
        let mut seen = std::collections::HashMap::new();
        for b in BOOKS {
            for a in b.aliases.iter().copied().chain(std::iter::once(normalize(b.name).leak() as &str)) {
                if let Some(prev) = seen.insert(a.to_string(), b.id) {
                    assert_eq!(prev, b.id, "alias {a} used by {prev} and {}", b.id);
                }
            }
        }
    }
}
