//! vPhone cross-platform mobile toolchain (phony Xcode, Flutter/RN doctor, AltStore deploy hints).

use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};

fn resolve_vphone_root() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("VPHONE_ROOT") {
        let path = PathBuf::from(p);
        if path.exists() {
            return Some(path);
        }
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("Virtual-iPhone-Emulator"));
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("Virtual-iPhone-Emulator"));
        }
    }
    if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
        candidates.push(PathBuf::from(&home).join("Desktop").join("Virtual-iPhone-Emulator"));
        candidates.push(
            PathBuf::from(&home)
                .join("Desktop")
                .join("vscodium-rust")
                .join("Virtual-iPhone-Emulator"),
        );
    }
    candidates.push(PathBuf::from(
        r"C:\Users\HADES\Desktop\Virtual-iPhone-Emulator",
    ));
    candidates.push(PathBuf::from(
        r"C:\Users\HADES\Desktop\vscodium-rust\Virtual-iPhone-Emulator",
    ));
    for c in candidates {
        if c.is_dir() {
            return Some(c);
        }
    }
    None
}

fn toolchain_dir(root: &Path) -> PathBuf {
    root.join("toolchain")
}

fn run_script(script: &Path) -> Result<(String, String, i32), String> {
    if !script.exists() {
        return Err(format!("Script not found: {}", script.display()));
    }
    #[cfg(windows)]
    let output = Command::new("cmd")
        .args(["/C", script.to_str().unwrap_or_default()])
        .output()
        .map_err(|e| format!("Failed to run {}: {e}", script.display()))?;
    #[cfg(not(windows))]
    let output = Command::new("bash")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run {}: {e}", script.display()))?;

    Ok((
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    ))
}

#[tauri::command]
pub fn resolve_mobile_toolchain_paths() -> Result<Value, String> {
    let root = resolve_vphone_root();
    let toolchain = root.as_ref().map(|r| toolchain_dir(r));
    Ok(json!({
        "vphone_root": root.as_ref().map(|p| p.to_string_lossy().to_string()),
        "toolchain_dir": toolchain.as_ref().map(|p| p.to_string_lossy().to_string()),
        "doctor_script": toolchain.as_ref().map(|t| {
            #[cfg(windows)]
            { t.join("vphone-doctor.bat").to_string_lossy().to_string() }
            #[cfg(not(windows))]
            { t.join("vphone-doctor.sh").to_string_lossy().to_string() }
        }),
        "install_script": toolchain.as_ref().map(|t| {
            #[cfg(windows)]
            { t.join("install_doctor.bat").to_string_lossy().to_string() }
            #[cfg(not(windows))]
            { t.join("install_doctor.sh").to_string_lossy().to_string() }
        }),
        "flutter_on_path": which::which("flutter").is_ok(),
        "altstore_hint": "Install AltServer separately (AGPL). Use Deploy IPA once .ipa is built.",
    }))
}

#[tauri::command]
pub fn run_vphone_doctor() -> Result<Value, String> {
    let root = resolve_vphone_root()
        .ok_or_else(|| "Virtual-iPhone-Emulator not found. Set VPHONE_ROOT or clone beside the IDE.".to_string())?;
    let tc = toolchain_dir(&root);
    #[cfg(windows)]
    let script = tc.join("vphone-doctor.bat");
    #[cfg(not(windows))]
    let script = tc.join("vphone-doctor.sh");

    let (stdout, stderr, code) = run_script(&script)?;
    Ok(json!({
        "ok": code == 0,
        "exit_code": code,
        "stdout": stdout,
        "stderr": stderr,
        "vphone_root": root.to_string_lossy(),
    }))
}

#[tauri::command]
pub fn install_vphone_toolchain() -> Result<Value, String> {
    let root = resolve_vphone_root()
        .ok_or_else(|| "Virtual-iPhone-Emulator not found. Set VPHONE_ROOT.".to_string())?;
    let tc = toolchain_dir(&root);
    #[cfg(windows)]
    let script = tc.join("install_doctor.bat");
    #[cfg(not(windows))]
    let script = tc.join("install_doctor.sh");

    let (stdout, stderr, code) = run_script(&script)?;
    Ok(json!({
        "ok": code == 0,
        "exit_code": code,
        "stdout": stdout,
        "stderr": stderr,
    }))
}

#[tauri::command]
pub fn get_mobile_toolchain_env() -> Result<Value, String> {
    let root = resolve_vphone_root();
    let developer_dir = root.as_ref().map(|_r| {
        #[cfg(windows)]
        {
            r"C:\Program Files\vphone\Xcode.app\Contents\Developer".to_string()
        }
        #[cfg(target_os = "macos")]
        {
            r.join("toolchain")
                .join("Xcode.app")
                .join("Contents")
                .join("Developer")
                .to_string_lossy()
                .to_string()
        }
        #[cfg(all(not(windows), not(target_os = "macos")))]
        {
            r.join("toolchain")
                .join("Xcode.app")
                .join("Contents")
                .join("Developer")
                .to_string_lossy()
                .to_string()
        }
    });
    Ok(json!({
        "DEVELOPER_DIR": developer_dir,
        "VPHONE_ROOT": root.as_ref().map(|p| p.to_string_lossy().to_string()),
        "PATH_PREFIX": root.as_ref().map(|p| toolchain_dir(p).join("bin").to_string_lossy().to_string()),
    }))
}
