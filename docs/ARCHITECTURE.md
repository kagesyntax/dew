# Architecture

```
dew/
├── crates/core/            # pure Rust, no UI: tested in CI on any host
│   └── src/
│       ├── scripture/      # books.rs (66 books + aliases), parser.rs (find refs in text), reference.rs
│       ├── manual.rs       # Lesson / Manual model (Radiant Life layout)
│       └── version.rs      # update manifest + semver compare
├── src/
│   ├── main.rs  app.rs  routes.rs  config.rs
│   ├── state/              # AppState: settings, progress (highlights, bookmarks), navigation return point
│   ├── platform/           # storage (localStorage), theme, clock, js bridges
│   ├── bible/              # translations, async per-book loader with cache, verse of the day
│   ├── manual/             # bundled → cached → Turso lesson store
│   ├── services/           # turso.rs (Hrana HTTP), updater.rs
│   ├── components/         # dx components (sheet, switch, toggle_group), restyled iOS-like
│   ├── ui/                 # tab bar, large titles, grouped lists, ScriptureText + preview sheet
│   └── views/              # onboarding/, today, bible/, manual/, settings
├── assets/                 # tailwind output, app.css tokens, bible/<tr>/<BOOK>.json, manual/manual.json
├── public/                 # manifest, service worker, icons, version.json
├── db/                     # Turso schema + seed script
└── .github/workflows/      # ci.yml, release.yml
```

## Key flows
**Scripture links**: `church_core::scripture::segment(text)` splits text into plain runs and
`Reference`s. `ui::ScriptureText` renders references as buttons; the action depends on
`Settings.link_mode` (Preview sheet / Inline / Open Bible).

**Return to lesson**: before jumping to the reader, `state::navigation::jump_with_return`
stores `{label, route, scroll_y}`. The reader shows `ReturnPill`; tapping it pushes the lesson
route and sets `pending_scroll`, which `LessonPage` restores after render.

**Data**: Bible books are lazy-loaded with `dioxus::asset_resolver::read_asset_bytes`, which
works on web (HTTP) and native (bundle). Lessons come from Turso when configured, cached
in localStorage, falling back to the bundled JSON.
