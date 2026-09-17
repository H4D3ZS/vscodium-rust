//! Mobile domain: iOS simulator/emulator lifecycle, Android SDK/scrcpy/logcat,
//! and the shared mobile toolchain detection.
//!
//! Modules that depend on `tauri` (AppHandle, Emitter, etc.) are gated behind
//! `#[cfg(feature = "tauri")]` so the domain compiles cleanly for the native
//! gpui shell (`--no-default-features`). Tauri-agnostic modules compile always.

// ── Tauri-independent mobile modules (compile in both shells) ────────────────
pub mod ios_crosscompile; // cross-compile helpers — no tauri
pub mod ios_package; // IPA packaging — no tauri
pub mod ioscpy_bridge; // wire protocol — pure std::net, no tauri
pub mod mobile_toolchain;
pub mod wda_client; // WDA HTTP client — pure reqwest, no tauri // tool detection — no tauri

// ── Tauri-dependent mobile modules (AppHandle, Emitter, broadcast channels) ──
#[cfg(feature = "tauri")]
pub mod android_sdk;
#[cfg(feature = "tauri")]
pub mod emulator_stream;
#[cfg(feature = "tauri")]
pub mod ios_run;
#[cfg(feature = "tauri")]
pub mod ios_sim_embed;
#[cfg(feature = "tauri")]
pub mod ios_sim_native;
#[cfg(feature = "tauri")]
pub mod ios_simulator;
#[cfg(feature = "tauri")]
pub mod ios_stream;
#[cfg(feature = "tauri")]
pub mod iphone_bounty;
#[cfg(feature = "tauri")]
pub mod iphone_control;
#[cfg(feature = "tauri")]
pub mod iphone_deploy;
#[cfg(feature = "tauri")]
pub mod iphone_device;
#[cfg(feature = "tauri")]
pub mod iphone_emulator;
#[cfg(feature = "tauri")]
pub mod logcat_service;
#[cfg(feature = "tauri")]
pub mod scrcpy;
