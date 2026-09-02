//! Headless stub for AttachmentManager — provides the type the engine needs
//! without pulling in daemon (which brings candle-core / half version conflicts).

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachmentInfo {
    pub path: String,
    pub name: String,
    pub gist: Option<String>,
    pub thumbnail: Option<String>,
    pub data: Option<String>,
}

pub struct AttachmentManager;

impl AttachmentManager {
    pub fn new() -> Self { Self }

    pub async fn process_file(&self, _path: PathBuf, _model: &str) -> Result<AttachmentInfo, String> {
        Err("Attachment processing requires the Tauri shell".into())
    }
}
