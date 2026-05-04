use crate::EditorState;
use tauri::State;
use tokio::process::Command;
use std::path::PathBuf;

#[tauri::command]
pub async fn set_android_sdk_path(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut sdk = state.android_sdk_path.lock().await;
    *sdk = Some(path);
    Ok(())
}

#[tauri::command]
pub async fn adb_list_emulators(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let sdk_path = state.android_sdk_path.lock().await;
    let emulator_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = PathBuf::from(path);
        if p.join("emulator/emulator").exists() {
            p.join("emulator/emulator").to_string_lossy().to_string()
        } else if p.join("emulator/emulator.exe").exists() {
            p.join("emulator/emulator.exe").to_string_lossy().to_string()
        } else {
            "emulator".to_string()
        }
    } else {
        "emulator".to_string()
    };

    let output = Command::new(emulator_cmd)
        .arg("-list-avds")
        .output()
        .await
        .map_err(|e| format!("Emulator error: {}", e))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

#[tauri::command]
pub async fn spawn_emulator(state: State<'_, EditorState>, avd: String) -> Result<(), String> {
    let sdk_path = state.android_sdk_path.lock().await;
    let emulator_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = PathBuf::from(path);
        if p.join("emulator/emulator").exists() {
            p.join("emulator/emulator").to_string_lossy().to_string()
        } else if p.join("emulator/emulator.exe").exists() {
            p.join("emulator/emulator.exe").to_string_lossy().to_string()
        } else {
            "emulator".to_string()
        }
    } else {
        "emulator".to_string()
    };

    let _child = Command::new(emulator_cmd)
        .arg("-avd")
        .arg(avd)
        .spawn()
        .map_err(|e| format!("Failed to spawn emulator: {}", e))?;
        
    Ok(())
}
#[tauri::command]
pub async fn adb_list_devices(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let sdk_path = state.android_sdk_path.lock().await;
    let adb_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = std::path::PathBuf::from(path);
        if p.join("adb").exists() {
            p.join("adb").to_string_lossy().to_string()
        } else if p.join("platform-tools").join("adb").exists() {
            p.join("platform-tools")
                .join("adb")
                .to_string_lossy()
                .to_string()
        } else {
            "adb".to_string()
        }
    } else {
        "adb".to_string()
    };

    let output = Command::new(&adb_cmd)
        .arg("devices")
        .output()
        .await
        .map_err(|e| format!("ADB error ({}): {}", adb_cmd, e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    for line in stdout.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "device" {
            devices.push(parts[0].to_string());
        }
    }
    Ok(devices)
}

#[tauri::command]
pub async fn set_active_device(state: State<'_, EditorState>, device: String) -> Result<(), String> {
    let mut active = state.active_device.lock().await;
    *active = Some(device);
    Ok(())
}

#[tauri::command]
pub fn adb_install_and_run(_state: State<'_, EditorState>, _apk_path: String) -> Result<(), String> {
    // Stub for APK installation and launching
    Ok(())
}

#[tauri::command]
pub async fn get_android_config(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    let sdk_path = state.android_sdk_path.lock().await;
    let adb_found = if let Some(path) = sdk_path.as_ref() {
        std::path::PathBuf::from(path)
            .join("platform-tools/adb")
            .exists()
    } else {
        false
    };
 
    Ok(serde_json::json!({
        "sdk_path": *sdk_path,
        "adb_found": adb_found
    }))
}
