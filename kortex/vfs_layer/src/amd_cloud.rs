//! AMD ROCm Cloud Integration - MI300X Cloud-Burst Gateway
//! 
//! This module provides seamless integration with AMD Developer Cloud
//! for bursting heavy compute workloads to MI300X GPUs.
//! 
//! Architecture:
//! - Local: Ollama + AIM Proxy (port 1536) for lightweight tasks
//! - Cloud: AMD MI300X via vLLM for heavy reasoning/embedding
//! 
//! Demo Mode: Visual indicator shows when cloud-burst is active

use std::env;
use std::time::{Duration, Instant};

/// AMD Cloud Configuration
#[derive(Debug, Clone)]
pub struct AmdCloudConfig {
    /// AMD Developer Cloud API endpoint
    pub api_endpoint: String,
    /// MI300X GPU droplet ID (for managed instances)
    pub droplet_id: Option<String>,
    /// vLLM endpoint on AMD cloud
    pub vllm_endpoint: String,
    /// API key for authentication
    pub api_key: Option<String>,
    /// Enable cloud burst mode
    pub cloud_burst_enabled: bool,
    /// Auto-scale threshold (context size in tokens)
    pub autoscale_threshold: usize,
}

impl Default for AmdCloudConfig {
    fn default() -> Self {
        Self {
            api_endpoint: env::var("AMD_CLOUD_API")
                .unwrap_or_else(|_| "https://api.amdcloud.io".to_string()),
            droplet_id: env::var("AMD_DROPLET_ID").ok(),
            vllm_endpoint: env::var("AMD_VLLM_ENDPOINT")
                .unwrap_or_else(|_| "https://vllm.amdcloud.io:8000".to_string()),
            api_key: env::var("AMD_API_KEY").ok(),
            cloud_burst_enabled: true,
            autoscale_threshold: 32_000, // Burst when context > 32K tokens
        }
    }
}

/// Cloud Burst State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudBurstState {
    /// Running locally on Ollama
    Local,
    /// Bursting to AMD MI300X
    CloudBurst,
    /// Hybrid mode (local + cloud)
    Hybrid,
    /// Offline/Fallback mode
    Offline,
}

/// AMD MI300X Cloud Gateway
pub struct AmdCloudGateway {
    config: AmdCloudConfig,
    state: CloudBurstState,
    last_burst_time: Option<Instant>,
    burst_count: u64,
    total_cloud_tokens: u64,
    /// Connection pool for vLLM
    client: reqwest::Client,
}

impl AmdCloudGateway {
    /// Create new AMD Cloud Gateway
    pub fn new(config: AmdCloudConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300)) // 5 min timeout for large batches
            .pool_max_idle_per_host(10)
            .build()
            .unwrap_or_default();

        Self {
            config,
            state: CloudBurstState::Local,
            last_burst_time: None,
            burst_count: 0,
            total_cloud_tokens: 0,
            client,
        }
    }

    /// Check if cloud burst should be activated
    pub fn should_burst(&self, context_size: usize) -> bool {
        self.config.cloud_burst_enabled 
            && context_size > self.config.autoscale_threshold
            && self.state != CloudBurstState::Offline
    }

    /// Activate cloud burst mode
    pub async fn activate_burst(&mut self) -> Result<(), AmdCloudError> {
        // Health check AMD cloud endpoint
        let health_url = format!("{}/health", self.config.vllm_endpoint);
        
        match self.client.get(&health_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                self.state = CloudBurstState::CloudBurst;
                self.last_burst_time = Some(Instant::now());
                self.burst_count += 1;
                log::info!("[AMD Cloud] Burst activated - MI300X ready");
                Ok(())
            }
            Ok(resp) => {
                log::warn!("[AMD Cloud] Health check failed: {}", resp.status());
                self.state = CloudBurstState::Offline;
                Err(AmdCloudError::HealthCheckFailed(resp.status().to_string()))
            }
            Err(e) => {
                log::error!("[AMD Cloud] Connection error: {}", e);
                self.state = CloudBurstState::Offline;
                Err(AmdCloudError::ConnectionFailed(e.to_string()))
            }
        }
    }

    /// Deactivate cloud burst, return to local
    pub fn deactivate_burst(&mut self) {
        if self.state == CloudBurstState::CloudBurst {
            log::info!("[AMD Cloud] Burst deactivated - returning to local");
            self.state = CloudBurstState::Local;
        }
    }

    /// Send embedding batch to MI300X
    pub async fn batch_embed(
        &self,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>, AmdCloudError> {
        if self.state != CloudBurstState::CloudBurst {
            return Err(AmdCloudError::NotInBurstMode);
        }

        let embed_url = format!("{}/v1/embeddings", self.config.vllm_endpoint);
        
        let payload = serde_json::json!({
            "model": "mistral-large",
            "input": texts,
            "encoding_format": "float"
        });

        let mut request = self.client.post(&embed_url).json(&payload);
        
        if let Some(ref key) = self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            return Err(AmdCloudError::ApiError(
                response.status().to_string(),
                response.text().await.unwrap_or_default()
            ));
        }

        let result: serde_json::Value = response.json().await?;
        
        // Parse embeddings from vLLM response
        let embeddings = result["data"]
            .as_array()
            .ok_or_else(|| AmdCloudError::ParseError("Missing data array".into()))?
            .iter()
            .filter_map(|item| {
                item["embedding"]
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_f64().map(|f| f as f32))
                            .collect()
                    })
            })
            .collect();

        Ok(embeddings)
    }

    /// Send completion request to MI300X
    pub async fn cloud_complete(
        &mut self,
        prompt: String,
        max_tokens: Option<usize>,
    ) -> Result<String, AmdCloudError> {
        if self.state != CloudBurstState::CloudBurst {
            return Err(AmdCloudError::NotInBurstMode);
        }

        let complete_url = format!("{}/v1/completions", self.config.vllm_endpoint);
        
        let mut payload = serde_json::json!({
            "model": "mistral-large",
            "prompt": prompt,
            "max_tokens": max_tokens.unwrap_or(2048),
            "stream": false
        });

        // Add AMD-specific optimizations
        payload["use_flash_attn"] = serde_json::json!(true); // Flash Attention for MI300X
        payload["quantization"] = serde_json::json!("fp8");  // FP8 for MI300X

        let mut request = self.client.post(&complete_url).json(&payload);
        
        if let Some(ref key) = self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response = request.send().await?;
        
        if !response.status().is_success() {
            return Err(AmdCloudError::ApiError(
                response.status().to_string(),
                response.text().await.unwrap_or_default()
            ));
        }

        let result: serde_json::Value = response.json().await?;
        
        let completion = result["choices"][0]["text"]
            .as_str()
            .ok_or_else(|| AmdCloudError::ParseError("Missing completion text".into()))?
            .to_string();

        self.total_cloud_tokens += result["usage"]["total_tokens"]
            .as_u64()
            .unwrap_or(0);

        Ok(completion)
    }

    /// Get current gateway status
    pub fn status(&self) -> AmdCloudStatus {
        AmdCloudStatus {
            state: self.state,
            burst_count: self.burst_count,
            total_cloud_tokens: self.total_cloud_tokens,
            last_burst_time: self.last_burst_time,
            vllm_endpoint: self.config.vllm_endpoint.clone(),
        }
    }

    /// Get configuration
    pub fn config(&self) -> &AmdCloudConfig {
        &self.config
    }
}

