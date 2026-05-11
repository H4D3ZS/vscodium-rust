//! Semantic Fault Handler
//!
//! Monitors attention head activations and triggers inflation requests
//! when cluster activation scores exceed the 0.85 threshold.
//!
//! ## Architecture
//!
//! ```text
//! Attention Heads (llama.cpp)
//!         │
//!         ▼
//! ┌───────────────────┐
//! │ Activation Buffer │ ← Ring buffer of head activations
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │ Cluster Aggregator│ → Computes per-cluster scores
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │ Fault Detector    │ → Triggers at ≥0.85
//! └─────────┬─────────┘
//!           │
//!           ▼
//! ┌───────────────────┐
//! │ Inflation Queue   │ → Async requests to JIT engine
//! └───────────────────┘
//! ```

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, Mutex, RwLock};
use tracing::{info, warn, error};

use crate::thermal::ThermalGovernor;
use super::semantic_map::LimbicIndex;

/// Attention head activation sample
#[derive(Debug, Clone, Copy)]
pub struct ActivationSample {
    /// Layer index
    pub layer: u32,
    /// Head index within layer
    pub head: u32,
    /// Cluster ID this head is attending to
    pub cluster_id: u32,
    /// Activation value (0.0-1.0)
    pub activation: f32,
    /// Timestamp
    pub timestamp: Instant,
}

/// Attention cluster - aggregates head activations
#[derive(Debug, Clone)]
pub struct AttentionCluster {
    /// Cluster ID
    pub cluster_id: u32,
    /// Rolling average activation
    pub avg_activation: f32,
    /// Number of samples in rolling window
    pub sample_count: usize,
    /// Maximum activation seen
    pub max_activation: f32,
    /// Last activation time
    pub last_activation: Instant,
    /// Heads contributing to this cluster
    pub contributing_heads: Vec<(u32, u32)>,  // (layer, head)
}

impl AttentionCluster {
    /// Create a new attention cluster
    pub fn new(cluster_id: u32) -> Self {
        Self {
            cluster_id,
            avg_activation: 0.0,
            sample_count: 0,
            max_activation: 0.0,
            last_activation: Instant::now(),
            contributing_heads: Vec::new(),
        }
    }

    /// Update with new activation sample
    pub fn update(&mut self, sample: &ActivationSample) {
        // Exponential moving average
        let alpha = 0.1;
        self.avg_activation = (1.0 - alpha) * self.avg_activation + alpha * sample.activation;
        
        self.sample_count = (self.sample_count + 1).min(100);  // Cap at 100 samples
        self.max_activation = self.max_activation.max(sample.activation);
        self.last_activation = sample.timestamp;
        
        // Track contributing heads
        let head_key = (sample.layer, sample.head);
        if !self.contributing_heads.contains(&head_key) {
            self.contributing_heads.push(head_key);
        }
    }

    /// Check if cluster exceeds fault threshold
    pub fn needs_inflation(&self, threshold: f32) -> bool {
        self.avg_activation >= threshold
    }

    /// Reset cluster state
    pub fn reset(&mut self) {
        self.avg_activation = 0.0;
        self.sample_count = 0;
        self.max_activation = 0.0;
    }
}

/// Inflation request from fault handler to JIT engine
#[derive(Debug, Clone)]
pub struct InflationRequest {
    /// Cluster ID to inflate
    pub cluster_id: u32,
    /// Priority (higher = more urgent)
    pub priority: u32,
    /// Requested timestamp
    pub requested_at: Instant,
    /// Limbic index metadata
    pub index: Option<LimbicIndex>,
    /// Thermal throttle state at request time
    pub thermal_throttle_ratio: f32,
}

impl InflationRequest {
    /// Create a new inflation request
    pub fn new(cluster_id: u32, priority: u32) -> Self {
        Self {
            cluster_id,
            priority,
            requested_at: Instant::now(),
            index: None,
            thermal_throttle_ratio: 1.0,
        }
    }

    /// Attach limbic index metadata
    pub fn with_index(mut self, index: LimbicIndex) -> Self {
        self.index = Some(index);
        self
    }

    /// Set thermal throttle ratio
    pub fn with_thermal_ratio(mut self, ratio: f32) -> Self {
        self.thermal_throttle_ratio = ratio;
        self
    }
}

