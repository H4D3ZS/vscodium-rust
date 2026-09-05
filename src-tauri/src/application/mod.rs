//! Application layer: Tauri command wrappers. Commands stay thin — parse args,
//! call domain logic, map errors to String. See ARCHITECTURE.md.

#[cfg(feature = "tauri")]
pub mod asymmetric_orchestrator;
#[cfg(feature = "tauri")]
pub mod autonomous_supervisor;
#[cfg(feature = "tauri")]
pub mod commands;
#[cfg(feature = "tauri")]
pub mod jobs;
