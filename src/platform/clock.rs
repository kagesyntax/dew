//! Local date helpers (chrono's `wasmbind` feature reads the browser clock on the web).

use chrono::{Datelike, Local, NaiveDate, Timelike};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

pub fn today_iso() -> String {
    today().format("%Y-%m-%d").to_string()
}

/// "Good morning" / "Good afternoon" / "Good evening".
pub fn greeting() -> &'static str {
    match Local::now().hour() {
        5..=11 => "Good morning",
        12..=16 => "Good afternoon",
        _ => "Good evening",
    }
}

/// "TUESDAY, OCTOBER 13"
pub fn header_date() -> String {
    today().format("%A, %B %-d").to_string().to_uppercase()
}

/// 0 = Monday … 6 = Sunday
pub fn weekday_from_monday() -> u32 {
    today().weekday().num_days_from_monday()
}

pub fn day_of_year() -> u32 {
    today().ordinal()
}

/// "Sunday, October 18" from an ISO date.
pub fn pretty_date(iso: &str) -> String {
    NaiveDate::parse_from_str(iso, "%Y-%m-%d")
        .map(|d| d.format("%A, %B %-d").to_string())
        .unwrap_or_else(|_| iso.to_string())
}

/// "Oct 18"
pub fn short_date(iso: &str) -> String {
    NaiveDate::parse_from_str(iso, "%Y-%m-%d")
        .map(|d| d.format("%b %-d").to_string())
        .unwrap_or_else(|_| iso.to_string())
}