/// Fault handler state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultState {
    /// Monitoring activations
    Monitoring,
    /// Fault detected, requesting inflation
    FaultDetected,
    /// Inflation in progress
    Inflating,
    /// Thermal throttle active
    ThermalThrottled,
    /// Error state
    Error,
}

/// Semantic Fault Handler
///
/// Monitors attention head activations and triggers inflation requests.
pub struct FaultHandler {
    /// Attention clusters being tracked
    clusters: Arc<RwLock<Vec<AttentionCluster>>>,
    /// Activation sample buffer (ring buffer)
    activation_buffer: Arc<Mutex<VecDeque<ActivationSample>>>,
    /// Buffer capacity
    buffer_capacity: usize,
    /// Fault threshold (default: 0.85)
    fault_threshold: f32,
    /// Current state
    state: AtomicU32,  // FaultState as u32
    /// Inflation request channel
    inflation_tx: mpsc::Sender<InflationRequest>,
    /// Inflation request receiver
    inflation_rx: Arc<Mutex<mpsc::Receiver<InflationRequest>>>,
    /// Thermal governor reference
    thermal_governor: Arc<ThermalGovernor>,
    /// Fault count
    fault_count: AtomicU64,
    /// Enabled flag
    enabled: AtomicBool,
}

impl FaultHandler {
    /// Create a new fault handler
    pub fn new(
        num_clusters: usize,
        buffer_capacity: usize,
        fault_threshold: f32,
        thermal_governor: Arc<ThermalGovernor>,
    ) -> Self {
        let (inflation_tx, inflation_rx) = mpsc::channel(64);
        
        let mut clusters = Vec::with_capacity(num_clusters);
        for i in 0..num_clusters {
            clusters.push(AttentionCluster::new(i as u32));
        }
        
        Self {
            clusters: Arc::new(RwLock::new(clusters)),
            activation_buffer: Arc::new(Mutex::new(VecDeque::with_capacity(buffer_capacity))),
            buffer_capacity,
            fault_threshold,
            state: AtomicU32::new(FaultState::Monitoring as u32),
            inflation_tx,
            inflation_rx: Arc::new(Mutex::new(inflation_rx)),
            thermal_governor,
            fault_count: AtomicU64::new(0),
            enabled: AtomicBool::new(true),
        }
    }

    /// Record an attention head activation
    pub async fn record_activation(&self, sample: ActivationSample) {
        if !self.enabled.load(Ordering::SeqCst) {
            return;
        }

        // Add to activation buffer
        {
            let mut buffer = self.activation_buffer.lock().await;
            if buffer.len() >= self.buffer_capacity {
                buffer.pop_front();  // Remove oldest
            }
            buffer.push_back(sample);
        }

        // Update cluster
        {
            let mut clusters = self.clusters.write().await;
            if let Some(cluster) = clusters.get_mut(sample.cluster_id as usize) {
                cluster.update(&sample);
                
                // Check for fault
                if cluster.needs_inflation(self.fault_threshold) {
                    self.trigger_fault(sample.cluster_id).await;
                }
            }
        }
    }

    /// Trigger a fault for a cluster
    async fn trigger_fault(&self, cluster_id: u32) {
        let current_state = self.state.load(Ordering::SeqCst);
        if current_state == FaultState::FaultDetected as u32 {
            return;  // Already handling a fault
        }

        self.state.store(FaultState::FaultDetected as u32, Ordering::SeqCst);
        self.fault_count.fetch_add(1, Ordering::SeqCst);

        info!(
            "FAULT DETECTED: cluster {} activation ≥ {:.2}",
            cluster_id, self.fault_threshold
        );

        // Check thermal state
        let throttle_ratio = self.thermal_governor.throttle_ratio();
        if throttle_ratio < 1.0 {
            warn!("Thermal throttle active ({:.0}%), delaying inflation", throttle_ratio * 100.0);
            self.state.store(FaultState::ThermalThrottled as u32, Ordering::SeqCst);
            
            // Wait for thermal to clear (with timeout)
            tokio::time::timeout(
                std::time::Duration::from_millis(100),
                self.thermal_governor.apply_back_pressure(),
            )
            .await
            .ok();
        }

        // Create inflation request
        let clusters = self.clusters.read().await;
        let request = if let Some(cluster) = clusters.get(cluster_id as usize) {
            InflationRequest::new(cluster_id, 100)
                .with_thermal_ratio(self.thermal_governor.throttle_ratio())
        } else {
            InflationRequest::new(cluster_id, 100)
                .with_thermal_ratio(self.thermal_governor.throttle_ratio())
        };
        drop(clusters);

        // Send to inflation engine
        if let Err(e) = self.inflation_tx.send(request).await {
            error!("Failed to send inflation request: {}", e);
            self.state.store(FaultState::Error as u32, Ordering::SeqCst);
        } else {
            self.state.store(FaultState::Inflating as u32, Ordering::SeqCst);
        }
    }

