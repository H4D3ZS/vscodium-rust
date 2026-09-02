use super::{AndroidDevice, AvdEntry, SdkConfig};

/// Port: Android SDK / ADB / emulator operations (no Tauri, no tokio in impl signature for sync helpers).
pub trait AndroidPlatformRepository: Send + Sync {
    fn resolve_sdk_config(&self, override_path: Option<&str>) -> SdkConfig;

    fn list_devices(&self, sdk_override: Option<&str>) -> Result<Vec<AndroidDevice>, String>;

    fn list_avds(&self, sdk_override: Option<&str>) -> Result<Vec<AvdEntry>, String>;

    fn spawn_emulator(&self, sdk_override: Option<&str>, avd: &str) -> Result<(), String>;

    fn install_apk(&self, sdk_override: Option<&str>, device: Option<&str>, apk_path: &str) -> Result<(), String>;

    fn launch_activity(
        &self,
        sdk_override: Option<&str>,
        device: Option<&str>,
        package: &str,
        activity: &str,
    ) -> Result<(), String>;
}
