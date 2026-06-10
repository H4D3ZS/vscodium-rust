//! Application layer: Tauri command wrappers. Commands stay thin — parse args,
//! call domain logic, map errors to String. See docs/overhaul/CONVENTIONS.md §2.

pub mod commands;
pub mod jobs;
