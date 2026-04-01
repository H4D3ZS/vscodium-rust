use serde::Serialize;
use std::sync::Mutex;
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

pub struct PerformanceMonitor {
    sys: Mutex<System>,
    pid: Pid,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        let sys = System::new(); // Don't refresh_all on main thread
        let pid = Pid::from(std::process::id() as usize);
        Self {
            sys: Mutex::new(sys),
            pid,
        }
    }

    pub fn get_stats(&self) -> Option<ProcessStats> {
        let mut sys = self.sys.lock().unwrap();
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

    pub fn get_memory_pressure(&self) -> MemoryPressure {
        let mut sys = self.sys.lock().unwrap();
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

    #[test]
    fn test_performance_stats() {
        let monitor = PerformanceMonitor::new();
        let stats = monitor.get_stats();

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
