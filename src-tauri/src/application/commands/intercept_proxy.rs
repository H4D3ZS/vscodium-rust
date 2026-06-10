//! Tauri IPC for the native intercepting proxy.

use crate::intercept_proxy::{self, Flow, ProxyStatus};

#[tauri::command]
pub async fn proxy_start(port: Option<u16>) -> Result<u16, String> {
    intercept_proxy::start_proxy(port.unwrap_or(8888)).await
}

#[tauri::command]
pub fn proxy_stop() -> ProxyStatus {
    intercept_proxy::stop_proxy();
    intercept_proxy::status()
}

#[tauri::command]
pub fn proxy_status() -> ProxyStatus {
    intercept_proxy::status()
}

#[tauri::command]
pub fn proxy_flows(limit: Option<usize>) -> Vec<Flow> {
    intercept_proxy::list_flows(limit.unwrap_or(200))
}

#[tauri::command]
pub fn proxy_clear() {
    intercept_proxy::clear_flows();
}

#[tauri::command]
pub async fn proxy_replay(id: u64) -> Result<serde_json::Value, String> {
    intercept_proxy::replay_flow(id).await
}
