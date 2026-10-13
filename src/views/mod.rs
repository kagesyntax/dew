//! Screens. Each tab has its own module; layouts live in `layout`.

pub mod bible;
mod layout;
pub mod manual;
mod not_found;
pub mod onboarding;
mod settings;
mod today;

pub use layout::{Shell, TabLayout};
pub use not_found::NotFound;
pub use settings::SettingsView;
pub use today::Today;
