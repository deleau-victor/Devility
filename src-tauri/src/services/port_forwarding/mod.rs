#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(all(unix, not(target_os = "macos")))]
mod unix;

#[cfg(target_os = "windows")]
pub use windows::{add_rule, list_rules};

#[cfg(target_os = "macos")]
pub use macos::{add_rule, list_rules};

#[cfg(all(unix, not(target_os = "macos")))]
pub use unix::{add_rule, list_rules};


pub fn remove_rule(listen_address: String, listen_port: u16) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        windows::remove_rule(listen_address, listen_port)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // on ignore listen_address sur macOS/unix
        let _ = listen_address; // évite le warning unused
        macos::remove_rule(listen_port)
    }
}
