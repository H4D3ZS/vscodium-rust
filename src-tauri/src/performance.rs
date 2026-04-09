use serde::Serialize;
use sysinfo::{Pid, System};

#[derive(Serialize, Clone)]
pub struct ProcessStats {
    pub memory_mb: u64,
    pub cpu_usage: f32,
    pub total_ram_gb: u64,
    pub available_ram_gb: u64,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum MemoryPressure {
    Normal,   // < 80% used
    Warning,  // 80-90% used
    Critical, // > 90% used
}

#[derive(Serialize, Clone, Debug)]
pub struct InferenceStats {
    pub device: String, // "ANE", "CPU", "GPU"
    pub latency_ms: u64,
    pub timestamp: u64,
}

pub struct PerformanceMonitor {
    sys: tokio::sync::Mutex<System>,
    pid: Pid,
    inference_history: tokio::sync::Mutex<Vec<InferenceStats>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        let sys = System::new();
        let pid = Pid::from(std::process::id() as usize);
        Self {
            sys: tokio::sync::Mutex::new(sys),
            pid,
            inference_history: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    pub async fn record_inference(&self, device: String, latency_ms: u64) {
        let mut history = self.inference_history.lock().await;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        history.push(InferenceStats {
            device,
            latency_ms,
            timestamp,
        });

        // Keep last 100 entries
        if history.len() > 100 {
            history.remove(0);
        }
    }

    pub async fn get_inference_history(&self) -> Vec<InferenceStats> {
        self.inference_history.lock().await.clone()
    }

    pub async fn get_stats(&self) -> Option<ProcessStats> {
        let mut sys = self.sys.lock().await;
        sys.refresh_process(self.pid);
        sys.refresh_memory(); // Add system memory refresh

        if let Some(process) = sys.process(self.pid) {
            let memory_mb = process.memory() / 1024 / 1024;
            let cpu_usage = process.cpu_usage();

            // System-wide stats
            let total_ram_gb = sys.total_memory() / 1024 / 1024 / 1024;
            let available_ram_gb = sys.available_memory() / 1024 / 1024 / 1024;

            Some(ProcessStats {
                memory_mb,
                cpu_usage,
                total_ram_gb,
                available_ram_gb,
            })
        } else {
            None
        }
    }

    pub async fn get_memory_pressure(&self) -> MemoryPressure {
        let mut sys = self.sys.lock().await;
        sys.refresh_memory();

        let total = sys.total_memory();
        if total == 0 {
            return MemoryPressure::Normal;
        }

        let used = total - sys.available_memory();
        let usage_pct = (used as f64 / total as f64) * 100.0;

        if usage_pct > 90.0 {
            MemoryPressure::Critical
        } else if usage_pct > 80.0 {
            MemoryPressure::Warning
        } else {
            MemoryPressure::Normal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_stats() {
        let monitor = PerformanceMonitor::new();
        let stats = monitor.get_stats().await;

        assert!(stats.is_some());
        let s = stats.unwrap();

        // Basic sanity checks
        assert!(s.total_ram_gb > 0);
        assert!(s.memory_mb > 0);
        println!(
            "Memory: {}MB, CPU: {}%, Total RAM: {}GB",
            s.memory_mb, s.cpu_usage, s.total_ram_gb
        );
    }
}
