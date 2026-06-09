/// Dynamic model management for local Ollama
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModel {
    pub name: String,
    pub size: String,
    pub modified: String,
    pub param_count: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub context_window: usize,
    pub recommended: bool,
    pub supports_12b_and_below: bool,
}

/// Parse model parameter count from name (e.g., "qwen3.5:12b" → 12)
pub fn parse_param_count(model_name: &str) -> Option<usize> {
    let lower = model_name.to_lowercase();

    // Match patterns like "12b", "7b", "70b"
    if let Some(start) = lower.find(':') {
        if let Some(b_pos) = lower[start..].find('b') {
            let size_str = &lower[start+1..start+b_pos];
            return size_str.parse::<usize>().ok();
        }
    }

    None
}

/// Get recommended context window for a model
pub fn get_context_window(model_name: &str) -> usize {
    let lower = model_name.to_lowercase();

    // Qwen models
    if lower.contains("qwen3.6") { return 65_536; }
    if lower.contains("qwen3") { return 32_768; }
    if lower.contains("qwen2.5") { return 32_768; }

    // Generic: use parameter count
    if let Some(params) = parse_param_count(model_name) {
        return match params {
            0..=7 => 8_192,
            8..=13 => 16_384,
            14..=32 => 32_768,
            _ => 65_536,
        };
    }

    32_768 // Default
}

/// Check if model is suitable for offline 12b-and-below work
pub fn is_suitable_for_offline(model_name: &str) -> bool {
    if let Some(params) = parse_param_count(model_name) {
        params <= 12
    } else {
        // If we can't parse, assume it's okay
        true
    }
}
