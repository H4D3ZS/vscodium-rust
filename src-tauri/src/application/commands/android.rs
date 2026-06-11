//! Thin Tauri adapters — Android bounded context (DDD delivery layer).

use crate::EditorState;
use serde_json::json;
use tauri::State;

async fn override_sdk(state: &State<'_, EditorState>) -> Option<String> {
    state.mobile.android_sdk_path.lock().await.clone()
}

#[tauri::command]
pub async fn set_android_sdk_path(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut sdk = state.mobile.android_sdk_path.lock().await;
    *sdk = Some(path);
    Ok(())
}

#[tauri::command]
pub async fn get_android_config(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    let override_path = override_sdk(&state).await;
    let cfg = state.mobile.android.sdk_config(override_path.as_deref());
    Ok(json!({
        "sdk_path": cfg.sdk_path,
        "adb_found": cfg.adb_found,
        "emulator_found": cfg.emulator_found,
    }))
}

#[tauri::command]
pub async fn adb_list_devices(state: State<'_, EditorState>) -> Result<Vec<serde_json::Value>, String> {
    let override_path = override_sdk(&state).await;
    let devices = state
        .mobile.android
        .list_devices(override_path.as_deref())?;
    Ok(devices
        .into_iter()
        .map(|d| json!({ "id": d.id, "state": d.state }))
        .collect())
}

#[tauri::command]
pub async fn adb_list_emulators(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let override_path = override_sdk(&state).await;
    Ok(state
        .mobile.android
        .list_avds(override_path.as_deref())?
        .into_iter()
        .map(|a| a.name)
        .collect())
}

#[tauri::command]
pub async fn spawn_emulator(state: State<'_, EditorState>, avd: String) -> Result<(), String> {
    let override_path = override_sdk(&state).await;
    state
        .mobile.android
        .spawn_emulator(override_path.as_deref(), &avd)
}

#[tauri::command]
pub async fn set_active_device(state: State<'_, EditorState>, device: String) -> Result<(), String> {
    let mut active = state.mobile.active_device.lock().await;
    *active = Some(device);
    Ok(())
}

#[tauri::command]
pub async fn adb_install_and_run(
    state: State<'_, EditorState>,
    apk_path: String,
    package: Option<String>,
    activity: Option<String>,
) -> Result<(), String> {
    let override_path = override_sdk(&state).await;
    let device = state.mobile.active_device.lock().await.clone();
    state.mobile.android.install_and_launch(
        override_path.as_deref(),
        device.as_deref(),
        &apk_path,
        package.as_deref(),
        activity.as_deref(),
    )
}
