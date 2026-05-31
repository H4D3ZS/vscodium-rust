use tauri::State;
use crate::EditorState;
use crate::extension_host;
use crate::marketplace;
use serde_json::{json, Value};
use std::fs;

#[tauri::command]
pub async fn ext_host_init(state: State<'_, EditorState>, app: tauri::AppHandle) -> Result<(), String> {
    let mut eh = state.ext_host.lock().await;
    eh.scan_extensions().map_err(|e| e.to_string())?;
    eh.start(app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ext_host_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut eh = state.ext_host.lock().await;
    eh.send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_extensions(
    query: String,
) -> Result<Vec<marketplace::MarketplaceExtension>, String> {
    marketplace::search_extensions(query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_extension(
    state: State<'_, EditorState>,
    publisher: String,
    name: String,
    version: String,
) -> Result<extension_host::ExtensionMetadata, String> {
    let extensions_dir = {
        let eh = state.ext_host.lock().await;
        eh.primary_extensions_dir()
    };

    let _id = marketplace::install_extension(
        publisher.clone(),
        name.clone(),
        version.clone(),
        extensions_dir.clone(),
    )
    .await
    .map_err(|e| e.to_string())?;

    // Dynamically load the extension
    let target_dir = extensions_dir.join(format!("{}.{}-{}", publisher, name, version));
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }
    let mut package_json_path = target_dir.join("package.json");
    if !package_json_path.exists() {
        package_json_path = target_dir.join("extension").join("package.json");
    }

    if package_json_path.exists() {
        let content = fs::read_to_string(&package_json_path).map_err(|e| e.to_string())?;
        if let Ok(mut meta) = serde_json::from_str::<extension_host::ExtensionMetadata>(&content) {
            meta.extension_path = package_json_path.parent().unwrap().to_path_buf();
            if meta.id.is_empty() {
                meta.id = format!("{}.{}", publisher, name);
            }
            let mut eh = state.ext_host.lock().await;
            let _ = eh.add_extension(meta.clone());
            return Ok(meta);
        }
    }

    Err("Failed to load installed extension metadata".to_string())
}

#[tauri::command]
pub async fn get_popular_extensions() -> Result<Vec<marketplace::MarketplaceExtension>, String> {
    marketplace::get_popular_extensions()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_extension_details(id: String) -> Result<Value, String> {
    let parts: Vec<&str> = id.split('.').collect();
    if parts.len() < 2 {
        return Err("Invalid extension ID".to_string());
    }
    let publisher = parts[0].to_string();
    let name = parts[1..].join(".").to_string();
    marketplace::get_extension_details(publisher, name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn uninstall_extension(
    state: State<'_, EditorState>,
    publisher: String,
    name: String,
    version: Option<String>,
) -> Result<(), String> {
    let mut eh = state.ext_host.lock().await;
    let extensions_dir = eh.primary_extensions_dir();

    let target_prefix = format!("{}.{}", publisher, name);
    println!("Uninstalling extension: {}.{} (v:{:?})", publisher, name, version);

    if let Ok(entries) = std::fs::read_dir(&extensions_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap_or_default().to_string_lossy();
                let matches = if folder_name.starts_with(&target_prefix) {
                    if let Some(ref v) = version {
                        folder_name.contains(v)
                    } else {
                        true
                    }
                } else {
                    false
                };

                if matches {
                    println!("Found matching extension folder: {}", folder_name);
                    // Try to delete. If it fails (file lock), try to rename it to hide it.
                    if let Err(e) = std::fs::remove_dir_all(&path) {
                        println!("Failed to delete extension dir {}: {}. Attempting rename fallback...", folder_name, e);
                        let deprecated_path = path.with_extension(format!("deprecated-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
                        if let Err(re) = std::fs::rename(&path, &deprecated_path) {
                            println!("Rename fallback failed for {}: {}", folder_name, re);
                            return Err(format!("Failed to uninstall extension: Folder is locked and cannot be removed or renamed. Please close any background processes or restart the IDE. Error: {}", e));
                        } else {
                            println!("Successfully renamed {} to {} to hide it.", folder_name, deprecated_path.display());
                        }
                    } else {
                        println!("Successfully deleted extension folder: {}", folder_name);
                    }
                }
            }
        }
    }

    let _ = eh.scan_extensions();
    println!("Scan extensions complete. Total running/installed: {}", eh.extensions.len());
    Ok(())
}

#[tauri::command]
pub async fn get_installed_extensions(
    state: State<'_, EditorState>,
) -> Result<Vec<extension_host::ExtensionMetadata>, String> {
    let eh = state.ext_host.lock().await;
    Ok(eh.extensions.clone())
}

#[tauri::command]
pub fn install_vsix(_state: State<'_, EditorState>, path: String) -> Result<(), String> {
    // Basic stub for manual VSIX installation
    println!("Installing VSIX from {}", path);
    Ok(())
}

#[tauri::command]
pub async fn get_running_extensions(state: State<'_, EditorState>) -> Result<Vec<extension_host::ExtensionMetadata>, String> {
    let eh = state.ext_host.lock().await;
    Ok(eh.extensions.clone())
}
#[tauri::command]
pub async fn get_installed_themes(state: State<'_, EditorState>) -> Result<Vec<Value>, String> {
    let mut themes = Vec::new();
    let eh = state.ext_host.lock().await;

    for ext in &eh.extensions {
        if let Some(contributes) = &ext.contributes {
            if let Some(themes_list) = contributes.get("themes").and_then(|t| t.as_array()) {
                for theme in themes_list {
                    let mut theme_obj = theme.clone();
                    if let Some(obj) = theme_obj.as_object_mut() {
                        let label = obj.get("label").and_then(|l| l.as_str()).unwrap_or("Unknown Theme");
                        obj.insert("id".to_string(), json!(format!("{}.{}", ext.id, label)));
                        obj.insert("extensionId".to_string(), json!(ext.id));
                        if let Some(path) = obj.get("path").and_then(|p| p.as_str()) {
                             let abs_path = ext.extension_path.join(path);
                             obj.insert("absolutePath".to_string(), json!(abs_path.to_string_lossy()));
                        }
                    }
                    themes.push(theme_obj);
                }
            }
        }
    }
    Ok(themes)
}

#[tauri::command]
pub async fn get_icon_theme_mapping(state: State<'_, EditorState>) -> Result<Value, String> {
    let mut icon_themes = Vec::new();
    let eh = state.ext_host.lock().await;

    for ext in &eh.extensions {
        if let Some(contributes) = &ext.contributes {
            if let Some(icons_list) = contributes.get("iconThemes").and_then(|t| t.as_array()) {
                for icon_theme in icons_list {
                    let mut theme_obj = icon_theme.clone();
                    if let Some(obj) = theme_obj.as_object_mut() {
                        let label = obj.get("label").and_then(|l| l.as_str()).unwrap_or("Unknown Icon Theme");
                        obj.insert("id".to_string(), json!(format!("{}.{}", ext.id, label)));
                        obj.insert("extensionId".to_string(), json!(ext.id));
                        if let Some(path) = obj.get("path").and_then(|p| p.as_str()) {
                             let abs_path = ext.extension_path.join(path);
                             obj.insert("absolutePath".to_string(), json!(abs_path.to_string_lossy()));
                        }
                    }
                    icon_themes.push(theme_obj);
                }
            }
        }
    }
    Ok(json!(icon_themes))
}

#[tauri::command]
pub async fn get_extension_contributions(state: State<'_, EditorState>) -> Result<Value, String> {
    let mut all_contributes = serde_json::Map::new();
    let eh = state.ext_host.lock().await;

    for ext in &eh.extensions {
        if let Some(contributes) = &ext.contributes {
            if let Some(obj) = contributes.as_object() {
                for (key, val) in obj {
                    let entry = all_contributes.entry(key.clone()).or_insert(json!([]));
                    if let Some(arr) = entry.as_array_mut() {
                        arr.push(json!({
                            "extensionId": ext.id,
                            "value": val
                        }));
                    }
                }
            }
        }
    }
    Ok(Value::Object(all_contributes))
}

#[tauri::command]
pub async fn load_extension_theme(_state: State<'_, EditorState>, theme_path: String) -> Result<Value, String> {
    let path = std::path::PathBuf::from(theme_path);
    if !path.exists() {
        return Err(format!("Theme file not found: {}", path.display()));
    }
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let theme: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(theme)
}

#[tauri::command]
pub async fn refresh_popular_extensions() -> Result<Vec<marketplace::MarketplaceExtension>, String> {
    get_popular_extensions().await
}

#[tauri::command]
pub async fn refresh_installed_extensions(
    state: State<'_, EditorState>,
) -> Result<Vec<extension_host::ExtensionMetadata>, String> {
    get_installed_extensions(state).await
}

#[tauri::command]
pub async fn check_activation_event(
    state: State<'_, EditorState>,
    event: String,
) -> Result<(), String> {
    let mut am = state.activation_manager.lock().await;
    am.check_activation_requests(&event, state.ext_host.clone()).await;
    Ok(())
}

