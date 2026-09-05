//! # Clean Architecture + DDD layout (Rust backend)
//!
//! Layers (inner → outer, dependencies point inward only):
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │  Tauri commands (presentation / delivery)               │
//! │  e.g. performance_commands::get_process_stats           │
//! ├─────────────────────────────────────────────────────────┤
//! │  Application services                                   │
//! │  Orchestrate use-cases; no OS/sysinfo details here      │
//! ├─────────────────────────────────────────────────────────┤
//! │  Domain                                                 │
//! │  Entities + repository traits (ports) — pure Rust types │
//! ├─────────────────────────────────────────────────────────┤
//! │  Infrastructure                                         │
//! │  sysinfo, Win32, SQLite — implements domain ports       │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! **Why DDD here?** The IDE has many subsystems (AI, editor, emulators, MCP).
//! Without bounded contexts, `state.rs` becomes a god-object and memory stats
//! lie (host-only RSS vs full WebView2 tree). Each folder under `architecture/`
//! is one bounded context you can open without an agent.

pub mod domain;
pub mod infrastructure;

#[cfg(feature = "tauri")]
pub mod application {
    pub mod android_service;
    pub mod gradle_service;
}