/// Cloud Gateway Status
#[derive(Debug, Clone)]
pub struct AmdCloudStatus {
    pub state: CloudBurstState,
    pub burst_count: u64,
    pub total_cloud_tokens: u64,
    pub last_burst_time: Option<Instant>,
    pub vllm_endpoint: String,
}

/// AMD Cloud Errors
#[derive(Debug, thiserror::Error)]
pub enum AmdCloudError {
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
    
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Not in burst mode - activate cloud burst first")]
    NotInBurstMode,
    
    #[error("API error ({0}): {1}")]
    ApiError(String, String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
}

/// AMD MI300X Hardware Profile
#[derive(Debug, Clone)]
pub struct MI300XProfile {
    /// VRAM in GB (192GB per GPU)
    pub vram_gb: usize,
    /// GPU count
    pub gpu_count: usize,
    /// Compute units
    pub compute_units: usize,
    /// FP8 TOPS
    pub fp8_tops: usize,
}

impl Default for MI300XProfile {
    fn default() -> Self {
        Self {
            vram_gb: 192,
            gpu_count: 1,
            compute_units: 304,
            fp8_tops: 2600, // 2.6 TFLOPS FP8 per GPU
        }
    }
}

impl MI300XProfile {
    /// Get total VRAM across all GPUs
    pub fn total_vram_gb(&self) -> usize {
        self.vram_gb * self.gpu_count
    }

    /// Get total FP8 throughput
    pub fn total_fp8_tops(&self) -> usize {
        self.fp8_tops * self.gpu_count
    }

    /// Estimate max context size for given model
    pub fn max_context_for_model(&self, model_params_b: usize) -> usize {
        // Rough estimate: VRAM / (2 * params_bytes) for KV cache
        let params_bytes = model_params_b * 2; // FP16
        let available_vram_bytes = self.total_vram_gb() * 1024 * 1024 * 1024;
        available_vram_bytes / (params_bytes * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mi300x_profile() {
        let profile = MI300XProfile::default();
        assert_eq!(profile.vram_gb, 192);
        assert_eq!(profile.total_vram_gb(), 192);
        assert_eq!(profile.fp8_tops, 2600);
    }

    #[test]
    fn test_cloud_burst_threshold() {
        let config = AmdCloudConfig {
            autoscale_threshold: 32_000,
            ..Default::default()
        };
        let gateway = AmdCloudGateway::new(config);
        
        assert!(!gateway.should_burst(16_000));
        assert!(gateway.should_burst(64_000));
    }
}
