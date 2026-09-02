use std::path::{Path, PathBuf};
use std::process::Command;

use crate::android_sdk;
use crate::architecture::domain::android::{
    AndroidDevice, AndroidPlatformRepository, AvdEntry, SdkConfig,
};
use crate::process_ext::hidden_command;

pub struct AdbAndroidRepository;

impl AdbAndroidRepository {
    pub fn new() -> Self {
        Self
    }

    fn sdk_root(override_path: Option<&str>) -> PathBuf {
        if let Some(p) = override_path {
            return PathBuf::from(p);
        }
        PathBuf::from(android_sdk::get_android_sdk_path())
    }

    fn adb_cmd(sdk_override: Option<&str>) -> Command {
        if let Some(path) = sdk_override {
            let base = PathBuf::from(path);
            for rel in ["platform-tools/adb.exe", "platform-tools/adb", "adb.exe", "adb"] {
                let cand = base.join(rel);
                if cand.is_file() {
                    return hidden_command(cand.to_string_lossy().to_string());
                }
            }
        }
        android_sdk::get_adb_cmd()
    }

    fn emulator_cmd(sdk_override: Option<&str>) -> String {
        if let Some(path) = sdk_override {
            let base = PathBuf::from(path);
            for rel in ["emulator/emulator.exe", "emulator/emulator"] {
                let cand = base.join(rel);
                if cand.is_file() {
                    return cand.to_string_lossy().to_string();
                }
            }
        }
        android_sdk::emulator_path().to_string_lossy().to_string()
    }
}

impl Default for AdbAndroidRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidPlatformRepository for AdbAndroidRepository {
    fn resolve_sdk_config(&self, override_path: Option<&str>) -> SdkConfig {
        let sdk_path = override_path.map(String::from).or_else(|| {
            let root = Self::sdk_root(None);
            if root.is_dir() {
                Some(root.to_string_lossy().to_string())
            } else {
                None
            }
        });
        SdkConfig {
            adb_found: android_sdk::adb_exists(),
            emulator_found: android_sdk::emulator_exists(),
            sdk_path,
        }
    }

    fn list_devices(&self, sdk_override: Option<&str>) -> Result<Vec<AndroidDevice>, String> {
        let output = Self::adb_cmd(sdk_override)
            .arg("devices")
            .output()
            .map_err(|e| format!("ADB devices failed: {e}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout
            .lines()
            .skip(1)
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(AndroidDevice {
                        id: parts[0].to_string(),
                        state: parts[1].to_string(),
                    })
                } else {
                    None
                }
            })
            .collect())
    }

    fn list_avds(&self, sdk_override: Option<&str>) -> Result<Vec<AvdEntry>, String> {
        let cmd = Self::emulator_cmd(sdk_override);
        let output = Command::new(&cmd)
            .arg("-list-avds")
            .output()
            .map_err(|e| format!("emulator -list-avds failed: {e}"))?;
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|name| AvdEntry {
                name: name.to_string(),
            })
            .collect())
    }

    fn spawn_emulator(&self, sdk_override: Option<&str>, avd: &str) -> Result<(), String> {
        let cmd = Self::emulator_cmd(sdk_override);
        Command::new(&cmd)
            .arg("-avd")
            .arg(avd)
            .spawn()
            .map_err(|e| format!("Failed to spawn emulator: {e}"))?;
        Ok(())
    }

    fn install_apk(&self, sdk_override: Option<&str>, device: Option<&str>, apk_path: &str) -> Result<(), String> {
        if !Path::new(apk_path).is_file() {
            return Err(format!("APK not found: {apk_path}"));
        }
        let mut cmd = Self::adb_cmd(sdk_override);
        if let Some(serial) = device {
            cmd.args(["-s", serial]);
        }
        let status = cmd
            .args(["install", "-r", apk_path])
            .status()
            .map_err(|e| format!("adb install failed: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err("adb install returned non-zero exit code".into())
        }
    }

    fn launch_activity(
        &self,
        sdk_override: Option<&str>,
        device: Option<&str>,
        package: &str,
        activity: &str,
    ) -> Result<(), String> {
        let mut cmd = Self::adb_cmd(sdk_override);
        if let Some(serial) = device {
            cmd.args(["-s", serial]);
        }
        let status = cmd
            .args([
                "shell",
                "am",
                "start",
                "-n",
                &format!("{package}/{activity}"),
            ])
            .status()
            .map_err(|e| format!("adb shell am start failed: {e}"))?;
        if status.success() {
            Ok(())
        } else {
            Err("adb shell am start failed".into())
        }
    }
}
