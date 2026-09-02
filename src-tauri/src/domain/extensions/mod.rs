//! Extensions domain: VSCode-compatible extension host, marketplace/VSIX,
//! activation events, keybindings, and context keys.

#[cfg(feature = "tauri")]
pub mod activation;
pub mod context_key;
#[cfg(feature = "tauri")]
pub mod extension_host;
pub mod keybindings;
pub mod marketplace;
pub mod hermes_skills;
pub mod module_registry;
pub mod skill_audit;
pub mod skill_store;
