//! Thermal Back-Pressure Governor
//! 
//! Monitors GPU temperature and power draw, autonomously throttling
//! paging frequency when thresholds are exceeded.
//! 
//! Targets:
//! - AMD RX 580 (8GB VRAM) via amdgpu sysfs (Linux) or WMI (Windows)
//! - Thermal throttle: 72°C
//! - Power throttle: 150W

use anyhow::Result;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{watch, Mutex};
use tracing::{info, error};

#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "windows")]
use wmi::{WMIConnection, COMLibrary};

/// GPU telemetry data
#[derive(Debug, Clone, Copy)]
pub struct GpuTelemetry {
    /// Temperature in Celsius
    pub temperature_c: f32,
    /// Power draw in Watts
    pub power_watts: f32,
    /// GPU utilization percentage (0.0 - 1.0)
    pub utilization: f32,
    /// VRAM usage in bytes
    pub vram_used: u64,
    /// Timestamp of reading
    pub timestamp: Instant,
}

impl Default for GpuTelemetry {
    fn default() -> Self {
        Self {
            temperature_c: 0.0,
            power_watts: 0.0,
            utilization: 0.0,
            vram_used: 0,
            timestamp: Instant::now(),
        }
    }
}

/// Thermal policy configuration
#[derive(Debug, Clone)]
pub struct ThermalPolicy {
    /// Temperature threshold for throttling (°C)
    pub throttle_temp_c: f32,
    /// Critical temperature for emergency shutdown (°C)
    pub critical_temp_c: f32,
    /// Power threshold for throttling (Watts)
    pub throttle_power_w: f32,
    /// Throttle ratio when threshold exceeded (0.0 = full stop, 1.0 = no throttle)
    pub throttle_ratio: f32,
    /// Sampling interval
    pub sample_interval: Duration,
}

impl Default for ThermalPolicy {
    fn default() -> Self {
        Self {
            throttle_temp_c: 72.0,
            critical_temp_c: 80.0,
            throttle_power_w: 150.0,
            throttle_ratio: 0.5,
            sample_interval: Duration::from_millis(500),
        }
    }
}

/// Thermal governor state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorState {
    /// Normal operation - no throttling
    Normal,
    /// Thermal throttling active
    ThermalThrottle,
    /// Power throttling active
    PowerThrottle,
    /// Critical - emergency shutdown
    Critical,
}

/// Thermal Back-Pressure Governor
/// 
/// Monitors GPU telemetry and applies back-pressure to weight streaming
/// when thermal or power thresholds are exceeded.
pub struct ThermalGovernor {
    policy: ThermalPolicy,
    state: GovernorState,
    telemetry: GpuTelemetry,
    throttle_tx: watch::Sender<f32>,
    throttle_rx: watch::Receiver<f32>,
    last_sample: Instant,
    
    #[cfg(target_os = "windows")]
    #[allow(dead_code)]
    wmi: Option<Arc<Mutex<WMIConnection>>>,
}

impl ThermalGovernor {
    /// Create new thermal governor with default policy
    pub fn new() -> Result<Self> {
        Self::with_policy(ThermalPolicy::default())
    }
    
    /// Create new thermal governor with custom policy
    pub fn with_policy(policy: ThermalPolicy) -> Result<Self> {
        #[cfg(target_os = "windows")]
        let wmi = {
            let com_con = COMLibrary::new()?;
            WMIConnection::new(com_con).map(|w| Arc::new(Mutex::new(w))).ok()
        };
        
        let (throttle_tx, throttle_rx) = watch::channel(1.0);
        
        Ok(Self {
            policy,
            state: GovernorState::Normal,
            telemetry: GpuTelemetry::default(),
            throttle_tx,
            throttle_rx,
            last_sample: Instant::now(),
            #[cfg(target_os = "windows")]
            wmi,
        })
    }
    
    /// Sample GPU telemetry
    pub fn sample(&mut self) -> Result<GpuTelemetry> {
        let telemetry = self.read_telemetry()?;
        self.telemetry = telemetry;
        self.last_sample = Instant::now();
        self.update_state();
        Ok(telemetry)
    }

