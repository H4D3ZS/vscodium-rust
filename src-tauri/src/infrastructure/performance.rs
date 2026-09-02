use serde::Serialize;

use crate::architecture::domain::performance::ProcessMemorySnapshot;
use crate::architecture::infrastructure::performance::SysinfoProcessMemoryRepository;
use crate::architecture::domain::performance::ProcessMemoryRepository;

/// Legacy DTO kept for older call sites. Prefer `ProcessMemorySnapshot`.
#[derive(Serialize, Clone)]
pub struct ProcessStats {
    /// Total working set of host + WebView2/child processes (honest RAM).
    pub memory_mb: u64,
    pub cpu_usage: f32,
    pub total_ram_gb: u64,
    pub available_ram_gb: u64,
    /// Full snapshot for status-bar tooltip / diagnostics.
    pub snapshot: ProcessMemorySnapshot,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum MemoryPressure {
    Normal,
    Warning,
    Critical,
}

#[derive(Serialize, Clone, Debug)]
pub struct InferenceStats {
    pub device: String,
    pub latency_ms: u64,
    pub timestamp: u64,
}

pub struct PerformanceMonitor {
    memory_repo: SysinfoProcessMemoryRepository,
    root_pid: u32,
    inference_history: tokio::sync::Mutex<Vec<InferenceStats>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            memory_repo: SysinfoProcessMemoryRepository::new(),
            root_pid: std::process::id(),
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

        if history.len() > 100 {
            history.remove(0);
        }
    }

    pub async fn get_inference_history(&self) -> Vec<InferenceStats> {
        self.inference_history.lock().await.clone()
    }

    pub async fn get_stats(&self) -> Option<ProcessStats> {
        let snapshot = self.memory_repo.capture_snapshot(self.root_pid)?;
        Some(ProcessStats {
            memory_mb: snapshot.legacy_memory_mb(),
            cpu_usage: snapshot.cpu_usage_percent,
            total_ram_gb: snapshot.system_total_ram_gb,
            available_ram_gb: snapshot.system_available_ram_gb,
            snapshot,
        })
    }

    pub async fn get_memory_pressure(&self) -> MemoryPressure {
        // System-wide pressure — independent of IDE tree size.
        let snapshot = match self.memory_repo.capture_snapshot(self.root_pid) {
            Some(s) => s,
            None => return MemoryPressure::Normal,
        };
        let total_gb = snapshot.system_total_ram_gb;
        if total_gb == 0 {
            return MemoryPressure::Normal;
        }
        let used_gb = total_gb.saturating_sub(snapshot.system_available_ram_gb);
        let usage_pct = (used_gb as f64 / total_gb as f64) * 100.0;

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
    async fn test_performance_stats_tree() {
        let monitor = PerformanceMonitor::new();
        let stats = monitor.get_stats().await.expect("stats");
        assert!(stats.memory_mb > 0);
        assert!(stats.snapshot.total_working_set_mb >= stats.snapshot.host_working_set_mb);
        eprintln!(
            "host={}MB children={}MB total={}MB private={}MB ({} children)",
            stats.snapshot.host_working_set_mb,
            stats.snapshot.child_working_set_mb,
            stats.snapshot.total_working_set_mb,
            stats.snapshot.total_private_mb,
            stats.snapshot.child_process_count
        );
    }
}
