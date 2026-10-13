-- Turso / libSQL schema for publishing Sunday School lessons to the app.
-- Apply with:  turso db shell <db-name> < db/schema.sql
-- The app only needs a READ-ONLY token:  turso db tokens create <db-name> --read-only

CREATE TABLE IF NOT EXISTS lessons (
    id           TEXT PRIMARY KEY,          -- e.g. '2026-10-18'
    unit_no      INTEGER NOT NULL,
    study_no     INTEGER NOT NULL,
    lesson_date  TEXT NOT NULL,             -- ISO date of the Sunday it is taught
    title        TEXT NOT NULL,
    payload      TEXT NOT NULL,             -- full Lesson JSON (see crates/core/src/manual.rs)
    published    INTEGER NOT NULL DEFAULT 1,
    updated_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_lessons_date ON lessons (lesson_date);

-- Optional: church announcements shown on the Today tab in a future release.
CREATE TABLE IF NOT EXISTS announcements (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      TEXT NOT NULL,
    body       TEXT NOT NULL,
    starts_on  TEXT,
    ends_on    TEXT
);
