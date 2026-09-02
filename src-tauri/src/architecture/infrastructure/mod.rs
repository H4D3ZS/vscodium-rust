//! Infrastructure — adapters for external systems (OS, HTTP, DB).

#[cfg(feature = "tauri")]
pub mod android;
#[cfg(feature = "tauri")]
pub mod gradle;
pub mod performance;
pub mod test;
