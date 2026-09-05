//! Infrastructure layer: external-world adapters — processes, sockets, files,
//! browsers, MCP transport, system metrics. No business logic here.
//! See ARCHITECTURE.md.

pub mod browser;
#[cfg(feature = "tauri")]
pub mod browser_actuation;
pub mod mcp_client;
pub mod mcp_registry;
pub mod mcp_resolver;
pub mod mcp_server;
pub mod performance;
pub mod platform;
pub mod process_ext;
pub mod process_registry;
pub mod vfs_bridge;
pub mod airi_bridge;
pub mod binary_analyzer;
#[cfg(feature = "tauri")]
pub mod claurst_bridge;
pub mod ghost_runtime;
#[cfg(feature = "tauri")]
pub mod hermes_gateway;
pub mod system_profile;
#[cfg(feature = "tauri")]
pub mod telemetry_pipeline;
pub mod web_chat_driver;
pub mod webchat_openai_shim;
pub mod webui_mcp_bridge;
#[cfg(feature = "tauri")]
pub mod fcc_sidecar;
