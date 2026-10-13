//! Screens. Each tab has its own module; layouts live in `layout`.

mod acknowledgements;
pub mod bible;
mod layout;
pub mod manual;
mod not_found;
pub mod onboarding;
mod settings;
mod software_update;
mod today;
mod whats_new;

pub use acknowledgements::Acknowledgements;
pub use layout::{Shell, TabLayout};
pub use not_found::NotFound;
pub use settings::SettingsView;
pub use software_update::{SoftwareUpdate, APP_ICON};
pub use today::Today;
pub use whats_new::WhatsNew;
