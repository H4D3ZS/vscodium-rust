//! Shared Android SDK / ADB path resolution for emulator + scrcpy modules.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::process_ext::hidden_command;

/// Resolve Android SDK root from env vars and common install locations.
pub fn get_android_sdk_path() -> String {
    for var in ["ANDROID_HOME", "ANDROID_SDK_ROOT"] {
        if let Ok(val) = std::env::var(var) {
            if Path::new(&val).exists() {
                return val;
            }
        }
    }
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| "C:\\Users\\Default".to_string());
    let locations = [
        format!("{home}\\AppData\\Local\\Android\\Sdk"),
        format!("{home}\\Android\\Sdk"),
        "C:\\Android\\Sdk".to_string(),
        "C:\\Program Files\\Android\\Sdk".to_string(),
        "C:\\Program Files (x86)\\Android\\Sdk".to_string(),
    ];
    for loc in &locations {
        if Path::new(loc).exists() {
            return loc.clone();
        }
    }
    format!("{home}\\AppData\\Local\\Android\\Sdk")
}

pub fn adb_path() -> PathBuf {
    let sdk = get_android_sdk_path();
    let candidates = [
        PathBuf::from(&sdk).join("platform-tools").join("adb.exe"),
        PathBuf::from(&sdk).join("platform-tools").join("adb"),
        PathBuf::from(&sdk).join("adb.exe"),
        PathBuf::from(&sdk).join("adb"),
    ];
    for p in &candidates {
        if p.exists() {
            return p.clone();
        }
    }
    PathBuf::from("adb")
}

pub fn emulator_path() -> PathBuf {
    let sdk = get_android_sdk_path();
    let candidates = [
        PathBuf::from(&sdk).join("emulator").join("emulator.exe"),
        PathBuf::from(&sdk).join("emulator").join("emulator"),
    ];
    for p in &candidates {
        if p.exists() {
            return p.clone();
        }
    }
    PathBuf::from("emulator")
}

pub fn get_adb_cmd() -> Command {
    let path = adb_path();
    if path.exists() {
        hidden_command(path.to_string_lossy().to_string())
    } else {
        hidden_command("adb".to_string())
    }
}

pub fn adb_exists() -> bool {
    adb_path().exists() || which::which("adb").is_ok()
}

pub fn emulator_exists() -> bool {
    emulator_path().exists() || which::which("emulator").is_ok()
}
