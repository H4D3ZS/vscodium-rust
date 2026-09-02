use std::sync::Arc;

use crate::architecture::domain::android::{AndroidDevice, AvdEntry, AndroidPlatformRepository, SdkConfig};
use crate::architecture::infrastructure::android::AdbAndroidRepository;

pub struct AndroidService {
    repo: Arc<AdbAndroidRepository>,
}

impl AndroidService {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(AdbAndroidRepository::new()),
        }
    }

    pub fn sdk_config(&self, override_path: Option<&str>) -> SdkConfig {
        self.repo.resolve_sdk_config(override_path)
    }

    pub fn list_devices(&self, override_path: Option<&str>) -> Result<Vec<AndroidDevice>, String> {
        self.repo.list_devices(override_path)
    }

    pub fn list_avds(&self, override_path: Option<&str>) -> Result<Vec<AvdEntry>, String> {
        self.repo.list_avds(override_path)
    }

    pub fn spawn_emulator(&self, override_path: Option<&str>, avd: &str) -> Result<(), String> {
        self.repo.spawn_emulator(override_path, avd)
    }

    pub fn install_and_launch(
        &self,
        override_path: Option<&str>,
        device: Option<&str>,
        apk_path: &str,
        package: Option<&str>,
        activity: Option<&str>,
    ) -> Result<(), String> {
        self.repo.install_apk(override_path, device, apk_path)?;
        if let (Some(pkg), Some(act)) = (package, activity) {
            self.repo
                .launch_activity(override_path, device, pkg, act)?;
        }
        Ok(())
    }
}

impl Default for AndroidService {
    fn default() -> Self {
        Self::new()
    }
}
