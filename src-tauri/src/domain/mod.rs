//! Domain layer: pure business logic. No `tauri::` imports allowed here.
//! See ARCHITECTURE.md.

pub mod ai;
pub mod compat;
pub mod correlation;
pub mod editor;
pub mod extensions;
pub mod fingerprint;
pub mod indexing;
pub mod memory;
pub mod mobile;
pub mod mobhunt;
pub mod safe_io;
pub mod security;
pub mod sentinel;
pub mod services;
pub mod skills;
pub mod tools;
pub mod types;
pub mod vcs;
pub mod workspace;

// `domain.rs` historically held shared types; keep `crate::domain::FileEntry` etc. working.
pub use types::*;
