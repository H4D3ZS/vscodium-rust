use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogcatEntry {
    pub raw: String,
    pub level: Option<String>,
    pub tag: Option<String>,
    pub message: String,
}