    /// Clone for FFI usage (creates independent governor with same policy)
    pub fn clone_for_ffi(&self) -> Self {
        let (throttle_tx, throttle_rx) = watch::channel(*self.throttle_rx.borrow());
        
        Self {
            policy: self.policy.clone(),
            state: self.state,
            telemetry: self.telemetry,
            throttle_tx,
            throttle_rx,
            last_sample: Instant::now(),
            #[cfg(target_os = "windows")]
            wmi: self.wmi.clone(),
        }
    }
    
    /// Read telemetry from GPU
    #[cfg(target_os = "linux")]
    fn read_telemetry(&self) -> Result<GpuTelemetry> {
        // AMDGPU sysfs paths
        const TEMP_PATH: &str = "/sys/class/drm/card0/device/hwmon/hwmon*/temp1_input";
        const POWER_PATH: &str = "/sys/class/drm/card0/device/hwmon/hwmon*/power1_input";
        
        // Temperature (millidegrees Celsius)
        let temp_raw = self.read_hwmon_value(TEMP_PATH)?;
        let temperature_c = temp_raw as f32 / 1000.0;
        
        // Power (microwatts)
        let power_raw = self.read_hwmon_value(POWER_PATH).unwrap_or(0);
        let power_watts = power_raw as f32 / 1_000_000.0;
        
        // Utilization (from amdgpu-powersmith or similar)
        let utilization = self.read_gpu_utilization().unwrap_or(0.0);
        
        // VRAM usage
        let vram_used = self.read_vram_usage().unwrap_or(0);
        
        Ok(GpuTelemetry {
            temperature_c,
            power_watts,
            utilization,
            vram_used,
            timestamp: Instant::now(),
        })
    }
    
    #[cfg(target_os = "linux")]
    fn read_hwmon_value(&self, pattern: &str) -> Result<u32> {
        use glob::glob;
        for entry in glob(pattern)? {
            if let Ok(path) = entry {
                let content = fs::read_to_string(path)?;
                return content.trim().parse::<u32>()
                    .with_context(|| "Failed to parse hwmon value");
            }
        }
        anyhow::bail!("No hwmon device found")
    }
    
