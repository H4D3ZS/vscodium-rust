//! AMD Cloud Burst - Demo Mode for Hackathon
//! 
//! This module provides visual indicators and telemetry
//! for demonstrating AMD MI300X integration during the hackathon.

use crate::amd_cloud::{AmdCloudGateway, AmdCloudStatus};
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::Instant;

/// Demo Mode Configuration
#[derive(Debug, Clone)]
pub struct DemoModeConfig {
    /// Enable visual indicators
    pub show_indicators: bool,
    /// Log all cloud bursts
    pub log_bursts: bool,
    /// Auto-burst for demo purposes
    pub auto_burst_threshold: Option<usize>,
}

impl Default for DemoModeConfig {
    fn default() -> Self {
        Self {
            show_indicators: true,
            log_bursts: true,
            auto_burst_threshold: Some(50_000), // Auto-burst at 50K tokens for demo
        }
    }
}

/// AMD Demo Mode Manager
pub struct AmdDemoMode {
    config: DemoModeConfig,
    gateway: Option<AmdCloudGateway>,
    /// Statistics for demo display
    stats: DemoStats,
    /// Visual state for UI
    visual_state: AtomicVisualState,
}

/// Demo Statistics
#[derive(Debug)]
pub struct DemoStats {
    pub total_requests: AtomicU64,
    pub cloud_requests: AtomicU64,
    pub local_requests: AtomicU64,
    pub total_tokens_processed: AtomicU64,
    pub cloud_tokens_processed: AtomicU64,
    pub avg_cloud_latency_ms: AtomicU64,
    pub start_time: Instant,
}

impl Default for DemoStats {
    fn default() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            cloud_requests: AtomicU64::new(0),
            local_requests: AtomicU64::new(0),
            total_tokens_processed: AtomicU64::new(0),
            cloud_tokens_processed: AtomicU64::new(0),
            avg_cloud_latency_ms: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }
}

/// Visual State for UI (atomic for lock-free access)
pub struct AtomicVisualState {
    pub is_bursting: AtomicBool,
    pub cloud_health: AtomicU64, // 0-100
    pub current_mode: AtomicU64, // 0=Local, 1=Cloud, 2=Hybrid
}

impl Default for AtomicVisualState {
    fn default() -> Self {
        Self {
            is_bursting: AtomicBool::new(false),
            cloud_health: AtomicU64::new(100),
            current_mode: AtomicU64::new(0),
        }
    }
}

impl AmdDemoMode {
    /// Create new demo mode manager
    pub fn new(config: DemoModeConfig) -> Self {
        Self {
            config,
            gateway: None,
            stats: DemoStats::default(),
            visual_state: AtomicVisualState::default(),
        }
    }

    /// Initialize with AMD cloud gateway
    pub fn with_gateway(mut self, gateway: AmdCloudGateway) -> Self {
        self.gateway = Some(gateway);
        self
    }

    /// Record a request and update stats
    pub fn record_request(&self, was_cloud: bool, tokens: u64, latency_ms: u64) {
        self.stats.total_requests.fetch_add(1, Ordering::Relaxed);
        
        if was_cloud {
            self.stats.cloud_requests.fetch_add(1, Ordering::Relaxed);
            self.stats.cloud_tokens_processed.fetch_add(tokens, Ordering::Relaxed);
            self.visual_state.is_bursting.store(true, Ordering::Relaxed);
            self.visual_state.current_mode.store(1, Ordering::Relaxed);
        } else {
            self.stats.local_requests.fetch_add(1, Ordering::Relaxed);
            self.visual_state.is_bursting.store(false, Ordering::Relaxed);
            self.visual_state.current_mode.store(0, Ordering::Relaxed);
        }
        
        self.stats.total_tokens_processed.fetch_add(tokens, Ordering::Relaxed);
        
        // Update average latency (simplified)
        let current_avg = self.stats.avg_cloud_latency_ms.load(Ordering::Relaxed);
        let new_avg = (current_avg + latency_ms) / 2;
        self.stats.avg_cloud_latency_ms.store(new_avg, Ordering::Relaxed);
    }

