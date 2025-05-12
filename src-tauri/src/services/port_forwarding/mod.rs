#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(all(unix, not(target_os = "macos")))]
mod unix;

#[cfg(target_os = "windows")]
pub use windows::{add_rule, list_rules, remove_rule};

#[cfg(target_os = "macos")]
pub use macos::{add_rule, list_rules, remove_rule};

#[cfg(all(unix, not(target_os = "macos")))]
pub use unix::{add_rule, list_rules, remove_rule};
