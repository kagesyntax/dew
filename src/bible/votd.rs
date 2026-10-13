//! Verse of the Day: a curated rotation, picked by day of year so everyone in the church sees
//! the same verse on the same day.

const VERSES: &[&str] = &[
    "John 3:16",
    "Psalm 23:1",
    "Philippians 4:13",
    "Jeremiah 29:11",
    "Romans 8:28",
    "Proverbs 3:5-6",
    "Isaiah 40:31",
    "Joshua 1:9",
    "Matthew 11:28",
    "Psalm 46:1",
    "2 Corinthians 5:17",
    "Hebrews 11:1",
    "Galatians 5:22-23",
    "Philippians 4:6-7",
    "Romans 12:2",
    "Psalm 119:105",
    "Matthew 6:33",
    "Isaiah 41:10",
    "1 Corinthians 13:4-5",
    "Lamentations 3:22-23",
    "Psalm 27:1",
    "John 14:6",
    "Ephesians 2:8-9",
    "2 Timothy 1:7",
    "James 1:5",
    "1 Peter 5:7",
    "Micah 6:8",
    "Psalm 37:4",
    "Colossians 3:23",
    "Romans 15:13",
    "Hebrews 12:1-2",
];

pub fn verse_of_the_day(day_of_year: u32) -> &'static str {
    VERSES[(day_of_year as usize) % VERSES.len()]
}
