//! Workspace domain: specs/work-items DB, background workers, IDE shell
//! detection, visual lab graphs, attachments, test running, stop hooks.

#[cfg(feature = "tauri")]
pub mod attachment_manager;
pub mod ide_shell;
pub mod kairos;
pub mod specs_db;
#[cfg(feature = "tauri")]
pub mod stop_hooks;
pub mod test_runner_service;
pub mod visual_lab;
pub mod workers;