    /// Get current status for UI display
    pub fn get_status(&self) -> DemoStatus {
        DemoStatus {
            is_bursting: self.visual_state.is_bursting.load(Ordering::Relaxed),
            cloud_health: self.visual_state.cloud_health.load(Ordering::Relaxed) as u8,
            mode: match self.visual_state.current_mode.load(Ordering::Relaxed) {
                0 => ComputeMode::Local,
                1 => ComputeMode::Cloud,
                _ => ComputeMode::Hybrid,
            },
            total_requests: self.stats.total_requests.load(Ordering::Relaxed),
            cloud_requests: self.stats.cloud_requests.load(Ordering::Relaxed),
            local_requests: self.stats.local_requests.load(Ordering::Relaxed),
            total_tokens: self.stats.total_tokens_processed.load(Ordering::Relaxed),
            cloud_tokens: self.stats.cloud_tokens_processed.load(Ordering::Relaxed),
            uptime_secs: self.stats.start_time.elapsed().as_secs(),
        }
    }

    /// Get gateway status if available
    pub fn gateway_status(&self) -> Option<AmdCloudStatus> {
        self.gateway.as_ref().map(|g| g.status())
    }

    /// Check if demo mode is active
    pub fn is_active(&self) -> bool {
        self.config.show_indicators
    }

    /// Get config
    pub fn config(&self) -> &DemoModeConfig {
        &self.config
    }
}

/// Demo Status for UI
#[derive(Debug, Clone)]
pub struct DemoStatus {
    pub is_bursting: bool,
    pub cloud_health: u8,
    pub mode: ComputeMode,
    pub total_requests: u64,
    pub cloud_requests: u64,
    pub local_requests: u64,
    pub total_tokens: u64,
    pub cloud_tokens: u64,
    pub uptime_secs: u64,
}

/// Compute Mode Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeMode {
    Local,
    Cloud,
    Hybrid,
}

impl std::fmt::Display for ComputeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeMode::Local => write!(f, "🏠 Local (Ollama)"),
            ComputeMode::Cloud => write!(f, "☁️ AMD Cloud (MI300X)"),
            ComputeMode::Hybrid => write!(f, " Hybrid"),
        }
    }
}

/// Demo visualization helpers
pub mod viz {
    use super::*;

    /// Generate ASCII progress bar for demo display
    pub fn progress_bar(percent: u8, width: usize) -> String {
        let filled = (percent as usize * width / 100).min(width);
        let empty = width - filled;
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }

    /// Generate demo status display
    pub fn render_status(status: &DemoStatus) -> String {
        let mode_icon = match status.mode {
            ComputeMode::Local => "🏠",
            ComputeMode::Cloud => "☁️",
            ComputeMode::Hybrid => "🔀",
        };

        let burst_indicator = if status.is_bursting {
            "⚡ BURSTING"
        } else {
            "✓ Idle"
        };

        format!(
            r#"
╔═══════════════════════════════════════════╗
║  AMD MI300X Cloud-Burst Demo Status       ║
╠═══════════════════════════════════════════╣
║  Mode: {} {:>25} ║
║  Status: {:>33} ║
║                                           ║
║  Requests:                                ║
║    Total: {:>6}  Cloud: {:>6}  Local: {:>6}║
║                                           ║
║  Tokens Processed:                        ║
║    Total: {:>8}K  Cloud: {:>8}K          ║
║                                           ║
║  Uptime: {:>6}s  Health: {:>3}%          ║
╚═══════════════════════════════════════════╝
"#,
            mode_icon,
            format!("{}", status.mode),
            burst_indicator,
            status.total_requests,
            status.cloud_requests,
            status.local_requests,
            status.total_tokens / 1000,
            status.cloud_tokens / 1000,
            status.uptime_secs,
            status.cloud_health,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_stats() {
        let demo = AmdDemoMode::new(DemoModeConfig::default());
        
        demo.record_request(false, 1000, 50);
        demo.record_request(true, 5000, 200);
        demo.record_request(false, 2000, 30);
        
        let status = demo.get_status();
        assert_eq!(status.total_requests, 3);
        assert_eq!(status.cloud_requests, 1);
        assert_eq!(status.local_requests, 2);
    }

    #[test]
    fn test_progress_bar() {
        assert_eq!(viz::progress_bar(50, 10), "█████░░░░░");
        assert_eq!(viz::progress_bar(100, 5), "█████");
        assert_eq!(viz::progress_bar(0, 8), "░░░░░░░░");
    }
}
