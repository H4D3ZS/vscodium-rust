//! Domain layer: pure business logic. No `tauri::` imports allowed here.
//! See docs/overhaul/CONVENTIONS.md §1.

pub mod editor;
pub mod extensions;
pub mod indexing;
pub mod memory;
pub mod mobile;
pub mod security;
pub mod types;
pub mod vcs;

// `domain.rs` historically held shared types; keep `crate::domain::FileEntry` etc. working.
pub use types::*;
