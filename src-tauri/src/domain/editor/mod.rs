//! Editor domain: LSP client/router/bundling/catalog and debug adapter protocol.

#[cfg(feature = "tauri")]
pub mod debug_adapter;
pub mod lsp;
pub mod lsp_bundle;
pub mod lsp_catalog;
pub mod lsp_manager;
pub mod lsp_router;
#[cfg(feature = "tauri")]
pub mod lsp_store;
