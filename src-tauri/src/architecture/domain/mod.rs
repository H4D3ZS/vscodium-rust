//! Domain layer — business types and ports (repository traits).
//! No `tauri`, `sysinfo`, or `tokio` imports allowed in this tree.

#[cfg(feature = "tauri")]
pub mod android;
#[cfg(feature = "tauri")]
pub mod gradle;
#[cfg(feature = "tauri")]
pub mod logcat;
pub mod performance;
pub mod test;
