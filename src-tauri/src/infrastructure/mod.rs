//! Infrastructure layer: external-world adapters — processes, sockets, files,
//! browsers, MCP transport, system metrics. No business logic here.
//! See docs/overhaul/CONVENTIONS.md §1.

pub mod browser;
pub mod browser_actuation;
pub mod mcp_client;
pub mod mcp_registry;
pub mod mcp_resolver;
pub mod mcp_server;
pub mod performance;
pub mod platform;
pub mod process_ext;
pub mod vfs_bridge;
pub mod airi_bridge;
pub mod binary_analyzer;
pub mod claurst_bridge;
pub mod ghost_runtime;
pub mod hermes_gateway;
pub mod system_profile;
pub mod telemetry_pipeline;
pub mod web_chat_driver;
pub mod webchat_openai_shim;
pub mod webui_mcp_bridge;
