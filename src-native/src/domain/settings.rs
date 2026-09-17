#[derive(Clone, Debug)]
pub struct ModelOption {
    pub id: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Debug)]
pub struct SettingsStore {
    pub active_model: String,
    pub inference_url: String,
    pub context_tokens: usize,
    pub d3d12_enabled: bool,
    pub directwrite_enabled: bool,
}

impl Default for SettingsStore {
    fn default() -> Self {
        Self {
            active_model: "Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M".to_string(),
            inference_url: "http://localhost:13305".to_string(),
            context_tokens: 98304,
            d3d12_enabled: true,
            directwrite_enabled: true,
        }
    }
}
