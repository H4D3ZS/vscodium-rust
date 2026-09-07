//! Tauri commands for HADES optional module installer.

use crate::module_registry;
use serde_json::Value;

#[tauri::command]
pub async fn modules_fetch_catalog(url: Option<String>) -> Result<Value, String> {
    let catalog = module_registry::fetch_catalog(url).await?;
    Ok(module_registry::merge_catalog_with_installed(&catalog))
}

#[tauri::command]
pub fn modules_list_installed() -> Result<Vec<module_registry::InstalledModule>, String> {
    module_registry::list_installed()
}

#[tauri::command]
pub async fn modules_install(id: String, catalog_url: Option<String>) -> Result<module_registry::InstalledModule, String> {
    module_registry::install_module(id, catalog_url).await
}

#[tauri::command]
pub fn modules_uninstall(id: String) -> Result<(), String> {
    module_registry::uninstall_module(id)
}

#[tauri::command]
pub fn modules_set_enabled(id: String, enabled: bool) -> Result<module_registry::InstalledModule, String> {
    module_registry::set_module_enabled(id, enabled)
}

#[tauri::command]
pub fn modules_launch(id: String) -> Result<Value, String> {
    module_registry::launch_module(id)
}

#[tauri::command]
pub fn modules_default_catalog_url() -> String {
    module_registry::DEFAULT_CATALOG_URL.to_string()
}
