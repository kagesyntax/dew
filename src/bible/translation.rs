//! Bundled translations. Only public-domain texts are shipped inside the app; copyrighted ones
//! (LSB, NIV, ESV...) are offered as links to their publishers because their licences forbid
//! storing whole books in an app (LSB: max 1,000 verses).

#[derive(Debug, PartialEq)]
pub struct Translation {
    pub id: &'static str,
    pub abbreviation: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub copyright: &'static str,
}

pub static TRANSLATIONS: &[Translation] = &[
    Translation {
        id: "kjv",
        abbreviation: "KJV",
        name: "King James Version",
        description: "The classic 1611 English Bible, loved for its majestic language.",
        copyright: "Public domain.",
    },
    Translation {
        id: "web",
        abbreviation: "WEB",
        name: "World English Bible",
        description: "A modern update of the ASV in everyday English.",
        copyright: "Public domain.",
    },
    Translation {
        id: "bsb",
        abbreviation: "BSB",
        name: "Berean Standard Bible",
        description: "A modern, readable translation close to the original languages.",
        copyright: "Public domain (dedicated by the Berean Bible translation committee, 2023).",
    },
    Translation {
        id: "asv",
        abbreviation: "ASV",
        name: "American Standard Version",
        description: "The 1901 literal revision of the KJV, ancestor of the NASB and LSB.",
        copyright: "Public domain.",
    },
];

/// Copyrighted translation the church may prefer, opened on the publisher's free reader.
pub const LSB_READER_URL: &str = "https://read.lsbible.org/";

pub fn translation(id: &str) -> &'static Translation {
    TRANSLATIONS.iter().find(|t| t.id == id).unwrap_or(&TRANSLATIONS[0])
}