    /// Get inflation request receiver
    pub fn inflation_receiver(&self) -> Arc<Mutex<mpsc::Receiver<InflationRequest>>> {
        self.inflation_rx.clone()
    }

    /// Get current state
    pub fn state(&self) -> FaultState {
        match self.state.load(Ordering::SeqCst) {
            0 => FaultState::Monitoring,
            1 => FaultState::FaultDetected,
            2 => FaultState::Inflating,
            3 => FaultState::ThermalThrottled,
            _ => FaultState::Error,
        }
    }

    /// Get cluster by ID
    pub async fn get_cluster(&self, cluster_id: u32) -> Option<AttentionCluster> {
        let clusters = self.clusters.read().await;
        clusters.get(cluster_id as usize).cloned()
    }

    /// Get all clusters above threshold
    pub async fn clusters_above_threshold(&self) -> Vec<AttentionCluster> {
        let clusters = self.clusters.read().await;
        clusters
            .iter()
            .filter(|c| c.needs_inflation(self.fault_threshold))
            .cloned()
            .collect()
    }

    /// Get fault count
    pub fn fault_count(&self) -> u64 {
        self.fault_count.load(Ordering::SeqCst)
    }

    /// Enable/disable fault handler
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    /// Reset all clusters
    pub async fn reset(&self) {
        let mut clusters = self.clusters.write().await;
        for cluster in clusters.iter_mut() {
            cluster.reset();
        }
        self.state.store(FaultState::Monitoring as u32, Ordering::SeqCst);
    }

    /// Get activation buffer snapshot
    pub async fn get_activation_buffer(&self) -> Vec<ActivationSample> {
        let buffer = self.activation_buffer.lock().await;
        buffer.iter().cloned().collect()
    }
}

// Safety: FaultHandler can be shared across threads
unsafe impl Send for FaultHandler {}
unsafe impl Sync for FaultHandler {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thermal::ThermalPolicy;

    #[tokio::test]
    async fn test_fault_handler_creation() {
        let thermal = Arc::new(ThermalGovernor::with_policy(ThermalPolicy::default()).unwrap());
        let handler = FaultHandler::new(10, 1000, 0.85, thermal);
        
        assert_eq!(handler.state(), FaultState::Monitoring);
        assert_eq!(handler.fault_count(), 0);
    }

    #[tokio::test]
    async fn test_activation_recording() {
        let thermal = Arc::new(ThermalGovernor::default());
        let handler = FaultHandler::new(10, 1000, 0.85, thermal);
        
        // Record low activation
        let sample = ActivationSample {
            layer: 0,
            head: 0,
            cluster_id: 0,
            activation: 0.5,
            timestamp: Instant::now(),
        };
        handler.record_activation(sample).await;
        
        let cluster = handler.get_cluster(0).await.unwrap();
        assert!(!cluster.needs_inflation(0.85));
    }

    #[tokio::test]
    async fn test_fault_triggering() {
        let thermal = Arc::new(ThermalGovernor::default());
        let handler = FaultHandler::new(10, 1000, 0.85, thermal);
        
        // Record high activations to trigger fault
        for _ in 0..20 {
            let sample = ActivationSample {
                layer: 0,
                head: 0,
                cluster_id: 0,
                activation: 0.95,
                timestamp: Instant::now(),
            };
            handler.record_activation(sample).await;
        }
        
        // Should have triggered fault
        assert!(handler.fault_count() > 0);
    }
}
