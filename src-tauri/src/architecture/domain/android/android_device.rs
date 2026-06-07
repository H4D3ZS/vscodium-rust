use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidDevice {
    pub id: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvdEntry {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkConfig {
    pub sdk_path: Option<String>,
    pub adb_found: bool,
    pub emulator_found: bool,
}
