//! Domain layer: pure business logic. No `tauri::` imports allowed here.
//! See ARCHITECTURE.md.

pub mod ai;
pub mod tools;
pub mod compat;
pub mod editor;
pub mod extensions;
pub mod indexing;
pub mod memory;
pub mod mobile;
pub mod safe_io;
pub mod security;
pub mod services;
pub mod skills;
pub mod types;
pub mod vcs;
pub mod workspace;

// `domain.rs` historically held shared types; keep `crate::domain::FileEntry` etc. working.
pub use types::*;