    #[cfg(target_os = "linux")]
    fn read_gpu_utilization(&self) -> Result<f32> {
        // Try to read from amdgpu-powersmith or fallback to 0
        if let Ok(output) = std::process::Command::new("rocm-smi")
            .arg("--showutilization")
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Parse output like "GPU[0] Utilization: 45%"
            for line in stdout.lines() {
                if line.contains("Utilization") {
                    if let Some(pct) = line.split('%').next() {
                        if let Some(val) = pct.split_whitespace().last() {
                            return val.parse::<f32>()
                                .map(|v| v / 100.0)
                                .or(Ok(0.0));
                        }
                    }
                }
            }
        }
        Ok(0.0)
    }
    
    #[cfg(target_os = "linux")]
    fn read_vram_usage(&self) -> Result<u64> {
        // Read from DRM memory info
        const MEM_INFO_PATH: &str = "/sys/class/drm/card0/device/mem_info_vram_used";
        if let Ok(content) = fs::read_to_string(MEM_INFO_PATH) {
            return content.trim().parse::<u64>()
                .with_context(|| "Failed to parse VRAM usage");
        }
        Ok(0)
    }
    
    /// Read telemetry from GPU (Windows)
    #[cfg(target_os = "windows")]
    fn read_telemetry(&self) -> Result<GpuTelemetry> {
        // Simplified WMI telemetry - placeholders for now
        // Production would use AMD Adrenalin SDK or NVIDIA NVAPI
        
        let temperature_c = self.query_wmi_temperature()?;
        let power_watts = self.query_wmi_power()?;
        let utilization = self.query_wmi_utilization()?;
        let vram_used = self.query_wmi_vram()?;
        
        Ok(GpuTelemetry {
            temperature_c,
            power_watts,
            utilization,
            vram_used,
            timestamp: Instant::now(),
        })
    }
    
    #[cfg(target_os = "windows")]
    fn query_wmi_temperature(&self) -> Result<f32> {
        // MSAcpi_ThermalZoneTemperature returns Kelvin * 10
        // This is motherboard temp, not GPU - for GPU need vendor SDK
        // WMI crate 0.14 uses typed queries - simplified placeholder
        Ok(0.0)
    }
    
    #[cfg(target_os = "windows")]
    fn query_wmi_power(&self) -> Result<f32> {
        // Power data not directly available via WMI
        // Would need AMD Adrenalin SDK or NVIDIA NVAPI
        Ok(0.0)
    }
    
    #[cfg(target_os = "windows")]
    fn query_wmi_utilization(&self) -> Result<f32> {
        // GPU utilization via WMI (if driver exposes it)
        // WMI crate 0.14 uses typed queries - simplified placeholder
        Ok(0.0)
    }
    
    #[cfg(target_os = "windows")]
    fn query_wmi_vram(&self) -> Result<u64> {
        // WMI crate 0.14 uses typed queries - simplified placeholder
        Ok(0)
    }
    
    /// Update governor state based on telemetry
    fn update_state(&mut self) {
        let temp = self.telemetry.temperature_c;
        let power = self.telemetry.power_watts;
        
        let new_state = if temp >= self.policy.critical_temp_c {
            GovernorState::Critical
        } else if temp >= self.policy.throttle_temp_c {
            GovernorState::ThermalThrottle
        } else if power >= self.policy.throttle_power_w {
            GovernorState::PowerThrottle
        } else {
            GovernorState::Normal
        };
        
        if new_state != self.state {
            info!(
                "Thermal governor state: {:?} -> {:?} (temp: {:.1}°C, power: {:.1}W)",
                self.state, new_state, temp, power
            );
            self.state = new_state;
            
            // Update throttle ratio
            let throttle_ratio = match new_state {
                GovernorState::Normal => 1.0,
                GovernorState::ThermalThrottle | GovernorState::PowerThrottle => {
                    self.policy.throttle_ratio
                }
                GovernorState::Critical => 0.0,
            };
            
            let _ = self.throttle_tx.send(throttle_ratio);
        }
        
        if new_state == GovernorState::Critical {
            error!("CRITICAL: GPU temperature {:.1}°C exceeds critical threshold {:.1}°C", 
                   temp, self.policy.critical_temp_c);
        }
    }
    
    /// Get current throttle ratio (0.0 = stopped, 1.0 = full speed)
    pub fn throttle_ratio(&self) -> f32 {
        *self.throttle_rx.borrow()
    }
    
    /// Subscribe to throttle ratio changes
    pub fn subscribe_throttle(&self) -> watch::Receiver<f32> {
        self.throttle_rx.clone()
    }
    
    /// Get current state
    pub fn state(&self) -> GovernorState {
        self.state
    }
    
    /// Get latest telemetry
    pub fn telemetry(&self) -> GpuTelemetry {
        self.telemetry
    }
    
    /// Check if should sample again
    pub fn should_sample(&self) -> bool {
        self.last_sample.elapsed() >= self.policy.sample_interval
    }
    
    /// Apply delay based on thermal pressure (non-blocking)
    pub async fn apply_back_pressure(&self) {
        let ratio = self.throttle_ratio();
        if ratio < 1.0 {
            let delay_ms = ((1.0 - ratio) * 50.0) as u64;
            if delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }
}

impl Default for ThermalGovernor {
    fn default() -> Self {
        Self::new().expect("Failed to create thermal governor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thermal_policy_defaults() {
        let policy = ThermalPolicy::default();
        assert_eq!(policy.throttle_temp_c, 72.0);
        assert_eq!(policy.critical_temp_c, 80.0);
        assert_eq!(policy.throttle_power_w, 150.0);
    }
    
    #[test]
    fn test_governor_creation() {
        let governor = ThermalGovernor::new();
        assert!(governor.is_ok());
    }
}
