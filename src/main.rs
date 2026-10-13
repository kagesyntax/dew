//! Dew: the Bible and the teens' Sunday School manual for AGC Oshodi, in one app.
//!
//! Module map (see docs/ARCHITECTURE.md):
//! * `app`        root component, global styles, providers, router
//! * `routes`     every screen and its URL
//! * `config`     compile-time configuration (church name, Turso, update feed)
//! * `state`      app-wide reactive state (settings, progress, navigation return point)
//! * `platform`   persistence + theme + JS bridges, isolated behind small functions
//! * `bible`      translations and chapter loading
//! * `manual`     manual loading (bundled -> cache -> Turso)
//! * `services`   Turso HTTP client and the updater
//! * `ui`         reusable app-specific components (tab bar, lists, scripture links)
//! * `components` styled primitives copied in with `dx components add`
//! * `views`      screens

mod app;
mod bible;
mod components;
mod config;
mod manual;
mod platform;
mod routes;
mod services;
mod state;
mod ui;
mod views;

fn main() {
    dioxus::launch(app::App);
}
